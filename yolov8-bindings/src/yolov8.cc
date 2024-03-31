#include "yolov8-bindings/include/yolov8.hh"
#include "yolov8-bindings/src/lib.rs.h"
// #include <opencv2/opencv.hpp>

std::unique_ptr<YoloV8> new_engine(rust::Vec<uint8_t> modelData)
{
    return std::unique_ptr<YoloV8>(new YoloV8(modelData));
}

YoloV8::YoloV8(rust::Vec<uint8_t> modelData)
{
    initLibNvInferPlugins(&this->gLogger, "");
    this->runtime = nvinfer1::createInferRuntime(this->gLogger);
    assert(this->runtime != nullptr);

    this->engine = this->runtime->deserializeCudaEngine(modelData.data(), modelData.size());
    assert(this->engine != nullptr);
    this->context = this->engine->createExecutionContext();

    assert(this->context != nullptr);
    cudaStreamCreate(&this->stream);
    this->num_bindings = this->engine->getNbBindings();

    for (int i = 0; i < this->num_bindings; ++i)
    {
        Binding binding;
        nvinfer1::Dims dims;
        nvinfer1::DataType dtype = this->engine->getBindingDataType(i);
        std::string name = this->engine->getBindingName(i);
        binding.name = name;
        binding.dsize = type_to_size(dtype);

        bool IsInput = engine->bindingIsInput(i);
        if (IsInput)
        {
            this->num_inputs += 1;
            dims = this->engine->getProfileDimensions(i, 0, nvinfer1::OptProfileSelector::kMAX);
            binding.size = get_size_by_dims(dims);
            binding.dims = dims;
            this->input_bindings.push_back(binding);
            // set max opt shape
            this->context->setBindingDimensions(i, dims);
        }
        else
        {
            dims = this->context->getBindingDimensions(i);
            binding.size = get_size_by_dims(dims);
            binding.dims = dims;
            this->output_bindings.push_back(binding);
            this->num_outputs += 1;
        }
    }
}

YoloV8::~YoloV8()
{
    // this->context->destroy();
    // this->engine->destroy();
    // this->runtime->destroy();
    // cudaStreamDestroy(this->stream);
    // for (auto &ptr : this->device_ptrs)
    // {
    //     CHECK(cudaFree(ptr));
    // }

    // for (auto &ptr : this->host_ptrs)
    // {
    //     CHECK(cudaFreeHost(ptr));
    // }
}

void YoloV8::make_pipe()
{
    for (auto &bindings : this->input_bindings)
    {
        void *d_ptr;
        CHECK(cudaMalloc(&d_ptr, bindings.size * bindings.dsize));
        this->device_ptrs.push_back(d_ptr);
    }

    for (auto &bindings : this->output_bindings)
    {
        void *d_ptr, *h_ptr;
        size_t size = bindings.size * bindings.dsize;
        CHECK(cudaMalloc(&d_ptr, size));
        CHECK(cudaHostAlloc(&h_ptr, size, 0));
        this->device_ptrs.push_back(d_ptr);
        this->host_ptrs.push_back(h_ptr);
    }

    for (int i = 0; i < 10; i++)
    {
        for (auto &bindings : this->input_bindings)
        {
            size_t size = bindings.size * bindings.dsize;
            void *h_ptr = malloc(size);
            memset(h_ptr, 0, size);
            CHECK(cudaMemcpyAsync(this->device_ptrs[0], h_ptr, size, cudaMemcpyHostToDevice, this->stream));
            free(h_ptr);
        }
        this->infer();
    }
}

void YoloV8::copy_from_image(rust::Vec<uint8_t> image, int32_t width, int32_t height)
{
    cv::Mat img(height, width, CV_8UC3, image.data());
    cv::Mat nchw;
    cv::dnn::blobFromImage(img, nchw, 1 / 255.f, cv::Size(), cv::Scalar(0, 0, 0), true, false, CV_32F);

    this->context->setBindingDimensions(0, nvinfer1::Dims{4, {1, 3, height, width}});
    CHECK(cudaMemcpyAsync(
        this->device_ptrs[0], nchw.ptr<float>(), nchw.total() * nchw.elemSize(), cudaMemcpyHostToDevice, this->stream));
}

void YoloV8::infer()
{
    this->context->enqueueV2(this->device_ptrs.data(), this->stream, nullptr);
    for (int i = 0; i < this->num_outputs; i++)
    {
        size_t osize = this->output_bindings[i].size * this->output_bindings[i].dsize;
        CHECK(cudaMemcpyAsync(
            this->host_ptrs[i], this->device_ptrs[i + this->num_inputs], osize, cudaMemcpyDeviceToHost, this->stream));
    }
    cudaStreamSynchronize(this->stream);
}

rust::Vec<Result> YoloV8::get_results()
{
    rust::Vec<Result> results;
    int *num_dets = static_cast<int *>(this->host_ptrs[0]);
    auto *boxes = static_cast<float *>(this->host_ptrs[1]);
    auto *scores = static_cast<float *>(this->host_ptrs[2]);
    int *labels = static_cast<int *>(this->host_ptrs[3]);

    for (int i = 0; i < num_dets[0]; i++)
    {
        float *ptr = boxes + i * 4;

        float x0 = *ptr++;
        float y0 = *ptr++;
        float x1 = *ptr++;
        float y1 = *ptr;

        Result result;
        result.class_id = *(labels + i);
        result.confidence = *(scores + i);
        result.x0 = static_cast<int32_t>(x0);
        result.x1 = static_cast<int32_t>(x1);
        result.y0 = static_cast<int32_t>(y0);
        result.y1 = static_cast<int32_t>(y1);
        results.push_back(result);
    }
    return results;
}
