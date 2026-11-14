use crate::data::{activity::Activity, stats::{DayStats, WeekStats}, week_data::WeekData};

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
    week_payload: [Option<DayPayload>; 7],
    pub version: String
}

impl From<&WeekData> for Payload {
    fn from(week_data: &WeekData) -> Self {
        let today = week_data.today().clone().unwrap_or_default();
        let act = Activity::from(&today);

        let today_stats = DayStats::from(&today);
        let dur_secs = today_stats.duration.as_secs();

        let week_payload: [Option<DayPayload>; 7] = week_data.days.clone().map(|data_opt|
            data_opt.map(|data| {
                let stats = DayStats::from(&data);
                let day_dur_secs = stats.duration.as_secs();
                let time = u64::div_ceil(day_dur_secs, 60);

                DayPayload {
                    activity: Activity::from(&data),
                    adjusted: data.adjusted,
                    stats,
                    time
                }
            })
        );

        Self {
            activity: act,
            today_stats,
            week_payload,
            today: (dur_secs/60u64) as u32,
            week_stats: WeekStats::from(&week_data.days),
            version: String::new()
        }
    }
}
