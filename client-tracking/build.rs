//use std::process::Command;

fn main() {
    // // Determine which model and export type to use based on feature flags
    // let engine = {
    //     if cfg!(feature = "engine-onnx") {
    //         "--onnx"
    //     } else if cfg!(feature = "engine-tensorrt") {
    //         "--tensorrt"
    //     } else {
    //         ""
    //     }
    // };

    // let model = {
    //     if cfg!(feature = "model-yolov8s") {
    //         "yolov8s"
    //     } else if cfg!(feature = "model-yolov8n") {
    //         "yolov8n"
    //     } else if cfg!(feature = "model-yolov8m") {
    //         "yolov8m"
    //     } else if cfg!(feature = "model-yolov8s-oiv7") {
    //         "yolov8s-oiv7"
    //     } else {
    //         ""
    //     }
    // };

    // let string_command = format!("get_models.py --model {} {}", model, engine);
    // println!("{}", string_command);
    // Command::new("python")
    //     .arg("get_models.py")
    //     //.arg(format!("--model {}", model))
    //     .arg("--model")
    //     .arg(model)
    //     .arg(engine)
    //     .output()
    //     .expect("Failed to execute Python script");
}
