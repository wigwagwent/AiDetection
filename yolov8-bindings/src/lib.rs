use cxx::UniquePtr;
use image::{DynamicImage, GenericImageView};
use std::{fs::File, io::Read, path::Path, pin::Pin, time::Duration};

pub fn new_engine(engine_path: &Path) -> UniquePtr<ffi::YoloV8> {
    let trt_model_stream = read_model_file(engine_path);
    ffi::new_engine(trt_model_stream)
}

pub fn make_pipe(yolov8: Pin<&mut ffi::YoloV8>) {
    yolov8.make_pipe();
}

pub fn copy_from_image(yolov8: Pin<&mut ffi::YoloV8>, image: DynamicImage) {
    let (width, height) = image.dimensions();
    let mut image = image;
    if width != 640 && height != 640 {
        println!("Resizing image to 640x640");
        image = image.resize_to_fill(640, 640, image::imageops::FilterType::Nearest);
    }
    let (width, height) = image.dimensions();
    let image = rgb_to_bgr_bytes(&image);
    yolov8.copy_from_image(image, width as i32, height as i32);
}

pub fn infer(yolov8: Pin<&mut ffi::YoloV8>) -> Duration {
    let start_time = std::time::Instant::now();
    yolov8.infer();
    let end_time = std::time::Instant::now();
    end_time.duration_since(start_time)
}

pub fn get_results(yolov8: Pin<&mut ffi::YoloV8>) -> Vec<ffi::Result> {
    let results = yolov8.get_results();
    let restults3 = results.clone();
    restults3
}

fn read_model_file(file_path: &Path) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    println!("Length of model file: {}", buffer.len());
    buffer
}

fn rgb_to_bgr_bytes(img: &DynamicImage) -> Vec<u8> {
    let (width, height) = img.dimensions();
    let mut bgr_bytes = vec![0; (width * height * 3) as usize]; // 3 bytes per pixel (BGR)

    // Convert RGB to BGR
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;
            let bgr_pixel = [pixel[2], pixel[1], pixel[0]]; // Rearrange RGB channels to BGR
            let index = ((y * width + x) * 3) as usize;
            bgr_bytes[index] = bgr_pixel[0];
            bgr_bytes[index + 1] = bgr_pixel[1];
            bgr_bytes[index + 2] = bgr_pixel[2];
        }
    }

    bgr_bytes
}

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
