use image::DynamicImage;
use shared_types::{client::ReturnData, tracking::TrackingResult};
use tokio::time::Instant;

#[cfg(feature = "engine-onnx")]
use self::yolo_onnx::YoloOnnx;
#[cfg(feature = "engine-tensorrt")]
use self::yolo_tensorrt::YoloTensorrt;

mod iou_helper;

#[cfg(feature = "engine-onnx")]
mod yolo_onnx;
#[cfg(feature = "engine-tensorrt")]
mod yolo_tensorrt;

pub trait ObjectDetection {
    fn process_image(&mut self, img: image::DynamicImage);
    fn detect_objects(&mut self);
    fn process_results(
        &mut self,
        origin_img_width: u32,
        origin_img_height: u32,
    ) -> Vec<TrackingResult>;
}

pub fn process_img(
    object_detection: &mut impl ObjectDetection,
    img: DynamicImage,
    origin_img_width: u32,
    origin_img_height: u32,
) -> ReturnData {
    let start_time = Instant::now();

    object_detection.process_image(img);
    object_detection.detect_objects();
    let output = object_detection.process_results(origin_img_width, origin_img_height);

    // let _ = real_img.save("test.jpg");
    let total_time = start_time.elapsed();
    ReturnData {
        tracking_time: total_time,
        tracking_results: output,
    }
}

pub fn new_object_detection() -> impl ObjectDetection {
    #[cfg(feature = "engine-onnx")]
    let object_detection = YoloOnnx::new();

    #[cfg(feature = "engine-tensorrt")]
    let object_detection = YoloTensorrt::new();

    object_detection
}
