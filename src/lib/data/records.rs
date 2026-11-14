use std::ops::AddAssign;

use serde::{Serialize, Deserialize};
use chrono::{serde::ts_seconds, Utc};

use crate::data::measure::*;
use crate::data::activity::AdjustedType;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Record {
    pub clicks: MeasureCount,
    pub inputs: MeasureCount
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DayRecord {
    #[serde(with = "ts_seconds")]
    pub date: MeasureDate,
    pub clicks: Vec<ClickMeasure>,
    pub inputs: Vec<InputMeasure>,
    pub adjusted: AdjustedType
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
    where T: Serialize + Default + AddAssign {
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
}
