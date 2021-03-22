pub use crate::math::FloatVector;
use crate::math::{approx_eq, PhysicsVector};
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl FloatVector<'_> for Vec2 {
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn size() -> usize {
        2
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
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

macro_rules! impl_binop {
    ($Op: ident, $op_fn: ident, $sym: tt) => {
        ///
        /// Vec Vec multiplication
        /// &v * &v
        ///
        impl $Op<&Vec2> for &Vec2 {
            type Output = Vec2;

            fn $op_fn(self, other: &Vec2) -> Self::Output {
                Vec2 {
                    x: self.x $sym other.x,
                    y: self.y $sym other.y,
                }
            }
        }

        ///
        /// Vec Vec multiplication
        /// v * &v
        ///
        impl $Op<Vec2> for &Vec2 {
            type Output = Vec2;

            fn $op_fn(self, other: Vec2) -> Self::Output {
                self $sym &other
            }
        }

        ///
        /// Vec Vec multiplication
        /// &v * v
        ///
        impl $Op<&Vec2> for Vec2 {
            type Output = Vec2;

            fn $op_fn(self, other: &Vec2) -> Self::Output {
                &self $sym other
            }
        }

        ///
        /// Vec Vec multiplication
        /// v * v
        ///
        impl $Op<Vec2> for Vec2 {
            type Output = Vec2;

            fn $op_fn(self, other: Vec2) -> Self::Output {
                &self $sym &other
            }
        }

        ///
        /// f32 Vec multiplication
        /// s * &v
        ///
        impl<'a> $Op<f32> for &'a Vec2 {
            type Output = Vec2;

            fn $op_fn(self, other: f32) -> Self::Output {
                Vec2 {
                    x: self.x $sym other,
                    y: self.y $sym other,
                }
            }
        }


        ///
        /// f32 Vec multiplication
        /// s * v
        ///
        impl $Op<f32> for Vec2 {
            type Output = Vec2;

            fn $op_fn(self, other: f32) -> Self::Output {
                &self $sym other
            }
        }


        ///
        /// Vec f32 multiplication
        /// &v * s
        ///
        impl<'a> $Op<&'a Vec2> for f32 {
            type Output = Vec2;

            fn $op_fn(self, other: &'a Vec2) -> Self::Output {
                Vec2 {
                    x: self $sym other.x,
                    y: self $sym other.y,
                }
            }
        }

        ///
        /// Vec f32 multiplication
        /// v * s
        ///
        impl $Op<Vec2> for f32 {
            type Output = Vec2;

            fn $op_fn(self, other: Vec2) -> Self::Output {
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
impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
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
        i - 2.0 * i.dot(n) * n
    }

    fn refract(i: &Self, n: &Self, eta: f32) -> Option<Self> {
        debug_assert!(
            n.is_unit(),
            "The refraction function only works with normalized normal vectors"
        );
        let n_dot_i: f32 = n.dot(i);
        let k: f32 = 1.0 - eta * eta * (1.0 - n_dot_i * n_dot_i);
        if k < 0.0 {
            None
        } else {
            Some(eta * i - (eta * n_dot_i + k.sqrt()) * n)
        }
    }
}
