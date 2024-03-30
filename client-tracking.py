import requests
import cv2
import numpy as np
import torch
from models import TRTModule
from models.torch_utils import det_postprocess
from models.utils import blob, letterbox, path_to_list

# Function to fetch image from API
def fetch_image(api_url):
    try:
        response = requests.get(api_url)
        # Check if request was successful
        if response.status_code == 200:
            # Decode the image
            img_array = np.frombuffer(response.content, dtype=np.uint8)
            img = cv2.imdecode(img_array, cv2.IMREAD_COLOR)
            return img
        else:
            print("Failed to fetch image. Status code:", response.status_code)
            return None
    except Exception as e:
        print("Error fetching image:", str(e))
        return None

# Function to process image with TensorRT model
def process_image_with_tensorrt(image, engine):
    H, W = engine.inp_info[0].shape[-2:]

    bgr, ratio, dwdh = letterbox(image, (W, H))
    rgb = cv2.cvtColor(bgr, cv2.COLOR_BGR2RGB)
    tensor = blob(rgb, return_seg=False)
    dwdh = torch.asarray(dwdh * 2, dtype=torch.float32)
    tensor = torch.asarray(tensor)

    # inference
    data = engine(tensor)

    bboxes, scores, labels = det_postprocess(data)
    if bboxes.numel() == 0:
        # if no bounding box
        return []

    bboxes -= dwdh
    bboxes /= ratio

    detections = []
    for (bbox, score, label) in zip(bboxes, scores, labels):
        bbox = bbox.round().int().tolist()
        cls_id = int(label)
        cls = CLASSES[cls_id]
        detections.append({"bbox": bbox, "score": score, "class": cls})

    return detections

if __name__ == "__main__":
    # Default device
    device = torch.device('cuda:0')

    # YOLOv8 engine file path
    engine_file = "./models/yolov8n.engine"

    # Load TensorRT engine
    Engine = TRTModule(engine_file, device)

    # API URL to fetch images from
    api_url = input("Enter API URL: ")

    while True:
        # Fetch image from API
        image = fetch_image(api_url)
        if image is not None:
            # Process image with TensorRT model
            detections = process_image_with_tensorrt(image, Engine)

            # Handle detections data
            for detection in detections:
                print("Bounding Box:", detection["bbox"])
                print("Score:", detection["score"])
                print("Class:", detection["class"])

        else:
            print("Failed to fetch image. Exiting...")
            break
