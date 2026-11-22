use chrono::{DateTime, NaiveTime, Utc};
use serde::{self, Deserialize, Serialize};

pub type MeasureCount = u32;
pub type MeasureDate = DateTime<Utc>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measure
{
    pub count: MeasureCount,
    pub time: NaiveTime
}
