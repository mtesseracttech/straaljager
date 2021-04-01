use crate::math::{approx_eq, ApproxEq};
use std::fmt::Debug;
use std::ops::AddAssign;
use std::ops::Neg;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{Index, IndexMut};

pub type Vec2 = Vector<f32, 2>;
pub type Vec3 = Vector<f32, 3>;
pub type Vec4 = Vector<f32, 4>;
pub type VecN<const S: usize> = Vector<f32, S>;

pub type IVec2 = Vector<i32, 2>;
pub type IVec3 = Vector<i32, 3>;
pub type IVec4 = Vector<i32, 4>;
pub type IVecN<const S: usize> = Vector<i32, S>;

pub struct Vector<T, const S: usize> {
    pub e: [T; S],
}

impl<T, const S: usize> Vector<T, S> {
    ///
    /// Create a vector from a slice with type T and size S
    ///
    pub fn from_slice(e: [T; S]) -> Self {
        Self { e }
    }

    ///
    /// The amount of elements this vector contains
    ///
    pub const fn size(&self) -> usize {
        S
    }
}

impl<T: Default + Copy + Mul<Output = T> + AddAssign, const S: usize> Vector<T, S> {
    ///
    /// Dot product of two vectors by pairwise multiplication and taking the sum
    /// of the resulting vector
    ///
    pub fn dot(&self, other: &Self) -> T {
        let mut result = T::default();
        for i in 0..S {
            result += self.e[i] * other.e[i];
        }
        result
    }

    ///
    /// The squared length of a vector using pairwise multiplicaion and taking
    /// the sum of the resulting vector
    ///
    pub fn length_squared(&self) -> T {
        let mut result = T::default();
        for i in 0..S {
            result += self.e[i] * self.e[i];
        }
        result
    }
}

impl<const S: usize> Vector<f32, S> {
    ///
    /// The length of the vector
    ///
    pub fn length(&self) -> f32 {
        let mut result = 0.0;
        for i in 0..S {
            result += self.e[i] * self.e[i];
        }
        result.sqrt()
    }

    ///
    /// Get a normalized version of this vector
    ///
    pub fn normalized(&self) -> Self {
        let one_over_length = 1.0 / self.length();
        let mut e = [0.0; S];
        for i in 0..S {
            e[i] = self.e[i] * one_over_length;
        }
        Vector::<f32, S> { e }
    }

    ///
    /// Normalize the vector in-place
    ///
    pub fn normalize(&mut self) {
        let one_over_length = 1.0 / self.length();
        for i in 0..S {
            self.e[i] *= one_over_length;
        }
    }

    ///
    /// Check if this vector is a unit vector
    ///
    pub fn is_unit(&self) -> bool {
        approx_eq(self.length(), 1.0)
    }
}

macro_rules! impl_binop {
    ($Op: ident, $op_fn: ident, $sym: tt) => {
        ///
        /// Vector vector math operation
        ///
        impl<T: Default + Copy + $Op<Output = T>, const S: usize> $Op<Vector<T, S>>
            for Vector<T, S>
        {
            type Output = Vector<T, S>;

            fn $op_fn(self, other: Vector<T, S>) -> Self::Output {
                let mut e = [T::default(); S];
                for i in 0..S {
                    e[i] = self.e[i] $sym other.e[i];
                }
                Vector::<T, S> { e }
            }
        }

        ///
        /// Vector vector reference math operation
        ///
        impl<T: Default + Copy + $Op<Output = T>, const S: usize> $Op<&Vector<T, S>>
            for Vector<T, S>
        {
            type Output = Vector<T, S>;

            fn $op_fn(self, other: &Vector<T, S>) -> Self::Output {
                let mut e = [T::default(); S];
                for i in 0..S {
                    e[i] = self.e[i] $sym other.e[i];
                }
                Vector::<T, S> { e }
            }
        }

        ///
        /// Vector reference vector math operation
        ///
        impl<T: Default + Copy + $Op<Output = T>, const S: usize> $Op<Vector<T, S>>
            for &Vector<T, S>
        {
            type Output = Vector<T, S>;

            fn $op_fn(self, other: Vector<T, S>) -> Self::Output {
                let mut e = [T::default(); S];
                for i in 0..S {
                    e[i] = self.e[i] $sym other.e[i];
                }
                Vector::<T, S> { e }
            }
        }

        ///
        /// Vector reference vector reference math operation
        ///
        impl<T: Default + Copy + $Op<Output = T>, const S: usize> $Op<&Vector<T, S>>
            for &Vector<T, S>
        {
            type Output = Vector<T, S>;

            fn $op_fn(self, other: &Vector<T, S>) -> Self::Output {
                let mut e = [T::default(); S];
                for i in 0..S {
                    e[i] = self.e[i] $sym other.e[i];
                }
                Vector::<T, S> { e }
            }
        }

        ///
        /// Vector scalar math operation
        ///
        impl<T: Default + Copy + $Op<Output = T>, const S: usize> $Op<T> for Vector<T, S> {
            type Output = Self;
            fn $op_fn(self, other: T) -> Self::Output {
                let mut e = [T::default(); S];
                for i in 0..S {
                    e[i] = self.e[i] $sym other;
                }
                Vector::<T, S> { e }
            }
        }

        ///
        /// Vector scalar reference math operation
        ///
        impl<T: Default + Copy + $Op<Output = T>, const S: usize> $Op<&T> for Vector<T, S> {
            type Output = Self;
            fn $op_fn(self, other: &T) -> Self::Output {
                let mut e = [T::default(); S];
                for i in 0..S {
                    e[i] = self.e[i] $sym *other;
                }
                Vector::<T, S> { e }
            }
        }

        ///
        /// Vector reference scalar math operation
        ///
        impl<T: Default + Copy + $Op<Output = T>, const S: usize> $Op<T> for &Vector<T, S> {
            type Output = Vector<T, S>;
            fn $op_fn(self, other: T) -> Self::Output {
                let mut e = [T::default(); S];
                for i in 0..S {
                    e[i] = self.e[i] $sym other;
                }
                Vector::<T, S> { e }
            }
        }

        ///
        /// Vector reference scalar reference math operation
        ///
        impl<T: Default + Copy + $Op<Output = T>, const S: usize> $Op<&T> for &Vector<T, S> {
            type Output = Vector<T, S>;
            fn $op_fn(self, other: &T) -> Self::Output {
                let mut e = [T::default(); S];
                for i in 0..S {
                    e[i] = self.e[i] $sym *other;
                }
                Vector::<T, S> { e }
            }
        }
    };
}

impl_binop!(Add, add, +);
impl_binop!(Sub, sub, -);
impl_binop!(Mul, mul, *);
impl_binop!(Div, div, /);

///
/// Negate operator:
/// -v
///
impl<T: Neg<Output = T> + Default + Copy, const S: usize> Neg for Vector<T, S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut e = [T::default(); S];
        for i in 0..S {
            e[i] = -self.e[i];
        }
        Self { e }
    }
}

///
/// Borrowed negate operator
/// - &v
///
impl<T: Neg<Output = T> + Default + Copy, const S: usize> Neg for &Vector<T, S> {
    type Output = Vector<T, S>;

    fn neg(self) -> Self::Output {
        let mut e = [T::default(); S];
        for i in 0..S {
            e[i] = -self.e[i];
        }
        Vector::<T, S> { e }
    }
}

///
/// Index accessor
///
impl<T, const S: usize> Index<usize> for Vector<T, S> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        debug_assert!(i < S);
        &self.e[i]
    }
}

///
/// Mutable index accessor
///
impl<T, const S: usize> IndexMut<usize> for Vector<T, S> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        debug_assert!(i < S);
        &mut self.e[i]
    }
}

///
/// PartialEq implementation using approx_eq
///
impl<T: ApproxEq, const S: usize> PartialEq for Vector<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.e
            .iter()
            .zip(other.e.iter())
            .all(|(a, b)| a.approx_eq(b))
    }
}

///
/// Debug implementation which prints elements and size
///
impl<T: Debug, const S: usize> Debug for Vector<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("size", &S)
            .field("elements", &self.e)
            .finish()
    }
}

// Size specific implemntations

impl<T: Mul<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy> Vector<T, 3> {
    ///
    /// Cross product implementation which is only implemented for vectors of
    /// size 3
    ///
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                -(self.e[0] * other.e[2] - self.e[2] * other.e[0]),
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }
}

impl<T> Vector<T, 2> {
    pub fn new(x: T, y: T) -> Self {
        Self { e: [x, y] }
    }
}

impl<T> Vector<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { e: [x, y, z] }
    }
}

impl<T> Vector<T, 4> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { e: [x, y, z, w] }
    }
}
