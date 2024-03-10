feature_utils::mandatory_and_unique!("model-yolov8s", "model-yolov8n", "model-yolov8m", "model-yolov8s-oiv7");

use std::process::Command;
use std::fs;

fn main() {
    // Check if all model files exist in the "models" directory
    let model_files = vec!["yolov8s.onnx", "yolov8n.onnx", "yolov8m.onnx", "yolov8s-oiv7.onnx"];
    let mut all_models_exist = true;
    for model_file in &model_files {
        if !fs::metadata(format!("models/{}", model_file)).is_ok() {
            all_models_exist = false;
            break;
        }
    }

    // If not all models exist, execute the Python script to fetch them
    if !all_models_exist {
        let output = Command::new("python")
            .arg("get_models.py")
            .output()
            .expect("Failed to execute Python script");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}


