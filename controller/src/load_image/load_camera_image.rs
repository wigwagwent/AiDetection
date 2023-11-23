use crate::ImageStore;

use super::LoadImages;

#[derive(Default)]
pub struct CameraImage {}

impl LoadImages for CameraImage {
    fn get_image(&mut self, _store: &ImageStore) {
        todo!()
    }
}
