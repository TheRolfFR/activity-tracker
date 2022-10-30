use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::data::MeasureCount;

#[derive(Debug, Clone, Copy, Default, TS, Serialize, Deserialize)]
pub struct Record {
    pub clicks: MeasureCount,
    pub inputs: MeasureCount
}