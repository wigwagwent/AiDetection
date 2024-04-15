use dashmap::DashMap;
use image::Rgb;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageBuffer};
use shared_types::server::ImageManager;
use std::{convert::Infallible, sync::Arc};

use crate::api_v1::api_shared::api_helper::file_not_found;
use crate::api_v1::api_shared::api_helper::send_image;

pub async fn image_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
    image_id: &usize,
) -> Result<impl warp::Reply, Infallible> {
    let image = image_store.get(image_id);

    if let Some(image) = image {
        send_image(&resize_and_pad(&image.image)).await
    } else {
        file_not_found()
    }
}

// Slow until in release mode
fn resize_and_pad2(image: &DynamicImage) -> DynamicImage {
    let resized_image = image.resize(640, 640, FilterType::Lanczos3);
    let (new_width, new_height) = resized_image.dimensions();

    let mut padded_image = ImageBuffer::new(640, 640);
    for x in 0..new_width {
        for y in 0..new_height {
            let pixel = resized_image.get_pixel(x, y);
            padded_image.put_pixel(x, y, pixel);
        }
    }

    let img = DynamicImage::from(padded_image);
    img
}

fn resize_and_pad(image: &DynamicImage) -> DynamicImage {
    let resized_image = image.resize(640, 640, FilterType::Lanczos3);
    let (new_width, new_height) = resized_image.dimensions();
    let mut padded_image = ImageBuffer::from_pixel(640, 640, Rgb([0, 0, 0]));

    for x in 0..new_width {
        for y in 0..new_height {
            let pixel = resized_image.get_pixel(x, y);
            let rgb_pixel = [
                pixel[0], // Red channel
                pixel[1], // Green channel
                pixel[2], // Blue channel
            ];
            padded_image.put_pixel(x, y, Rgb(rgb_pixel));
        }
    }
    let img = DynamicImage::ImageRgb8(padded_image);
    img
}
