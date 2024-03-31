use dashmap::DashMap;
use image::{DynamicImage, ImageFormat};
use shared_types::server::{ImageInformation, ImageManager, ProcessingStatus};
use std::{convert::Infallible, io::Cursor, sync::Arc};
use warp::reply::{self, Reply, Response};

use crate::controller_helper::markup_image::add_tracking_data_to_image;
use crate::NEXT_IMAGE_ID;

pub async fn get_latest_img(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> Result<impl warp::Reply, Infallible> {
    get_img_with_id(
        image_store,
        &(NEXT_IMAGE_ID.load(std::sync::atomic::Ordering::Relaxed) - 1),
    )
    .await
}

pub async fn get_latest_tracked_image(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> Result<impl warp::Reply, Infallible> {
    let latest = match get_latest_tracked_image_id(image_store.clone()) {
        Some(latest) => latest,
        None => return file_not_found(),
    };

    let image = image_store.get(&latest);

    let image = match image {
        Some(image) => image,
        None => return file_not_found(),
    };

    let outlines = image.tracked.clone();

    let outlines = match outlines {
        Some(outlines) => outlines,
        None => {
            println!("No tracked data available");
            return Ok(warp::http::StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    let tracked_image = add_tracking_data_to_image(&image.raw, outlines);
    send_image(&tracked_image).await
}

pub async fn get_latest_tracked_data(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> Result<impl warp::Reply, Infallible> {
    let latest = match get_latest_tracked_image_id(image_store.clone()) {
        Some(latest) => latest,
        None => return file_not_found(),
    };

    let image = image_store.get(&latest);

    let image: dashmap::mapref::one::Ref<'_, usize, ImageManager> = match image {
        Some(image) => image,
        None => return file_not_found(),
    };

    let image_information = ImageInformation::new(&image);
    Ok(reply::json(&image_information).into_response())
}

fn get_latest_tracked_image_id(image_store: Arc<DashMap<usize, ImageManager>>) -> Option<usize> {
    let mut latest_processed: Option<usize> = None;
    for image in image_store.iter() {
        if image.detection_status == ProcessingStatus::Finished
            && (latest_processed.is_none() || image.key() > &latest_processed.unwrap())
        {
            latest_processed = Some(image.key().clone());
        }
    }
    latest_processed
}

pub async fn get_img_with_id(
    image_store: Arc<DashMap<usize, ImageManager>>,
    image_id: &usize,
) -> Result<impl warp::Reply, Infallible> {
    let latest_image = image_store.get(image_id);

    if let Some(latest_image) = latest_image {
        send_image(&latest_image.raw).await
    } else {
        file_not_found()
    }
}

async fn send_image(image: &DynamicImage) -> Result<Response, Infallible> {
    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Jpeg)
        .expect("");

    Ok(warp::reply::with_header(
        bytes,
        "Content-Type",
        "image/jpeg",
    ))
    .map(Reply::into_response)
}

fn file_not_found() -> Result<Response, Infallible> {
    Ok(
        warp::reply::with_status("No image available", warp::http::StatusCode::NOT_FOUND)
            .into_response(),
    )
}
