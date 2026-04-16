use serde::{Deserialize, Serialize};

use crate::PanLaw;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StereoMode {
    Directional,
    Positional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Stereo {
    pub pan_law: PanLaw,
    pub mode: StereoMode,
}
