use serde::{Deserialize, Serialize};

use crate::PanLaw;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Headphones {
    pub pan_law: PanLaw,
    pub max_itd_nanos: u32,
    pub distance_curve: DistanceCurve,
    pub distance_effect: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum DistanceCurve {
    Linear,
    Exponential,
    Sine,
    SquareRoot,
}
