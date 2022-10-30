use std::time::Duration;

use chrono::{DateTime, Utc, serde::{ts_seconds, ts_seconds_option}};
use ts_rs::TS;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct ActivityLabels {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct ActivitySeries<T: serde::Serialize + TS> {
    pub points: Vec<T>,
    pub labels: ActivityLabels
}

pub type MeasureCount = u32;
pub type MeasureDate = DateTime<Utc>;
pub type ClickMeasure = Measure<u32>;
pub type InputMeasure = Measure<u32>;
pub type AdjustedType = i32;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct Measure<T: serde::Serialize + Default + TS> {
    pub count: T,
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub date: MeasureDate
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
pub struct Activity {
    pub clicks_per_minute: f64,
    pub click_series: ActivitySeries<ClickMeasure>,

    pub inputs_per_minute: f64,
    pub input_series: ActivitySeries<InputMeasure>,

    pub adjusted: i32
}

pub const ONE_MINUTE: Duration = Duration::from_secs(60);
pub const ONE_SECOND: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct ActivityPeriod {
    #[serde(with = "ts_seconds")]
    pub start: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub end: Option<DateTime<Utc>>,
    pub level: Activity
}
