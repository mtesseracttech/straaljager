use crate::math::approx_eq;
pub use crate::math::FloatVector;
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};

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
}

impl FloatVector<'_> for Vec4 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    fn size() -> usize {
        3
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
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

macro_rules! impl_binop {
    ($Op: ident, $op_fn: ident, $sym: tt) => {
        ///
        /// Vec Vec multiplication
        /// &v * &v
        ///
        impl $Op<&Vec4> for &Vec4 {
            type Output = Vec4;

            fn $op_fn(self, other: &Vec4) -> Self::Output {
                Vec4 {
                    x: self.x $sym other.x,
                    y: self.y $sym other.y,
                    z: self.z $sym other.z,
                    w: self.w $sym other.w,
                }
            }
        }

        ///
        /// Vec Vec multiplication
        /// v * &v
        ///
        impl $Op<Vec4> for &Vec4 {
            type Output = Vec4;

            fn $op_fn(self, other: Vec4) -> Self::Output {
                self $sym &other
            }
        }

        ///
        /// Vec Vec multiplication
        /// &v * v
        ///
        impl $Op<&Vec4> for Vec4 {
            type Output = Vec4;

            fn $op_fn(self, other: &Vec4) -> Self::Output {
                &self $sym other
            }
        }

        ///
        /// Vec Vec multiplication
        /// v * v
        ///
        impl $Op<Vec4> for Vec4 {
            type Output = Vec4;

            fn $op_fn(self, other: Vec4) -> Self::Output {
                &self $sym &other
            }
        }

        ///
        /// f32 Vec multiplication
        /// s * &v
        ///
        impl<'a> $Op<f32> for &'a Vec4 {
            type Output = Vec4;

            fn $op_fn(self, other: f32) -> Self::Output {
                Vec4 {
                    x: self.x $sym other,
                    y: self.y $sym other,
                    z: self.z $sym other,
                    w: self.w $sym other,
                }
            }
        }


        ///
        /// f32 Vec multiplication
        /// s * v
        ///
        impl $Op<f32> for Vec4 {
            type Output = Vec4;

            fn $op_fn(self, other: f32) -> Self::Output {
                &self $sym other
            }
        }


        ///
        /// Vec f32 multiplication
        /// &v * s
        ///
        impl<'a> $Op<&'a Vec4> for f32 {
            type Output = Vec4;

            fn $op_fn(self, other: &'a Vec4) -> Self::Output {
                Vec4 {
                    x: self $sym other.x,
                    y: self $sym other.y,
                    z: self $sym other.z,
                    w: self $sym other.w,
                }
            }
        }

        ///
        /// Vec f32 multiplication
        /// v * s
        ///
        impl $Op<Vec4> for f32 {
            type Output = Vec4;

            fn $op_fn(self, other: Vec4) -> Self::Output {
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
impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
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
