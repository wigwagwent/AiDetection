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

#[derive(Clone, PartialEq, Eq)]
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
