use serde::{Deserialize, Serialize};
pub mod client;
pub mod server;
pub mod tracking;

#[derive(Serialize, Deserialize)]
pub enum ProcessingType {
    Dehaze,
    ObjectDetection,
    Tracking,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageProperties {
    pub img_id: usize,
    pub origin_width: u32,
    pub origin_height: u32,
}
