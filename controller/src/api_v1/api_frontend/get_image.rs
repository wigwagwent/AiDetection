use std::convert::Infallible;

use warp::reply::Reply;

use crate::api_v1::api_shared::api_helper::file_not_found;
use crate::ImageStore;
use crate::{
    api_v1::api_shared::api_helper::send_image,
    controller_helper::markup_image::add_tracking_data_to_image,
};

pub async fn image_get(
    image_store: ImageStore,
    image_id: &usize,
) -> Result<impl warp::Reply, Infallible> {
    let image = image_store.get(image_id);

    if let Some(image) = image {
        send_image(&image.image).await
    } else {
        file_not_found()
    }
}

pub async fn image_tracked_get(
    image_store: ImageStore,
    image_id: &usize,
) -> Result<impl warp::Reply, Infallible> {
    let image = image_store.get(&image_id);

    if let Some(image) = image {
        let tracked_image = add_tracking_data_to_image(&image.image, &image.detection_objects);
        send_image(&tracked_image).await
    } else {
        file_not_found()
    }
}

pub async fn image_tracked_get_rgb8(
    image_store: ImageStore,
    image_id: &usize,
) -> Result<impl warp::Reply, Infallible> {
    let image = image_store.get(image_id);

    if let Some(image) = image {
        let img = image.image.to_rgb8().into_vec();

        Ok(warp::reply::with_header(
            img,
            "Content-Type",
            "application/octet-stream",
        ))
        .map(Reply::into_response)
    } else {
        file_not_found()
    }
}
