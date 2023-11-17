use serde::{Deserialize, Serialize};

use self::yolo::{YoloClasses80, YoloClassesOIV7};

pub mod yolo;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy)]
pub struct TrackingResult {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub label: ItemLabel,
    pub probablility: f32,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy)]
pub enum ItemLabel {
    YoloClasses80(YoloClasses80),
    YoloClassesOIV7(YoloClassesOIV7),
}
