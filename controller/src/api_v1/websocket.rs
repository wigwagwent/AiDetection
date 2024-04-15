use futures_util::{SinkExt, StreamExt, TryFutureExt};
use std::sync::atomic::Ordering;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    filters::ws::{Message, WebSocket},
    reject::Rejection,
    reply::Reply,
    Filter,
};

use crate::{Clients, ImageStore, NEXT_CLIENT_ID};

use super::api_shared::api_helper::{get_latest_tracked_image_id, with_image_store};

pub fn websocket_interface(
    image_store: ImageStore,
    clients: Clients,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::ws()
        .and(with_image_store(image_store))
        .and(warp::any().map(move || clients.clone()))
        .map(|ws: warp::ws::Ws, image_store, clients| {
            ws.on_upgrade(move |socket| client_connected(socket, clients, image_store))
        })
}

pub fn send_message_on_change(store: ImageStore, clients: Clients) {
    let mut last_tracked_image: Option<usize> = None;

    loop {
        let latest = get_latest_tracked_image_id(store.clone());
        if latest != last_tracked_image {
            last_tracked_image = latest;
            for client in clients.iter() {
                client.value().send(Message::text("update")).unwrap();
            }
        }
    }
}

pub async fn client_connected(ws: WebSocket, clients: Clients, _image_store: ImageStore) {
    let my_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    println!("new user interface client: {}", my_id);

    let (mut client_ws_tx, mut client_ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            client_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    clients.insert(my_id, tx);

    while let Some(_result) = client_ws_rx.next().await {}
    user_disconnected(my_id, &clients).await;
}

async fn user_disconnected(my_id: usize, users: &Clients) {
    users.remove(&my_id);
}
