use chrono::{Local, Datelike};

use crate::{data::records::{Record, DayRecord}, storage::DayRecordFile};

#[derive(Debug, Default)]
pub struct WeekData {
    pub(crate) days: [Option<DayRecord>; 7],
    pub(crate) last_dow: u32
}

impl WeekData {
    pub(crate) fn today_index() -> u32 {
        Local::now().weekday().number_from_monday() - 1
    }

    pub(crate) fn update(&mut self) {
        let today_dow = Self::today_index();

        // new day here (I hope it will never happen)
        if self.last_dow != today_dow && today_dow < self.last_dow {
            for d in self.days.iter_mut() { *d = None }
        }

        self.last_dow = today_dow;
    }

    pub(crate) fn today(&self) -> &Option<DayRecord> {
        let today_dow = Self::today_index();
        &self.days[today_dow as usize]
    }

    pub (crate) fn today_edit<F>(&mut self, func: F)
    where F: FnOnce(&mut DayRecord) {
        self.update();
        let today_dow = Self::today_index();
        let today_opt = &mut self.days[today_dow as usize];

        match today_opt {
            Some(today) => {
                func(today);
            },
            None => {
                let mut today = DayRecord::default();
                func(&mut today);
                *today_opt = Some(today)
            }
        }
    }

    pub(crate) fn load() -> WeekData {
        // load previous days data and today data
        let mut res = WeekData::default();

        let today_dow = Self::today_index();
        for dow in 0..=today_dow {
            res.days[dow as usize] = DayRecordFile::load_activity_day(dow);
        }

        res
    }

    pub(crate) fn update_today(&mut self, record: Record) {
        self.today_edit(|today| {
            today.increase_click(record.clicks);
            today.increase_input(record.inputs);
        });
    }

    /// Adjust time with desired value in minute
    pub(crate) fn adjust_today(&mut self, adjusted: i32) {
        self.today_edit(|today| {
            today.adjusted = adjusted;
        });
    }
}
