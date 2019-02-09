use std::io;

extern crate image;
use image::{ImageBuffer, Rgba};

mod math;
use math::Vec3;


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


#[derive(Debug)]
struct Ray {
    origin: Vec3,
    direction: Vec3
}
 
impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}


fn main() -> io::Result<()> {
    let img = ImageBuffer::from_fn(400, 300, |x, y| {
        let r = (x as f32) / 400.0;
        let g = (y as f32) / 300.0;
        color(r, g, 0.2, 1.0)
    });
    img.save("image.png")?;
    Ok(())
}
