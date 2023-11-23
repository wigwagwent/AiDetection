use std::{thread, time::Duration};

use futures_util::{future, pin_mut, StreamExt};
mod image_processing;
use image::EncodableLayout;
use shared_types::{
    client::ReturnData,
    server::{SentData, SentDataType},
};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let url = url::Url::parse("ws://127.0.0.1:3030/websocket").unwrap();

    let (tx, rx) = futures_channel::mpsc::unbounded();
    let tx2 = tx.clone();

    // Send to the server saying we run object detection
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let client_type = ReturnData {
            img_id: 0,
            time_cost: Duration::from_secs(0),
            data_type: shared_types::client::ReturnDataType::ClientType(
                shared_types::ProcessingType::ObjectDetection,
            ),
        };
        let _ = tx.unbounded_send(Message::Binary(bincode::serialize(&client_type).unwrap()));
    });

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let client_ws_tx = rx.map(Ok).forward(write);
    let client_ws_rx = {
        read.for_each(|message| async {
            let data: SentData = bincode::deserialize(
                message
                    .expect("this to get the message")
                    .into_data()
                    .as_bytes(),
            )
            .expect("or else this is broken");
            match data.data_type {
                SentDataType::Image => image_processing::receive_img(data.raw_data, tx2.clone()),
                SentDataType::ImageProperties => todo!("Image Properties"),
            }
        })
    };

    pin_mut!(client_ws_tx, client_ws_rx);
    future::select(client_ws_tx, client_ws_rx).await;
}
