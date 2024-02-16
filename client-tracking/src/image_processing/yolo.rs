use image::GenericImageView;
use lazy_static::lazy_static;
use ndarray::{s, Array, Axis, IxDyn};
use ort::{
    inputs, CPUExecutionProvider, CUDAExecutionProvider, Session, TensorRTExecutionProvider,
};

#[allow(unused_imports)] //based on features
use shared_types::tracking::{
    yolo::{YoloClasses80, YoloClassesOIV7},
    TrackingResult,
};
use std::cmp::Ordering;

use super::ObjectDetection;

lazy_static! {
    static ref MODEL: Session = {
        ort::init()
            .with_execution_providers([
                TensorRTExecutionProvider::default().build(),
                CUDAExecutionProvider::default().build(),
                CPUExecutionProvider::default().build(),
            ])
            .commit()
            .unwrap();

        #[cfg(feature = "model-yolov8-s")]
        let model = Session::builder()
            .unwrap()
            .with_model_from_file("models/model-yolov8-s.onnx")
            .unwrap();
        #[cfg(feature = "model-yolov8-n")]
        let model = Session::builder()
            .unwrap()
            .with_model_from_file("models/model-yolov8-n.onnx")
            .unwrap();
        #[cfg(feature = "model-yolov8-m")]
        let model = Session::builder()
            .unwrap()
            .with_model_from_file("models/model-yolov8-m.onnx")
            .unwrap();
        #[cfg(feature = "model-yolov8-s-oiv7")]
        let model = Session::builder()
            .unwrap()
            .with_model_from_file("models/model-yolov8-s-oiv7.onnx")
            .unwrap();
        model
    };
}

pub struct Yolo {
    prepared_img: Array<f32, IxDyn>,
    processed_img: Array<f32, IxDyn>,
    origin_img_width: u32,
    origin_img_height: u32,
}

impl Yolo {
    pub fn new(origin_img_width: u32, origin_img_height: u32) -> Self {
        Self {
            prepared_img: Array::zeros((1, 3, 640, 640)).into_dyn(),
            processed_img: Array::zeros(0).into_dyn(), //gets replaced
            origin_img_width,
            origin_img_height,
        }
    }
}

impl ObjectDetection for Yolo {
    /// Function used to convert input image to tensor,
    /// required as an input to YOLOv8 object detection
    /// network.
    /// Returns the input tensor, original image width and height
    fn process_image(&mut self, img: image::DynamicImage) {
        //let img = img.resize_exact(640, 640, FilterType::CatmullRom); //Should be resized from the server,
        // TODO: send size the client want to the server
        for pixel in img.pixels() {
            let x = pixel.0 as usize;
            let y = pixel.1 as usize;
            let [r, g, b, _] = pixel.2 .0;
            self.prepared_img[[0, 0, y, x]] = (r as f32) / 255.0;
            self.prepared_img[[0, 1, y, x]] = (g as f32) / 255.0;
            self.prepared_img[[0, 2, y, x]] = (b as f32) / 255.0;
        }
    }

    /// Function used to pass provided input tensor to
    /// YOLOv8 neural network and return result
    /// Returns raw output of YOLOv8 network as a single dimension
    /// array
    fn detect_objects(&mut self) {
        let input_as_values = &self.prepared_img.as_standard_layout();
        let model_inputs = ort::Value::from_array(input_as_values).unwrap();
        let outputs = MODEL.run(inputs![model_inputs].unwrap()).unwrap();
        let output = outputs["output0"]
            .extract_tensor::<f32>()
            .unwrap()
            .view()
            .t()
            .into_owned();
        self.processed_img = output;
    }

    /// Function used to convert RAW output from YOLOv8 to an array
    /// of detected objects. Each object contain the bounding box of
    /// this object, the type of object and the probability
    /// Returns array of detected objects in a format [(x1,y1,x2,y2,object_type,probability),..]
    fn process_results(&self) -> Vec<TrackingResult> {
        let mut boxes = Vec::new();
        let output = self.processed_img.slice(s![.., .., 0]);
        for row in output.axis_iter(Axis(0)) {
            let row: Vec<_> = row.iter().copied().collect();
            let (class_id, prob) = row
                .iter()
                .skip(4)
                .enumerate()
                .map(|(index, value)| (index, *value))
                .reduce(|accum, row| if row.1 > accum.1 { row } else { accum })
                .unwrap();
            if prob < 0.5 {
                continue;
            }

            #[cfg(feature = "model-yolov8-s")]
            let label =
                shared_types::tracking::ItemLabel::YoloClasses80(YoloClasses80::from(class_id));

            #[cfg(feature = "model-yolov8-n")]
            let label =
                shared_types::tracking::ItemLabel::YoloClasses80(YoloClasses80::from(class_id));

            #[cfg(feature = "model-yolov8-m")]
            let label =
                shared_types::tracking::ItemLabel::YoloClasses80(YoloClasses80::from(class_id));

            #[cfg(feature = "model-yolov8-s-oiv7")]
            let label =
                shared_types::tracking::ItemLabel::YoloClassesOIV7(YoloClassesOIV7::from(class_id));

            let xc = row[0] / 640.0 * (self.origin_img_width as f32);
            let yc = row[1] / 640.0 * (self.origin_img_height as f32);
            let w = row[2] / 640.0 * (self.origin_img_width as f32);
            let h = row[3] / 640.0 * (self.origin_img_height as f32);
            let item_box = TrackingResult {
                x_bottom_corner: (xc - w / 2.0) as i32,
                y_bottom_corner: (yc - h / 2.0) as i32,
                x_length: w as u32,
                y_height: h as u32,
                label,
                probability: prob,
            };
            boxes.push(item_box);
        }

        boxes.sort_by(|a, b| {
            b.probability
                .partial_cmp(&a.probability)
                .unwrap_or(Ordering::Equal)
        });

        let mut result = Vec::new();

        while !boxes.is_empty() {
            result.push(boxes[0]);
            let first_result = boxes[0];
            boxes.retain(|box1| iou(&first_result, box1) < 0.7);
        }
        result
    }
}

/// Function calculates "Intersection-over-union" coefficient for specified two boxes
/// https://pyimagesearch.com/2016/11/07/intersection-over-union-iou-for-object-detection/.
/// Returns Intersection over union ratio as a float number
fn iou(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    intersection(box1, box2) / union(box1, box2)
}

/// Function calculates union area of two boxes
/// Returns Area of the boxes union as a float number
fn union(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    let box1_area = box1.x_length * box1.y_height;
    let box2_area = box2.x_length * box2.y_height;
    (box1_area + box2_area) as f32 - intersection(box1, box2)
}

/// Function calculates intersection area of two boxes
/// Returns Area of intersection of the boxes as a float number
fn intersection(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    let x_bottom = box1.x_bottom_corner.max(box2.x_bottom_corner);
    let y_bottom = box1.y_bottom_corner.max(box2.y_bottom_corner);
    let x_top = (box1.x_bottom_corner + box1.x_length as i32)
        .min(box2.x_bottom_corner + box2.x_length as i32);
    let y_top = (box1.y_bottom_corner + box1.y_height as i32)
        .min(box2.y_bottom_corner + box2.y_height as i32);
    ((x_top - x_bottom) as f32) * ((y_top - y_bottom) as f32)
}
