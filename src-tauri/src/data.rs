use std::time::Duration;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ActivityLabels {
    pub x: &'static str,
    pub y: &'static str,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ActivitySeries<T: serde::Serialize> {
    pub points: Vec<T>,
    pub labels: ActivityLabels
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Activity {
    pub clicks_per_minute: f32,
    pub click_series: ActivitySeries<usize>,

    pub inputs_per_minute: f32,
    pub input_series: ActivitySeries<usize>
}

pub const ONE_MINUTE: Duration = Duration::from_secs(60);
pub const ONE_SECOND: Duration = Duration::from_secs(1);

pub const CLICK_LABELS: ActivityLabels = ActivityLabels {
    x: "Clicks",
    y: "min"
};
pub const KEY_LABELS: ActivityLabels = ActivityLabels {
    x: "Inputs",
    y: "sec"
};