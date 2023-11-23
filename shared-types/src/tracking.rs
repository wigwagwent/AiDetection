use serde::{Deserialize, Serialize};

use self::yolo::{YoloClasses80, YoloClassesOIV7};

pub mod yolo;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy)]
pub struct TrackingResult {
    pub x_bottom_corner: i32,
    pub y_bottom_corner: i32,
    pub x_length: u32,
    pub y_height: u32,
    pub label: ItemLabel,
    pub probability: f32,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum ItemLabel {
    YoloClasses80(YoloClasses80),
    YoloClassesOIV7(YoloClassesOIV7),
}
