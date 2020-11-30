use crate::math::approx_eq;
use std::ops::IndexMut;
use std::ops::{Index, Mul};

use crate::math::vec3::*;
use crate::math::vec4::*;

#[derive(Debug)]
pub struct Mat4 {
    pub r0: Vec4,
    pub r1: Vec4,
    pub r2: Vec4,
    pub r3: Vec4,
}

impl Mat4 {
    pub fn new(d: [f32; 16]) -> Self {
        Mat4 {
            r0: Vec4::new(d[0], d[1], d[2], d[3]),
            r1: Vec4::new(d[4], d[5], d[6], d[7]),
            r2: Vec4::new(d[8], d[9], d[10], d[11]),
            r3: Vec4::new(d[12], d[13], d[14], d[15]),
        }
    }

    pub fn zero() -> Self {
        Mat4 {
            r0: Vec4::zero(),
            r1: Vec4::zero(),
            r2: Vec4::zero(),
            r3: Vec4::zero(),
        }
    }

    pub fn identity() -> Self {
        Mat4 {
            r0: Vec4::new(1.0, 0.0, 0.0, 0.0),
            r1: Vec4::new(0.0, 1.0, 0.0, 0.0),
            r2: Vec4::new(0.0, 0.0, 1.0, 0.0),
            r3: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        let right_left = right - left;
        let top_bottom = top - bottom;
        let far_near = far - near;

        let mut result = Mat4::identity();
        result.r0.x = 2.0f32 / right_left;
        result.r0.w = -(right + left) / right_left;

        result.r1.y = 2.0f32 / top_bottom;
        result.r1.w = -(top + bottom) / top_bottom;

        result.r2.z = -2.0f32 / far_near;
        result.r2.w = -(far + near) / far_near;

        result
    }

    pub fn from_translation(translation: Vec3) -> Self {
        let mut result = Self::identity();

        result.r0.w = translation.x;
        result.r1.w = translation.y;
        result.r2.w = translation.z;

        result
    }

    pub fn from_scale(scale: Vec3) -> Self {
        let mut result = Self::identity();

        result.r0.x = scale.x;
        result.r1.y = scale.y;
        result.r2.z = scale.z;

        result
    }

    pub fn transposed(&self) -> Mat4 {
        Mat4 {
            r0: Vec4::new(self.r0.x, self.r1.x, self.r2.x, self.r3.x),
            r1: Vec4::new(self.r0.y, self.r1.y, self.r2.y, self.r3.y),
            r2: Vec4::new(self.r0.z, self.r1.z, self.r2.z, self.r3.z),
            r3: Vec4::new(self.r0.w, self.r1.w, self.r2.w, self.r3.w),
        }
    }
}

///
/// Matrix * matrix implementation
/// m1 * m2
///
impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Self::Output {
        let t = other.transposed();
        Mat4 {
            r0: Vec4::new(
                self.r0.dot(&t.r0),
                self.r0.dot(&t.r1),
                self.r0.dot(&t.r2),
                self.r0.dot(&t.r3),
            ),
            r1: Vec4::new(
                self.r1.dot(&t.r0),
                self.r1.dot(&t.r1),
                self.r1.dot(&t.r2),
                self.r1.dot(&t.r3),
            ),
            r2: Vec4::new(
                self.r2.dot(&t.r0),
                self.r2.dot(&t.r1),
                self.r2.dot(&t.r2),
                self.r2.dot(&t.r3),
            ),
            r3: Vec4::new(
                self.r3.dot(&t.r0),
                self.r3.dot(&t.r1),
                self.r3.dot(&t.r2),
                self.r3.dot(&t.r3),
            ),
        }
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for Mat4 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

///
/// Index Accessor
///
impl Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= 16 {
            panic!("Requested an invalid index on a Mat4: {}", index);
        }

        match index {
            0 => &self.r0,
            1 => &self.r1,
            2 => &self.r2,
            3 => &self.r3,
            _ => panic!("Requested an invalid index on a Mat4: {}", index),
        }
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r3,
            _ => panic!("Requested an invalid index on a Mat4: {}", index),
        }
    }
}
