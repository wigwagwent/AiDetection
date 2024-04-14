mod api_frontend;
mod api_shared;
mod api_tracking;

use dashmap::DashMap;
use std::sync::Arc;
use warp::Filter;

use crate::ImageManager;

use self::api_frontend::api_frontend_interface;

// http://127.0.0.1:3000/api/v1/frontend
// http://127.0.0.1:3000/api/v1/tracking
pub fn api_interface(
    image_store: Arc<DashMap<usize, ImageManager>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("frontend")
        .and(api_frontend_interface(image_store.clone()))
        .or(warp::path("tracking").and(api_tracking::api_tracking_interface(image_store)))
}
