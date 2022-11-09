use std::{fs::File, path::PathBuf, io::Read};

use chrono::Utc;

use crate::{day_record::DayRecord, week_data::WeekData};

pub struct DayRecordFile;

impl DayRecordFile {
    fn today_index() -> u32 {
        WeekData::today_index()
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
            if let Ok(val) = serde_json::from_str::<DayRecord>(&buff) {
                if Utc::now().signed_duration_since(val.date).num_hours() >= 6*24 {
                    DayRecord::new()
                } else {
                    val
                }
            } else {
                dbg!(&buff);
                panic!("Panicked while loading {day} activity");
            }
        } else {
            DayRecord::new()
        }
    }

    pub fn save(day_record: DayRecord) {
        let path = Self::path_activity_today();
        std::fs::write(
            path,
            serde_json::to_string_pretty(&day_record).unwrap(),
        ).unwrap();
    }
}