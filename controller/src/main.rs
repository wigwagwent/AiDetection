mod controller;
mod websocket;

use dashmap::DashMap;
use shared_types::server::ClientData;
use std::sync::atomic::AtomicUsize;
use std::{sync::Arc, thread};
use warp::Filter;

use crate::controller::controller_thread;

static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

type Clients = Arc<DashMap<usize, ClientData>>;

#[tokio::main]
async fn main() {
    let clients = Clients::default();

    thread::spawn({
        let controller_clients = clients.clone();
        move || controller_thread(controller_clients)
    });

    let clients = warp::any().map(move || clients.clone());

    let ws_connection =
        warp::path("websocket")
            .and(warp::ws())
            .and(clients)
            .map(|ws: warp::ws::Ws, clients| {
                ws.on_upgrade(move |socket| websocket::client_connected(socket, clients))
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
