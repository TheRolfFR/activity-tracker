use std::collections::HashMap;

use chrono::{Local, Datelike};
use log::error;

use crate::{day_record::DayRecord, data::records::Record, storage::DayRecordFile};

pub struct WeekData {
    pub(crate) days: HashMap<u32, DayRecord>,
    pub(crate) last_dow: u32
}

impl WeekData {
    pub fn today_index() -> u32 {
        Local::now().weekday().number_from_monday() - 1
    }

    pub(crate) fn get_today(&mut self) -> &mut DayRecord {
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
}
