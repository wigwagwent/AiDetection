#pragma once
#include <memory>
#include "rust/cxx.h"
#include "NvInferPlugin.h"
#include "common.hh"
#include "fstream"

struct Result;

class YoloV8
{
public:
    YoloV8(rust::Vec<uint8_t> modelData);
    ~YoloV8();
    void make_pipe();
    void copy_from_image(rust::Vec<uint8_t> image, int32_t width, int32_t height);
    void infer();
    rust::Vec<Result> get_results();
    int num_bindings;
    int num_inputs = 0;
    int num_outputs = 0;
    std::vector<Binding> input_bindings;
    std::vector<Binding> output_bindings;
    std::vector<void *> host_ptrs;
    std::vector<void *> device_ptrs;

    PreParam pparam;

private:
    nvinfer1::ICudaEngine *engine = nullptr;
    nvinfer1::IRuntime *runtime = nullptr;
    nvinfer1::IExecutionContext *context = nullptr;
    cudaStream_t stream = nullptr;
    Logger gLogger{nvinfer1::ILogger::Severity::kERROR};
};

std::unique_ptr<YoloV8> new_engine(rust::Vec<uint8_t> trtModelStream);
