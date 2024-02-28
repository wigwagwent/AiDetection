#[cfg(feature = "load-camera")]
use super::LoadImages;
#[cfg(feature = "load-camera")]
use crate::{ImageStore, NEXT_IMAGE_ID};
#[cfg(feature = "load-camera")]
use libcamera::{
    camera_manager::CameraManager,
    framebuffer_allocator::{FrameBuffer, FrameBufferAllocator},
    framebuffer_map::MemoryMappedFrameBuffer,
    pixel_format::PixelFormat,
    request::ReuseFlag,
    stream::StreamRole,
};
#[cfg(feature = "load-camera")]
use shared_types::server::{ImageManager, ProcessingStatus};
#[cfg(feature = "load-camera")]
use std::{thread, time::Duration};

#[cfg(feature = "load-camera")]
const PIXEL_FORMAT_MJPEG: PixelFormat =
    PixelFormat::new(u32::from_le_bytes([b'M', b'J', b'P', b'G']), 0);
#[cfg(feature = "load-camera")]
pub struct CameraImage {
    framerate: f32,
}
#[cfg(feature = "load-camera")]
impl Default for CameraImage {
    fn default() -> Self {
        Self { framerate: 5.0 }
    }
}
#[cfg(feature = "load-camera")]
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
            .set_pixel_format(PIXEL_FORMAT_MJPEG);

        assert_eq!(
            config.get(0).unwrap().get_pixel_format(),
            PIXEL_FORMAT_MJPEG,
            "MJPEG is not supported by the camera"
        );
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

        // Completed capture requests are returned as a callback
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

            //println!("Waiting for camera request execution");
            let mut req = rx
                .recv_timeout(Duration::from_secs(2))
                .expect("Camera request failed");

            //println!("Camera request {:?} completed!", req);
            //println!("Metadata: {:#?}", req.metadata());

            // Get framebuffer for our stream
            let framebuffer: &MemoryMappedFrameBuffer<FrameBuffer> = req.buffer(&stream).unwrap();

            let planes = framebuffer.data();
            let jpeg_data = planes.get(0).unwrap();

            let img = image::load_from_memory(&jpeg_data).unwrap();
            req.reuse(ReuseFlag::REUSE_BUFFERS);
            camera.queue_request(req).unwrap();

            let img_id = NEXT_IMAGE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let new_img_store_val: ImageManager = ImageManager {
                raw: img,
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
