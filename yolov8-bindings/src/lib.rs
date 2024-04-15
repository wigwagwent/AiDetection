use cxx::UniquePtr;
use std::{fs::File, io::Read, path::Path};

pub use ffi::Result;
pub use ffi::YoloV8;

pub fn new_engine(engine_path: &Path) -> UniquePtr<ffi::YoloV8> {
    let trt_model_stream = read_model_file(engine_path);
    ffi::new_engine(trt_model_stream)
}

fn read_model_file(file_path: &Path) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    println!("Length of model file: {}", buffer.len());
    buffer
}

unsafe impl Send for ffi::YoloV8 {}

#[cxx::bridge]
mod ffi {
    #[derive(Debug, Clone, Copy)]
    struct Result {
        class_id: i32,
        confidence: f32,
        x0: i32,
        x1: i32,
        y0: i32,
        y1: i32,
    }

    unsafe extern "C++" {
        include!("yolov8-bindings/include/yolov8.hh");

        type YoloV8;

        fn new_engine(modelData: Vec<u8>) -> UniquePtr<YoloV8>;

        fn make_pipe(self: Pin<&mut YoloV8>);
        fn copy_from_image(self: Pin<&mut YoloV8>, image: Vec<u8>, width: i32, height: i32);
        fn infer(self: Pin<&mut YoloV8>);
        fn get_results(self: Pin<&mut YoloV8>) -> Vec<Result>;
    }
}
