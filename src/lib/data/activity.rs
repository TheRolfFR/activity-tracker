use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::data::{measure::{Measure, MeasureCount}, records::{DayRecord, Record}};

pub type AdjustedType = i32;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub clicks_per_minute: f64,
    pub clicks: Vec<Measure>,

    pub inputs_per_minute: f64,
    pub inputs: Vec<Measure>,

    pub adjusted: AdjustedType
}

impl From<& DayRecord> for Activity {
    fn from(day_record: & DayRecord) -> Self {
        let mut clicks = day_record.clicks.clone();
        let mut inputs = day_record.inputs.clone();

        clicks.sort_by(
            |a,b| b.time.cmp(&a.time)
        );
        let five_clicks = day_record.clicks.clone();
        let fold_clicks = five_clicks.iter().fold(
            (0u32,0u32),
            |acc, cur| (acc.0 + cur.count, acc.1 + 1)
        );

        inputs.sort_by(
            |a,b| b.time.cmp(&a.time)
        );
        let five_inputs = day_record.inputs.clone();
        let fold_inputs = five_inputs.iter().fold(
            (0u32,0u32),
            |acc, cur| (acc.0 + cur.count, acc.1 + 1)
        );

        Activity {
            clicks_per_minute: f64::from(fold_clicks.0) / f64::from(fold_clicks.1.max(1)),
            clicks,

            inputs_per_minute: f64::from(fold_inputs.0) / f64::from(fold_inputs.1.max(1)),
            inputs,

            adjusted: day_record.adjusted
        }
    }
}

impl Activity {
    fn update_measure(list: &mut Vec<Measure>, time: NaiveTime, count: MeasureCount) {
        let opt_item = list.iter_mut().find(|m| m.time == time);

        match opt_item {
            Some(item) => {
                item.count = count;
            },
            None => {
                list.push(Measure { count, time });
            }
        };
    }

    pub fn update(&mut self, time: NaiveTime, record: Record) {
        Self::update_measure(&mut self.clicks, time, record.clicks);
        Self::update_measure(&mut self.inputs, time, record.inputs);
    }
}
