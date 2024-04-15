mod get_data;
mod get_image;

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::ImageStore;

use self::{
    get_data::{image_data_get, latest_image_data_get, latest_tracking_data_get},
    get_image::{image_get, image_tracked_get},
};

use super::api_shared::api_helper::with_image_store;

pub fn api_frontend_interface(
    image_store: ImageStore,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    route_image_get(image_store.clone())
        .or(route_image_tracked_get(image_store.clone()))
        .or(route_image_data_get(image_store.clone()))
        .or(route_latest_image_data_get(image_store.clone()))
        .or(route_latest_tracking_data_get(image_store))
}

/// Returns the image with the given id
/// GET http://127.0.0.1:3000/api/v1/frontend/image/{id}
pub fn route_image_get(
    image_store: ImageStore,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("image")
        .and(warp::path::param::<usize>())
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(|id: usize, image_store: ImageStore| async move {
            image_get(image_store.clone(), &id).await
        })
}

/// Returns the image with the given id that has tracking results drawn on it
/// GET http://127.0.0.1:3000/api/v1/frontend/image-tracked/{id}
pub fn route_image_tracked_get(
    image_store: ImageStore,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("image-tracked")
        .and(warp::path::param::<usize>())
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(|id: usize, image_store: ImageStore| async move {
            image_tracked_get(image_store.clone(), &id).await
        })
}

/// Returns the image details with the given id
/// GET http://127.0.0.1:3000/api/v1/frontend/image-data/{id}
pub fn route_image_data_get(
    image_store: ImageStore,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("image-data")
        .and(warp::path::param::<usize>())
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(|id: usize, image_store: ImageStore| async move {
            image_data_get(image_store.clone(), &id).await
        })
}

/// Returns the image details of the latest image
/// GET http://127.0.0.1:3000/api/v1/frontend/latest-image-data
pub fn route_latest_image_data_get(
    image_store: ImageStore,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("latest-image-data")
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(latest_image_data_get)
}

/// Returns the image details of the latest image with tracking data
/// GET http://127.0.0.1:3000/api/v1/frontend/tracking-image-data
pub fn route_latest_tracking_data_get(
    image_store: ImageStore,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("tracking-image-data")
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(latest_tracking_data_get)
}
