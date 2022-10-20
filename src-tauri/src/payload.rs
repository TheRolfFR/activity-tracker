use chrono::Duration;
use ts_rs::TS;

use crate::{data::Activity, week_stats::WeekStats, day_record_file::DayRecordFile};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
pub struct Payload {
    activity: Activity,
    today: u32,
    week_stats: WeekStats
}

impl Payload {
    pub fn new() -> Self {
        let act_dur = Duration::minutes(5);
        let mut day_rec = DayRecordFile::load_activity_today();
        let act = day_rec.get_activity();
        Self {
            activity: act,
            today: (day_rec.get_stats(act_dur).duration.num_seconds()/60i64) as u32,
            week_stats: WeekStats::now(act_dur)
        }
    }
}