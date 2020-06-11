use std::ops::IndexMut;
use std::ops::{Index, Mul};

use crate::Vec3;

struct Mat4 {
    m: [f32; 16],
}

impl Mat4 {
    pub fn zero() -> Self {
        Mat4 { m: [0.0; 16] }
    }

    pub fn identity() -> Self {
        let mut result: Mat4 = Mat4 { m: [0.0; 16] };
        result[0] = 1.0;
        result[5] = 1.0;
        result[10] = 1.0;
        result[15] = 1.0;
        result
    }

    pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        let right_left = right - left;
        let top_bottom = top - bottom;
        let far_near = far - near;

        let mut result = Mat4::identity();
        result[0] = 2.0f32 / right_left;
        result[3] = - (right + left) / right_left;

        result[5] = 2.0f32 / top_bottom;
        result[7] = - (top + bottom) / top_bottom;

        result[10] = -2.0f32 / far_near;
        result[11] = - (far + near) / far_near;

        result
    }

    pub fn scale(scale: Vec3) -> Self {
        let result = Self::identity();

        result[0] = scale.x;
        result[5] = scale.y;
        result[10] = scale.z;

        result
    }
}

///
/// Matrix * matrix implementation
/// m1 * m2
///
impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Self::Output {
        let mut result = Mat4::zero();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.m[i + j * 4] += self.m[i + k * 4] * other.m[k + j * 4];
                }
            }
        }

        result
    }
}

///
/// Index Accessor
///
impl Index<usize> for Mat4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= 16 {
            panic!("Requested an invalid index on a Mat4: {}", index);
        }

        &self.m[index]
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= 16 {
            panic!("Requested an invalid index on a Mat4: {}", index);
        }

        &mut self.m[index]
    }
}
