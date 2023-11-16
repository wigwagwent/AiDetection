use futures_util::{future, pin_mut, StreamExt};
mod image_processing;
use image::EncodableLayout;
use shared_types::server::{SentData, SentDataType};
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let url = url::Url::parse("ws://127.0.0.1:3030/websocket").unwrap();

    let (tx, rx) = futures_channel::mpsc::unbounded();
    let tx2 = tx.clone();

    // thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(20));
    //     let _ = tx.unbounded_send(Message::Text("Does this work to send data back".to_owned()));
    // });

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
