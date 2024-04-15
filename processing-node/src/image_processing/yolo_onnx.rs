use image::GenericImageView;
use ndarray::{s, Array, Axis, IxDyn};
use ort::{
    inputs, CPUExecutionProvider, CUDAExecutionProvider, ExecutionProvider, Session,
    TensorRTExecutionProvider,
};

#[allow(unused_imports)] //based on features
use shared_types::tracking::{
    yolo::{YoloClasses80, YoloClassesOIV7},
    TrackingResult,
};

use super::{
    iou_helper::{get_img_resized_size, iou_on_tracking_results},
    ObjectDetection,
};

pub struct YoloOnnx {
    model: Session,
    prepared_img: Array<f32, IxDyn>,
    processed_img: Array<f32, IxDyn>,
}

impl YoloOnnx {
    pub fn new() -> Self {
        let model: Session = {
            let provider = {
                let tensorrt = TensorRTExecutionProvider::default().build();
                let cuda = CUDAExecutionProvider::default().build();
                if tensorrt.is_available().unwrap() {
                    println!("Using TensorRT");
                    tensorrt
                } else if cuda.is_available().unwrap() {
                    println!("Using CUDA");
                    cuda
                } else {
                    println!("Using CPU");
                    CPUExecutionProvider::default().build()
                }
            };

            ort::init()
                .with_execution_providers([provider])
                .commit()
                .unwrap();

            #[cfg(feature = "model-yolov8s")]
            let model = Session::builder()
                .unwrap()
                .with_model_from_file("models/yolov8s.onnx")
                .unwrap();
            #[cfg(feature = "model-yolov8n")]
            let model = Session::builder()
                .unwrap()
                .with_model_from_file("models/yolov8n.onnx")
                .unwrap();
            #[cfg(feature = "model-yolov8m")]
            let model = Session::builder()
                .unwrap()
                .with_model_from_file("models/yolov8m.onnx")
                .unwrap();
            #[cfg(feature = "model-yolov8s-oiv7")]
            let model = Session::builder()
                .unwrap()
                .with_model_from_file("models/yolov8s-oiv7.onnx")
                .unwrap();
            model
        };
        Self {
            model,
            prepared_img: Array::zeros((1, 3, 640, 640)).into_dyn(),
            processed_img: Array::zeros(0).into_dyn(), //gets replaced
        }
    }
}

impl ObjectDetection for YoloOnnx {
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
        let outputs = self.model.run(inputs![model_inputs].unwrap()).unwrap();
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
    fn process_results(
        &mut self,
        origin_img_width: u32,
        origin_img_height: u32,
    ) -> Vec<TrackingResult> {
        let output = self.processed_img.slice(s![.., .., 0]);
        let tracking_data = {
            let mut tracking_data = Vec::new();
            for row in output.axis_iter(Axis(0)) {
                let row: Vec<_> = row.iter().copied().collect();
                let (class_id, confidence) = row
                    .iter()
                    .skip(4)
                    .enumerate()
                    .map(|(index, value)| (index, *value))
                    .reduce(|accum, row| if row.1 > accum.1 { row } else { accum })
                    .unwrap();

                if confidence < 0.5 {
                    continue;
                }

                #[cfg(any(
                    feature = "model-yolov8s",
                    feature = "model-yolov8n",
                    feature = "model-yolov8m"
                ))]
                let label = shared_types::tracking::ItemLabel::YoloClasses80(
                    YoloClasses80::from_repr(class_id as usize).unwrap(),
                );

                #[cfg(feature = "model-yolov8s-oiv7")]
                let label = shared_types::tracking::ItemLabel::YoloClassesOIV7(
                    YoloClassesOIV7::from_repr(class_id).unwrap(),
                );

                let (width, height) = get_img_resized_size(origin_img_width, origin_img_height);

                //row[0] x center
                //row[1] y center
                //row[2] width
                //row[3] height
                let item_box = TrackingResult {
                    label,
                    confidence,
                    x0: (((row[0] - row[2] / 2.0) / width) * origin_img_width as f32) as i32,
                    y0: (((row[1] - row[3] / 2.0) / height) * origin_img_height as f32) as i32,
                    x1: (((row[0] + row[2] / 2.0) / width) * origin_img_width as f32) as i32,
                    y1: (((row[1] + row[3] / 2.0) / height) * origin_img_height as f32) as i32,
                };

                tracking_data.push(item_box);
            }
            iou_on_tracking_results(tracking_data)
        };
        tracking_data
    }
}
