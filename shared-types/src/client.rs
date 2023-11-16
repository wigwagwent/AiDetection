use crate::yolo::ItemBox;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

#[derive(Serialize, Deserialize)]
pub enum ReturnDataType {
    ListOfItems(Vec<ItemBox>),
}

#[derive(Serialize, Deserialize)]
pub struct ReturnData {
    pub data_type: ReturnDataType,
    pub time_cost: Duration,
}
