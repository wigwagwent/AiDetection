use image::DynamicImage;
use serde::{Deserialize, Serialize};
pub mod client;
pub mod server;
pub mod tracking;

#[derive(Serialize, Deserialize)]
pub enum Processing {
    Dehaze,
    ObjectDetection,
    Tracking,
}

#[derive(Serialize, Deserialize)]
pub struct HardwareMonitor {
    pub processing: Processing, //% cpu usage
    pub cpu_temp: f32,
    pub load: f32,
    pub runtime: u32, //ms
    pub uptime: u32,  //ms
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageProperties {
    pub width: u32,
    pub height: u32,
    pub img_buffer: Vec<u8>,
}

impl ImageProperties {
    pub fn new(input_image: DynamicImage) -> Self {
        ImageProperties {
            width: input_image.width(),
            height: input_image.height(),
            img_buffer: input_image.clone().into_bytes(),
        }
    }

    pub fn new_scaled(input_image: DynamicImage, width: u32, height: u32) -> Self {
        ImageProperties {
            width: input_image.width(),
            height: input_image.height(),
            img_buffer: input_image
                .resize_exact(width, height, image::imageops::FilterType::CatmullRom)
                .into_bytes(),
        }
    }
}
