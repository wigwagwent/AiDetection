use std::{
    env,
    net::IpAddr,
    sync::{atomic::AtomicUsize, Arc},
};

use dashmap::DashMap;
use shared_types::server::ImageManager;
use tokio::{sync::mpsc, task};
use warp::Filter;

use crate::{
    api_v1::send_message_on_change, controller::controller_task, load_image::load_new_images_task,
};

mod api_v1;
mod controller;
mod controller_helper;
mod load_image;

static NEXT_IMAGE_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

type ImageStore = Arc<DashMap<usize, ImageManager>>;
type Clients = Arc<DashMap<usize, mpsc::UnboundedSender<warp::filters::ws::Message>>>;

#[tokio::main]
async fn main() {
    let (ip, port) = {
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
        (ip, port)
    };

    let image_store: Arc<DashMap<usize, ImageManager>> = ImageStore::default();
    let client_store = Clients::default();

    let store = Arc::clone(&image_store);
    task::spawn(async move {
        load_new_images_task(store);
    });

    let store = Arc::clone(&image_store);
    let client = Arc::clone(&client_store);
    task::spawn(async move {
        send_message_on_change(store, client);
    });

    let store = Arc::clone(&image_store);
    task::spawn(async move { controller_task(store) });

    let cors = warp::cors().allow_any_origin();

    let routes = warp::path("api")
        .and(warp::path("v1"))
        .and(api_v1::api_interface(image_store, client_store))
        .with(cors);

    println!("Now listening on {}:{}", ip, port);

    warp::serve(routes).run((ip, port)).await;
}
