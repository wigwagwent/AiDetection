#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Error;
use bytes::Bytes;
use eframe::egui::{self, TextureHandle};
use shared_types::server::ImageInformation;
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    env,
    sync::Arc,
};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver},
    time::Duration,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ip:port>", args[0]);
        return;
    }
    let controller_url = format!("http://{}/api/v1/frontend", &args[1]);

    let image_store: Arc<tokio::sync::Mutex<BinaryHeap<Reverse<ImageRawData>>>> =
        Arc::new(tokio::sync::Mutex::new(BinaryHeap::new()));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    let image_store_load = image_store.clone();
    tokio::spawn(async move {
        load_image_task(&controller_url, image_store_load).await;
    });

    let (tx_gui, rx_gui) = mpsc::unbounded_channel();
    let _ = eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            let frame = cc.egui_ctx.clone();

            tokio::spawn(async move {
                update_image_task(&frame, tx_gui, image_store, 30.0).await;
            });
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::new(&cc.egui_ctx, rx_gui))
        }),
    );
}

async fn update_image_task(
    ctx: &egui::Context,
    tx_gui: mpsc::UnboundedSender<TextureHandle>,
    image_store: Arc<tokio::sync::Mutex<BinaryHeap<Reverse<ImageRawData>>>>,
    framerate: f32,
) {
    loop {
        let time = Duration::from_secs_f32(1.0 / framerate);
        tokio::time::sleep(time).await;
        let mut image_store = image_store.lock().await;
        if image_store.len() > 10 {
            let image = image_store.pop();
            drop(image_store); // Release the lock

            if let Some(image) = image {
                let egui_image = egui::ColorImage::from_rgb(
                    [image.0.width as usize, image.0.height as usize],
                    image.0.image.as_ref(),
                );
                tx_gui
                    .send(ctx.load_texture("my-image", egui_image, Default::default()))
                    .expect("Failed to send message");
                ctx.request_repaint();
            }
        }
    }
}

async fn load_image_task(
    controller_url: &str,
    image_store: Arc<tokio::sync::Mutex<BinaryHeap<Reverse<ImageRawData>>>>,
) {
    loop {
        let image = fetch_image(controller_url).await;
        if let Ok(image) = image {
            image_store.lock().await.push(Reverse(image));
        }
    }
}

async fn fetch_image(controller_url: &str) -> Result<ImageRawData, Error> {
    let client = reqwest::Client::new();

    let image_info: ImageInformation = client
        .get(format!("{}/tracking-image-data", controller_url))
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    let image = client
        .get(format!(
            "{}/image-tracked-raw-rgb8/{}",
            controller_url, image_info.image_props.img_id
        ))
        .header("Accept", "application/octet-stream")
        .send()
        .await?;

    if !image.status().is_success() {
        return Err(anyhow::Error::msg("Failed to fetch image from server"));
    }

    if let Some(content_type) = image.headers().get("content-type") {
        if !content_type.to_str().unwrap().starts_with("image/jpeg") {
            return Err(anyhow::Error::msg("Invalid content type for image data"));
        }
    } else {
        return Err(anyhow::Error::msg(
            "No content type header found in response",
        ));
    }

    let bytes = image.bytes().await?;

    Ok(ImageRawData {
        image: bytes,
        width: image_info.image_props.origin_width,
        height: image_info.image_props.origin_height,
        id: image_info.image_props.img_id,
    })
}

struct MyApp {
    rx_gui: UnboundedReceiver<TextureHandle>,
    latest_img_data: TextureHandle,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(msg) = self.rx_gui.try_recv() {
            self.latest_img_data = msg;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.image((self.latest_img_data.id(), self.latest_img_data.size_vec2()));
            });
        });
    }
}

impl MyApp {
    fn new(ctx: &egui::Context, rx_gui: UnboundedReceiver<TextureHandle>) -> Self {
        Self {
            rx_gui,
            latest_img_data: {
                ctx.load_texture("my-image", egui::ColorImage::example(), Default::default())
            },
        }
    }
}

#[derive(Eq)]
struct ImageRawData {
    image: Bytes,
    width: u32,
    height: u32,
    id: usize,
}

impl Ord for ImageRawData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for ImageRawData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ImageRawData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
