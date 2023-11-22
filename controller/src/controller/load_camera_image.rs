use super::LoadImages;

pub struct CameraImage {}

impl Default for CameraImage {
    fn default() -> Self {
        Self {}
    }
}

impl LoadImages for CameraImage {
    fn get_image(&mut self) -> image::DynamicImage {
        todo!()
    }
}
