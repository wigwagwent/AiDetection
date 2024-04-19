use crate::{tracking::TrackingResult, ImageProperties};
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
pub enum ProcessingStatus {
    NotStarted,
    Finished,
}

#[derive(Clone)]
pub struct ImageManager {
    pub image: DynamicImage,
    pub dehazed: Option<DynamicImage>,
    pub dehazed_status: ProcessingStatus,
    pub dehazed_time: Option<Duration>,
    pub detection_objects: Option<Vec<TrackingResult>>,
    pub detection_status: ProcessingStatus,
    pub detection_time: Option<Duration>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageInformation {
    pub image_props: ImageProperties,
    pub dehazed_status: ProcessingStatus,
    pub dehazed_time: Option<Duration>,
    pub detection_objects: Option<Vec<TrackingResult>>,
    pub detection_status: ProcessingStatus,
    pub detection_time: Option<Duration>,
}

impl ImageInformation {
    pub fn new(img: &ImageManager, img_id: usize) -> Self {
        Self {
            image_props: ImageProperties {
                origin_width: img.image.width(),
                origin_height: img.image.height(),
                img_id,
            },
            dehazed_status: img.dehazed_status,
            dehazed_time: img.dehazed_time,
            detection_objects: img.detection_objects.clone(),
            detection_status: img.detection_status,
            detection_time: img.detection_time,
        }
    }
}
