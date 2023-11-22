use crate::{tracking::TrackingResult, ProcessingType};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

#[derive(Serialize, Deserialize)]
pub enum ReturnDataType {
    ListOfItems(Vec<TrackingResult>),
    ClientType(ProcessingType),
}

#[derive(Serialize, Deserialize)]
pub struct ReturnData {
    pub img_id: usize,
    pub process_type: ProcessingType,
    pub time_cost: Duration,
    pub data_type: ReturnDataType,
}
