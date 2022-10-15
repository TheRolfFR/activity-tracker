use std::time::Duration;
use std::time::SystemTime;

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct Measure<T: serde::Serialize + Default + TS> {
    pub count: T,
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub date: DateTime<Utc>
}

impl<T> Measure<T>
where T: serde::Serialize + Default + TS {
    pub fn new() -> Self {
        Measure { count: T::default(), date: DateTime::<Utc>::from(SystemTime::now()) }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct Activity {
    pub clicks_per_minute: f32,
    pub click_series: ActivitySeries<Measure<usize>>,

    pub inputs_per_minute: f32,
    pub input_series: ActivitySeries<Measure<usize>>
}

pub const ONE_MINUTE: Duration = Duration::from_secs(60);
pub const ONE_SECOND: Duration = Duration::from_secs(1);
pub const FIVE_MINUTES: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct ActivityPeriod {
    #[serde(with = "ts_seconds")]
    pub start: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub end: Option<DateTime<Utc>>,
    pub level: Activity
}

pub const DAYS: usize = 5;
pub const MINUTES_PER_DAY: usize = 1440;
pub const WEEK_DURATION: Duration = Duration::from_secs(135000); // 37H30
