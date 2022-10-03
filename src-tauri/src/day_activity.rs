use std::{fs::File, io::{Read}, path::PathBuf, time::Duration, cmp::min};

use bounded_vec_deque::BoundedVecDeque;
use chrono::{DateTime, Utc, serde::ts_seconds, Datelike, Local};
use ts_rs::TS;

use crate::data::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, TS)]
#[ts(export)]
pub struct DayActivity {
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub activities: Vec::<ActivityPeriod>,
    pub adjust_min: usize
}

impl DayActivity {
    fn new() -> Self {
        Self {
            date: Utc::now(),
            ..Default::default()
        }
    }
    fn today_index() -> u32 {
        Local::now().weekday().number_from_monday() - 1
    }
    fn path_activity_day(day_index: u32) -> PathBuf {
        std::env::temp_dir().join(format!("activity-tracker-{}.json", day_index))
    }
    fn path_activity_today() -> PathBuf {
        Self::path_activity_day(Self::today_index())
    }

    fn load_activity_day(day: u32) -> DayActivity {
        let path = Self::path_activity_day(day);
        let file: Option<File> = File::open(path).ok();
        let mut buff = String::new();

        if let Some(mut f) = file {
            f.read_to_string(&mut buff).unwrap();
            serde_json::from_str(&buff).unwrap()
        } else {
            DayActivity::new()
        }
    }
    pub(crate) fn load_activity_today() -> DayActivity {
        Self::load_activity_day(Self::today_index())
    }

    pub(crate) fn update(act: Activity) {
        let mut day_act = Self::load_activity_today();

        // get last one or create

        let opt_last_act = day_act.activities.last_mut();

        // gives Some if end None
        // gives None else
        let opt_last = opt_last_act.map(|a| a.end.and(None).unwrap_or(a) );

        if let Some(last) = opt_last {
            last.level = act;
        } else {
            day_act.activities.push(ActivityPeriod {
                start: Utc::now(),
                end: None,
                level: act
            });
        }

        // if no last one, add it, else replace
        Self::save(day_act);
    }

    pub(crate) fn stop() {
        let mut day_act = Self::load_activity_today();

        let opt_last_act = day_act.activities.last_mut();

        if let Some(last_act) = opt_last_act {
            last_act.end = Some(Utc::now());
            Self::save(day_act);
        } else {
            // do nothing, not supposed to happend
        }
    }

    fn save(day_act: DayActivity) {
        let path = Self::path_activity_today();
        std::fs::write(
            path,
            serde_json::to_string_pretty(&day_act).unwrap(),
        ).unwrap();
    }

    pub(crate) fn get_last_activity(self) -> (Vec<Measure<usize>>,Vec<Measure<usize>>) {
        let last_activity = self.activities.last();

        if let Some(period) = last_activity {
            let act = period.level.clone();
            (act.click_series.points, act.input_series.points)
        } else {
            (vec![], vec![])
        }
    }
}

pub type WeekActivity = BoundedVecDeque::<DayActivity>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct WeekStats {
    pub total: u64,
    pub done: u64,
    pub left: u64
}

impl WeekStats {
    pub fn week_activity() -> WeekActivity {
        let index = min(DayActivity::today_index(), DAYS as u32);

        let mut res = WeekActivity::new(DAYS);
        for i in 0..=index {
            res.push_back(DayActivity::load_activity_day(i));
        }
        
        res
    }
    pub fn now() -> WeekStats {
        let week_activity = Self::week_activity();
        
        let total_secs = WEEK_DURATION.as_secs();
        week_activity.iter().fold(WeekStats {
            total: total_secs,
            done: 0,
            left: total_secs
        }, move |mut acc,cur| {
            let day_sum = cur.activities.iter().fold(Duration::ZERO, |mut sum, act| {
                if let Some(end) = act.end {
                    sum += Duration::from_secs(
                        (end - act.start).num_seconds().try_into().unwrap_or_default()
                    );
                }

                return sum;
            });

            let day_sum_secs = day_sum.as_secs();
            acc.done += day_sum_secs;
            acc.left -= day_sum_secs;

            return acc
        })
    }
}