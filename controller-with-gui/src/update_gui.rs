use bytes::Bytes;
use eframe::egui;
use shared_types::server::{ImageInformation, ProcessingStatus};
use tokio::sync::mpsc;
use tokio::time::Duration;

use crate::{
    controller_helper::markup_image::add_tracking_data_to_image, GuiUpdateCommands, ImageStore,
};

pub async fn update_image_task(
    ctx: &egui::Context,
    tx_gui: mpsc::UnboundedSender<GuiUpdateCommands>,
    image_store: ImageStore,
) {
    let mut latest_img_id = None;
    loop {
        let time = Duration::from_secs_f32(0.001);
        tokio::time::sleep(time).await;
        let new_latest_img_id = get_latest_tracked_image_id(image_store.clone());
        if latest_img_id != new_latest_img_id {
            latest_img_id = new_latest_img_id;
            let image = image_store.get(&latest_img_id.unwrap()).unwrap();

            let marked_img = add_tracking_data_to_image(&image.image, &image.detection_objects);

            let marked_bytes = Bytes::from(marked_img.to_rgb8().into_vec());

            let egui_image = egui::ColorImage::from_rgb(
                [image.image.width() as usize, image.image.height() as usize],
                &marked_bytes,
            );
            let texture = ctx.load_texture("my-image", egui_image, Default::default());
            tx_gui
                .send(GuiUpdateCommands::UpdateImage(
                    texture,
                    format_info_string(ImageInformation::new(&image, latest_img_id.unwrap())),
                ))
                .expect("Failed to send message");
            ctx.request_repaint();
        }
    }
}

pub fn format_info_string(info: ImageInformation) -> String {
    let mut info_string = format!("Image ID: {}\n", info.image_props.img_id);

    if let Some(detection_time) = info.detection_time {
        info_string += &format!("Detection Time: {:.2?}", detection_time);
    } else {
        info_string += "Detection Time: N/A";
    }

    if let Some(detection_objects) = &info.detection_objects {
        info_string += "\nDetected Objects:\n";
        for result in detection_objects {
            info_string += &format!(
                "- Confidence: {:.1}%, Label: {}\n",
                result.confidence * 100.0,
                result.label.as_string(),
            );
        }
    } else {
        info_string += "\nDetected Objects: N/A";
    }

    info_string
}

pub fn get_latest_tracked_image_id(image_store: ImageStore) -> Option<usize> {
    let mut latest_processed: Option<usize> = None;
    for image in image_store.iter() {
        if image.detection_status == ProcessingStatus::Finished
            && (latest_processed.is_none() || image.key() > &latest_processed.unwrap())
        {
            latest_processed = Some(image.key().clone());
        }
    }
    latest_processed
}
