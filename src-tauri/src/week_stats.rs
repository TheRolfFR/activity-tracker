use std::collections::hash_map::Values;
use chrono::Duration;
use ts_rs::TS;

use crate::day_record::DayRecord;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct WeekStats {
    pub done: u32
}

impl<'a> From<Values<'a, u32, DayRecord>> for WeekStats {
    fn from(values: Values<u32,DayRecord>) -> Self {
        let act_dur = Duration::minutes(5);
        let stats = values.into_iter().fold(WeekStats {
            done: 0
        }, |mut acc,cur| {
            let stats = cur.get_stats(act_dur);
            let secs = stats.duration.as_secs();
            acc.done += secs as u32;
            acc
        });

        stats
    }
}