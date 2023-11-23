use std::{thread, time::Duration};

use shared_types::{
    server::{SentData, SentDataType},
    ImageProperties, ProcessingType,
};
use warp::filters::ws::Message;

use crate::{Clients, ImageStore, NEXT_IMAGE_ID};

pub fn controller_thread(clients: Clients, img_store: ImageStore) {
    loop {
        for client in clients.iter() {
            if client
                .client_busy
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                continue;
            }

            match &client.client_type {
                Some(client_type) => match client_type {
                    ProcessingType::Dehaze => {}
                    _ => continue,
                },
                None => continue,
            }
        }

        for client in clients.iter() {
            if client
                .client_busy
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                continue;
            }

            match &client.client_type {
                Some(client_type) => match client_type {
                    ProcessingType::ObjectDetection => {
                        let last_img = NEXT_IMAGE_ID.load(std::sync::atomic::Ordering::Relaxed) - 1;
                        let img = &img_store.get(&last_img).unwrap().raw;
                        let raw_img = ImageProperties::new_scaled(img.clone(), last_img, 640, 640);
                        let raw_data = bincode::serialize(&raw_img).unwrap();

                        let send_message = SentData {
                            data_type: SentDataType::Image,
                            raw_data,
                        };

                        let send_message = bincode::serialize(&send_message).unwrap();
                        if send_message.len() > 16777216 {
                            println!("Oversized package detected");
                            return;
                        }

                        client.link.send(Message::binary(send_message)).unwrap();
                        client
                            .client_busy
                            .store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    _ => continue,
                },
                None => continue,
            }
        }

        for client in clients.iter() {
            if client
                .client_busy
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                continue;
            }

            match &client.client_type {
                Some(client_type) => match client_type {
                    ProcessingType::Tracking => {}
                    _ => continue,
                },
                None => continue,
            }
        }

        if img_store.len() > 250 {
            let next_count = NEXT_IMAGE_ID.load(std::sync::atomic::Ordering::Relaxed);
            println!("Dropping images, only keeping the last 200");
            img_store.retain(|k, _| k > &(next_count - 200));
        }

        thread::sleep(Duration::from_secs_f32(0.01)); //check 100 times per second
    }
}
