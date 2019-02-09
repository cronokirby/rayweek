use std::io;

extern crate image;
use image::{ImageBuffer, Rgba};


fn clip(f: f32) -> f32 {
    if f > 1.0 {
        1.0
    } else if f < 0.0 {
        0.0
    } else {
        f
    }
}

fn color(r: f32, g: f32, b: f32, a: f32) -> Rgba<u8> {
    Rgba([
        (clip(r) * 255.0) as u8,
        (clip(g) * 255.0) as u8,
        (clip(b) * 255.0) as u8,
        (clip(a) * 255.0) as u8
    ])
}


fn main() -> io::Result<()> {
    let img = ImageBuffer::from_fn(1024, 768, |x, y| {
        let r = (x as f32) / 1024.0;
        let g = (y as f32) / 768.0;
        color(r, g, 0.2, 1.0)
    });
    img.save("image.png")?;
    Ok(())
}
