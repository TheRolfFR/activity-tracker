use std::collections::HashMap;

use crate::{day_record::DayStats, week_stats::WeekStats, data::activity::Activity, week_data::WeekData};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DayPayload {
    activity: Activity,
    stats: DayStats,
    time: u64,
    adjusted: i32
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payload {
    activity: Activity,
    today: u32,
    today_stats: DayStats,
    week_stats: WeekStats,
    week_payload: HashMap<u32, DayPayload>,
    pub version: String
}

impl From<&mut WeekData> for Payload {
    fn from(week_data: &mut WeekData) -> Self {
        let today = week_data.get_today();
        let act = today.get_activity();

        let today_stats = today.get_stats();
        let dur_secs = today_stats.duration.as_secs();

        let week_payload = week_data.days.iter().fold(HashMap::new(), |mut acc, (day, data)| {
            let stats = data.get_stats();
            let day_dur_secs = stats.duration.as_secs();
            let time = u64::div_ceil(day_dur_secs, 60);

            let this_day = DayPayload {
                activity: data.get_activity(),
                adjusted: data.adjusted,
                stats,
                time
            };

            acc.insert(*day, this_day);

            return acc;
        });

        Self {
            activity: act,
            today_stats,
            week_payload,
            today: (dur_secs/60u64) as u32,
            week_stats: WeekStats::from(week_data.days.values_mut()),
            version: String::new()
        }
    }
}
