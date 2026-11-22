use chrono::{Datelike, Local, NaiveTime, TimeDelta, Timelike};

use crate::data::{activity::{Activity, AdjustedType}, measure::MeasureDate, records::Record, stats::{DayStats, DurationStats}};

#[derive(Debug, Default, Clone)]
pub struct DayState {
    pub activity: Activity,
    pub stats: DayStats,
}

#[derive(Debug, Clone)]
pub struct ActivityState {
    week_days: [Option<DayState>; 7],
    week_stats: DurationStats,
    last_dow: u32
}

impl Default for ActivityState {
    fn default() -> Self {
        Self {
            week_days: Default::default(),
            week_stats: Default::default(),
            last_dow: Self::today_index()
        }
    }
}

impl ActivityState {
    pub fn date() -> MeasureDate {
        Local::now().to_utc()
    }
    pub(crate) fn time() -> NaiveTime {
        Self::date().time().with_second(0).unwrap()
    }
    pub(crate) fn today_index() -> u32 {
        Self::date().weekday().num_days_from_monday()
    }

    pub(crate) fn update(&mut self) {
        let today_dow = Self::today_index();

        // new day here (I hope it will never happen)
        if self.last_dow != today_dow && today_dow < self.last_dow {
            *self = Default::default();
        }
    }

    fn today_edit<F>(&mut self, func: F)
    where F: FnOnce(&mut DurationStats, &mut DayState) {
        // Update last dow and dau
        self.update();

        let today_dow = Self::today_index();
        let opt_today = &mut self.week_days[today_dow as usize];

        match opt_today {
            Some(today) => {
                func(&mut self.week_stats, today);
            },
            None => {
                let mut today = DayState::default();
                func(&mut self.week_stats, &mut today);
                *opt_today = Some(today)
            }
        };
    }

    pub fn update_today(&mut self, record: Record)
    {
        self.today_edit(|week_stats, day_state| {
            let time = Self::time();
            // Update today activity
            day_state.activity.update(time, record);
            // Update today stats
            let (b_act, a_act) = day_state.stats.update(time, record.clicks + record.inputs);

            // Update week stats
            *week_stats -= b_act;
            *week_stats += a_act;
        });
    }

    pub fn update_adjusted(&mut self, adjusted: AdjustedType) {
        self.today_edit(|week_stats, day_state| {
            let adjusted_before = day_state.activity.adjusted;
            let adjusted_diff = TimeDelta::seconds((adjusted - adjusted_before) as i64);
            day_state.activity.adjusted = adjusted;
            week_stats.active_duration += adjusted_diff;
            week_stats.duration += adjusted_diff;
        })
    }

    pub fn today(&self) -> Option<&DayState> {
        let today_dow = Self::today_index();
        self.week_days[today_dow as usize].as_ref()
    }
}
