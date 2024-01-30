use rascam::{info, SimpleCamera};

use crate::ImageStore;

use super::LoadImages;

#[derive(Default)]
pub struct CameraImage {
    framerate: f32,
}

impl Default for CameraImage {
    fn default() -> Self {
        Self {
            framerate: 15.0,
        }
    }
}

impl LoadImages for CameraImage {
    fn get_image(&mut self, store: &ImageStore) {
        let info = info().unwrap();
        if info.cameras.len() == 0 {
            panic!("No cameras detected");
        }
        info!("Found {} cameras", info);
        
        let mut camera = SimpleCamera::new(info.clone()).unwrap();
        camera.activate().unwrap();

        let mut count = 0;
        loop {
            thread::sleep(Duration::from_secs_f32(
                (1.0 / 60.0) * (60.0 / self.framerate),
            ));

            let img = camera.capture().unwrap();
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
                println!("Images Taken: {}", count);
            }
        }
    }
}
