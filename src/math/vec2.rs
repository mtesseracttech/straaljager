use crate::math::{approx_eq};
use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
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

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(self) -> Vec2 {
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
        approx_eq(&self.length_squared(), &1.0)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2 {{x: {}, y: {}}}", self.x, self.y)
    }
}

impl ops::Neg for Vec2 {
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
impl ops::Add<Vec2> for Vec2 {
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
impl ops::Sub<Vec2> for Vec2 {
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
impl ops::Mul<Vec2> for Vec2 {
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
impl ops::Div<Vec2> for Vec2 {
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
impl ops::Mul<f32> for Vec2 {
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
impl ops::Div<f32> for Vec2 {
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
impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}
