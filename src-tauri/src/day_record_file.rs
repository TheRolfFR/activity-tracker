use std::{fs::File, path::PathBuf, io::Read};

use chrono::{Local, Datelike};

use crate::{day_record::DayRecord, data::MeasureCount};

pub struct DayRecordFile;

impl DayRecordFile {
    pub fn today_index() -> u32 {
        Local::now().weekday().number_from_monday() - 1
    }
    fn path_activity_day(day_index: u32) -> PathBuf {
        std::env::temp_dir().join(format!("activity-tracker-{day_index}.json"))
    }
    fn path_activity_today() -> PathBuf {
        Self::path_activity_day(Self::today_index())
    }

    pub fn load_activity_day(day: u32) -> DayRecord {
        let path = Self::path_activity_day(day);
        let file: Option<File> = File::open(path).ok();
        let mut buff = String::new();

        if let Some(mut f) = file {
            f.read_to_string(&mut buff).unwrap();
            if let Ok(val) = serde_json::from_str(&buff) {
                val
            } else {
                dbg!(&buff);
                panic!("Panicked while loading {day} activity");
            }
        } else {
            DayRecord::new()
        }
    }
    pub(crate) fn load_activity_today() -> DayRecord {
        Self::load_activity_day(Self::today_index())
    }

    pub(crate) fn update(clicks: MeasureCount, inputs: MeasureCount) {
        let mut day_act = Self::load_activity_today();

        // get last one or create

        if clicks > 0 {
            day_act.increase_click(clicks);
        }

        if inputs > 0 {
            day_act.increase_input(inputs);
        }

        if clicks > 0 || inputs > 0 {
            Self::save(day_act);
        }
    }

    fn save(day_record: DayRecord) {
        let path = Self::path_activity_today();
        std::fs::write(
            path,
            serde_json::to_string_pretty(&day_record).unwrap(),
        ).unwrap();
    }
}