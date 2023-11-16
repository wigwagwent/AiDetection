use futures_channel::mpsc::UnboundedSender;
use image::{DynamicImage, EncodableLayout, ImageBuffer};
use shared_types::{
    client::{ReturnData, ReturnDataType},
    ImageProperties,
};
use tokio::time::Instant;
use tokio_tungstenite::tungstenite::Message;
mod yolo;

pub fn receive_img(raw_img: Vec<u8>, tx: UnboundedSender<Message>) {
    let start_time = Instant::now();
    let img: ImageProperties = bincode::deserialize(raw_img.as_bytes()).unwrap();
    let img_buf = match ImageBuffer::from_vec(img.width, img.height, img.img_buffer) {
        Some(buf) => buf,
        None => panic!("Error converting image"),
    };

    let real_img = DynamicImage::ImageRgb8(img_buf);
    // let _ = real_img.save("test.jpg");
    let output = yolo::detect_objects_on_image(real_img, img.width, img.height);

    let total_time = start_time.elapsed();
    let response: ReturnData = ReturnData {
        data_type: ReturnDataType::ListOfItems(output),
        time_cost: total_time,
    };

    tx.unbounded_send(Message::Binary(bincode::serialize(&response).unwrap()))
        .unwrap();
}
