mod controller;
mod websocket;

use dashmap::DashMap;
use shared_types::server::{ClientData, SentData, SentDataType};
use std::sync::atomic::AtomicUsize;
use std::time::Duration;
use std::{fs, sync::Arc, thread};
use warp::ws::Message;
use warp::Filter;

static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

type Clients = Arc<DashMap<usize, ClientData>>;

#[tokio::main]
async fn main() {
    let clients = Clients::default();

    //Camera Manager

    //Controller Manager

    //Test stuff
    //
    let send_clients = clients.clone();

    thread::spawn(move || loop {
        let folder_path = "/home/carter/Documents/Save_The_Sea_Turtles/MobileNetV3/Training/turtle"; //tutle photos
        let framerate: f32 = 30.0;
        let mut image_read: u32 = 0;
        let mut images_processed: u32 = 0;

        // Read the contents of the folder
        if let Ok(entries) = fs::read_dir(folder_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    thread::sleep(Duration::from_secs_f32((1.0 / 60.0) * (60.0 / framerate)));

                    let img = match controller::get_image_raw(entry.path()) {
                        Ok(image) => image,
                        Err(error) => {
                            println!("Image could not be read, {}", error);
                            continue;
                        }
                    };

                    image_read += 1;
                    if image_read % 100 == 0 {
                        println!("Images Read: {}", image_read);
                    }

                    for client in send_clients.iter() {
                        let (&_uid, tx) = client.pair();

                        if !tx.busy.load(std::sync::atomic::Ordering::Relaxed) {
                            let raw_data = bincode::serialize(&img.clone()).unwrap();

                            let send_message = SentData {
                                data_type: SentDataType::Image,
                                raw_data,
                            };
                            let send_message = bincode::serialize(&send_message).unwrap();
                            if send_message.len() > 16777216 {
                                continue;
                            }
                            images_processed += 1;
                            let _ = tx.link.send(Message::binary(send_message));
                            tx.busy.store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                }
            }
        } else {
            println!("Failed to read the folder contents.");
        }
        println!(
            "Looped through every image, read: {}, processed: {}",
            image_read, images_processed
        );
        thread::sleep(Duration::from_secs(360));
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
