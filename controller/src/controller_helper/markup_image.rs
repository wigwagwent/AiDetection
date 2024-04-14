use image::{DynamicImage, Rgba};
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};
use rusttype::Font;
use shared_types::tracking::TrackingResult;

pub fn add_tracking_data_to_image(
    image: &DynamicImage,
    outlines: &Option<Vec<TrackingResult>>,
) -> DynamicImage {
    let outlines = match outlines {
        Some(outlines) => outlines,
        None => return image.clone(),
    };
    let mut image = image.clone();
    for rectangle in outlines {
        draw_hollow_rect_mut(
            &mut image,
            Rect::at(rectangle.x0, rectangle.y0).of_size(
                (rectangle.x1 - rectangle.x0) as u32,
                (rectangle.y1 - rectangle.y0) as u32,
            ),
            Rgba([255, 0, 0, 0]),
        );

        let font_data: &[u8] = include_bytes!("MartianMono-NrRg.ttf");
        let _font = Font::try_from_bytes(font_data).expect("Error constructing Font");
        // draw_text_mut(
        //     &mut image,
        //     Rgba([255, 0, 0, 0]),
        //     rectangle.x0 + 2,
        //     rectangle.y0 + 2,
        //     Scale::uniform(20.0),
        //     &font,
        //     &rectangle.label.as_string(),
        // );
    }
    image
}
