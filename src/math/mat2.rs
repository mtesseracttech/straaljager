use std::ops::{Div, Index, IndexMut, Mul};

use crate::math::vec2::Vec2;

#[derive(Debug)]
pub struct Mat2 {
    pub r0: Vec2,
    pub r1: Vec2,
}

impl Mat2 {
    pub fn new(d: [f32; 4]) -> Self {
        Mat2 {
            r0: Vec2::new(d[0], d[1]),
            r1: Vec2::new(d[2], d[3]),
        }
    }

    pub fn zero() -> Self {
        Mat2 {
            r0: Vec2::zero(),
            r1: Vec2::zero(),
        }
    }

    pub fn identity() -> Self {
        Mat2 {
            r0: Vec2::new(1.0, 0.0),
            r1: Vec2::new(0.0, 1.0),
        }
    }

    pub fn transposed(&self) -> Mat2 {
        Mat2 {
            r0: Vec2::new(self.r0.x, self.r1.x),
            r1: Vec2::new(self.r0.y, self.r1.y),
        }
    }

    pub fn adjoint(&self) -> Mat2 {
        Mat2 {
            r0: Vec2::new(self.r1.y, -self.r0.y),
            r1: Vec2::new(-self.r1.x, self.r0.x),
        }
    }

    pub fn inverse(&self) -> Mat2 {
        let adjoint = self.adjoint();

        let det = self[0].dot(&adjoint.column(0));

        &adjoint * (1.0 / det)
    }

    pub fn column(&self, i: usize) -> Vec2 {
        assert!(i < 3);

        Vec2 {
            x: self.r0[i],
            y: self.r1[i],
        }
    }
}

///
/// Matrix * matrix implementation
/// m1 * m2
///
impl Mul<&Mat2> for &Mat2 {
    type Output = Mat2;

    fn mul(self, other: &Mat2) -> Self::Output {
        let t = other.transposed();
        Mat2 {
            r0: Vec2::new(self.r0.dot(&t.r0), self.r0.dot(&t.r1)),
            r1: Vec2::new(self.r1.dot(&t.r0), self.r1.dot(&t.r1)),
        }
    }
}

///
/// Matrix * Vec4 implementation
/// m * v
///
impl Mul<&Vec2> for &Mat2 {
    type Output = Vec2;

    fn mul(self, other: &Vec2) -> Self::Output {
        Vec2::new(self.r0.dot(&other), self.r1.dot(&other))
    }
}

///
/// Matrix * f32 implementation
/// m * v
///
impl Mul<f32> for &Mat2 {
    type Output = Mat2;

    fn mul(self, other: f32) -> Self::Output {
        Mat2 {
            r0: self.r0 * other,
            r1: self.r1 * other,
        }
    }
}

///
/// Matrix / f32 implementation
/// m / v
///
impl Div<f32> for &Mat2 {
    type Output = Mat2;

    fn div(self, other: f32) -> Self::Output {
        Mat2 {
            r0: self.r0 / other,
            r1: self.r1 / other,
        }
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for Mat2 {
    fn eq(&self, other: &Self) -> bool {
        (0..1).into_iter().all(|i| self[i] == other[i])
    }
}

///
/// Index Accessor
///
impl Index<usize> for Mat2 {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            _ => panic!("Requested an invalid index on a Mat4: {}", index),
        }
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for Mat2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            _ => panic!("Requested an invalid index on a Mat4: {}", index),
        }
    }
}
