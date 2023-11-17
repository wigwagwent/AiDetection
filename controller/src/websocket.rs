use futures_util::{SinkExt, StreamExt, TryFutureExt};
use image::Rgba;
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};
use shared_types::{client::ReturnData, server::ClientData};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::filters::ws::{Message, WebSocket};

use crate::{Clients, NEXT_CLIENT_ID};

pub async fn client_connected(ws: WebSocket, clients: Clients) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat user: {}", my_id);

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
        busy: AtomicBool::new(false),
    };

    // Save the sender in our list of connected users.
    clients.insert(my_id, data);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = client_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        user_message(my_id, msg, &clients).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_id, &clients).await;
}

async fn user_message(my_id: usize, msg: Message, clients: &Clients) {
    // Skip any non-Text messages...
    if msg.is_text() {
        let raw_msg = msg.to_str().expect("We know its a string");
        let new_msg = format!("<User#{}>: {}", my_id, raw_msg);
        println!("{}", new_msg);
    } else if msg.is_binary() {
        clients
            .get(&my_id)
            .unwrap()
            .busy
            .store(false, std::sync::atomic::Ordering::Relaxed);
        let data: ReturnData = bincode::deserialize(msg.as_bytes()).unwrap();

        //
        //
        //
        //
        //
        let mut img = match image::open("img.path") {
            Ok(image) => image,
            Err(error) => {
                panic!("Image could not be read, {}", error);
            }
        };

        let rectangles = match data.data_type {
            shared_types::client::ReturnDataType::ListOfItems(rec) => rec,
        };

        for rectangle in rectangles {
            draw_hollow_rect_mut(
                &mut img,
                Rect::at(rectangle.x_bottom_corner, rectangle.y_bottom_corner)
                    .of_size(rectangle.x_length, rectangle.y_height),
                Rgba([97, 51, 47, 0]),
            );
        }

        //
        //
        //
        //
        //
        //
        //
        //
        //let ser = serde_json::to_string_pretty(&data).expect("this to work");
        //println!("{}", ser);
    } else {
        return;
    }

    // New message from this user, send it to everyone else (except same uid)...
    // for item in clients.iter() {
    //     let (&uid, tx) = item.pair();
    //     if my_id != uid {
    //         if let Err(_disconnected) = tx.send(Message::text(new_msg.clone())) {
    //             // The tx is disconnected, our `user_disconnected` code
    //             // should be happening in another task, nothing more to
    //             // do here.
    //         }
    //     }
    // }
}

async fn user_disconnected(my_id: usize, users: &Clients) {
    users.remove(&my_id);
}
