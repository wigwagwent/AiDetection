use std::sync::Arc;

use dashmap::DashMap;
use shared_types::{client::ReturnData, server::ImageManager};
use warp::{reject::Rejection, reply::Reply, Filter};

use self::{get_image::image_get, post_data::latest_image_data_post};

use super::api_shared::{
    api_helper::with_image_store,
    get_data::{image_data_get, latest_image_data_get},
};

mod get_image;
mod post_data;

pub fn api_tracking_interface(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    route_image_get(image_store.clone())
        .or(route_image_data_get(image_store.clone()))
        .or(route_latest_image_data_get(image_store.clone()))
        .or(route_latest_image_data_post(image_store.clone()))
}

/// Returns the image with the given id with size 640x640
/// GET http://127.0.0.1:3000/api/v1/tracking/image/{id}
pub fn route_image_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("image")
        .and(warp::path::param::<usize>())
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(
            |id: usize, image_store: Arc<DashMap<usize, ImageManager>>| async move {
                image_get(image_store.clone(), &id).await
            },
        )
}

/// Returns the image details with the given id
/// GET http://127.0.0.1:3000/api/v1/tracking/image-data/{id}
pub fn route_image_data_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("image-data")
        .and(warp::path::param::<usize>())
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(
            |id: usize, image_store: Arc<DashMap<usize, ImageManager>>| async move {
                image_data_get(image_store.clone(), &id).await
            },
        )
}

/// Returns the image details of the latest image
/// GET http://127.0.0.1:3000/api/v1/tracking/latest-image-data
pub fn route_latest_image_data_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("image-latest-data")
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(latest_image_data_get)
}

/// Posts processed image data to the server
/// POST http://127.0.0.1:3000/api/v1/tracking/image-data/{id}
pub fn route_latest_image_data_post(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("image-data")
        .and(warp::path::param::<usize>())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_image_store(image_store))
        .and_then(
            |id: usize, img_data: ReturnData, image_store: Arc<DashMap<usize, ImageManager>>| async move {
                latest_image_data_post(image_store.clone(), &id, img_data).await
            },
        )
}
