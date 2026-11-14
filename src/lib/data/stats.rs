use std::collections::HashMap;
use serde::{self, Deserialize, Serialize};
use chrono::{serde::ts_seconds};
use serde_with::serde_as;
use std::time::Duration;

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
                let secs = stats.duration.as_secs();
                acc.done += secs;
            }
            acc
        });

        stats
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayActivityStat {
    #[serde(with = "ts_seconds")]
    pub from: MeasureDate,
    #[serde(with = "ts_seconds")]
    pub to: MeasureDate,
    pub count: MeasureCount
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_as]
pub struct DayStats {
    pub activities: Vec<DayActivityStat>,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub duration: Duration
}

impl From<&DayRecord> for DayStats {
    fn from(day_record: & DayRecord) -> Self {
        let clicks = day_record.clicks.clone();
        let inputs = day_record.inputs.clone();

        // get all timestamps
        let mut measures: HashMap<MeasureDate, MeasureCount> = HashMap::new();

        //* count clicks
        for measure in clicks.iter() {
            let possible_count = measures.get_mut(&measure.date);

            if let Some(count) = possible_count {
                *count += measure.count;
            } else {
                measures.insert(measure.date, measure.count);
            }
        }

        //* count inputs
        for measure in inputs.iter() {
            let possible_count = measures.get_mut(&measure.date);

            if let Some(count) = possible_count {
                *count += measure.count;
            } else {
                measures.insert(measure.date, measure.count);
            }
        }

        let mut measures_dates: Vec::<(MeasureDate, MeasureCount)> = measures.into_iter().collect();
        measures_dates.sort_by(|a,b| a.0.cmp(&b.0));

        //* create activities
        let mut measures_dates_iter = measures_dates.into_iter();
        let opt_last_date = measures_dates_iter.next();
        let activities: Vec<DayActivityStat> = match opt_last_date {
            None => {
                vec![] // zero
            },
            Some((last_date, last_count)) => {
                let mut act_vec = vec![];
                let mut from:MeasureDate = last_date.clone();
                let mut to: MeasureDate = last_date.clone();
                let mut count: MeasureCount = last_count;

                // two or plus
                for (cur_date, cur_count) in measures_dates_iter {
                    if cur_date - to <= FIVE_MINUTES
                    {
                        to = cur_date;
                        count += cur_count;
                    }
                    else
                    {
                        act_vec.push(DayActivityStat {
                            from,
                            to,
                            count
                        });
                        from = cur_date;
                        to = cur_date;
                        count = cur_count;
                    }
                }

                // one or plus
                act_vec.push(DayActivityStat {
                    from,
                    to,
                    count
                });

                act_vec
            }
        };

        let mut duration: std::time::Duration = std::time::Duration::ZERO;
        for activity in activities.iter() {
            let act_duration_secs = (activity.to - activity.from).num_seconds();
            let us_act_duration_secs = u64::try_from(act_duration_secs).expect("activity.from > activity.to");
            duration += std::time::Duration::from_secs(us_act_duration_secs);
        }

        Self {
            activities,
            duration
        }
    }
}
