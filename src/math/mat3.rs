use std::ops::{Div, Index, IndexMut, Mul};

use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Mat3 {
    pub r0: Vec3,
    pub r1: Vec3,
    pub r2: Vec3,
}

impl Mat3 {
    pub fn new(d: [f32; 9]) -> Self {
        Mat3 {
            r0: Vec3::new(d[0], d[1], d[2]),
            r1: Vec3::new(d[3], d[4], d[5]),
            r2: Vec3::new(d[6], d[7], d[8]),
        }
    }

    pub fn zero() -> Self {
        Mat3 {
            r0: Vec3::zero(),
            r1: Vec3::zero(),
            r2: Vec3::zero(),
        }
    }

    pub fn identity() -> Self {
        Mat3 {
            r0: Vec3::new(1.0, 0.0, 0.0),
            r1: Vec3::new(0.0, 1.0, 0.0),
            r2: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn transposed(&self) -> Mat3 {
        Mat3 {
            r0: Vec3::new(self.r0.x, self.r1.x, self.r2.x),
            r1: Vec3::new(self.r0.y, self.r1.y, self.r2.y),
            r2: Vec3::new(self.r0.z, self.r1.z, self.r2.z),
        }
    }

    pub fn adjoint(&self) -> Mat3 {
        let r0 = Vec3 {
            x: self[1][1] * self[2][2] - self[1][2] * self[2][1],
            y: self[0][2] * self[2][1] - self[0][1] * self[2][2],
            z: self[0][1] * self[1][2] - self[0][2] * self[1][1],
        };

        let r1 = Vec3 {
            x: self[1][2] * self[2][0] - self[1][0] * self[2][2],
            y: self[0][0] * self[2][2] - self[2][0] * self[0][2],
            z: self[1][0] * self[0][2] - self[0][0] * self[1][2],
        };

        let r2 = Vec3 {
            x: self[1][0] * self[2][1] - self[1][1] * self[2][0],
            y: self[0][1] * self[2][0] - self[0][0] * self[2][1],
            z: self[0][0] * self[1][1] - self[1][0] * self[0][1],
        };

        Mat3 { r0, r1, r2 }
    }

    pub fn inverse(&self) -> Mat3 {
        let adjoint = self.adjoint();

        let det = self[0].dot(&adjoint.column(0));

        &adjoint * (1.0 / det)
    }

    pub fn column(&self, i: usize) -> Vec3 {
        assert!(i < 3);

        Vec3 {
            x: self.r0[i],
            y: self.r1[i],
            z: self.r2[i],
        }
    }
}

///
/// Matrix * matrix implementation
/// m1 * m2
///
impl Mul<&Mat3> for &Mat3 {
    type Output = Mat3;

    fn mul(self, other: &Mat3) -> Self::Output {
        let t = other.transposed();
        Mat3 {
            r0: Vec3::new(self.r0.dot(&t.r0), self.r0.dot(&t.r1), self.r0.dot(&t.r2)),
            r1: Vec3::new(self.r1.dot(&t.r0), self.r1.dot(&t.r1), self.r1.dot(&t.r2)),
            r2: Vec3::new(self.r2.dot(&t.r0), self.r2.dot(&t.r1), self.r2.dot(&t.r2)),
        }
    }
}

///
/// Matrix * Vec4 implementation
/// m * v
///
impl Mul<&Vec3> for &Mat3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3::new(
            self.r0.dot(&other),
            self.r1.dot(&other),
            self.r2.dot(&other),
        )
    }
}

///
/// Matrix * f32 implementation
/// m * v
///
impl Mul<f32> for &Mat3 {
    type Output = Mat3;

    fn mul(self, other: f32) -> Self::Output {
        Mat3 {
            r0: self.r0 * other,
            r1: self.r1 * other,
            r2: self.r2 * other,
        }
    }
}

///
/// Matrix / f32 implementation
/// m / v
///
impl Div<f32> for &Mat3 {
    type Output = Mat3;

    fn div(self, other: f32) -> Self::Output {
        Mat3 {
            r0: self.r0 / other,
            r1: self.r1 / other,
            r2: self.r2 / other,
        }
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for Mat3 {
    fn eq(&self, other: &Self) -> bool {
        (0..3).into_iter().all(|i| self[i] == other[i])
    }
}

///
/// Index Accessor
///
impl Index<usize> for Mat3 {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            2 => &self.r2,
            _ => panic!("Requested an invalid index on a Mat4: {}", index),
        }
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            _ => panic!("Requested an invalid index on a Mat4: {}", index),
        }
    }
}
