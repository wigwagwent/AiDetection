mod controller;
mod load_image;
mod web_interface;
mod websocket;

use dashmap::DashMap;
use shared_types::server::{ClientData, ImageManager};
use std::net::IpAddr;
use std::sync::atomic::AtomicUsize;
use std::{env, sync::Arc, thread};
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
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <ip:port>", args[0]);
        return;
    }

    let ip_port_str = &args[1];
    let parts: Vec<&str> = ip_port_str.split(':').collect();

    if parts.len() != 2 {
        eprintln!("Invalid format for <ip:port>");
        return;
    }

    let ip: IpAddr = match parts[0].parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP address provided");
            return;
        }
    };

    let port: u16 = match parts[1].parse() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid port provided");
            return;
        }
    };

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

    println!("Now listening on {}:{}", ip, port);
    println!("http://{}:{}/image", ip, port);

    warp::serve(routes).run((ip, port)).await;
}
