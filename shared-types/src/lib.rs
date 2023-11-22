use image::DynamicImage;
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
    pub resize_width: u32,
    pub resize_height: u32,
    pub img_buffer: Vec<u8>,
}

impl ImageProperties {
    pub fn new(input_image: DynamicImage, img_id: usize) -> Self {
        ImageProperties {
            img_id,
            origin_width: input_image.width(),
            origin_height: input_image.height(),
            resize_width: input_image.width(),
            resize_height: input_image.height(),
            img_buffer: input_image.clone().into_bytes(),
        }
    }

    pub fn new_scaled(
        input_image: DynamicImage,
        img_id: usize,
        new_width: u32,
        new_height: u32,
    ) -> Self {
        ImageProperties {
            img_id,
            origin_width: input_image.width(),
            origin_height: input_image.height(),
            resize_width: new_width,
            resize_height: new_height,
            img_buffer: input_image
                .resize_exact(
                    new_width,
                    new_height,
                    image::imageops::FilterType::CatmullRom,
                )
                .into_bytes(),
        }
    }
}
