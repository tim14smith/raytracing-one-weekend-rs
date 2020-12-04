extern crate rand;
mod vec3;
use std::f64::consts::PI;
use std::f64::INFINITY;
use std::option::Option;
use vec3::*;
//use vec3::{unit_vector, Color, Point3, Vec3};

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
        &self.origin + (&self.direction * t)
    }
}

trait MatClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MatClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

trait Material: MatClone {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Copy)]
struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::rand_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::of(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

#[derive(Clone, Copy)]
struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&unit_vector(&r_in.direction), &rec.normal);
        *scattered = Ray::of(rec.p, reflected);
        *attenuation = self.albedo;
        dot(&scattered.direction, &rec.normal) > 0.0
    }
}

type MatPtr = Option<Box<dyn Material>>;

#[derive(Clone)]
struct HitRecord {
    p: Point3,
    normal: Vec3,
    mat_ptr: MatPtr,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            p: Point3::new(),
            normal: Vec3::new(),
            mat_ptr: None,
            t: 0.0,
            front_face: false,
        }
    }
}

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: MatPtr,
}

impl Sphere {
    pub fn of(cen: Point3, radius: f64, mat_ptr: MatPtr) -> Sphere {
        Sphere {
            center: cen,
            radius: radius,
            mat_ptr: mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = &r.origin - &self.center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
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
        let new_p = r.at(root);
        let outward_normal = (&new_p - &self.center) / self.radius;
        let fface = dot(&r.direction, &outward_normal) < 0.0;
        let new_normal = if fface {
            outward_normal
        } else {
            -&outward_normal
        };

        *rec = HitRecord {
            t: root,
            p: new_p,
            mat_ptr: self.mat_ptr.clone(),
            normal: new_normal,
            front_face: fface,
        };
        true
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for x in self {
            let temp_rec = &mut HitRecord::default();
            let hit = x.hit(&r, t_min, closest_so_far, temp_rec);
            if hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();
    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    );
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = &r.origin - &center;
    let a = r.direction.length_squared();
    let half_b = dot(&oc, &r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (a)
    }
}

fn ray_color<T: Hittable>(r: Ray, world: T, depth: u32) -> Color {
    let rec: &mut HitRecord = &mut HitRecord::default();

    if depth <= 0 {
        return Color::of(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, INFINITY, rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Color::new();
        let mat = rec.clone().mat_ptr;
        match mat {
            None => {
                println!("NO MATERIAL for {}, {}", r.origin, r.direction);
            }
            Some(x) => {
                if x.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * ray_color(scattered, world, depth - 1);
                }
            }
        }
        return Color::of(0.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (&Color::of(1.0, 1.0, 1.0) * (1.0 - t)) + (&Color::of(0.5, 0.7, 1.0) * t);
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new();
        let horizontal = Vec3::of(viewport_width, 0.0, 0.0);
        let vertical = Vec3::of(0.0, viewport_height, 0.0);
        Camera {
            origin: Point3::new(),
            horizontal: Vec3::of(viewport_width, 0.0, 0.0),
            vertical: Vec3::of(0.0, viewport_height, 0.0),
            lower_left_corner: &origin
                - (&horizontal / 2.0)
                - (&vertical / 2.0)
                - &Vec3::of(0.0, 0.0, focal_length),
        }
    }
}

fn get_ray(cam: &Camera, u: f64, v: f64) -> Ray {
    Ray {
        origin: cam.origin.clone(),
        direction: &cam.lower_left_corner + (&cam.horizontal * u) + (&cam.vertical * v)
            - &cam.origin,
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let cam = Camera::new();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j1 in 0..image_height {
        let j = image_height - j1 - 1;
        eprintln!("\rScanlines remaining {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::of(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                // World
                let material_ground = Lambertian {
                    albedo: Color::of(0.8, 0.8, 0.0),
                };
                let material_center = Lambertian {
                    albedo: Color::of(0.7, 0.3, 0.3),
                };
                let material_left = Metal {
                    albedo: Color::of(0.8, 0.8, 0.8),
                };
                let material_right = Metal {
                    albedo: Color::of(0.8, 0.6, 0.2),
                };
                let world = vec![
                    Sphere {
                        center: Point3::of(0.0, -100.5, -1.0),
                        radius: 100.0,
                        mat_ptr: Some(Box::new(material_ground)),
                    },
                    Sphere {
                        center: Point3::of(0.0, 0.0, -1.0),
                        radius: 0.5,
                        mat_ptr: Some(Box::new(material_center)),
                    },
                    Sphere {
                        center: Point3::of(-1.0, 0.0, -1.0),
                        radius: 0.5,
                        mat_ptr: Some(Box::new(material_left)),
                    },
                    Sphere {
                        center: Point3::of(1.0, 0.0, -1.0),
                        radius: 0.5,
                        mat_ptr: Some(Box::new(material_right)),
                    },
                ];
                let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                let r = get_ray(&cam, u, v);
                pixel_color = &pixel_color + &ray_color(r, world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done.\n");
}
