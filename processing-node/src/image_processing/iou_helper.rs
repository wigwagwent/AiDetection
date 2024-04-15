use std::cmp::Ordering;

use shared_types::tracking::TrackingResult;

pub fn get_img_resized_size(width: u32, height: u32) -> (f32, f32) {
    let (new_width, new_height) = if width > height {
        (640, (640 * height) / width)
    } else {
        ((640 * width) / height, 640)
    };
    (new_width as f32, new_height as f32)
}

pub fn iou_on_tracking_results(mut tracking_data: Vec<TrackingResult>) -> Vec<TrackingResult> {
    tracking_data.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(Ordering::Equal)
    });

    let mut result = Vec::new();

    while !tracking_data.is_empty() {
        result.push(tracking_data[0]);
        let first_result = tracking_data[0];
        tracking_data.retain(|box1| iou(&first_result, box1) < 0.7);
    }
    result
}

/// Function calculates "Intersection-over-union" coefficient for specified two boxes
/// https://pyimagesearch.com/2016/11/07/intersection-over-union-iou-for-object-detection/.
/// Returns Intersection over union ratio as a float number
pub fn iou(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    intersection(box1, box2) / union(box1, box2)
}

/// Function calculates union area of two boxes
/// Returns Area of the boxes union as a float number
fn union(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    let box1_area = (box1.x1 - box1.x0) * (box1.y1 - box1.y0);
    let box2_area = (box2.x1 - box2.x0) * (box2.y1 - box2.y0);
    assert!((box1.x1 - box1.x0) >= 0);
    assert!((box1.y1 - box1.y0) >= 0);
    assert!(box1_area >= 0);
    assert!(box2_area >= 0);
    (box1_area + box2_area) as f32 - intersection(box1, box2)
}

/// Function calculates intersection area of two boxes
/// Returns Area of intersection of the boxes as a float number
fn intersection(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    let x_bottom = box1.x0.min(box2.x0);
    let y_bottom = box1.y0.min(box2.y1);
    let x_top = box1.x1.max(box2.x1);
    let y_top = box1.y1.max(box2.y1);
    ((x_top - x_bottom) as f32) * ((y_top - y_bottom) as f32)
}
