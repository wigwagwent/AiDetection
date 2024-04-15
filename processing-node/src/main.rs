use image::DynamicImage;
use reqwest;
use std::env;
use tokio::sync::mpsc;

mod image_processing;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ip:port>", args[0]);
        return;
    }
    let controller_url = format!("http://{}/api/v1/tracking", &args[1]);

    let (sender, mut receiver) = mpsc::channel(2);

    let url = controller_url.clone();
    let load_img_task = tokio::spawn(async move {
        loop {
            let data = load_latest_img(&url).await;
            let img_data = match data {
                Ok(img_data) => img_data,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    continue;
                }
            };
            sender.send(img_data).await.expect("send failed");
        }
    });

    let process_img_task = tokio::spawn(async move {
        loop {
            process_img_setup(&mut receiver, &controller_url).await;
        }
    });

    tokio::try_join!(load_img_task, process_img_task).unwrap();
}

async fn load_latest_img(url: &str) -> Result<ImagePropertiesWithImage, anyhow::Error> {
    let image_details: shared_types::ImageProperties = {
        let url = format!("{}/latest-image-data", url);
        reqwest::get(&url)
            .await?
            .json::<shared_types::ImageProperties>()
            .await?
    };

    let image: DynamicImage = {
        let url = format!("{}/image/{}", url, image_details.img_id);
        let image_data = reqwest::get(&url).await?.bytes().await?;
        let dynamic_image =
            image::load_from_memory_with_format(&image_data, image::ImageFormat::Jpeg)?;
        dynamic_image
    };

    let img_data = ImagePropertiesWithImage {
        image_id: image_details.img_id,
        origin_img_width: image_details.origin_width,
        origin_img_height: image_details.origin_height,
        image,
    };

    Ok(img_data)
}

async fn process_img_setup(receiver: &mut mpsc::Receiver<ImagePropertiesWithImage>, url: &str) {
    let mut obj_dec = image_processing::new_object_detection();

    loop {
        if let Some(img_data) = receiver.recv().await {
            let results = image_processing::process_img(
                &mut obj_dec,
                img_data.image,
                img_data.origin_img_width,
                img_data.origin_img_height,
            );

            let url = format!("{}/image-data/{}", url, img_data.image_id);
            reqwest::Client::new()
                .post(&url)
                .json(&results)
                .send()
                .await
                .expect("Failed to send results");
        }
    }
}

struct ImagePropertiesWithImage {
    image_id: usize,
    origin_img_width: u32,
    origin_img_height: u32,
    image: DynamicImage,
}
