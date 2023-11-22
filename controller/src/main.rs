mod controller;
mod load_image;
mod websocket;

use dashmap::DashMap;
use shared_types::server::{ClientData, ImageManager};
use std::sync::atomic::AtomicUsize;
use std::{sync::Arc, thread};
use warp::Filter;

use crate::controller::controller_thread;
use crate::load_image::load_new_images_thread;

static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_IMAGE_ID: AtomicUsize = AtomicUsize::new(1);

type Clients = Arc<DashMap<usize, ClientData>>;
type ImageStore = Arc<DashMap<usize, ImageManager>>;

#[tokio::main]
async fn main() {
    let clients = Clients::default();
    let image_store = ImageStore::default();

    thread::spawn({
        let store = image_store.clone();
        move || load_new_images_thread(store)
    });

    thread::spawn({
        let store = image_store.clone();
        let controller_clients = clients.clone();
        move || controller_thread(controller_clients, store)
    });

    let clients = warp::any().map(move || clients.clone());
    let image_store = warp::any().map(move || image_store.clone());

    let ws_connection = warp::path("websocket")
        .and(warp::ws())
        .and(clients)
        .and(image_store)
        .map(|ws: warp::ws::Ws, clients, image_store| {
            ws.on_upgrade(move |socket| websocket::client_connected(socket, clients, image_store))
        });

    let ip: [u8; 4] = [127, 0, 0, 1];
    let port: u16 = 3030;
    // Start the Warp server
    println!(
        "Now listening on {}.{}.{}.{}:{}",
        ip[0], ip[1], ip[2], ip[3], port
    );
    warp::serve(ws_connection).run((ip, port)).await;
}

// for rectangle in objects {
//     draw_hollow_rect_mut(
//         &mut img,
//         Rect::at(rectangle.x_bottom_corner, rectangle.y_bottom_corner)
//             .of_size(rectangle.x_length, rectangle.y_height),
//         Rgba([97, 51, 47, 0]),
//     );
// }
