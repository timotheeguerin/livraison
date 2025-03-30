use std::io::Cursor;

use colorgrad::Gradient;
use msi_installer::ui::{self, dialog::DialogSize};

pub fn background_image() -> ui::control::Bitmap {
    ui::control::bitmap("Background", "MinimalistBackground")
        .pos((0, 0))
        .size((100, 234))
}

pub fn create_background_image() -> Vec<u8> {
    let g = colorgrad::GradientBuilder::new()
        .html_colors(&["#FFF", "#00F"])
        .mode(colorgrad::BlendMode::Rgb)
        .build::<colorgrad::LinearGradient>()
        .expect("Gradient should be valid");

    let size = DialogSize::minimal();
    let width = size.width;
    let height = size.height;
    let img = image::ImageBuffer::from_fn(width as u32, height as u32, |x, _| {
        image::Rgba(g.at(x as f32 / width as f32).to_rgba8())
    });

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Valid jpeg");
    bytes
}
