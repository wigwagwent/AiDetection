use futures_channel::mpsc::UnboundedSender;
use image::{DynamicImage, EncodableLayout, ImageBuffer};
use shared_types::{
    client::{ReturnData, ReturnDataType},
    tracking::TrackingResult,
    ImageProperties,
};
use tokio::time::Instant;
use tokio_tungstenite::tungstenite::Message;

use self::yolo::Yolo;

mod yolo;

pub trait ObjectDetection {
    fn process_image(&mut self, img: image::DynamicImage);
    fn detect_objects(&mut self);
    fn process_results(&self) -> Vec<TrackingResult>;
}

pub fn receive_img(raw_img: Vec<u8>, tx: UnboundedSender<Message>) {
    let start_time = Instant::now();
    let img: ImageProperties = bincode::deserialize(raw_img.as_bytes()).unwrap();
    let img_buf = match ImageBuffer::from_vec(img.resize_width, img.resize_height, img.img_buffer) {
        Some(buf) => buf,
        None => panic!("Error converting image"),
    };

    let real_img = DynamicImage::ImageRgb8(img_buf);
    let mut obj_dec = new_object_detection(img.origin_width, img.origin_height);

    obj_dec.process_image(real_img);
    obj_dec.detect_objects();
    let output = obj_dec.process_results();

    // let _ = real_img.save("test.jpg");
    let total_time = start_time.elapsed();
    let response: ReturnData = ReturnData {
        img_id: img.img_id,
        time_cost: total_time,
        data_type: ReturnDataType::ListOfItemsDetected(output),
    };

    tx.unbounded_send(Message::Binary(bincode::serialize(&response).unwrap()))
        .unwrap();
}

fn new_object_detection(origin_img_width: u32, origin_img_height: u32) -> impl ObjectDetection {
    #[cfg(any(
        feature = "model-yolov8s",
        feature = "model-yolov8n",
        feature = "model-yolov8m",
        feature = "model-yolov8s-oiv7"
    ))]
    Yolo::new(origin_img_width, origin_img_height)
}
