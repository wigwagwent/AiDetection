mod get_images;

use dashmap::DashMap;
use shared_types::server::ImageManager;
use std::sync::Arc;
use warp::{reject::Rejection, reply::Reply, Filter};

use self::get_images::{
    get_img_with_id, get_latest_img, get_latest_tracked_data, get_latest_tracked_image,
};

pub fn api_frontend_interface(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    route_latest_image_get(image_store.clone())
        .or(route_image_get(image_store.clone()))
        .or(route_latest_image_tracked_get(image_store.clone()))
        .or(route_latest_image_data_get(image_store.clone()))
}

// Define the route for serving the latest image with GET request
pub fn route_latest_image_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("image-latest")
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(get_latest_img)
}

// Define the route for serving the latest image with GET request
pub fn route_latest_image_tracked_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("image-latest-tracked")
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(get_latest_tracked_image)
}

// Define the route for serving the image with GET request
pub fn route_image_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("image")
        .and(warp::path::param::<usize>())
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(
            |id: usize, image_store: Arc<DashMap<usize, ImageManager>>| async move {
                get_img_with_id(image_store.clone(), &id).await
            },
        )
}

// Define the route for serving the latest image with GET request
pub fn route_latest_image_data_get(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("image-latest-data")
        .and(warp::get())
        .and(with_image_store(image_store))
        .and_then(get_latest_tracked_data)
}

fn with_image_store(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (Arc<DashMap<usize, ImageManager>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || image_store.clone())
}
