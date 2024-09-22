use std::collections::hash_map::ValuesMut;
use ts_rs::TS;

use crate::day_record::DayRecord;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct WeekStats {
    #[ts(type = "number")]
    pub done: u64
}

impl<'a> From<ValuesMut<'a, u32, DayRecord>> for WeekStats {
    fn from(values: ValuesMut<u32,DayRecord>) -> Self {
        let stats = values.into_iter().fold(WeekStats {
            done: 0
        }, |mut acc,cur| {
            let stats = cur.get_stats();
            let secs = stats.duration.as_secs();
            acc.done += secs;
            acc
        });

        stats
    }
}