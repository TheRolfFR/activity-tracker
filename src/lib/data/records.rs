use serde::{Serialize, Deserialize};
use chrono::{serde::ts_seconds};

use crate::data::measure::*;
use crate::data::state::DayState;
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
    pub clicks: Vec<Measure>,
    pub inputs: Vec<Measure>,
    pub adjusted: AdjustedType
}

impl DayRecord {
    pub(crate) fn from_day_state(date: MeasureDate, value: &DayState) -> Self {
        Self {
            date,
            clicks: value.activity.clicks.clone(),
            inputs: value.activity.inputs.clone(),
            adjusted: value.activity.adjusted
        }
    }
}
