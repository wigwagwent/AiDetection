mod controller;
mod load_image;
mod websocket;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use dashmap::DashMap;
use image::{ImageOutputFormat, Rgba};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use shared_types::server::{ClientData, ImageManager, ProcessingStatus};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;
use std::{sync::Arc, thread};
use warp::Filter;

use crate::controller::controller_thread;
use crate::load_image::load_new_images_thread;

static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_IMAGE_ID: AtomicUsize = AtomicUsize::new(1);

type Clients = Arc<DashMap<usize, ClientData>>;
type ImageStore = Arc<Mutex<HashMap<usize, ImageManager>>>;

#[tokio::main]
async fn main() {
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

    let ip: [u8; 4] = [127, 0, 0, 1];
    let port: u16 = 3030;
    // Start the Warp server
    println!(
        "Now listening on {}.{}.{}.{}:{}",
        ip[0], ip[1], ip[2], ip[3], port
    );
    println!(
        "http://{}.{}.{}.{}:{}/image",
        ip[0], ip[1], ip[2], ip[3], port
    );
    warp::serve(routes).run((ip, port)).await;
}

fn image_html(image_store: ImageStore) -> String {
    let images = image_store.lock().unwrap();
    let mut latest_processed: Option<usize> = None;
    for (id, image) in images.iter() {
        if image.detection_status == ProcessingStatus::Finished {
            if latest_processed.is_none() {
                latest_processed = Some(id.clone());
            } else if id > &latest_processed.expect("empty check already happend") {
                latest_processed = Some(id.clone());
            }
        }
    }

    if latest_processed.is_none() {
        return r#"No processed pictures found"#.into();
    }

    let image = images.get(&latest_processed.unwrap()).unwrap();
    let outlines = image
        .tracked
        .clone()
        .expect("Tracking results said it was done");
    let mut image = image.raw.clone();
    let mut items_in_img: String = String::new();

    for rectangle in outlines {
        items_in_img += format!(
            "Item: {:#?}, Probability: {:.2}<br>",
            rectangle.label, rectangle.probability
        )
        .as_str();
        draw_hollow_rect_mut(
            &mut image,
            Rect::at(rectangle.x_bottom_corner, rectangle.y_bottom_corner)
                .of_size(rectangle.x_length, rectangle.y_height),
            Rgba([255, 0, 0, 0]),
        );
    }

    let mut image_data: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = BASE64.encode(image_data);

    format!(
        r#"
            <html>
                <body>
                    <img id="exampleImage" src="data:image/png;base64,{}" alt="Example Image">
                    <p>{}</p>
                    <script>
                        setTimeout(function () {{ location.reload(true); }}, 5000);
                    </script>
                </body>
            </html>
        "#,
        res_base64, items_in_img
    )
}
