use shared_types::{server::ImageManager, ImageProperties};
use std::convert::Infallible;
use warp::reply::{self, Reply, Response};

use crate::{api_v1::api_shared::api_helper::file_not_found, ImageStore, NEXT_IMAGE_ID};

pub async fn image_data_get(
    image_store: ImageStore,
    image_id: &usize,
) -> Result<Response, Infallible> {
    let image = image_store.get(&image_id);

    let image: dashmap::mapref::one::Ref<'_, usize, ImageManager> = match image {
        Some(image) => image,
        None => return file_not_found(),
    };

    let image_information = ImageProperties::new(&image.image, image_id.clone());
    Ok(reply::json(&image_information).into_response())
}

pub async fn latest_image_data_get(
    image_store: ImageStore,
) -> Result<impl warp::Reply, Infallible> {
    image_data_get(
        image_store,
        &(NEXT_IMAGE_ID.load(std::sync::atomic::Ordering::Relaxed) - 1),
    )
    .await
}
