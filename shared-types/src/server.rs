use std::sync::atomic::AtomicBool;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize)]
pub enum SentDataType {
    Image,
    ImageProperties,
}

#[derive(Serialize, Deserialize)]
pub struct SentData {
    pub data_type: SentDataType,
    pub raw_data: Vec<u8>,
}

pub struct ClientData {
    pub link: mpsc::UnboundedSender<warp::filters::ws::Message>,
    pub busy: AtomicBool,
}
