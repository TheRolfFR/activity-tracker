use std::time::Duration;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ActivityLabels {
    pub x: String,
    pub y: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySeries<T>
where
    T: Serialize + TS,
{
    pub points: Vec<T>,
    pub labels: ActivityLabels
}

pub type MeasureCount = u32;
pub type MeasureDate = DateTime<Utc>;
pub type ClickMeasure = Measure<u32>;
pub type InputMeasure = Measure<u32>;
pub type AdjustedType = i32;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, concrete(T = u32))]
pub struct Measure<T>
where
    T: Serialize + Default + TS,
{
    pub count: T,
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub date: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub clicks_per_minute: f64,
    pub click_series: ActivitySeries<ClickMeasure>,

    pub inputs_per_minute: f64,
    pub input_series: ActivitySeries<InputMeasure>,

    pub adjusted: i32
}

pub const ONE_MINUTE: Duration = Duration::from_secs(60);
pub const ONE_SECOND: Duration = Duration::from_secs(1);
pub const FIVE_MINUTES: chrono::Duration = chrono::Duration::minutes(5);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityPeriod {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub level: Activity
}
