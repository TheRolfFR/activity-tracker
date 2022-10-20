use std::{vec, collections::HashMap, ops::AddAssign};
use crate::data::{ClickMeasure, InputMeasure, AdjustedType, MeasureDate, Measure, Activity, ActivitySeries, MeasureCount, ActivityLabels};

use chrono::{serde::ts_seconds, Duration, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, TS)]
#[ts(export)]
pub struct DayActivityStat {
    pub from: MeasureDate,
    pub to: MeasureDate,
    pub count: MeasureCount
}

#[derive(Debug, Clone, TS)]
#[ts(export)]
pub struct DayStats {
    pub activities: Vec<DayActivityStat>,
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

        let opt_last = vec.last_mut()
        .and_then(|o| {
            let entry_date = &o.date.timestamp()/60;

            if entry_date == minutes {
                Some(o)
            } else {
                None
            }
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
    pub fn get_stats(&self, act_duration: Duration) -> DayStats {
        let mut res = DayStats {
            activities: vec![],
            duration: Duration::zero()
        };

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

        let mut opt_act: Option<DayActivityStat> = None;
        for cur_date in dates.iter() {
            let opt_count = measures.get(&cur_date);

            match (opt_count, opt_act.as_mut()) {
                (None, _) => {},
                (Some(cur_count), Some(act)) => {
                    if cur_date.signed_duration_since(act.to) <= act_duration {
                        act.to = *cur_date;
                        act.count += cur_count;
                    } else {
                        act.to = *cur_date;
                        res.activities.push(act.clone());
                        let dur = act.to.signed_duration_since(act.from);
                        res.duration = res.duration + dur;

                        opt_act = None;
                    }
                },
                (Some(cur_count), None) =>{
                    opt_act = Some(DayActivityStat {
                        from: *cur_date,
                        to: *cur_date,
                        count: *cur_count
                    });
                },
            }
        }

        if let Some(act) = opt_act {
            res.duration = res.duration + act.to.signed_duration_since(act.from).min(act_duration);
        }

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