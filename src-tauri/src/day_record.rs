use std::{vec, collections::HashMap, ops::{AddAssign, Add}, time::Duration};
use crate::data::{ClickMeasure, InputMeasure, AdjustedType, MeasureDate, Measure, Activity, ActivitySeries, MeasureCount, ActivityLabels};

use chrono::{serde::ts_seconds, Utc};
use serde::{self, Deserialize, Serialize};
use serde_with::serde_as;
use sorted_vec::SortedSet;
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
    pub fn get_stats(&mut self, act_duration: chrono::Duration) -> DayStats {
        let mut res = DayStats {
            activities: vec![],
            duration: Duration::ZERO
        };

        self.clicks.sort_by(
            |a,b| b.date.cmp(&a.date)
        );
        self.inputs.sort_by(
            |a,b| b.date.cmp(&a.date)
        );

        // get all timestamps
        let mut measures: HashMap<MeasureDate, MeasureCount> = HashMap::new();
        let mut dates: SortedSet <MeasureDate> = SortedSet::new();

        for measure in self.clicks.iter() {
            dates.find_or_insert(measure.date);
            let possible_count = measures.get_mut(&measure.date);

            if let Some(count) = possible_count {
                *count += measure.count;
            } else {
                measures.insert(measure.date, measure.count);
            }
        }

        for measure in self.inputs.iter() {
            dates.find_or_insert(measure.date);
            let possible_count = measures.get_mut(&measure.date);

            if let Some(count) = possible_count {
                *count += measure.count;
            } else {
                measures.insert(measure.date, measure.count);
            }
        }

        let mut opt_last_activity: Option<DayActivityStat> = None;
        for cur_date in dates.iter() {
            let opt_count = measures.get(&cur_date);

            match (opt_count, opt_last_activity.as_mut()) {
                (None, None) => {},
                (None, Some(last_activity)) => {
                    dbg!(last_activity);
                },
                (Some(cur_count), Some(last_activity)) => {
                    let computed_duration = cur_date.signed_duration_since(last_activity.to);
                    if computed_duration <= act_duration {
                        last_activity.to = *cur_date;
                        last_activity.count += cur_count;
                    } else {
                        // close last activity
                        res.activities.push(last_activity.clone());
                        let dur: u64 = last_activity.to.signed_duration_since(last_activity.from).num_minutes().try_into().unwrap();
                        res.duration = res.duration.add(Duration::from_secs(dur*60));

                        // start new Activity
                        opt_last_activity = Some(DayActivityStat {
                            from: *cur_date,
                            to: *cur_date,
                            count: *cur_count
                        });
                    }
                },
                (Some(cur_count), None) =>{
                    opt_last_activity = Some(DayActivityStat {
                        from: *cur_date,
                        to: *cur_date,
                        count: *cur_count
                    });
                },
            }
        }

        if let Some(act) = opt_last_activity {
            let dur = act.to.signed_duration_since(act.from).min(act_duration);
            let dur_secs: u64 = dur.num_seconds().try_into().unwrap();
            res.duration = res.duration + Duration::from_secs(dur_secs*60);

            res.activities.push(act);
        }

        // dbg!(&dates);

        res
    }

    /// Creates act
    pub(crate) fn get_activity(&mut self) -> Activity {
        self.clicks.sort_by(
            |a,b| b.date.cmp(&a.date)
        );
        let five_clicks = self.clicks.clone();
        let fold_clicks = five_clicks.iter().fold(
            (0u32,0u32),
            |acc, cur| (acc.0 + cur.count, acc.1 + 1)
        );
        
        self.inputs.sort_by(
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