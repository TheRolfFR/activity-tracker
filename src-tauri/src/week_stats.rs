use chrono::Duration;
use ts_rs::TS;
use bounded_vec_deque::BoundedVecDeque;

use crate::{day_record_file::DayRecordFile, day_record::DayRecord, data::{DAYS, WEEK_DURATION}};

pub type WeekActivity = BoundedVecDeque::<DayRecord>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct WeekStats {
    pub total: u32,
    pub done: u32
}

impl WeekStats {
    pub fn week_activity() -> WeekActivity {
        let index = DayRecordFile::today_index().min(DAYS as u32);

        let mut res = WeekActivity::new(DAYS);
        for i in 0..=index {
            res.push_back(DayRecordFile::load_activity_day(i));
        }
        
        res
    }
    pub fn now(act_dur: Duration) -> WeekStats {
        let week_activity = Self::week_activity();
        
        let total_min = (WEEK_DURATION.as_secs()/60) as u32;
        let stats = week_activity.iter().fold(WeekStats {
            total: total_min,
            done: 0
        }, |mut acc,cur| {
            let stats = cur.get_stats(act_dur);
            let secs = stats.duration.num_seconds();
            acc.done += secs as u32;
            acc
        });

        stats
    }
}