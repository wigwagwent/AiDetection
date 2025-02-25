use cxx::UniquePtr;
use image::{DynamicImage, GenericImageView};
use yolov8_bindings;

#[allow(unused_imports)] //based on features
use shared_types::tracking::{
    yolo::{YoloClasses80, YoloClassesOIV7},
    TrackingResult,
};

use super::{
    iou_helper::{get_img_resized_size, iou_on_tracking_results},
    ObjectDetection,
};

pub struct YoloTensorrt {
    model: UniquePtr<yolov8_bindings::YoloV8>,
}

#[cfg(feature = "engine-tensorrt")]
impl YoloTensorrt {
    pub fn new() -> Self {
        Self {
            model: {
                let engine_path = std::path::Path::new("models/yolov8n.engine");
                let mut yolov8 = yolov8_bindings::new_engine(engine_path);
                yolov8.pin_mut().make_pipe();
                yolov8
            },
        }
    }
}

#[cfg(feature = "engine-tensorrt")]
impl ObjectDetection for YoloTensorrt {
    /// Function used to convert input image to tensor,
    /// required as an input to YOLOv8 object detection
    /// network.
    /// Returns the input tensor, original image width and height
    fn process_image(&mut self, img: DynamicImage) {
        let (width, height) = img.dimensions();
        let mut image = img;
        if width != 640 && height != 640 {
            println!("Resizing image to 640x640");
            image = image.resize_to_fill(640, 640, image::imageops::FilterType::Nearest);
        }
        let (width, height) = image.dimensions();
        let image = rgb_to_bgr_bytes(&image);
        self.model
            .pin_mut()
            .copy_from_image(image, width as i32, height as i32);
    }

    /// Function used to pass provided input tensor to
    /// YOLOv8 neural network and return result
    /// Returns raw output of YOLOv8 network as a single dimension
    /// array
    fn detect_objects(&mut self) {
        let start_time = std::time::Instant::now();
        self.model.pin_mut().infer();
        let end_time = std::time::Instant::now();
        //end_time.duration_since(start_time);
        println!("Inference time: {:?}", end_time.duration_since(start_time));
    }

    /// Function used to convert RAW output from YOLOv8 to an array
    /// of detected objects. Each object contain the bounding box of
    /// this object, the type of object and the probability
    /// Returns array of detected objects in a format [(x1,y1,x2,y2,object_type,probability),..]
    fn process_results(
        &mut self,
        origin_img_width: u32,
        origin_img_height: u32,
    ) -> Vec<TrackingResult> {
        let results = self.model.pin_mut().get_results().clone();
        let tracking_data = {
            let mut tracking_data: Vec<TrackingResult> = Vec::new();
            for result in &results {
                #[cfg(any(
                    feature = "model-yolov8s",
                    feature = "model-yolov8n",
                    feature = "model-yolov8m"
                ))]
                let label = shared_types::tracking::ItemLabel::YoloClasses80(
                    YoloClasses80::from_repr(result.class_id as usize).unwrap(),
                );

                #[cfg(feature = "model-yolov8s-oiv7")]
                let label = shared_types::tracking::ItemLabel::YoloClassesOIV7(
                    YoloClassesOIV7::from_repr(result.class_id).unwrap(),
                );
                let (width, height) = get_img_resized_size(origin_img_width, origin_img_height);
                let tracking_result = TrackingResult {
                    label,
                    confidence: result.confidence,
                    x0: (result.x0 as f32 / width * (origin_img_width as f32)) as i32,
                    x1: (result.x1 as f32 / width * (origin_img_width as f32)) as i32,
                    y0: (result.y0 as f32 / height * (origin_img_height as f32)) as i32,
                    y1: (result.y1 as f32 / height * (origin_img_height as f32)) as i32,
                };
                tracking_data.push(tracking_result);
            }

            iou_on_tracking_results(tracking_data)
        };
        tracking_data
    }
}

fn rgb_to_bgr_bytes(img: &DynamicImage) -> Vec<u8> {
    let (width, height) = img.dimensions();
    let mut bgr_bytes = vec![0; (width * height * 3) as usize]; // 3 bytes per pixel (BGR)

    // Convert RGB to BGR
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;
            let bgr_pixel = [pixel[2], pixel[1], pixel[0]]; // Rearrange RGB channels to BGR
            let index = ((y * width + x) * 3) as usize;
            bgr_bytes[index] = bgr_pixel[0];
            bgr_bytes[index + 1] = bgr_pixel[1];
            bgr_bytes[index + 2] = bgr_pixel[2];
        }
    }

    bgr_bytes
}
