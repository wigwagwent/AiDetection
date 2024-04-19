use image::{DynamicImage, ImageFormat};
use shared_types::server::ProcessingStatus;
use std::{convert::Infallible, io::Cursor};
use warp::{
    reply::{Reply, Response},
    Filter,
};

use crate::ImageStore;

pub fn get_latest_tracked_image_id(image_store: ImageStore) -> Option<usize> {
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

pub async fn send_image(image: &DynamicImage) -> Result<Response, Infallible> {
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

pub fn file_not_found() -> Result<Response, Infallible> {
    Ok(
        warp::reply::with_status("No image available", warp::http::StatusCode::NOT_FOUND)
            .into_response(),
    )
}

pub fn with_image_store(
    image_store: ImageStore,
) -> impl Filter<Extract = (ImageStore,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || image_store.clone())
}
