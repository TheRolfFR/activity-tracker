use std::{vec, collections::HashMap, ops::AddAssign, time::Duration};
use crate::data::{Activity, ActivityLabels, ActivitySeries, AdjustedType, ClickMeasure, InputMeasure, Measure, MeasureCount, MeasureDate, FIVE_MINUTES};

use chrono::{serde::ts_seconds, Utc};
use serde::{self, Deserialize, Serialize};
use serde_with::serde_as;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
pub struct DayRecord {
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub date: MeasureDate,
    pub clicks: Vec<ClickMeasure>,
    pub inputs: Vec<InputMeasure>,
    pub adjusted: AdjustedType
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DayActivityStat {
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub from: MeasureDate,
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub to: MeasureDate,
    pub count: MeasureCount
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[serde_as]
#[ts(export)]
pub struct DayStats {
    pub activities: Vec<DayActivityStat>,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    #[ts(type = "number")]
    pub duration: Duration
}

impl DayRecord {
    /// Creates new day record
    pub fn new() -> Self {
        Self {
            date: Utc::now(),
            ..Default::default()
        }
    }

    fn increase_or_insert<T>(vec: &mut Vec<Measure<T>>, val: T)
    where T: Serialize + Default + TS + AddAssign {
        let date = Utc::now();
        let minutes = date.timestamp()/60;

        let opt_last = vec.iter_mut().rev().find(|e| {
            minutes == e.date.timestamp()/60
        });

        if let Some(last) = opt_last {
            last.count += val;
        } else {
            vec.push(Measure::<T> {
                date: date,
                count: val
            });
        }
    }

    /// Increase latest click
    pub fn increase_click(&mut self, clicks: MeasureCount) {
        Self::increase_or_insert(&mut self.clicks, clicks);
    }

    /// Increase latest input
    pub fn increase_input(&mut self,inputs: MeasureCount) {
        Self::increase_or_insert(&mut self.inputs, inputs);
    }

    /// Computes stats from clicks and inputs
    pub fn get_stats(&self) -> DayStats {
        let clicks = self.clicks.clone();
        let inputs = self.inputs.clone();

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

        DayStats {
            activities,
            duration
        }
    }

    /// Creates act
    pub(crate) fn get_activity(&self) -> Activity {
        let mut clicks = self.clicks.clone();
        let mut inputs = self.inputs.clone();

        clicks.sort_by(
            |a,b| b.date.cmp(&a.date)
        );
        let five_clicks = self.clicks.clone();
        let fold_clicks = five_clicks.iter().fold(
            (0u32,0u32),
            |acc, cur| (acc.0 + cur.count, acc.1 + 1)
        );

        inputs.sort_by(
            |a,b| b.date.cmp(&a.date)
        );
        let five_inputs = self.inputs.clone();
        let fold_inputs = five_inputs.iter().fold(
            (0u32,0u32),
            |acc, cur| (acc.0 + cur.count, acc.1 + 1)
        );

        Activity {
            clicks_per_minute: f64::from(fold_clicks.0) / f64::from(fold_clicks.1.max(1)),
            click_series: ActivitySeries {
                points: five_clicks,
                labels: ActivityLabels {
                    x: "clicks".to_string(),
                    y: "count".to_string()
                }
            },
            inputs_per_minute: f64::from(fold_inputs.0) / f64::from(fold_inputs.1.max(1)),
            input_series: ActivitySeries {
                points: five_inputs,
                labels: ActivityLabels {
                    x: "inputs".to_string(),
                    y: "count".to_string()
                }
            },
            adjusted: self.adjusted
        }
    }
}