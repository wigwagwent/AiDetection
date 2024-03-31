use serde::{Deserialize, Serialize};

use self::yolo::{YoloClasses80, YoloClassesOIV7};

pub mod yolo;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct TrackingResult {
    pub label: ItemLabel,
    pub confidence: f32,
    pub x0: i32,
    pub x1: i32,
    pub y0: i32,
    pub y1: i32,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum ItemLabel {
    YoloClasses80(YoloClasses80),
    YoloClassesOIV7(YoloClassesOIV7),
}

impl ItemLabel {
    pub fn as_string(&self) -> String {
        match self {
            ItemLabel::YoloClasses80(x) => x.to_string(),
            ItemLabel::YoloClassesOIV7(x) => x.to_string(),
        }
    }
}
