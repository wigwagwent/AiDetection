use std::{fs, thread, time::Duration};

use shared_types::server::{ImageManager, ProcessingStatus};

use crate::{ImageStore, NEXT_IMAGE_ID};

use super::{LoadImageErr, LoadImages};

pub struct LocalImage {
    framerate: f32,
    path: String,
}

impl Default for LocalImage {
    fn default() -> Self {
        Self {
            framerate: 15.0,
            path: "/home/carter/Documents/Internship/Images/daySequence2/daySequence2/frames"
                .into(),
        }
    }
}

impl LoadImages for LocalImage {
    fn get_image(&mut self, store: &ImageStore) -> Result<(), LoadImageErr> {
        let mut count = 0;

        // Read the contents of the folder
        if let Ok(entries) = fs::read_dir(self.path.clone()) {
            for entry in entries {
                if let Ok(entry) = entry {
                    thread::sleep(Duration::from_secs_f32(
                        (1.0 / 60.0) * (60.0 / self.framerate),
                    ));
                    let img = match image::open(entry.path()) {
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
                        tracked: None,
                        tracking_status: ProcessingStatus::NotStarted,
                        detection_status: ProcessingStatus::NotStarted,
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

            return Ok(());
        } else {
            return Err(LoadImageErr::NotFound(format!("Folder: {}", self.path)));
        }
    }
}
