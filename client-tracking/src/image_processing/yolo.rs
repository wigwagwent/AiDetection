use image::GenericImageView;
use lazy_static::lazy_static;
use ndarray::{s, Array, Axis, IxDyn};
use ort::{Environment, Session, SessionBuilder, Value};
#[allow(unused_imports)] //based on features
use shared_types::tracking::{
    yolo::{YoloClasses80, YoloClassesOIV7},
    TrackingResult,
};
use std::{sync::Arc, vec};

use super::ObjectDetection;

lazy_static! {
    static ref MODEL: Session = {
        let env = Arc::new(Environment::builder().with_name("YOLOv8").build().unwrap());

        #[cfg(feature = "model-yolov8-s")]
        let session = SessionBuilder::new(&env)
            .unwrap()
            .with_model_from_file("model-yolov8-s.onnx")
            .unwrap();
        #[cfg(feature = "model-yolov8-n")]
        let session = SessionBuilder::new(&env)
            .unwrap()
            .with_model_from_file("model-yolov8-n.onnx")
            .unwrap();
        #[cfg(feature = "model-yolov8-m")]
        let session = SessionBuilder::new(&env)
            .unwrap()
            .with_model_from_file("model-yolov8-m.onnx")
            .unwrap();
        #[cfg(feature = "model-yolov8-s-oiv7")]
        let session = SessionBuilder::new(&env)
            .unwrap()
            .with_model_from_file("model-yolov8-s-oiv7.onnx")
            .unwrap();
        session
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
        let model_inputs = vec![Value::from_array(MODEL.allocator(), input_as_values).unwrap()];
        let outputs = MODEL.run(model_inputs).unwrap();
        let output = outputs
            .get(0)
            .unwrap()
            .try_extract::<f32>()
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
            let row: Vec<_> = row.iter().map(|x| *x).collect();
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
                x1: xc - w / 2.0,
                x2: xc + w / 2.0, //TODO: Fix me to be better
                y1: yc - h / 2.0,
                y2: yc + h / 2.0,
                label,
                probablility: prob,
            };
            boxes.push(item_box);
        }

        boxes.sort_by(|box1, box2| {
            box2.label
                .partial_cmp(&box1.label)
                .expect("I hope this works")
        });
        let mut result = Vec::new();
        while boxes.len() > 0 {
            result.push(boxes[0]);
            boxes = boxes
                .iter()
                .filter(|box1| iou(&boxes[0], box1) < 0.7)
                .map(|x| *x)
                .collect()
        }
        return result;
    }
}

// Function calculates "Intersection-over-union" coefficient for specified two boxes
// https://pyimagesearch.com/2016/11/07/intersection-over-union-iou-for-object-detection/.
// Returns Intersection over union ratio as a float number
fn iou(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    return intersection(box1, box2) / union(box1, box2);
}

// Function calculates union area of two boxes
// Returns Area of the boxes union as a float number
fn union(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    let box1_area = (box1.x2 - box1.x1) * (box1.y2 - box1.y1);
    let box2_area = (box2.x2 - box2.x1) * (box2.y2 - box2.y1);
    return box1_area + box2_area - intersection(box1, box2);
}

// Function calculates intersection area of two boxes
// Returns Area of intersection of the boxes as a float number
fn intersection(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    let x1 = box1.x1.max(box2.x1);
    let y1 = box1.y1.max(box2.y1);
    let x2 = box1.x2.min(box2.x2);
    let y2 = box1.y2.min(box2.y2);
    return (x2 - x1) * (y2 - y1);
}
