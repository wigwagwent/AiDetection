use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use shared_types::HardwareMonitor;
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
