use std::path::Path;

#[test]
fn test_inference() {
    let engine_path = Path::new("tests/yolov8.engine");
    let mut yolov8_ptr = yolov8_bindings::new_engine(engine_path);
    yolov8_bindings::make_pipe(yolov8_ptr.pin_mut());
    let image = image::open("tests/bus.jpg").unwrap();
    yolov8_bindings::copy_from_image(yolov8_ptr.pin_mut(), image);
    let time = yolov8_bindings::infer(yolov8_ptr.pin_mut());

    println!("Inference time: {:?}", time);
}
