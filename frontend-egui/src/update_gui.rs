use eframe::egui;
use shared_types::server::ImageInformation;
use tokio::sync::mpsc;
use tokio::time::Duration;

use crate::{GuiUpdateCommands, ImageStore};

pub async fn update_image_task(
    ctx: &egui::Context,
    tx_gui: mpsc::UnboundedSender<GuiUpdateCommands>,
    image_store: ImageStore,
    framerate: f32,
) {
    loop {
        //let time = Duration::from_secs_f32(1.0 / framerate);
        //tokio::time::sleep(time).await;
        let mut image_store = image_store.lock().await;
        if image_store.len() > 10 {
            while image_store.len() > 30 {
                let _ = image_store.pop();
            }
            let image = image_store.pop();
            drop(image_store); // Release the lock

            if let Some(image) = image {
                let egui_image = egui::ColorImage::from_rgb(
                    [
                        image.0.props.image_props.origin_width as usize,
                        image.0.props.image_props.origin_height as usize,
                    ],
                    image.0.image.as_ref(),
                );
                let texture = ctx.load_texture("my-image", egui_image, Default::default());
                tx_gui
                    .send(GuiUpdateCommands::UpdateImage(
                        texture,
                        format_info_string(image.0.props),
                    ))
                    .expect("Failed to send message");
                ctx.request_repaint();
            }
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
                "- Label: {}, Confidence: {:.1}%\n",
                result.label.as_string(),
                result.confidence * 100.0
            );
        }
    } else {
        info_string += "\nDetected Objects: N/A";
    }

    info_string
}
