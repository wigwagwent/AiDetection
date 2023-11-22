use crate::ImageStore;

use super::{LoadImageErr, LoadImages};

pub struct CameraImage {}

impl Default for CameraImage {
    fn default() -> Self {
        Self {}
    }
}

impl LoadImages for CameraImage {
    fn get_image(&mut self, _store: &ImageStore) -> Result<(), LoadImageErr> {
        todo!()
    }
}
