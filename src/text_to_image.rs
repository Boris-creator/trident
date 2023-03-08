use image::Rgba;
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::path::Path;

pub fn draw<T: ToString>(title: Vec<String>, table: Vec<(&str, T)>) {
    let path = Path::new("assets/template.jpg");

    let mut img = image::open(&path)
        .expect("No image found at provided path")
        .to_rgba8();
    let font = Vec::from(include_bytes!("../assets/DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = image::image_dimensions(&path).unwrap().1 as f32 * 0.05;
    let image_width = image::image_dimensions(&path).unwrap().0 as f32;
    let scale = Scale {
        x: height * 0.8,
        y: height * 0.8,
    };
    let title_scale = Scale {
        x: height * 0.7,
        y: height * 0.7,
    };
    let padding = 0.05 * image_width;
    let mut margin_top = padding as i32;

    for line in title {
        draw_text_mut(
            &mut img,
            Rgba([255u8, 255u8, 255u8, 1]),
            padding as i32,
            margin_top,
            title_scale,
            &font,
            &line,
        );
        let (_, h) = text_size(title_scale, &font, &line);
        let line_spacing = h as f32 * 1.4;
        margin_top += line_spacing as i32;
    }

    for row in table {
        let (w, h) = text_size(scale, &font, &row.1.to_string());
        let line_spacing = h as f32 * 1.6;
        margin_top += line_spacing as i32;
        draw_text_mut(
            &mut img,
            Rgba([255u8, 255u8, 255u8, 0.7 as u8]),
            padding as i32,
            margin_top,
            scale,
            &font,
            &row.0.to_string(),
        );
        draw_text_mut(
            &mut img,
            Rgba([255u8, 255u8, 255u8, 1]),
            (image_width - w as f32 - padding) as i32,
            margin_top,
            scale,
            &font,
            &row.1.to_string(),
        );
    }
    let _ = img.save(Path::new("assets/result.jpg")).unwrap();
}
