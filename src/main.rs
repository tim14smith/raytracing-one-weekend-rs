mod vec3;
use std::f64::consts::PI;
use std::f64::INFINITY;
use vec3::*;
//use vec3::{unit_vector, Color, Point3, Vec3};

#[derive(Clone)]
struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

trait Hittable {
    fn hit(self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.clone().origin - self.center.clone();
        let a = r.clone().direction.length_squared();
        let half_b = dot(oc.clone(), r.clone().direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return false;
            }
        }
        let new_p = r.clone().at(root);
        let outward_normal = (new_p.clone() - self.center) / self.radius;
        let fface = dot(r.direction, outward_normal.clone()) < 0.0;
        let new_normal = if fface {
            outward_normal
        } else {
            -outward_normal
        };
        *rec = HitRecord {
            t: root,
            p: new_p,
            normal: new_normal,
            front_face: fface,
        };
        true
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let temp_rec: &mut HitRecord = &mut HitRecord {
            front_face: false,
            normal: Vec3::new(),
            p: Point3::new(),
            t: 0.0,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for x in self {
            if x.hit(r.clone(), t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

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
    let a = r.clone().direction.length_squared();
    let half_b = dot(oc.clone(), r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (a)
    }
}

fn ray_color<T: Hittable>(r: Ray, world: T) -> Color {
    let rec: &mut HitRecord = &mut HitRecord {
        front_face: false,
        normal: Vec3::new(),
        p: Point3::new(),
        t: 0.0,
    };
    if world.hit(r.clone(), 0.0, INFINITY, rec) {
        return (rec.clone().normal + Color::of(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (Color::of(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::of(0.5, 0.7, 1.0) * t);
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let mut world = vec![
        Sphere {
            center: Point3::of(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Point3::of(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];

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
            let world = world.as_mut_slice().to_vec();
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::of(
                origin.clone(),
                lower_left_corner.clone() + (horizontal.clone() * u) + (vertical.clone() * v)
                    - origin.clone(),
            );
            let pixel_color = ray_color(r, world);
            write_color(pixel_color);
        }
    }
    eprintln!("Done.\n");
}
