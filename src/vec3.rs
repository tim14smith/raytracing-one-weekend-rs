use rand::prelude::*;
use rand::thread_rng;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

pub fn random_f64() -> f64 {
    thread_rng().gen_range(0.0, 1.0)
}

pub fn random_float(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min, max)
}

// Vec3 implementation

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    data: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            data: [0.0, 0.0, 0.0],
        }
    }
    pub fn of(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { data: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn length_squared(&self) -> f64 {
        (self.data[0] * self.data[0])
            + (self.data[1] * self.data[1])
            + (self.data[2] * self.data[2])
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn rand() -> Vec3 {
        Vec3::of(random_f64(), random_f64(), random_f64())
    }

    pub fn rand_range(min: f64, max: f64) -> Vec3 {
        Vec3::of(
            random_float(min, max),
            random_float(min, max),
            random_float(min, max),
        )
    }

    pub fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::rand_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn rand_unit_vector() -> Vec3 {
        unit_vector(&Vec3::rand_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::rand_in_unit_sphere();
        if dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        (self[0].abs() < s) && (self[1] < s) && (self[2] < s)
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::of(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl Sum<Vec3> for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::new(), Add::add)
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

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl<'a> Add<Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl<'a> Add<&'a Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl<'a> Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl<'a> Sub<Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl<'a> Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            data: [self.data[0] * t, self.data[1] * t, self.data[2] * t],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            data: [self.data[0] * t, self.data[1] * t, self.data[2] * t],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl<'a> Mul<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl<'a> Mul<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl<'a, 'b> Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl<'a> Div<f64> for &'a Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

impl<'a> Neg for &'a Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            data: [-self.data[0], -self.data[1], -self.data[2]],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            data: [-self.data[0], -self.data[1], -self.data[2]],
        }
    }
}

// Vec3 utility functions
pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
    (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2])
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        data: [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ],
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - (n * (dot(v, n) * 2.0))
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-uv, n).min(1.0);
    let r_out_perp = (uv + (n * cos_theta)) * etai_over_etat;
    let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}

pub type Color = Vec3;
pub type Point3 = Vec3;
