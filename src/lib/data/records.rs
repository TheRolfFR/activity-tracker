use serde::{Serialize, Deserialize};

use crate::data::measure::MeasureCount;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Record {
    pub clicks: MeasureCount,
    pub inputs: MeasureCount
}
