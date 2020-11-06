use crate::math::{approx_eq, PhysicsVector};
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn zero() -> Self {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(&self) -> Vec4 {
        let one_over_length: f32 = 1.0 / self.length();
        Vec4 {
            x: self.x * one_over_length,
            y: self.y * one_over_length,
            z: self.z * one_over_length,
            w: self.w * one_over_length,
        }
    }

    pub fn normalize(&mut self) {
        let one_over_length: f32 = 1.0 / self.length();
        self.x *= one_over_length;
        self.y *= one_over_length;
        self.z *= one_over_length;
        self.w *= one_over_length;
    }

    pub fn size() -> usize {
        4
    }

    pub fn is_unit(&self) -> bool {
        approx_eq(self.length_squared(), 1.0)
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vec4 {{x: {}, y: {}, z: {}, w: {}}}",
            self.x, self.y, self.z, self.w
        )
    }
}

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

///
/// Vector addition:
/// v1 + v2
///
impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

///
/// Vector subtraction:
/// v1 - v2
///
impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

///
/// Member-wise vector multiplication:
/// v1 * v2
///
impl Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

///
/// Member-wise vector division:
/// v1 / v2
///
impl Div<Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

///
/// Vector-scalar multiplication:
/// v * s
///
impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: f32) -> Self::Output {
        Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

///
/// Vector-scalar division:
/// v / s
///
impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, other: f32) -> Self::Output {
        Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

///
/// Reversed vector-scalar multiplication:
/// s * v
///
impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
            w: self * other.w,
        }
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x)
            && approx_eq(self.y, other.y)
            && approx_eq(self.z, other.z)
            && approx_eq(self.w, other.w)
    }
}

///
/// Index Accessor
/// v[0] == v.x
/// v[1] == v.y
/// v[2] == v.z
/// v[3] == v.w
///
impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Requested an invalid index on a Vec4: {}", index),
        }
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Requested an invalid index on a Vec4: {}", index),
        }
    }
}

impl PhysicsVector for Vec4 {
    fn reflect(i: &Self, n: &Self) -> Self {
        debug_assert!(
            n.is_unit(),
            "The reflect function only works with normalized normal vectors"
        );
        *i - 2.0 * i.dot(n) * *n
    }

    fn refract(i: &Self, n: &Self, eta: f32) -> Option<Self> {
        debug_assert!(
            n.is_unit(),
            "The refraction function only works with normalized normal vectors"
        );
        let n_dot_i = n.dot(i);
        let k = 1.0 - eta * eta * (1.0 - n_dot_i * n_dot_i);
        if k < 0.0 {
            None
        } else {
            Some(eta * *i - (eta * n_dot_i + k.sqrt()) * *n)
        }
    }
}
