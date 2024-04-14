use crate::tracking::TrackingResult;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct ReturnData {
    pub tracking_time: Duration,
    pub tracking_results: Vec<TrackingResult>,
}
