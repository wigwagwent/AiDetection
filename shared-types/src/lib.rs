use image::DynamicImage;
use serde::{Deserialize, Serialize};
pub mod client;
pub mod server;
pub mod tracking;

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageProperties {
    pub img_id: usize,
    pub origin_width: u32,
    pub origin_height: u32,
}

impl ImageProperties {
    pub fn new(image: &DynamicImage, img_id: usize) -> Self {
        Self {
            img_id,
            origin_width: image.width(),
            origin_height: image.height(),
        }
    }
}
