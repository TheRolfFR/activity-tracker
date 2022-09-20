use std::time::Duration;
use std::time::SystemTime;

use bounded_vec_deque::BoundedVecDeque;
use chrono::{DateTime, Utc, serde::{ts_seconds, ts_seconds_option}};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityLabels {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivitySeries<T: serde::Serialize> {
    pub points: Vec<T>,
    pub labels: ActivityLabels
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Measure<T: serde::Serialize + Default> {
    pub count: T,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>
}

impl<T> Measure<T>
where T: serde::Serialize + Default {
    pub fn new() -> Self {
        Measure { count: T::default(), date: DateTime::<Utc>::from(SystemTime::now()) }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Activity {
    pub clicks_per_minute: f32,
    pub click_series: ActivitySeries<Measure<usize>>,

    pub inputs_per_minute: f32,
    pub input_series: ActivitySeries<Measure<usize>>
}

pub const ONE_MINUTE: Duration = Duration::from_secs(60);
pub const ONE_SECOND: Duration = Duration::from_secs(1);
pub const FIVE_MINUTES: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityPeriod {
    #[serde(with = "ts_seconds")]
    pub start: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub end: Option<DateTime<Utc>>,
    pub level: Activity
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DayActivity {
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub activities: Vec::<ActivityPeriod>,
    pub adjust_min: usize
}

impl DayActivity {
    pub(crate) fn update(_act: Activity) {
        return
    }

    pub(crate) fn stop() {
        return
    }
}

pub type WeekActivity = BoundedVecDeque::<DayActivity>;
pub const DAYS: usize = 5;