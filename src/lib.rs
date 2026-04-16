//! Serializable and deserializable data structures for configuring spatial
//! audio rendering modes in the xpans Ecosystem.

pub mod headphones;
pub mod mono;
pub mod stereo;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "mode", content = "config")]
pub enum RenderConfig {
    Mono(mono::Mono),
    Stereo(stereo::Stereo),
    Headphones(headphones::Headphones),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanLaw {
    Linear,
    SquareRoot,
    Sine,
}
