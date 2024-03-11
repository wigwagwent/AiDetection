use std::{sync::atomic::AtomicBool, time::Duration};

use image::DynamicImage;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{tracking::TrackingResult, ProcessingType};

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

pub struct ClientData {
    pub link: mpsc::UnboundedSender<warp::filters::ws::Message>,
    pub client_busy: AtomicBool,
    pub client_type: Option<ProcessingType>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingStatus {
    NotStarted,
    Started,
    Finished,
    Error,
}

#[derive(Clone)]
pub struct ImageManager {
    pub raw: DynamicImage,
    pub dehazed: Option<DynamicImage>,
    pub dehazed_status: ProcessingStatus,
    pub dehazed_time: Option<Duration>,
    pub tracked: Option<Vec<TrackingResult>>,
    pub tracked_status: ProcessingStatus,
    pub tracked_time: Option<Duration>,
    pub detection_status: ProcessingStatus,
    //pub detection: Option<Vec<TrackingResult>>, Use tracked
    pub detection_time: Option<Duration>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageInformation {
    pub dehazed_status: ProcessingStatus,
    pub dehazed_time: Option<Duration>,
    pub tracked: Option<Vec<TrackingResult>>,
    pub tracked_status: ProcessingStatus,
    pub tracked_time: Option<Duration>,
    pub detection_status: ProcessingStatus,
    pub detection_time: Option<Duration>,
}

impl ImageInformation {
    pub fn new(img: &ImageManager) -> Self {
        Self {
            dehazed_status: img.dehazed_status.clone(),
            dehazed_time: img.dehazed_time,
            tracked: img.tracked.clone(),
            tracked_status: img.tracked_status.clone(),
            tracked_time: img.tracked_time,
            detection_status: img.detection_status.clone(),
            detection_time: img.detection_time,
        }
    }
}
