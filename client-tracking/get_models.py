import argparse
import os
import urllib.request
from ultralytics import YOLO

def download_model(model_name):
    model_file = f"{model_name}.pt"
    base_url = "https://github.com/ultralytics/assets/releases/download/v8.1.0/"
    model_path = f"models/{model_file}"
    if not os.path.exists(model_path):
        os.makedirs("models", exist_ok=True)
        url = base_url + model_file
        print(f"Downloading {model_file}...")
        urllib.request.urlretrieve(url, model_path)
        print(f"{model_file} downloaded successfully.")

def convert_to_onnx(model_name):
    model_path = f"models/{model_name}.pt"
    onnx_path = f"models/{model_name}.onnx"
    if not os.path.exists(onnx_path):
        model = YOLO(model_path)
        print(f"Converting {model_path} to ONNX...")
        model.export(format='onnx', dynamic=True, simplify=True)
        print(f"{model_path} converted to ONNX successfully.")
        os.remove(model_path)
    else:
        print(f"ONNX file already exists for {model_name}. Skipping ONNX conversion.")

def convert_to_tensorrt(model_name):
    model_path = f"models/{model_name}.pt"
    engine_path = f"models/{model_name}.engine"
    if not os.path.exists(engine_path):
        print(f"Building TensorRT engine for {model_name}...")
        model = YOLO(model_path)
        model.export(format='engine', dynamic=True, simplify=True, half=True)
        print(f"TensorRT engine built successfully for {model_name}.")
        os.remove(model_path)
    else:
        print(f"TensorRT engine already exists for {model_name}. Skipping TensorRT conversion.")

def parse_args():
    parser = argparse.ArgumentParser(description="Script for downloading and converting YOLOv8 models.")
    parser.add_argument('--model', type=str, required=True, help='Model name without the file extension (e.g., yolov8s)')
    parser.add_argument('--onnx', action='store_true', help='Convert to ONNX format')
    parser.add_argument('--tensorrt', action='store_true', help='Enable TensorRT conversion')
    args = parser.parse_args()
    if not args.onnx and not args.tensorrt:
        parser.error('At least one of --onnx or --tensorrt must be chosen.')
    return args

if __name__ == '__main__':
    args = parse_args()
    download_model(args.model)
    if args.onnx:
        convert_to_onnx(args.model)
    if args.tensorrt:
        convert_to_tensorrt(args.model)

