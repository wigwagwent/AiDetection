use shared_types::server::{ImageManager, ProcessingStatus};
use std::{fs, path::PathBuf, thread, time::Duration};

use crate::{ImageStore, NEXT_IMAGE_ID};

use super::LoadImages;

pub struct LocalImage {
    framerate: f32,
    path: String,
}

impl Default for LocalImage {
    fn default() -> Self {
        Self {
            framerate: 15.0,
            path: "local-images/frames".into(),
        }
    }
}

impl LoadImages for LocalImage {
    fn get_image(&mut self, store: &ImageStore) {
        let mut count = 0;

        let mut file_paths: Vec<PathBuf> = fs::read_dir(&self.path)
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();

        file_paths.sort();

        for entry in file_paths {
            thread::sleep(Duration::from_secs_f32(
                (1.0 / 60.0) * (60.0 / self.framerate),
            ));
            let img = match image::open(entry.as_path()) {
                Ok(image) => image,
                Err(error) => {
                    println!("Image could not be read, {}", error);
                    continue;
                }
            };

            let img_id = NEXT_IMAGE_ID.load(std::sync::atomic::Ordering::Relaxed);
            let new_img_store_val: ImageManager = ImageManager {
                raw: img,
                dehazed: None,
                dehazed_status: ProcessingStatus::NotStarted,
                dehazed_time: None,
                tracked: None,
                tracked_status: ProcessingStatus::NotStarted,
                tracked_time: None,
                detection_status: ProcessingStatus::NotStarted,
                detection_time: None,
            };
            let mut mut_store = store.lock().unwrap();
            mut_store.insert(img_id, new_img_store_val);
            NEXT_IMAGE_ID.store(img_id + 1, std::sync::atomic::Ordering::Relaxed);
            count += 1;
            if count % 100 == 0 {
                println!("Images Read: {}", count);
            }
        }
    }
}
