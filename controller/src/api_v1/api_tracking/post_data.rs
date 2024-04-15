use shared_types::{
    client::ReturnData,
    server::{ImageManager, ProcessingStatus},
};
use std::convert::Infallible;
use warp::reply::{self, Reply};

use crate::{api_v1::api_shared::api_helper::file_not_found, ImageStore};

pub async fn latest_image_data_post(
    image_store: ImageStore,
    image_id: &usize,
    img_data: ReturnData,
) -> Result<impl warp::Reply, Infallible> {
    let image = image_store.get_mut(&image_id);

    let mut image: dashmap::mapref::one::RefMut<'_, usize, ImageManager> = match image {
        Some(image) => image,
        None => return file_not_found(),
    };

    image.tracked_status = ProcessingStatus::Finished;
    image.detection_status = ProcessingStatus::Finished;
    image.tracked = Some(img_data.tracking_results);
    image.tracked_time = Some(img_data.tracking_time);

    Ok(reply::with_status(String::new(), warp::http::StatusCode::OK).into_response())
}
