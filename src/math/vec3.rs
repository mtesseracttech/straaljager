use crate::math::{approx_eq, PhysicsVector};
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn right() -> Self {
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn up() -> Self {
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn forward() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        let one_over_length: f32 = 1.0 / self.length();
        Vec3 {
            x: self.x * one_over_length,
            y: self.y * one_over_length,
            z: self.z * one_over_length,
        }
    }

    pub fn normalize(&mut self) {
        let one_over_length: f32 = 1.0 / self.length();
        self.x *= one_over_length;
        self.y *= one_over_length;
        self.z *= one_over_length;
    }

    pub fn size() -> usize {
        3
    }

    pub fn is_unit(self) -> bool {
        approx_eq(self.length_squared(), 1.0)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3 {{x: {}, y: {}, z: {}}}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

///
/// Vector addition:
/// v1 + v2
///
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

///
/// Vector subtraction
/// v1 - v2
///
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

///
/// member-wise vector * vector implementation
/// v1 * v2
///
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

///
/// Member-wise vector division
/// v1 / v2
///
impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

///
/// Vector-scalar multiplication
/// v * s
///
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self::Output {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

///
/// Vector-scalar division
/// v / s
///
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Self::Output {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

///
/// Reversed vector-scalar multiplication
/// s * v
///
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

///
/// Index Accessor
/// v[0] == v.x
/// v[1] == v.y
/// v[2] == v.z
///
impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Requested an invalid index on a Vec3: {}", index),
        }
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Requested an invalid index on a Vec3: {}", index),
        }
    }
}

impl PhysicsVector for Vec3 {
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

#[test]
fn test_vec3_new() {
    let vec: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    assert!(approx_eq(vec.x, 1.0));
    assert!(approx_eq(vec.y, 2.0));
    assert!(approx_eq(vec.z, 3.0));
}

#[test]
fn test_vec3_dot() {
    let vec1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let vec2: Vec3 = Vec3::new(5.0, 6.0, 7.0);
    assert!(approx_eq(vec1.dot(&vec2), 38.0));

    let vec3: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    assert!(approx_eq(vec3.dot(&vec3), 3.0))
}

#[test]
fn test_vec3_cross() {
    let vec1: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    let vec2: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    assert_eq!(vec1.cross(&vec2), Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn test_vec3_length_squared() {
    let vec1: Vec3 = Vec3::new(1.0, 3.0, 4.0);
    assert!(approx_eq(vec1.length_squared(), vec1.dot(&vec1)));
}

#[test]
fn test_vec3_length() {
    let vec1: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    assert!(approx_eq(vec1.length(), 1.0));

    let vec2: Vec3 = Vec3::new(1.0, -1.0, 1.0);
    assert!(approx_eq(vec2.length(), 3.0f32.sqrt()));

    let vec2: Vec3 = Vec3::new(2.0, 4.0, 5.0);
    assert!(approx_eq(vec2.length(), 45.0f32.sqrt()));
}

#[test]
fn test_vec3_normalized() {
    let vec1: Vec3 = Vec3::new(4.0, -6.0, 8.0);
    let vec1_norm: Vec3 = vec1.normalized();

    assert!(vec1_norm.is_unit());

    let diff = vec1 / vec1_norm;
    assert!(approx_eq(diff.x, diff.y) && approx_eq(diff.y, diff.z));
}

#[test]
fn test_vec3_normalize() {
    let mut vec1: Vec3 = Vec3::new(4.0, -6.0, 8.0);
    vec1.normalize();

    assert!(vec1.is_unit());
}

#[test]
fn test_vec3_negate() {
    let vec: Vec3 = -Vec3::new(1.0, -2.0, 3.0);

    assert_eq!(vec, Vec3::new(-1.0, 2.0, -3.0));
}

#[test]
fn test_vec3_add_vec3() {
    let vec1: Vec3 = Vec3::new(1.0, -2.0, 3.0);
    let vec2: Vec3 = Vec3::new(-5.0, 6.0, -7.0);
    let result: Vec3 = vec1 + vec2;

    assert_eq!(result, Vec3::new(-4.0, 4.0, -4.0));
}

#[test]
fn test_vec3_sub_vec3() {
    let vec1: Vec3 = Vec3::new(1.0, -2.0, 3.0);
    let vec2: Vec3 = Vec3::new(-5.0, 6.0, -7.0);
    let result: Vec3 = vec1 - vec2;

    assert_eq!(result, Vec3::new(6.0, -8.0, 10.0));
}

#[test]
fn test_vec3_mul_vec3() {
    let vec1: Vec3 = Vec3::new(1.0, -2.0, 3.0);
    let vec2: Vec3 = Vec3::new(-5.0, 6.0, -7.0);
    let result: Vec3 = vec1 * vec2;

    assert_eq!(result, Vec3::new(-5.0, -12.0, -21.0));
}


#[test]
fn test_vec3_div_vec3() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    let vec2: Vec3 = Vec3::new(-2.0, 3.0, -2.0);
    let result: Vec3 = vec1 / vec2;

    assert_eq!(result, Vec3::new(-2.5, -2.0, -3.5));
}

#[test]
fn test_vec3_mul_scalar() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    let result: Vec3 = vec1 * -2.0;

    assert_eq!(result, Vec3::new(-10.0, 12.0, -14.0));
}

#[test]
fn test_vec3_div_scalar() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    let result: Vec3 = vec1 / -2.0;

    assert_eq!(result, Vec3::new(-2.5, 3.0, -3.5));
}

#[test]
fn test_vec3_mul_scalar_reversed() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    let result: Vec3 = -2.0 * vec1;

    assert_eq!(result, Vec3::new(-10.0, 12.0, -14.0));
}

#[test]
fn test_vec3_index() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    assert!(approx_eq(vec1[0], 5.0));
    assert!(approx_eq(vec1[1], -6.0));
    assert!(approx_eq(vec1[2], 7.0));
}
