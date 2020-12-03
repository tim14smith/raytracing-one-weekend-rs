use std::fmt;
use std::ops::{Add, Div, Index, Mul, Neg, Sub};
// Vec3 implementation

#[derive(Clone)]
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
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            data: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            data: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
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

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            data: [-self.data[0], -self.data[1], -self.data[2]],
        }
    }
}

// Vec3 utility functions

pub fn vplus(a: Vec3, b: Vec3) -> Vec3 {
    a + b
}

pub fn vminus(a: Vec3, b: Vec3) -> Vec3 {
    a - b
}

pub fn vtimes(a: Vec3, t: f64) -> Vec3 {
    a * t
}

pub fn vdiv(a: Vec3, t: f64) -> Vec3 {
    a / t
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2])
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        data: [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[0],
            a[0] * b[1] - b[1] * a[0],
        ],
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v.clone() / v.length()
}

pub type Color = Vec3;
pub type Point3 = Vec3;
