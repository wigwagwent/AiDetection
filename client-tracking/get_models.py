import os

# List of model filenames
model_files = ['yolov8s.pt', 'yolov8n.pt', 'yolov8m.pt', 'yolov8s-oiv7.pt']

# Base URL for downloading the models
base_url = "https://github.com/ultralytics/assets/releases/download/v8.1.0/"

# Create models directory if it doesn't exist
if not os.path.exists("models"):
    os.makedirs("models")

# Loop through each model filename
for model_file in model_files:
    # Check if the model file already exists
    if not os.path.exists(f"models/{model_file}"):
        # If the file doesn't exist, download it
        os.system(f"wget {base_url}{model_file} -P models/")

    # Perform the export
    from ultralytics import YOLO
    model = YOLO(f"models/{model_file}")
    export_filename = os.path.splitext(model_file)[0] + ".onnx"
    export_path = os.path.join("models", export_filename)
    model.export(format='onnx', dynamic=True, simplify=True)
    
    # Delete the .pt file after exporting
    os.remove(f"models/{model_file}")

