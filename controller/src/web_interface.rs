use std::io::Cursor;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use image::ImageOutputFormat;
use shared_types::server::{ImageInformation, ProcessingStatus};

use crate::controller_helper::markup_image::add_tracking_data_to_image;
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
    let image_information = ImageInformation::new(&image);
    let image = add_tracking_data_to_image(&image.raw, outlines);
    let items_in_img = serde_json::to_string_pretty(&image_information).unwrap();

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
