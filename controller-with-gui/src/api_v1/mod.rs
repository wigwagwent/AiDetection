mod api_frontend;
mod api_shared;
mod api_tracking;
mod websocket;

pub use websocket::send_message_on_change;

use warp::Filter;

use crate::{Clients, ImageStore};

use self::api_frontend::api_frontend_interface;

// http://127.0.0.1:3000/api/v1/frontend
// http://127.0.0.1:3000/api/v1/tracking
// ws://127.0.1:3000/api/v1/websocket
pub fn api_interface(
    image_store: ImageStore,
    clients: Clients,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("frontend")
        .and(api_frontend_interface(image_store.clone()))
        .or(warp::path("tracking").and(api_tracking::api_tracking_interface(image_store.clone())))
        .or(warp::path("websocket").and(websocket::websocket_interface(image_store, clients)))
}
