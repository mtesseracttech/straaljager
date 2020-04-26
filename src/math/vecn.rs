use crate::math::approx_eq;
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct VecN {
    pub size: usize,
    pub data: Vec<f32>,
}

impl VecN {
    pub fn new(data: &[f32]) -> VecN {
        VecN {
            size: data.len(),
            data: data.to_vec(),
        }
    }

    pub fn zero(size: usize) -> VecN {
        VecN {
            size,
            data: vec![0.0; size],
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        assert!(self.size == other.size);

        let mut result: f32 = 0.0;
        for i in 0..self.size {
            result += self.data[i] * other.data[i];
        }
        result
    }

    pub fn length_squared(&self) -> f32 {
        let mut result: f32 = 0.0;
        for i in 0..self.size {
            result += self.data[i] * self.data[i];
        }
        result
    }

    pub fn length(&self) -> f32 {
        let mut result: f32 = 0.0;
        for i in 0..self.size {
            result += self.data[i] * self.data[i];
        }
        result.sqrt()
    }

    pub fn normalized(&self) -> Self {
        let one_over_length: f32 = 1.0 / self.length();

        let mut result: VecN = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] * one_over_length;
        }
        result
    }

    pub fn normalize(&mut self) {
        let one_over_length: f32 = 1.0 / self.length();

        for i in 0..self.size {
            self.data[i] *= one_over_length;
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_unit(&self) -> bool {
        approx_eq(self.length_squared(), 1.0)
    }
}

impl fmt::Display for VecN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VecN {{size: {}, data: {:?}}}", self.size, self.data)
    }
}

impl Neg for VecN {
    type Output = VecN;

    fn neg(self) -> Self::Output {
        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = -self.data[i];
        }

        result
    }
}

///
/// Vector addition:
/// v1 + v2
///
impl Add<VecN> for VecN {
    type Output = VecN;

    fn add(self, other: VecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] + other.data[i];
        }

        result
    }
}

///
/// Vector subtraction:
/// v1 - v2
///
impl Sub<VecN> for VecN {
    type Output = VecN;

    fn sub(self, other: VecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] - other.data[i];
        }

        result
    }
}

///
/// Member-wise vector * vector implementation
/// v1 - v2
///
impl Mul<VecN> for VecN {
    type Output = VecN;

    fn mul(self, other: VecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] * other.data[i];
        }

        result
    }
}

///
/// Member-wise vector division
/// v1 / v2
///
impl Div<VecN> for VecN {
    type Output = VecN;

    fn div(self, other: VecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] / other.data[i];
        }

        result
    }
}

///
/// Vector-scalar multiplication
/// v * s
///
impl Mul<f32> for VecN {
    type Output = VecN;

    fn mul(self, other: f32) -> Self::Output {
        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] * other;
        }

        result
    }
}

///
/// Vector-scalar division
/// v / s
///
impl Div<f32> for VecN {
    type Output = VecN;

    fn div(self, other: f32) -> Self::Output {
        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] / other;
        }

        result
    }
}

///
/// Reversed vector-scalar multiplication
/// s * v
///
impl Mul<VecN> for f32 {
    type Output = VecN;

    fn mul(self, other: VecN) -> Self::Output {
        let mut result = VecN::zero(other.size);
        for i in 0..other.size {
            result.data[i] = self * other.data[i];
        }

        result
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for VecN {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        for i in 0..self.size {
            if !approx_eq(self.data[i], other.data[i]) {
                return false;
            }
        }

        true
    }
}

///
/// Index Accessor
///
impl Index<usize> for VecN {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size);

        &self.data[index]
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for VecN {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size);

        &mut self.data[index]
    }
}

#[test]
fn test_vec4_new() {
    let vec: VecN = VecN::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(vec.size, 5);
    assert!(approx_eq(vec.data[0], 1.0));
    assert!(approx_eq(vec.data[1], 2.0));
    assert!(approx_eq(vec.data[2], 3.0));
    assert!(approx_eq(vec.data[3], 4.0));
    assert!(approx_eq(vec.data[4], 5.0));
}

#[test]
fn test_vec4_dot() {
    let vec1: VecN = VecN::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    let vec2: VecN = VecN::new(&[6.0, 7.0, 8.0, 9.0, 10.0]);
    assert!(approx_eq(vec1.dot(&vec2), 130.0));

    let vec3: VecN = VecN::new(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
    assert!(approx_eq(vec3.dot(&vec3), 6.0))
}

#[test]
fn test_vec4_length_squared() {
    let vec1: VecN = VecN::new(&[1.0, 3.0, 4.0, -4.0, 6.0, -8.0]);
    assert!(approx_eq(vec1.length_squared(), vec1.dot(&vec1)));
}

#[test]
fn test_vec4_length() {
    let vec1: VecN = VecN::new(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    assert!(approx_eq(vec1.length(), 1.0));

    let vec2: VecN = VecN::new(&[1.0, -1.0, 1.0, -1.0, 1.0, -1.0]);
    assert!(approx_eq(vec2.length(), 6.0f32.sqrt()));

    let vec2: VecN = VecN::new(&[2.0, 4.0, 5.0, -6.0, 7.0, -8.0]);
    assert!(approx_eq(vec2.length(), 194.0f32.sqrt()));
}

#[test]
fn test_vec4_normalized() {
    let vec1: VecN = VecN::new(&[4.0, -6.0, 8.0, -12.0, 14.0, -16.0]);
    let vec1_norm: VecN = vec1.normalized();

    assert!(vec1_norm.is_unit());

    let diff: VecN = vec1 / vec1_norm;
    assert!(
        approx_eq(diff[0], diff[1])
            && approx_eq(diff[1], diff[2])
            && approx_eq(diff[2], diff[3])
            && approx_eq(diff[3], diff[4])
            && approx_eq(diff[4], diff[5])
    );
}

#[test]
fn test_vec4_normalize() {
    let mut vec1: VecN = VecN::new(&[4.0, -6.0, 8.0, -12.0]);
    vec1.normalize();

    assert!(vec1.normalized() == vec1);
    assert!(vec1.is_unit());
}

#[test]
fn test_vec4_negate() {
    let vec: VecN = -VecN::new(&[1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);

    assert_eq!(vec, VecN::new(&[-1.0, 2.0, -3.0, 4.0, -5.0, 6.0]));
}

#[test]
fn test_vec4_add_vec4() {
    let vec1: VecN = VecN::new(&[1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);
    let vec2: VecN = VecN::new(&[-7.0, 8.0, -9.0, 10.0, -11.0, 12.0]);
    let result: VecN = vec1 + vec2;

    assert_eq!(result, VecN::new(&[-6.0, 6.0, -6.0, 6.0, -6.0, 6.0]));
}

#[test]
fn test_vec4_sub_vec4() {
    let vec1: VecN = VecN::new(&[1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);
    let vec2: VecN = VecN::new(&[-7.0, 8.0, -9.0, 10.0, -11.0, 12.0]);
    let result: VecN = vec1 - vec2;

    assert_eq!(result, VecN::new(&[8.0, -10.0, 12.0, -14.0, 16.0, -18.0]));
}

#[test]
fn test_vec4_mul_vec4() {
    let vec1: VecN = VecN::new(&[1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);
    let vec2: VecN = VecN::new(&[7.0, -8.0, 9.0, -10.0, 11.0, -12.0]);
    let result: VecN = vec1 * vec2;

    assert_eq!(result, VecN::new(&[7.0, 16.0, 27.0, 40.0, 55.0, 72.0]));
}

#[test]
fn test_vec4_div_vec4() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0]);
    let vec2: VecN = VecN::new(&[-2.0, 3.0, -2.0, 2.0]);
    let result: VecN = vec1 / vec2;

    assert_eq!(result, VecN::new(&[-2.5, -2.0, -3.5, -4.0]));
}

#[test]
fn test_vec4_mul_scalar() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0]);
    let result: VecN = vec1 * -2.0;

    assert_eq!(result, VecN::new(&[-10.0, 12.0, -14.0, 16.0]));
}

#[test]
fn test_vec4_div_scalar() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0]);
    let result: VecN = vec1 / -2.0;

    assert_eq!(result, VecN::new(&[-2.5, 3.0, -3.5, 4.0]));
}

#[test]
fn test_vec4_mul_scalar_reversed() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0]);
    let result: VecN = -2.0 * vec1;

    assert_eq!(result, VecN::new(&[-10.0, 12.0, -14.0, 16.0]));
}

#[test]
fn test_vec4_index() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0, 9.0, -10.0]);
    assert!(approx_eq(vec1[0], 5.0));
    assert!(approx_eq(vec1[1], -6.0));
    assert!(approx_eq(vec1[2], 7.0));
    assert!(approx_eq(vec1[3], -8.0));
    assert!(approx_eq(vec1[4], 9.0));
    assert!(approx_eq(vec1[5], -10.0));
}
