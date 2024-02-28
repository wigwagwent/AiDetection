use std::io::Cursor;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use image::{ImageOutputFormat, Rgba};
use imageproc::drawing::{draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use shared_types::server::ProcessingStatus;

use crate::ImageStore;

pub fn image_html(image_store: ImageStore) -> String {
    let mut latest_processed: Option<usize> = None;
    for image in image_store.iter() {
        if image.detection_status == ProcessingStatus::Finished
            && (latest_processed.is_none() || image.key() > &latest_processed.unwrap())
        {
            latest_processed = Some(image.key().clone());
        }
    }

    if latest_processed.is_none() {
        return r#"No processed pictures found"#.into();
    }

    let image = image_store.get(&latest_processed.unwrap()).unwrap();
    let outlines = image
        .tracked
        .clone()
        .expect("Tracking results said it was done");
    let time = image.detection_time.unwrap();
    let mut image = image.raw.clone();
    let mut items_in_img: String = String::new();

    for rectangle in outlines {
        items_in_img += format!(
            "Item: {}, Probability: {:.2}<br>",
            rectangle.label.as_string(),
            rectangle.probability
        )
        .as_str();
        draw_hollow_rect_mut(
            &mut image,
            Rect::at(rectangle.x_bottom_corner, rectangle.y_bottom_corner)
                .of_size(rectangle.x_length, rectangle.y_height),
            Rgba([255, 0, 0, 0]),
        );

        let font_data: &[u8] = include_bytes!("MartianMono-NrRg.ttf");
        let font = Font::try_from_bytes(font_data).expect("Error constructing Font");
        draw_text_mut(
            &mut image,
            Rgba([255, 0, 0, 0]),
            rectangle.x_bottom_corner + 2,
            rectangle.y_bottom_corner + 2,
            Scale::uniform(20.0),
            &font,
            &rectangle.label.as_string(),
        );
    }

    let mut image_data: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = BASE64.encode(image_data);

    format!(
        r#"
            <html>
                <head>
                    <style>
                        body {{
                            background-color: black;
                            color: white;
                        }}
                </style>
            </head>
                <body>
                    <img id="exampleImage" src="data:image/png;base64,{}" alt="Example Image">
                    <p>Processing Time: {:?}</p>
                    <p>{}</p>
                    <script>
                        setTimeout(function () {{ location.reload(true); }}, 500);
                    </script>
                </body>
            </html>
        "#,
        res_base64, time, items_in_img
    )
}
