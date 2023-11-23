use std::io::Cursor;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use image::{ImageOutputFormat, Rgba};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use shared_types::server::ProcessingStatus;

use crate::ImageStore;

pub fn image_html(image_store: ImageStore) -> String {
    let images = image_store.lock().unwrap();
    let mut latest_processed: Option<usize> = None;
    for (id, image) in images.iter() {
        if image.detection_status == ProcessingStatus::Finished
            && (latest_processed.is_none() || id > &latest_processed.unwrap())
        {
            latest_processed = Some(*id);
        }
    }

    if latest_processed.is_none() {
        return r#"No processed pictures found"#.into();
    }

    let image = images.get(&latest_processed.unwrap()).unwrap();
    let outlines = image
        .tracked
        .clone()
        .expect("Tracking results said it was done");
    let mut image = image.raw.clone();
    let mut items_in_img: String = String::new();

    for rectangle in outlines {
        items_in_img += format!(
            "Item: {:#?}, Probability: {:.2}<br>",
            rectangle.label, rectangle.probability
        )
        .as_str();
        draw_hollow_rect_mut(
            &mut image,
            Rect::at(rectangle.x_bottom_corner, rectangle.y_bottom_corner)
                .of_size(rectangle.x_length, rectangle.y_height),
            Rgba([255, 0, 0, 0]),
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
                <body>
                    <img id="exampleImage" src="data:image/png;base64,{}" alt="Example Image">
                    <p>{}</p>
                    <script>
                        setTimeout(function () {{ location.reload(true); }}, 500);
                    </script>
                </body>
            </html>
        "#,
        res_base64, items_in_img
    )
}
