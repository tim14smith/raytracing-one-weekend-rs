use std::fmt;
use std::ops::Index;

// Vec3 implementation

pub struct Vec3 {
    data: [f64; 3],
}

impl Vec3 {
    fn new() -> Vec3 {
        Vec3 {
            data: [0.0, 0.0, 0.0],
        }
    }
    fn of(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { data: [e0, e1, e2] }
    }

    fn x(&self) -> f64 {
        self.data[0]
    }

    fn y(&self) -> f64 {
        self.data[1]
    }

    fn z(&self) -> f64 {
        self.data[2]
    }

    fn neg(&self) -> Vec3 {
        Vec3 {
            data: [-self.data[0], -self.data[1], -self.data[2]],
        }
    }

    fn plus(&self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
            ],
        }
    }

    fn times(&self, t: f64) -> Vec3 {
        Vec3 {
            data: [self.data[0] * t, self.data[1] * t, self.data[2] * t],
        }
    }

    fn div(&self, t: f64) -> Vec3 {
        self.times(1.0 / t)
    }

    fn length_squared(&self) -> f64 {
        (self.data[0] * self.data[0])
            + (self.data[1] * self.data[1])
            + (self.data[2] * self.data[2])
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.data[i]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

// Vec3 utility functions

fn vplus(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        data: [a[0] + b[0], a[1] + b[1], a[2] + b[2]],
    }
}

fn vminus(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        data: [a[0] - b[0], a[1] - b[1], a[2] - b[2]],
    }
}

fn vtimes(a: Vec3, t: f64) -> Vec3 {
    a.times(t)
}

fn vdiv(a: Vec3, t: f64) -> Vec3 {
    a.div(t)
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2])
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        data: [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[0],
            a[0] * b[1] - b[1] * a[0],
        ],
    }
}

fn unit_vector(v: Vec3) {
    v.div(v.length());
}

type Point3 = Vec3;
type Color = Vec3;

// Color functions

fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    );
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j1 in 0..image_height {
        let j = image_height - j1 - 1;
        eprintln!("\rScanlines remaining {} ", j);
        for i in 0..image_width {
            let pixel_color = Color::of(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            write_color(pixel_color);
        }
    }
    eprintln!("Done.\n");
}
