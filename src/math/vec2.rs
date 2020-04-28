use crate::math::{approx_eq, PhysicsVector};
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Vec2 {
        let one_over_length: f32 = 1.0 / self.length();
        Vec2 {
            x: self.x * one_over_length,
            y: self.y * one_over_length,
        }
    }

    pub fn normalize(&mut self) {
        let one_over_length: f32 = 1.0 / self.length();
        self.x *= one_over_length;
        self.y *= one_over_length;
    }

    pub fn size() -> usize {
        2
    }

    pub fn is_unit(&self) -> bool {
        approx_eq(self.length_squared(), 1.0)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2 {{x: {}, y: {}}}", self.x, self.y)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

///
/// Vector addition:
/// v1 + v2
///
impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

///
/// Vector subtraction
/// v1 - v2
///
impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

///
/// Member-wise vector * vector implementation
/// v1 * v2
///
impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

///
/// Member-wise vector division
/// v1 / v2
///
impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

///
/// Vector-scalar multiplication
/// v * s
///
impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Self::Output {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

///
/// Vector-scalar division
/// v / s
///
impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f32) -> Self::Output {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

///
/// Reversed vector-scalar multiplication
/// s * v
///
impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y)
    }
}

///
/// Index Accessor
/// v[0] == v.x
/// v[1] == v.y
///
impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Requested an invalid index on a Vec2: {}", index),
        }
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Requested an invalid index on a Vec2: {}", index),
        }
    }
}

impl PhysicsVector for Vec2 {
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
