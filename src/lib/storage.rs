use std::{fs::File, path::PathBuf, io::Read};

use chrono::Utc;

use crate::{data::records::DayRecord, week_data::WeekData};

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

    pub fn load_activity_day(day: u32) -> Option<DayRecord> {
        let path = Self::path_activity_day(day);
        let file: Option<File> = File::open(path).ok();

        file.and_then(|mut f| {
            let mut buff = String::new();
            f.read_to_string(&mut buff).unwrap();

            match serde_json::from_str::<DayRecord>(&buff) {
                Ok(val) => Some(val),
                Err(err) => {
                    eprintln!("{}", &buff);
                    eprintln!("{err}");
                    panic!("Panicked while loading {day} activity");
                }
            }
        }).and_then(|day_record| {
            if Utc::now().signed_duration_since(day_record.date).num_days() >= 6 {
                // Do not return record if older than 6 days
                None
            } else {
                Some(day_record)
            }
        })
    }

    pub fn save(day_record: &DayRecord) {
        let path = Self::path_activity_today();
        std::fs::write(
            path,
            serde_json::to_string_pretty(day_record).unwrap(),
        ).unwrap();
    }
}
