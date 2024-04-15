use std::convert::Infallible;

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
        let tracked_image = add_tracking_data_to_image(&image.image, &image.tracked);
        send_image(&tracked_image).await
    } else {
        file_not_found()
    }
}
