mod controller;
mod load_image;
mod web_interface;
mod websocket;

use dashmap::DashMap;
use shared_types::server::{ClientData, ImageManager};
use std::sync::atomic::AtomicUsize;
use std::{sync::Arc, thread};
use warp::Filter;

use crate::controller::controller_thread;
use crate::load_image::load_new_images_thread;
use crate::web_interface::image_html;

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
    let img_store = image_store.clone();
    let image_store1 = warp::any().map(move || image_store.clone());
    let image_store2 = warp::any().map(move || img_store.clone());

    let ws_connection = warp::path("websocket")
        .and(warp::ws())
        .and(clients)
        .and(image_store1)
        .map(|ws: warp::ws::Ws, clients, image_store| {
            ws.on_upgrade(move |socket| websocket::client_connected(socket, clients, image_store))
        });

    let web_image_connection = warp::path!("image")
        .and(image_store2)
        .map(move |img_store| warp::reply::html(image_html(img_store)));

    let routes = ws_connection.or(web_image_connection);

    let ip: [u8; 4] = [127, 0, 0, 1];
    let port: u16 = 3030;
    // Start the Warp server
    println!(
        "Now listening on {}.{}.{}.{}:{}",
        ip[0], ip[1], ip[2], ip[3], port
    );
    println!(
        "http://{}.{}.{}.{}:{}/image",
        ip[0], ip[1], ip[2], ip[3], port
    );
    warp::serve(routes).run((ip, port)).await;
}
