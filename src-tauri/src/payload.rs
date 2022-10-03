use ts_rs::TS;

use crate::{data::Activity, day_activity::WeekStats};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct Payload {
    activity: Activity,
    week_stats: WeekStats
}

impl Payload {
    pub fn new(act: Activity) -> Self {
        Self { activity: act, week_stats: WeekStats::now() }
    }
}