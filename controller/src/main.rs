use std::{
    env,
    net::IpAddr,
    sync::{atomic::AtomicUsize, Arc},
};

use dashmap::DashMap;
use shared_types::server::ImageManager;
use tokio::task;
use warp::Filter;

use crate::{controller::controller_task, load_image::load_new_images_task};

mod api_v1;
mod controller;
mod controller_helper;
mod load_image;

static NEXT_IMAGE_ID: AtomicUsize = AtomicUsize::new(1);

type ImageStore = Arc<DashMap<usize, ImageManager>>;

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

    let store = Arc::clone(&image_store);
    task::spawn(async move {
        load_new_images_task(store);
    });
    let store = Arc::clone(&image_store);
    task::spawn(async move { controller_task(store) });

    let routes = warp::path("api")
        .and(warp::path("v1"))
        .and(api_v1::api_interface(image_store));

    println!("Now listening on {}:{}", ip, port);
    println!("http://{}:{}/image", ip, port);

    warp::serve(routes).run((ip, port)).await;
}
