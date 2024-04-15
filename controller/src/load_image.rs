use crate::ImageStore;

#[cfg(feature = "load-camera")]
use self::load_camera_image::CameraImage;
#[cfg(feature = "load-file")]
use self::load_local_image::LocalImage;

#[cfg(feature = "load-camera")]
mod load_camera_image;
#[cfg(feature = "load-file")]
mod load_local_image;

pub trait LoadImages {
    fn get_image(&mut self, store: &ImageStore);
}

fn new_load_images() -> impl LoadImages {
    #[cfg(feature = "load-file")]
    let load = LocalImage::default();

    #[cfg(feature = "load-camera")]
    let load = CameraImage::default();
    load
}

pub fn load_new_images_task(store: ImageStore) {
    let mut load = new_load_images();

    loop {
        load.get_image(&store);
        println!("Loop Restarted");
    }
}
