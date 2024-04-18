use image::Rgb;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageBuffer};
use std::convert::Infallible;

use crate::api_v1::api_shared::api_helper::file_not_found;
use crate::api_v1::api_shared::api_helper::send_image;
use crate::ImageStore;

pub async fn image_get(
    image_store: ImageStore,
    image_id: &usize,
) -> Result<impl warp::Reply, Infallible> {
    let image = image_store.get(image_id);

    if let Some(image) = image {
        send_image(&resize_and_pad(&image.image).await).await
    } else {
        file_not_found()
    }
}

async fn resize_and_pad(image: &DynamicImage) -> DynamicImage {
    let image = {
        if image.width() >= 640 && image.height() >= 640 {
            image.resize(640, 640, FilterType::Lanczos3)
        } else {
            image.clone()
        }
    };

    let (new_width, new_height) = image.dimensions();

    if new_width == 640 && new_height == 640 {
        return image;
    }

    let mut padded_image = ImageBuffer::from_pixel(640, 640, Rgb([0, 0, 0]));

    for x in 0..new_width {
        for y in 0..new_height {
            let pixel = image.get_pixel(x, y);
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
