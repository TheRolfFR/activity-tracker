use std::collections::HashMap;

use chrono::{Local, Datelike, Duration};
use log::error;
use ts_rs::TS;

use crate::{day_record::{DayRecord, DayStats}, records::Record, week_stats::WeekStats, data::Activity, day_record_file::DayRecordFile};

pub struct WeekData {
    days: HashMap<u32, DayRecord>,
    last_dow: u32
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct DayPayload {
    activity: Activity,
    stats: DayStats,
    time: u64,
    adjusted: i32
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
pub struct Payload {
    activity: Activity,
    today: u32,
    today_stats: DayStats,
    week_stats: WeekStats,
    week_payload: HashMap<u32, DayPayload>,
    pub version: String
}

impl WeekData {
    pub fn today_index() -> u32 {
        Local::now().weekday().number_from_monday() - 1
    }

    fn get_today(&mut self) -> &mut DayRecord {
        let today_dow = Self::today_index();

        // new day here (I hope it will never happen)
        if self.last_dow != today_dow {
            if today_dow < self.last_dow {
                // new week
                self.days.clear();
            }

            // generally new day
            self.last_dow = today_dow;
            self.days.insert(today_dow, DayRecord::new());
        }

        if let Some(today) = self.days.get_mut(&today_dow) {
            today
        } else {
            error!("Failed to get today data");
            unreachable!("Failed to get today data");
        }
    }

    pub(crate) fn load() -> WeekData {
        // load previous days data and today data
        let today_dow = Self::today_index();

        let mut res = WeekData {
            days: HashMap::new(),
            last_dow: today_dow,
        };


        for dow in 0..=today_dow {
            res.days.insert(dow, DayRecordFile::load_activity_day(dow));
        }

        res
    }

    pub(crate) fn update_today(&mut self, record: Record) {
        let today = self.get_today();

        today.increase_click(record.clicks);
        today.increase_input(record.inputs);
    }

    /// Adjust time with desired value in minute
    pub(crate) fn adjust_today(&mut self, adjusted: i32) {
        let today = self.get_today();

        today.adjusted = adjusted;
    }

    pub(crate) fn to_payload(&mut self) -> Payload {
        let act_dur = Duration::minutes(5);
        let today = self.get_today();
        let act = today.get_activity();

        let today_stats = today.get_stats(act_dur);
        let dur_secs = today_stats.duration.as_secs();

        let week_payload = self.days.iter().fold(HashMap::new(), |mut acc, (day, data)| {
            let stats = data.get_stats(act_dur);
            let time = stats.duration.as_secs();

            let this_day = DayPayload {
                activity: data.get_activity(),
                adjusted: data.adjusted,
                stats,
                time 
            };

            acc.insert(day.clone(), this_day);

            return acc;
        });

        Payload {
            activity: act,
            today_stats,
            week_payload,
            today: (dur_secs/60u64) as u32,
            week_stats: WeekStats::from(self.days.values_mut()),
            version: String::new()
        }
    }

    pub(crate) fn save(&mut self) {
        let today = self.get_today().clone();
        DayRecordFile::save(today);
    }
}