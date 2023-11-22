use dashmap::DashMap;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use shared_types::{
    server::{ClientData, SentData, SentDataType},
    HardwareMonitor, ImageProperties,
};
use std::{path::PathBuf, sync::Arc, thread, time::Duration};
use warp::filters::ws::Message;

#[allow(unused_imports)]
use self::{load_camera_image::CameraImage, load_local_image::LocalImage};
mod load_camera_image;
mod load_local_image;

#[derive(Serialize, Deserialize)]
pub struct ImageManager {
    pub raw: PathBuf,
    pub dehazed: Option<PathBuf>,
    pub tracked: Option<PathBuf>,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HardwareMonitorKey {
    pub img_id: u32,
    pub mac: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ControllerInfo {
    pub images: Arc<DashMap<u32, ImageManager>>,
    pub monitor: Arc<DashMap<HardwareMonitorKey, HardwareMonitor>>,
}

// impl ControllerInfo {
//     pub fn new() -> ControllerInfo {
//         ControllerInfo {
//             images: Arc::new(DashMap::new()),
//             monitor: Arc::new(DashMap::new()),
//         }
//     }
// }

pub trait LoadImages {
    fn get_image(&mut self) -> DynamicImage;
}

fn new_load_images() -> impl LoadImages {
    #[cfg(any(feature = "load-file"))]
    let load = LocalImage::default();

    #[cfg(any(feature = "load-camera"))]
    let load = CameraImage::default();
    load
}

pub fn controller_thread(clients: Arc<DashMap<usize, ClientData>>) {
    let mut load = new_load_images();
    loop {
        let mut images_processed: u32 = 0;
        let img = load.get_image();

        for client in clients.iter() {
            let (&_uid, tx) = client.pair();

            if !tx.busy.load(std::sync::atomic::Ordering::Relaxed) {
                let raw_img = ImageProperties::new_scaled(img.clone(), 640, 640);
                let raw_data = bincode::serialize(&raw_img).unwrap();

                let send_message = SentData {
                    data_type: SentDataType::Image,
                    raw_data,
                };
                let send_message = bincode::serialize(&send_message).unwrap();
                if send_message.len() > 16777216 {
                    println!("Oversized package detected");
                    continue;
                }
                images_processed += 1;
                let _ = tx.link.send(Message::binary(send_message));
                tx.busy.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }

        println!(
            "Looped through every image, processed: {}",
            images_processed
        );
        thread::sleep(Duration::from_secs(360));
    }
}
