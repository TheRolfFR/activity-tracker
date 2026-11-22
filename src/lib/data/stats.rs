use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};
use chrono::{NaiveTime, TimeDelta};
use serde::{self, Deserialize, Serialize};

use crate::data::FIVE_MINUTES;
use crate::data::measure::MeasureCount;
use crate::data::records::DayRecord;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct DayActivityStat {
    pub from: NaiveTime,
    pub to: NaiveTime,
    pub count: MeasureCount
}

#[derive(Debug, Clone, Default)]
pub struct DurationStats {
    pub duration: TimeDelta,
    pub active_duration: TimeDelta
}

impl AddAssign for DurationStats {
    fn add_assign(&mut self, other: Self) {
        self.duration = self.duration + other.duration;
        self.active_duration = self.active_duration + other.active_duration;
    }
}

impl SubAssign for DurationStats {
    fn sub_assign(&mut self, other: Self) {
        self.duration = self.duration - other.duration;
        self.active_duration = self.active_duration - other.active_duration;
    }
}

#[derive(Debug, Clone)]
pub struct DayStats {
    pub activities: Vec<DayActivityStat>,
    pub last_activity: Option<DayActivityStat>,
    pub duration_stats: DurationStats
}

impl Default for DayStats {
    fn default() -> Self {
        Self {
            activities: vec![],
            last_activity: None,
            duration_stats: Default::default()
        }
    }
}

impl DayStats {
    pub(crate) fn update(&mut self, time: NaiveTime, count: MeasureCount) -> (DurationStats, DurationStats) {
        let before = self.duration_stats.clone();

        match self.last_activity.as_mut() {
            Some(last_act) if (time - last_act.to) >= FIVE_MINUTES => {
                self.activities.push(*last_act);
                self.duration_stats.active_duration += last_act.to - last_act.from;

                // Replace last activity
                *last_act = DayActivityStat {
                    from: time,
                    to: time,
                    count
                };
            },
            None => {
                // No active_duration change
                // Create last activity
                self.last_activity = Some(DayActivityStat {
                    from: time,
                    to: time,
                    count
                });
            }
            Some(last_act) => {
                // retrieve previous last activity duration
                self.duration_stats.active_duration -= last_act.to - last_act.from;

                last_act.to = time;
                last_act.count += count;

                // Add last activity duration
                self.duration_stats.active_duration += last_act.to - last_act.from;
            }
        }

        self.duration_stats.active_duration = self.update_duration();

        (before, self.duration_stats.clone())
    }

    fn update_duration(&self) -> TimeDelta {
        if self.last_activity.is_none() {
            return TimeDelta::zero();
        }

        // Find closest activity before 12h30
        let midday = NaiveTime::from_hms_opt(12, 30, 0).unwrap();
        let first_morning_act = self.activities.iter().find(|act| act.to <= midday);
        let last_morning_act = self.activities.iter().rev().find(|act| act.to <= midday);
        let morning_duration = match (first_morning_act, last_morning_act) {
            (Some(first), Some(last)) => last.to - first.from,
            _ => TimeDelta::zero()
        };

        let first_afternoon_act = self.activities.iter().find(|act| act.from > midday);
        let last_afternoon_act = self.activities.iter().rev().find(|act| act.from > midday);
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
        let mut measures: HashMap<NaiveTime, MeasureCount> = HashMap::new();

        //* count clicks
        for measure in clicks.iter().chain(inputs.iter()) {
            *measures.entry(measure.time).or_insert(0) += measure.count;
        }

        let mut measures_dates: Vec::<_> = measures.into_iter().collect();
        measures_dates.sort_by_key(|m| m.0);

        let mut day_stat = DayStats::default();
        for md in measures_dates {
            day_stat.update(md.0, md.1);
        }

        day_stat
    }
}
