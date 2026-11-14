use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::data::{measure::*, records::DayRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLabels {
    pub x: String,
    pub y: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySeries<T>
where
    T: Serialize,
{
    pub points: Vec<T>,
    pub labels: ActivityLabels
}

pub type AdjustedType = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub clicks_per_minute: f64,
    pub click_series: ActivitySeries<ClickMeasure>,

    pub inputs_per_minute: f64,
    pub input_series: ActivitySeries<InputMeasure>,

    pub adjusted: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityPeriod {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub level: Activity
}

impl From<& DayRecord> for Activity {
    fn from(day_record: & DayRecord) -> Self {
        let mut clicks = day_record.clicks.clone();
        let mut inputs = day_record.inputs.clone();

        clicks.sort_by(
            |a,b| b.date.cmp(&a.date)
        );
        let five_clicks = day_record.clicks.clone();
        let fold_clicks = five_clicks.iter().fold(
            (0u32,0u32),
            |acc, cur| (acc.0 + cur.count, acc.1 + 1)
        );

        inputs.sort_by(
            |a,b| b.date.cmp(&a.date)
        );
        let five_inputs = day_record.inputs.clone();
        let fold_inputs = five_inputs.iter().fold(
            (0u32,0u32),
            |acc, cur| (acc.0 + cur.count, acc.1 + 1)
        );

        Activity {
            clicks_per_minute: f64::from(fold_clicks.0) / f64::from(fold_clicks.1.max(1)),
            click_series: ActivitySeries {
                points: five_clicks,
                labels: ActivityLabels {
                    x: "clicks".to_string(),
                    y: "count".to_string()
                }
            },
            inputs_per_minute: f64::from(fold_inputs.0) / f64::from(fold_inputs.1.max(1)),
            input_series: ActivitySeries {
                points: five_inputs,
                labels: ActivityLabels {
                    x: "inputs".to_string(),
                    y: "count".to_string()
                }
            },
            adjusted: day_record.adjusted
        }
    }
}
