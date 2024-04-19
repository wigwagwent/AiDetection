#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, FontId, TextureHandle};
use load_images::{load_image_task, ImageRawData};
use std::{cmp::Reverse, collections::BinaryHeap, env, sync::Arc};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use update_gui::update_image_task;

mod load_images;
mod update_gui;

type ImageStore = Arc<tokio::sync::Mutex<BinaryHeap<Reverse<ImageRawData>>>>;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ip:port>", args[0]);
        return;
    }
    let controller_url = format!("http://{}/api/v1/frontend", &args[1]);

    let image_store: ImageStore = Arc::new(tokio::sync::Mutex::new(BinaryHeap::new()));

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

struct MyApp {
    rx_gui: UnboundedReceiver<GuiUpdateCommands>,
    latest_img_data: TextureHandle,
    image_props: String,
}

enum GuiUpdateCommands {
    UpdateImage(TextureHandle, String),
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(msg) = self.rx_gui.try_recv() {
            match msg {
                GuiUpdateCommands::UpdateImage(img, img_props) => {
                    self.latest_img_data = img;
                    self.image_props = img_props;
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.image((self.latest_img_data.id(), self.latest_img_data.size_vec2()));
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(self.image_props.clone())
                                .font(FontId::proportional(35.0)),
                        );
                    });
                });
            });
        });
    }
}

impl MyApp {
    fn new(ctx: &egui::Context, rx_gui: UnboundedReceiver<GuiUpdateCommands>) -> Self {
        Self {
            rx_gui,
            latest_img_data: {
                ctx.load_texture("my-image", egui::ColorImage::example(), Default::default())
            },
            image_props: Default::default(),
        }
    }
}
