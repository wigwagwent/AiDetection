use std::cmp::{Ordering, Reverse};

use anyhow::Error;
use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, RgbImage};
use shared_types::server::ImageInformation;
use tokio::time::Duration;

use crate::ImageStore;

mod markup_image;

pub async fn load_image_task(controller_url: &str, image_store: ImageStore) {
    println!("Starting image loading task");
    loop {
        //let time = Duration::from_secs_f32(1.0 / 45.0);
        //tokio::time::sleep(time).await;

        let image_info = fetch_latest_image_info(controller_url).await;

        let image_info = match image_info {
            Ok(info) => info,
            Err(err) => {
                println!("Failed to fetch image information, error: {}", err);
                continue;
            }
        };

        let is_duplicate = image_store
            .lock()
            .await
            .iter()
            .any(|Reverse(existing_image)| {
                existing_image.props.image_props.img_id == image_info.image_props.img_id
            });
        if is_duplicate {
            continue;
        }

        let bytes = fetch_image(controller_url, image_info.image_props.img_id).await;
        if let Ok(bytes) = bytes {
            let image_buffer: RgbImage = ImageBuffer::from_raw(
                image_info.image_props.origin_width,
                image_info.image_props.origin_height,
                bytes.into(),
            )
            .unwrap();
            let image = DynamicImage::from(image_buffer);
            let marked_img =
                markup_image::add_tracking_data_to_image(&image, &image_info.detection_objects);

            let marked_bytes = Bytes::from(marked_img.to_rgb8().into_vec());

            let image = ImageRawData {
                image: marked_bytes,
                props: image_info,
            };
            image_store.lock().await.push(Reverse(image));
        } else {
            println!("Failed to fetch image, error: {}", bytes.err().unwrap());
        }
    }
}

async fn fetch_latest_image_info(controller_url: &str) -> Result<ImageInformation, Error> {
    let client = reqwest::Client::new();
    let image_info: ImageInformation = client
        .get(format!("{}/tracking-image-data", controller_url))
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(image_info)
}

async fn fetch_image(controller_url: &str, img_id: usize) -> Result<Bytes, Error> {
    let client = reqwest::Client::new();

    let image = client
        .get(format!("{}/image-tracked-rgb8/{}", controller_url, img_id))
        .header("Accept", "application/octet-stream")
        .send()
        .await?;

    if !image.status().is_success() {
        return Err(anyhow::Error::msg("Failed to fetch image from server"));
    }

    if let Some(content_type) = image.headers().get("content-type") {
        if !content_type
            .to_str()
            .unwrap()
            .starts_with("application/octet-stream")
        {
            return Err(anyhow::Error::msg("Invalid content type for image data"));
        }
    } else {
        return Err(anyhow::Error::msg(
            "No content type header found in response",
        ));
    }

    Ok(image.bytes().await?)
}

pub struct ImageRawData {
    pub image: Bytes,
    pub props: ImageInformation,
}

impl Ord for ImageRawData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.props
            .image_props
            .img_id
            .cmp(&other.props.image_props.img_id)
    }
}

impl PartialOrd for ImageRawData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ImageRawData {
    fn eq(&self, other: &Self) -> bool {
        self.props.image_props.img_id == other.props.image_props.img_id
    }
}

impl Eq for ImageRawData {
    // This is not used by the BinaryHeap, but is required by the trait
}
