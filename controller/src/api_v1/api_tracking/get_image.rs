use dashmap::DashMap;
use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
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

fn resize_and_pad(image: &DynamicImage) -> DynamicImage {
    let resized_image = image.resize(640, 640, FilterType::Lanczos3);
    let (new_width, new_height) = image.dimensions();

    let mut padded_image = ImageBuffer::new(640, 640);
    for x in 0..new_width {
        for y in 0..new_height {
            let pixel = resized_image.get_pixel(x, y);
            padded_image.put_pixel(x, y, pixel);
        }
    }

    let img = DynamicImage::from(padded_image);
    img.save("resize.jpg").unwrap();
    img
}
