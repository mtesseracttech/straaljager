use crate::math::ApproxEq;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Neg;
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
}

///
/// Negate operator:
/// - v
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

impl<T> Vector<T, 4> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { e: [x, y, z, w] }
    }
}
