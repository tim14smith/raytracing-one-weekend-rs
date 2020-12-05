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
struct Dielectric {
    ir: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::of(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(&r_in.direction);
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction: Vec3;

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = Ray::of(rec.p, direction);
        true
    }
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
    fuzz: f64,
}

impl Metal {
    fn new(a: Color, f: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
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
        *scattered = Ray::of(rec.p, reflected + (Vec3::rand_in_unit_sphere() * self.fuzz));
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
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner =
            &origin - (&horizontal / 2.0) - (&vertical / 2.0) - (w * focus_dist);

        let lens_radius = aperture / 2.0;
        Camera {
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
            origin: lookfrom,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }
}

fn get_ray(cam: &Camera, s: f64, t: f64) -> Ray {
    let rd = Vec3::random_in_unit_disk() * cam.lens_radius;
    let offset = (cam.u * rd.x()) + (cam.v * rd.y());
    Ray {
        origin: &cam.origin + offset,
        direction: &cam.lower_left_corner + (&cam.horizontal * s) + (&cam.vertical * t)
            - &cam.origin
            - offset,
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
    let lookfrom = Point3::of(3.0, 3.0, 2.0);
    let lookat = Point3::of(0.0, 0.0, -1.0);
    let vup = Vec3::of(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let fov = 20.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
                    albedo: Color::of(0.1, 0.2, 0.5),
                };
                let material_left = Dielectric { ir: 1.5 };
                let material_right = Metal::new(Color::of(0.8, 0.6, 0.2), 0.0);
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
                        center: Point3::of(-1.0, 0.0, -1.0),
                        radius: -0.45,
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
