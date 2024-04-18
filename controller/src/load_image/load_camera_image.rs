use super::LoadImages;
use crate::{ImageStore, NEXT_IMAGE_ID};
use image::{DynamicImage, ImageBuffer, ImageFormat};
use libcamera::{
    camera_manager::CameraManager,
    framebuffer_allocator::{FrameBuffer, FrameBufferAllocator},
    framebuffer_map::MemoryMappedFrameBuffer,
    geometry::Size,
    pixel_format::PixelFormat,
    request::ReuseFlag,
    stream::StreamRole,
};
use shared_types::server::{ImageManager, ProcessingStatus};
use std::{thread, time::Duration};

const PIXEL_FORMAT_YUYV: PixelFormat =
    PixelFormat::new(u32::from_le_bytes([b'Y', b'U', b'Y', b'V']), 0);

pub struct CameraImage {
    framerate: f32,
    size: Size,
}

impl Default for CameraImage {
    fn default() -> Self {
        Self {
            framerate: 5.0,
            size: Size {
                width: 1920,
                height: 1080,
            },
        }
    }
}

impl LoadImages for CameraImage {
    fn get_image(&mut self, store: &ImageStore) {
        let mut count = 0;

        let camera_manager = CameraManager::new().unwrap();
        let cameras = camera_manager.cameras();
        let camera = cameras.get(0).expect("No Cameras Found");
        let mut camera = camera.acquire().unwrap();
        let mut config = camera
            .generate_configuration(&[StreamRole::StillCapture])
            .unwrap();
        config
            .get_mut(0)
            .unwrap()
            .set_pixel_format(PIXEL_FORMAT_YUYV);

        config.get_mut(0).unwrap().set_size(self.size);

        assert_eq!(
            config.get(0).unwrap().get_pixel_format(),
            PIXEL_FORMAT_YUYV,
            "YUYV is not supported by the camera"
        );
        println!("Available formats: {:#?}", config.get(0).unwrap().formats());
        camera.configure(&mut config).unwrap();

        let mut alloc = FrameBufferAllocator::new(&camera);

        // Allocate frame buffers for the stream
        let cfg = config.get(0).unwrap();
        let stream = cfg.stream().unwrap();
        let buffers = alloc.alloc(&stream).unwrap();

        // Convert FrameBuffer to MemoryMappedFrameBuffer, which allows reading &[u8]
        let buffers = buffers
            .into_iter()
            .map(|buf| MemoryMappedFrameBuffer::new(buf).unwrap())
            .collect::<Vec<_>>();

        // Create capture requests and attach buffers
        let mut reqs = buffers
            .into_iter()
            .enumerate()
            .map(|(i, buf)| {
                let mut req = camera.create_request(Some(i as u64)).unwrap();
                req.add_buffer(&stream, buf).unwrap();
                req
            })
            .collect::<Vec<_>>();

        let (tx, rx) = std::sync::mpsc::channel();
        camera.on_request_completed(move |req| {
            tx.send(req).unwrap();
        });

        camera.start(None).unwrap();
        camera.queue_request(reqs.pop().unwrap()).unwrap();

        loop {
            thread::sleep(Duration::from_secs_f32(
                (1.0 / 60.0) * (60.0 / self.framerate),
            ));

            let mut req = rx
                .recv_timeout(Duration::from_secs(2))
                .expect("Camera request failed");

            let framebuffer: &MemoryMappedFrameBuffer<FrameBuffer> = req.buffer(&stream).unwrap();
            let planes = framebuffer.data();
            let image_data = planes.get(0).unwrap();
            let rgb_data = yuyv_to_rgb(&image_data, self.size);

            let img = DynamicImage::ImageRgb8(
                ImageBuffer::from_raw(self.size.width, self.size.height, rgb_data).unwrap(),
            );
            req.reuse(ReuseFlag::REUSE_BUFFERS);
            camera.queue_request(req).unwrap();

            let img_id = NEXT_IMAGE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let new_img_store_val: ImageManager = ImageManager {
                image: img,
                dehazed: None,
                dehazed_status: ProcessingStatus::NotStarted,
                dehazed_time: None,
                tracked: None,
                tracked_status: ProcessingStatus::NotStarted,
                tracked_time: None,
                detection_status: ProcessingStatus::NotStarted,
                detection_time: None,
            };
            store.insert(img_id, new_img_store_val);
            NEXT_IMAGE_ID.store(img_id + 1, std::sync::atomic::Ordering::Relaxed);
            count += 1;
            if count % 100 == 0 {
                println!("Images Taken: {}", count);
            }
        }
    }
}

fn yuyv_to_rgb(yuyv_data: &[u8], size: Size) -> Vec<u8> {
    let mut rgb_data = Vec::with_capacity((size.width * size.height * 3) as usize);

    for i in (0..(size.width * size.height * 2)).step_by(4) {
        let y0 = yuyv_data[i as usize] as f32;
        let u = yuyv_data[(i + 1) as usize] as f32 - 128.0;
        let y1 = yuyv_data[(i + 2) as usize] as f32;
        let v = yuyv_data[(i + 3) as usize] as f32 - 128.0;

        let c = y0 - 16.0;
        let d = u;
        let e = v;
        let r = (298.0 * c + 409.0 * e + 128.0) / 256.0;
        let g = (298.0 * c - 100.0 * d - 208.0 * e + 128.0) / 256.0;
        let b = (298.0 * c + 516.0 * d + 128.0) / 256.0;

        rgb_data.push(r.max(0.0).min(255.0) as u8);
        rgb_data.push(g.max(0.0).min(255.0) as u8);
        rgb_data.push(b.max(0.0).min(255.0) as u8);

        let c = y1 - 16.0;
        let r = (298.0 * c + 409.0 * e + 128.0) / 256.0;
        let g = (298.0 * c - 100.0 * d - 208.0 * e + 128.0) / 256.0;
        let b = (298.0 * c + 516.0 * d + 128.0) / 256.0;

        rgb_data.push(r.max(0.0).min(255.0) as u8);
        rgb_data.push(g.max(0.0).min(255.0) as u8);
        rgb_data.push(b.max(0.0).min(255.0) as u8);
    }

    rgb_data
}
