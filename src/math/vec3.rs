use crate::math::approx_eq;
pub use crate::math::FloatVector;
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};

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

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl FloatVector<'_> for Vec3 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn size() -> usize {
        3
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
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

macro_rules! impl_binop {
    ($Op: ident, $op_fn: ident, $sym: tt) => {
        ///
        /// Vec Vec multiplication
        /// &v * &v
        ///
        impl $Op<&Vec3> for &Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: &Vec3) -> Self::Output {
                Vec3 {
                    x: self.x $sym other.x,
                    y: self.y $sym other.y,
                    z: self.z $sym other.z,
                }
            }
        }

        ///
        /// Vec Vec multiplication
        /// v * &v
        ///
        impl $Op<Vec3> for &Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: Vec3) -> Self::Output {
                self $sym &other
            }
        }

        ///
        /// Vec Vec multiplication
        /// &v * v
        ///
        impl $Op<&Vec3> for Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: &Vec3) -> Self::Output {
                &self $sym other
            }
        }

        ///
        /// Vec Vec multiplication
        /// v * v
        ///
        impl $Op<Vec3> for Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: Vec3) -> Self::Output {
                &self $sym &other
            }
        }

        ///
        /// f32 Vec multiplication
        /// s * &v
        ///
        impl<'a> $Op<f32> for &'a Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: f32) -> Self::Output {
                Vec3 {
                    x: self.x $sym other,
                    y: self.y $sym other,
                    z: self.z $sym other,
                }
            }
        }


        ///
        /// f32 Vec multiplication
        /// s * v
        ///
        impl $Op<f32> for Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: f32) -> Self::Output {
                &self $sym other
            }
        }


        ///
        /// Vec f32 multiplication
        /// &v * s
        ///
        impl<'a> $Op<&'a Vec3> for f32 {
            type Output = Vec3;

            fn $op_fn(self, other: &'a Vec3) -> Self::Output {
                Vec3 {
                    x: self $sym other.x,
                    y: self $sym other.y,
                    z: self $sym other.z,
                }
            }
        }

        ///
        /// Vec f32 multiplication
        /// v * s
        ///
        impl $Op<Vec3> for f32 {
            type Output = Vec3;

            fn $op_fn(self, other: Vec3) -> Self::Output {
                self $sym &other
            }
        }
    };
}

impl_binop!(Add, add, +);
impl_binop!(Sub, sub, -);
impl_binop!(Mul, mul, *);
impl_binop!(Div, div, /);

///
/// Vector-scalar multiplication assign
/// v * s
///
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
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
