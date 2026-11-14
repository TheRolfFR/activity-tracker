use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::data::measure::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLabels {
    pub x: String,
    pub y: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySeries<T>
where
    T: Serialize,
{
    pub points: Vec<T>,
    pub labels: ActivityLabels
}

pub type AdjustedType = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub clicks_per_minute: f64,
    pub click_series: ActivitySeries<ClickMeasure>,

    pub inputs_per_minute: f64,
    pub input_series: ActivitySeries<InputMeasure>,

    pub adjusted: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityPeriod {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub level: Activity
}
