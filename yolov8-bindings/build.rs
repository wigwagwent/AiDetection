use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/yolov8.cc");
    println!("cargo:rerun-if-changed=include/yolov8.hh");
    println!("cargo:rerun-if-changed=include/common.hh");

    println!("cargo:rerun-if-env-changed=CUDA_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=CUDA_LIBRARIES");
    println!("cargo:rerun-if-env-changed=TENSORRT_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=TENSORRT_LIBRARIES");
    println!("cargo:rerun-if-env-changed=OPENCV_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=OPENCV_LIBRARIES");

    let cuda_include_dir = match env::var("CUDA_INCLUDE_DIR") {
        Ok(val) => val,
        Err(_) => {
            println!("cargo:warning=CUDA_INCLUDE_DIR not set, using default: /usr/local/cuda-12.2/targets/aarch64-linux/include");
            "/usr/local/cuda-12.2/targets/aarch64-linux/include".to_string()
        }
    };

    let cuda_lib_dir = match env::var("CUDA_LIBRARIES") {
        Ok(val) => val,
        Err(_) => {
            println!("cargo:warning=CUDA_LIBRARIES not set, using default: /usr/local/cuda/lib64");
            "/usr/local/cuda/lib64".to_string()
        }
    };

    let tensorrt_include_dir = match env::var("TENSORRT_INCLUDE_DIR") {
        Ok(val) => val,
        Err(_) => {
            println!("cargo:warning=TENSORRT_INCLUDE_DIR not set, using default: /usr/include/aarch64-linux-gnu");
            "/usr/include/aarch64-linux-gnu".to_string()
        }
    };

    let tensorrt_lib_dir = match env::var("TENSORRT_LIBRARIES") {
        Ok(val) => val,
        Err(_) => {
            println!("cargo:warning=TENSORRT_LIBRARIES not set, using default: /usr/lib/x86_64-linux-gnu");
            "/usr/lib/x86_64-linux-gnu".to_string()
        }
    };

    let opencv_include_dir = match env::var("OPENCV_INCLUDE_DIR") {
        Ok(val) => val,
        Err(_) => {
            println!(
                "cargo:warning=OPENCV_INCLUDE_DIR not set, using default: /usr/include/opencv4"
            );
            "/usr/include/opencv4".to_string()
        }
    };

    let opencv_lib_dir = match env::var("OPENCV_LIBRARIES") {
        Ok(val) => val,
        Err(_) => {
            println!(
                "cargo:warning=OPENCV_LIBRARIES not set, using default: /usr/lib/x86_64-linux-gnu"
            );
            "/usr/lib/x86_64-linux-gnu".to_string()
        }
    };

    cxx_build::bridge("src/lib.rs")
        .file("src/yolov8.cc")
        .flag("-std=c++17")
        .include(cuda_include_dir)
        .include(opencv_include_dir)
        .include(tensorrt_include_dir)
        .include("opencv")
        .compile("test-bindings");

    println!("cargo:rustc-link-search=native={}", cuda_lib_dir);
    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rustc-link-lib=dylib=cublas");
    println!("cargo:rustc-link-lib=dylib=curand");

    println!("cargo:rustc-link-search=native={}", tensorrt_lib_dir);
    println!("cargo:rustc-link-lib=dylib=nvinfer");
    println!("cargo:rustc-link-lib=dylib=nvinfer_plugin");

    println!("cargo:rustc-link-search=native={}", opencv_lib_dir);
    println!("cargo:rustc-link-lib=dylib=opencv_core");
    println!("cargo:rustc-link-lib=dylib=opencv_imgproc");
    println!("cargo:rustc-link-lib=dylib=opencv_highgui");
    println!("cargo:rustc-link-lib=dylib=opencv_dnn");

    // println!("cargo:include={}", cuda_include_dir);
    // println!("cargo:include={}", opencv_include_dir);
    // println!("cargo:include={}", tensorrt_include_dir);
}
