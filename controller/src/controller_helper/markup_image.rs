use image::{DynamicImage, Rgba};
use imageproc::{
    drawing::{draw_hollow_rect_mut, draw_text_mut},
    rect::Rect,
};
use rusttype::{Font, Scale};
use shared_types::tracking::TrackingResult;

pub fn add_tracking_data_to_image(
    image: &DynamicImage,
    outlines: Vec<TrackingResult>,
) -> DynamicImage {
    let mut image = image.clone();
    for rectangle in outlines {
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
    image
}
