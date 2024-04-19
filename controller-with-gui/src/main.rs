use std::{
    env,
    net::IpAddr,
    sync::{atomic::AtomicUsize, Arc},
};

use dashmap::DashMap;
use eframe::egui::{self, FontId, TextureHandle};
use shared_types::server::ImageManager;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver},
    task,
};
use warp::Filter;

use crate::{
    api_v1::send_message_on_change, controller::controller_task, load_image::load_new_images_task,
    update_gui::update_image_task,
};

mod api_v1;
mod controller;
mod controller_helper;
mod load_image;
mod update_gui;

static NEXT_IMAGE_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

type ImageStore = Arc<DashMap<usize, ImageManager>>;
type Clients = Arc<DashMap<usize, mpsc::UnboundedSender<warp::filters::ws::Message>>>;

#[tokio::main]
async fn main() {
    let (ip, port) = {
        let args: Vec<String> = env::args().collect();

        if args.len() != 2 {
            eprintln!("Usage: {} <ip:port>", args[0]);
            return;
        }

        let ip_port_str = &args[1];
        let parts: Vec<&str> = ip_port_str.split(':').collect();

        if parts.len() != 2 {
            eprintln!("Invalid format for <ip:port>");
            return;
        }

        let ip: IpAddr = match parts[0].parse() {
            Ok(ip) => ip,
            Err(_) => {
                eprintln!("Invalid IP address provided");
                return;
            }
        };

        let port: u16 = match parts[1].parse() {
            Ok(port) => port,
            Err(_) => {
                eprintln!("Invalid port provided");
                return;
            }
        };
        (ip, port)
    };

    let image_store: Arc<DashMap<usize, ImageManager>> = ImageStore::default();
    let client_store = Clients::default();

    let store = Arc::clone(&image_store);
    task::spawn(async move {
        load_new_images_task(store);
    });

    let store = Arc::clone(&image_store);
    let client = Arc::clone(&client_store);
    task::spawn(async move {
        send_message_on_change(store, client);
    });

    let store = Arc::clone(&image_store);
    task::spawn(async move { controller_task(store) });

    let cors = warp::cors().allow_any_origin();

    let routes = warp::path("api")
        .and(warp::path("v1"))
        .and(api_v1::api_interface(image_store.clone(), client_store))
        .with(cors);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    task::spawn(async move {
        println!("Now listening on {}:{}", ip, port);
        warp::serve(routes).run((ip, port)).await;
    });

    let (tx_gui, rx_gui) = mpsc::unbounded_channel();
    let _ = eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            let frame = cc.egui_ctx.clone();

            tokio::spawn(async move {
                update_image_task(&frame, tx_gui, image_store).await;
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
