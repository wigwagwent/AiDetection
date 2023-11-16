use dashmap::DashMap;
use image::ImageError;
use serde::{Deserialize, Serialize};
use shared_types::{HardwareMonitor, ImageProperties};
use std::{path::PathBuf, sync::Arc};

#[derive(Serialize, Deserialize)]
pub struct ImageManager {
    pub raw: PathBuf,
    pub dehazed: Option<PathBuf>,
    pub tracked: Option<PathBuf>,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HardwareMonitorKey {
    pub img_id: u32,
    pub mac: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ControllerInfo {
    pub images: Arc<DashMap<u32, ImageManager>>,
    pub monitor: Arc<DashMap<HardwareMonitorKey, HardwareMonitor>>,
}

// impl ControllerInfo {
//     pub fn new() -> ControllerInfo {
//         ControllerInfo {
//             images: Arc::new(DashMap::new()),
//             monitor: Arc::new(DashMap::new()),
//         }
//     }
// }

pub fn get_image_raw(path: PathBuf) -> Result<ImageProperties, ImageError> {
    let img = match image::open(&path) {
        Ok(image) => image,
        Err(error) => return Err(error),
    };
    Ok(ImageProperties::new(img))
}
