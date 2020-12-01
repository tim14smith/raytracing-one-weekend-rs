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
        self.origin.plus(self.direction.times(t))
    }
}

fn ray_color(r: Ray) -> Color {
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return vtimes(Color::of(1.0, 1.0, 1.0), 1.0 - t).plus(vtimes(Color::of(0.5, 0.7, 1.0), t));
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
    let lower_left_corner = origin
        .minus(horizontal.div(2.0))
        .minus(vertical.div(2.0))
        .minus(Vec3::of(0.0, 0.0, focal_length));

    println!("P3\n{} {}\n255", image_width, image_height);

    for j1 in 0..image_height {
        let j = image_height - j1 - 1;
        eprintln!("\rScanlines remaining {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::of(
                Point3::new(), // was origin, borrow after move
                lower_left_corner
                    .plus(horizontal.times(u))
                    .plus(vertical.times(v))
                    .minus(Point3::new()), // was origin, borrow after move
            );
            let pixel_color = ray_color(r);
            write_color(pixel_color);
        }
    }
    eprintln!("Done.\n");
}
