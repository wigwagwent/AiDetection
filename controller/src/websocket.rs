use futures_util::{SinkExt, StreamExt, TryFutureExt};
use shared_types::{
    client::{ReturnData, ReturnDataType},
    server::{ClientData, ProcessingStatus},
};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::filters::ws::{Message, WebSocket};

use crate::{Clients, ImageStore, NEXT_CLIENT_ID};

pub async fn client_connected(ws: WebSocket, clients: Clients, image_store: ImageStore) {
    let my_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new processing node: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (mut client_ws_tx, mut client_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
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

    let data = ClientData {
        link: tx,
        client_busy: AtomicBool::new(false),
        client_type: None,
    };

    clients.insert(my_id, data);

    while let Some(result) = client_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        user_message(my_id, msg, &clients, &image_store).await;
    }
    user_disconnected(my_id, &clients).await;
}

async fn user_message(my_id: usize, msg: Message, clients: &Clients, image_store: &ImageStore) {
    if msg.is_text() {
        let raw_msg = msg.to_str().expect("We know its a string");
        let new_msg = format!("<Node#{}>: {}", my_id, raw_msg);
        println!("{}", new_msg);
    } else if msg.is_binary() {
        let data: ReturnData = bincode::deserialize(msg.as_bytes()).unwrap();
        match data.data_type {
            ReturnDataType::ListOfItemsDetected(objects) => {
                clients
                    .get(&my_id)
                    .unwrap()
                    .client_busy
                    .store(false, std::sync::atomic::Ordering::Relaxed);

                let mut mut_store = image_store.get_mut(&data.img_id).unwrap();

                mut_store.detection_status = ProcessingStatus::Finished;
                mut_store.tracked = Some(objects);
                mut_store.detection_time = Some(data.time_cost);
            }

            ReturnDataType::ClientType(client_type) => {
                clients.get_mut(&my_id).unwrap().client_type = Some(client_type)
            }
        };
    } else {
        return;
    }
}

async fn user_disconnected(my_id: usize, users: &Clients) {
    users.remove(&my_id);
}
