use std::{thread, time::Duration};

use crate::{ImageStore, NEXT_IMAGE_ID};

pub fn controller_task(img_store: ImageStore) {
    loop {
        if img_store.len() > 250 {
            let next_count = NEXT_IMAGE_ID.load(std::sync::atomic::Ordering::Relaxed);
            println!("Dropping images, only keeping the last 200");
            img_store.retain(|k, _| k > &(next_count - 200));
        }

        thread::sleep(Duration::from_secs_f32(1.0)); //check every second
    }
}
