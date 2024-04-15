use crate::tracking::TrackingResult;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub enum SentDataType {
    Image,
    ImageProperties,
}

#[derive(Serialize, Deserialize)]
pub struct SentData {
    pub data_type: SentDataType,
    pub raw_data: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
pub enum ProcessingStatus {
    NotStarted,
    Started,
    Finished,
    Error,
}

#[derive(Clone)]
pub struct ImageManager {
    pub image: DynamicImage,
    pub dehazed: Option<DynamicImage>,
    pub dehazed_status: ProcessingStatus,
    pub dehazed_time: Option<Duration>,
    pub tracked: Option<Vec<TrackingResult>>,
    pub tracked_status: ProcessingStatus,
    pub tracked_time: Option<Duration>,
    pub detection_status: ProcessingStatus,
    pub detection_time: Option<Duration>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageInformation {
    pub image_id: usize,
    pub dehazed_status: ProcessingStatus,
    pub dehazed_time: Option<Duration>,
    pub tracked: Option<Vec<TrackingResult>>,
    pub tracked_status: ProcessingStatus,
    pub tracked_time: Option<Duration>,
    pub detection_status: ProcessingStatus,
    pub detection_time: Option<Duration>,
}

impl ImageInformation {
    pub fn new(img: &ImageManager, img_id: usize) -> Self {
        Self {
            image_id: img_id,
            dehazed_status: img.dehazed_status,
            dehazed_time: img.dehazed_time,
            tracked: img.tracked.clone(),
            tracked_status: img.tracked_status,
            tracked_time: img.tracked_time,
            detection_status: img.detection_status,
            detection_time: img.detection_time,
        }
    }
}
