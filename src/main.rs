use std::f32;
use std::io;

extern crate image;
use image::{ImageBuffer, Rgba};
extern crate rand;
use rand::prelude::*;

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

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let rand = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let p = rand * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p
        }
    }
}


struct HitRec {
    t: f32,
    p: Vec3,
    normal: Vec3
}


trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRec>;
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

    fn cast(&self, rng: &mut ThreadRng, target: &impl Hittable) -> Vec3 {
        if let Some(rec) = target.hit(self, 0.0001, f32::MAX) {
            let dir = rec.normal + random_in_unit_sphere(rng);
            Ray::new(rec.p, dir).cast(rng, target) * 0.5
        } else {
            let unit = self.direction.norm();
            let t = 0.5 * (unit.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
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
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRec> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let b = oc.dot(ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let delta = b * b - a * c;
        if delta < 0.0 {
            return None;
        } 
        let mut solution = None;
        let left_solution = (-b - delta.sqrt()) / a;
        if left_solution < t_max && left_solution > t_min {
            solution = Some(left_solution);
        }
        if solution.is_none() {
            let right_solution = (-b + delta.sqrt()) / a;
            if right_solution < t_max && right_solution > t_min {
                solution = Some(right_solution);
            }
        }
        solution.map(|t| {
            let p = ray.point_at(t);
            let normal = (p - self.center) / self.radius;
            HitRec { t, p, normal }
        })
    }
}


struct Hittables {
    data: Vec<Box<Hittable>>
}

impl Hittables {
    fn new() -> Self {
        Hittables { data: Vec::new() }
    }

    fn add<H: Hittable + 'static>(&mut self, object: H) {
        self.data.push(Box::new(object))
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRec> {
        let mut res = None;
        let mut closest = t_max;
        for hittable in self.data.iter() {
            let next_hit = hittable.hit(ray, t_min, closest);
            if let Some(rec) = next_hit {
                closest = rec.t;
                res = Some(rec);
            }
        }
        res
    }
}


fn main() -> io::Result<()> {
    let mut hittables = Hittables::new();
    hittables.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    hittables.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut rng = rand::thread_rng();
    let samples = 100;

    let img = ImageBuffer::from_fn(200, 100, |x, y| {
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples {
            let u = ((x as f32) + rng.gen::<f32>()) / 200.0;
            // We want the y coordinate to go up
            let v = 1.0 - ((y as f32) - rng.gen::<f32>()) / 100.0;
            let pos = lower_left + horizontal * u + vertical * v;
            col += Ray::new(origin, pos).cast(&mut rng, &hittables);
        }
        col /= samples as f32;
        col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
        color(col.x, col.y, col.z, 1.0)
    });
    img.save("image.png")?;
    Ok(())
}
