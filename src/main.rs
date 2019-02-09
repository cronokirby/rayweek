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

    fn cast(&self) -> Vec3 {
        if Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5).hits(self) {
            return Vec3::new(1.0, 0.0, 0.0);
        }

        let mut unit = self.direction;
        unit.norm();
        let t = 0.5 * (unit.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    fn hits(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let b = oc.dot(ray.direction) * 2.0;
        let c = oc.dot(oc) - self.radius * self.radius;
        let delta = b * b - 4.0 * a * c;
        delta > 0.0
    }
}


fn main() -> io::Result<()> {
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let img = ImageBuffer::from_fn(400, 200, |x, y| {
        let u = (x as f32) / 400.0;
        // We want the y coordinate to go up
        let v = 1.0 - (y as f32) / 200.0;
        let pos = lower_left + horizontal * u + vertical * v;
        let col = Ray::new(origin, pos).cast();
        color(col.x, col.y, col.z, 1.0)
    });
    img.save("image.png")?;
    Ok(())
}
