mod vec3;
use vec3::*;
//use vec3::{unit_vector, Color, Point3, Vec3};

pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    );
}

#[derive(Clone)]
struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Vec3::new(),
            direction: Vec3::new(),
        }
    }
    pub fn of(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + (self.direction.clone() * t)
    }
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.clone().origin - center;
    let a = dot(r.clone().direction, r.clone().direction);
    let b = 2.0 * dot(oc.clone(), r.direction);
    let c = dot(oc.clone(), oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::of(0.0, 0.0, -1.0), 0.5, r.clone());
    if t > 0.0 {
        let n = unit_vector(r.clone().at(t) - Vec3::of(0.0, 0.0, -1.0));
        return Color::of(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (Color::of(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::of(0.5, 0.7, 1.0) * t);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::of(viewport_width, 0.0, 0.0);
    let vertical = Vec3::of(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone()
        - (horizontal.clone() / 2.0)
        - (vertical.clone() / 2.0)
        - Vec3::of(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j1 in 0..image_height {
        let j = image_height - j1 - 1;
        eprintln!("\rScanlines remaining {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::of(
                origin.clone(),
                lower_left_corner.clone() + (horizontal.clone() * u) + (vertical.clone() * v)
                    - origin.clone(),
            );
            let pixel_color = ray_color(r);
            write_color(pixel_color);
        }
    }
    eprintln!("Done.\n");
}
