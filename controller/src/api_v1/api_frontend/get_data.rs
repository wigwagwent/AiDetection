use std::{convert::Infallible, sync::Arc};

use dashmap::DashMap;
use shared_types::server::{ImageInformation, ImageManager};
use warp::reply::{self, Reply, Response};

use crate::{
    api_v1::api_shared::api_helper::{file_not_found, get_latest_tracked_image_id},
    NEXT_IMAGE_ID,
};

pub async fn image_data_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
    image_id: &usize,
) -> Result<Response, Infallible> {
    let image = image_store.get(&image_id);

    let image: dashmap::mapref::one::Ref<'_, usize, ImageManager> = match image {
        Some(image) => image,
        None => return file_not_found(),
    };

    let image_information = ImageInformation::new(&image, image_id.clone());
    Ok(reply::json(&image_information).into_response())
}

pub async fn latest_image_data_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> Result<impl warp::Reply, Infallible> {
    image_data_get(
        image_store,
        &(NEXT_IMAGE_ID.load(std::sync::atomic::Ordering::Relaxed) - 1),
    )
    .await
}

pub async fn latest_tracking_data_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> Result<Response, Infallible> {
    let latest = match get_latest_tracked_image_id(image_store.clone()) {
        Some(latest) => latest,
        None => return file_not_found(),
    };

    image_data_get(image_store, &latest).await
}
