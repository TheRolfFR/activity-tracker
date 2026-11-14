use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measure<T>
where
    T: Serialize + Default ,
{
    pub count: T,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>
}

pub type MeasureCount = u32;
pub type MeasureDate = DateTime<Utc>;
pub type ClickMeasure = Measure<u32>;
pub type InputMeasure = Measure<u32>;
