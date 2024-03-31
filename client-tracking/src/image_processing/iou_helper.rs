use shared_types::tracking::TrackingResult;

/// Function calculates "Intersection-over-union" coefficient for specified two boxes
/// https://pyimagesearch.com/2016/11/07/intersection-over-union-iou-for-object-detection/.
/// Returns Intersection over union ratio as a float number
pub fn iou(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    println!("iou: {:?}", intersection(box1, box2) / union(box1, box2));
    intersection(box1, box2) / union(box1, box2)
}

/// Function calculates union area of two boxes
/// Returns Area of the boxes union as a float number
fn union(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    let box1_area = (box1.x1 - box1.x0) * (box1.y1 - box1.y0);
    let box2_area = (box2.x1 - box2.x0) * (box2.y1 - box2.y0);
    println!("box1_area: {:?}, box2_area: {:?}", box1_area, box2_area);
    assert!(box1_area >= 0);
    assert!(box2_area >= 0);
    assert!((box1.x1 - box1.x0) >= 0);
    assert!((box1.y1 - box1.y0) >= 0);
    (box1_area + box2_area) as f32 - intersection(box1, box2)
}

/// Function calculates intersection area of two boxes
/// Returns Area of intersection of the boxes as a float number
fn intersection(box1: &TrackingResult, box2: &TrackingResult) -> f32 {
    let x_bottom = box1.x0.min(box2.x0);
    let y_bottom = box1.y0.min(box2.y1);
    let x_top = box1.x1.max(box2.x1);
    let y_top = box1.y1.max(box2.y1);
    println!(
        "x_bottom: {:?}, y_bottom: {:?}, x_top: {:?}, y_top: {:?}",
        x_bottom, y_bottom, x_top, y_top
    );
    ((x_top - x_bottom) as f32) * ((y_top - y_bottom) as f32)
}
