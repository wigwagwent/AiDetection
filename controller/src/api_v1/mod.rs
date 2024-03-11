mod api_frontend;
mod api_object_detection;
mod api_tracking;

use dashmap::DashMap;
use shared_types::server::ImageManager;
use std::sync::Arc;
use warp::Filter;

use self::api_frontend::api_frontend_interface;

pub fn api_interface(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("frontend").and(api_frontend_interface(image_store.clone()))
}
