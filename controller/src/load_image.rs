use std::{thread, time::Duration};

use crate::ImageStore;

#[allow(unused_imports)]
use self::{load_camera_image::CameraImage, load_local_image::LocalImage};
mod load_camera_image;
mod load_local_image;

pub trait LoadImages {
    fn get_image(&mut self, store: &ImageStore) -> Result<(), LoadImageErr>;
}

pub enum LoadImageErr {
    NotFound(String),
}

fn new_load_images() -> impl LoadImages {
    #[cfg(feature = "load-file")]
    let load = LocalImage::default();

    #[cfg(feature = "load-camera")]
    let load = CameraImage::default();
    load
}

pub fn load_new_images_thread(store: ImageStore) {
    let mut load = new_load_images();

    loop {
        let result = load.get_image(&store);

        match result {
            Ok(_) => continue,
            Err(err) => match err {
                LoadImageErr::NotFound(path) => println!("Image not found at: '{}'", path),
            },
        }

        println!("End of loop, starting over in 1 minute");
        thread::sleep(Duration::from_secs(60));
        println!("Loop Restarted");
    }
}
