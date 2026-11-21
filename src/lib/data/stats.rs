use std::collections::HashMap;
use chrono::{NaiveTime, TimeDelta};
use serde::{self, Deserialize, Serialize};
use chrono::{serde::ts_seconds};

use crate::data::FIVE_MINUTES;
use crate::data::measure::{MeasureCount, MeasureDate};
use crate::data::records::DayRecord;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WeekStats {
    pub done: u64
}

impl<'a> From<&[Option<DayRecord>; 7]> for WeekStats {
    fn from(values: &[Option<DayRecord>; 7]) -> Self {
        let stats = values.into_iter().fold(WeekStats {
            done: 0
        }, |mut acc,cur_opt| {
            if let Some(cur) = cur_opt {
                let stats = DayStats::from(cur);
                let secs = stats.duration.num_seconds() as u64;
                acc.done += secs;
            }
            acc
        });

        stats
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct DayActivityStat {
    #[serde(with = "ts_seconds")]
    pub from: MeasureDate,
    #[serde(with = "ts_seconds")]
    pub to: MeasureDate,
    pub count: MeasureCount
}

#[derive(Debug, Clone)]
pub struct DayStats {
    pub activities: Vec<DayActivityStat>,
    pub last_activity: Option<DayActivityStat>,
    pub duration: TimeDelta,
    pub active_duration: TimeDelta
}

impl Default for DayStats {
    fn default() -> Self {
        Self {
            activities: vec![],
            last_activity: None,
            duration: TimeDelta::zero(),
            active_duration: TimeDelta::zero()
        }
    }
}

impl DayStats {
    fn add(&mut self, (date, count): (MeasureDate, MeasureCount)) {
        match self.last_activity.as_mut() {
            Some(last_act) if (date - last_act.to) >= FIVE_MINUTES => {
                self.activities.push(*last_act);
                self.active_duration += last_act.to - last_act.from;

                // Replace last activity
                *last_act = DayActivityStat {
                    from: date,
                    to: date,
                    count
                };
            },
            None => {
                // No active_duration change
                // Create last activity
                self.last_activity = Some(DayActivityStat {
                    from: date,
                    to: date,
                    count
                });
            }
            Some(last_act) => {
                // retrieve previous last activity duration
                self.active_duration -= last_act.to - last_act.from;

                last_act.to = date;
                last_act.count += count;

                // Add last activity duration
                self.active_duration += last_act.to - last_act.from;
            }
        }

        self.update_duration();
    }

    fn update_duration(&self) -> TimeDelta {
        if self.last_activity.is_none() {
            return TimeDelta::zero();
        }

        // Find closest activity before 12h30
        let midday = NaiveTime::from_hms_opt(12, 30, 0).unwrap();
        let first_morning_act = self.activities.iter().find(|act| act.to.time() <= midday);
        let last_morning_act = self.activities.iter().rev().find(|act| act.to.time() <= midday);
        let morning_duration = match (first_morning_act, last_morning_act) {
            (Some(first), Some(last)) => last.to - first.from,
            _ => TimeDelta::zero()
        };

        let first_afternoon_act = self.activities.iter().find(|act| act.from.time() > midday);
        let last_afternoon_act = self.activities.iter().rev().find(|act| act.from.time() > midday);
        let afternoon_duration = match (first_afternoon_act, last_afternoon_act) {
            (Some(first), Some(last)) => last.to - first.from,
            _ => TimeDelta::zero()
        };

        morning_duration + afternoon_duration
    }
}

impl From<&DayRecord> for DayStats {
    fn from(day_record: & DayRecord) -> Self {
        let clicks = &day_record.clicks;
        let inputs = &day_record.inputs;

        // get all timestamps
        let mut measures: HashMap<MeasureDate, MeasureCount> = HashMap::new();

        //* count clicks
        for measure in clicks.iter().chain(inputs.iter()) {
            *measures.entry(measure.date).or_insert(0) += measure.count;
        }

        let mut measures_dates: Vec::<(MeasureDate, MeasureCount)> = measures.into_iter().collect();
        measures_dates.sort_by_key(|m| m.0);

        let mut day_stat = DayStats::default();
        for md in measures_dates {
            day_stat.add(md);
        }

        day_stat
    }
}
