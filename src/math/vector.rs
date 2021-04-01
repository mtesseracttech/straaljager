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
    pub fn from_slice(e: [T; S]) -> Self {
        Self { e }
    }

    pub fn size(&self) -> usize {
        S
    }
}

impl<T: Default + Copy + Mul<Output = T> + AddAssign, const S: usize> Vector<T, S> {
    pub fn dot(&self, other: &Self) -> T {
        let mut result = T::default();
        for i in 0..S {
            result += self.e[i] * other.e[i];
        }
        result
    }

    pub fn length_squared(&self) -> T {
        let mut result = T::default();
        for i in 0..S {
            result += self.e[i] * self.e[i];
        }
        result
    }
}

impl<const S: usize> Vector<f32, S> {
    pub fn length(&self) -> f32 {
        let mut result = 0.0;
        for i in 0..S {
            result += self.e[i] * self.e[i];
        }
        result.sqrt()
    }

    pub fn normalized(&self) -> Self {
        let one_over_length = 1.0 / self.length();
        let mut e = [0.0; S];
        for i in 0..S {
            e[i] = self.e[i] * one_over_length;
        }
        Vector::<f32, S> { e }
    }

    pub fn normalize(&mut self) {
        let one_over_length = 1.0 / self.length();
        for i in 0..S {
            self.e[i] *= one_over_length;
        }
    }

    pub fn is_unit(&self) -> bool {
        approx_eq(self.length(), 1.0)
    }
}

macro_rules! impl_binop {
    ($Op: ident, $op_fn: ident, $sym: tt) => {
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
    };
}

impl_binop!(Add, add, +);
impl_binop!(Sub, sub, -);
impl_binop!(Mul, mul, *);
impl_binop!(Div, div, /);

impl<T: Default + Copy + Div<Output = T>, const S: usize> Div<T> for Vector<T, S> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let mut e = [T::default(); S];
        for i in 0..S {
            e[i] = self.e[i] / other;
        }
        Vector::<T, S> { e }
    }
}

impl<T: Default + Copy + Mul<Output = T>, const S: usize> Mul<T> for Vector<T, S> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let mut e = [T::default(); S];
        for i in 0..S {
            e[i] = self.e[i] * other;
        }
        Vector::<T, S> { e }
    }
}

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

impl<T, const S: usize> Index<usize> for Vector<T, S> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        debug_assert!(i < S);
        &self.e[i]
    }
}

impl<T, const S: usize> IndexMut<usize> for Vector<T, S> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        debug_assert!(i < S);
        &mut self.e[i]
    }
}

impl<T: ApproxEq, const S: usize> PartialEq for Vector<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.e
            .iter()
            .zip(other.e.iter())
            .all(|(a, b)| a.approx_eq(b))
    }
}

impl<T: Debug, const S: usize> Debug for Vector<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("size", &S)
            .field("elements", &self.e)
            .finish()
    }
}

// Size specific implemntations

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

impl<T: Mul<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy> Vector<T, 3> {
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

impl<T> Vector<T, 4> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { e: [x, y, z, w] }
    }
}
