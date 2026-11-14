use std::time::Duration;

pub mod activity;
pub mod measure;
pub mod payload;
pub mod records;
pub mod stats;

pub const ONE_MINUTE: Duration = Duration::from_secs(60);
pub const ONE_SECOND: Duration = Duration::from_secs(1);
pub const FIVE_MINUTES: chrono::Duration = chrono::Duration::minutes(5);
