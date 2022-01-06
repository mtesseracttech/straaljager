use std::{
    fmt::Debug,
    ops::{AddAssign, Index, IndexMut, Mul},
};

use super::{ApproxEq, One, Vector, Zero};

pub type Mat2 = Matrix<f32, 2, 2>;
pub type Mat3 = Matrix<f32, 3, 3>;
pub type Mat4 = Matrix<f32, 4, 4>;
pub type MatN<const S: usize> = Matrix<f32, S, S>;

pub type IMat2 = Matrix<i32, 2, 2>;
pub type IMat3 = Matrix<i32, 3, 3>;
pub type IMat4 = Matrix<i32, 4, 4>;
pub type IMatN<const S: usize> = Matrix<i32, S, S>;

pub struct Matrix<T, const R: usize, const C: usize> {
    // Matrix is stored as an array of columns
    pub columns: [Vector<T, R>; C],
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(elements: [[T; R]; C]) -> Self {
        Self {
            columns: elements.map(|x| Vector::<T, R>::from_slice(x)),
        }
    }

    ///
    /// The amount of columns this matrix contains
    ///
    pub const fn columns(&self) -> usize {
        C
    }

    ///
    /// The amount of rows this matrix contains
    ///
    pub const fn rows(&self) -> usize {
        R
    }

    pub fn column(&self, i: usize) -> &Vector<T, R> {
        assert!(i < C);

        &self.columns[i]
    }
}

// TODO: find a way to not use zero and copy
impl<T: Zero + Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn transposed(&self) -> Matrix<T, C, R> {
        let mut matrix = Matrix::<T, C, R>::zero();

        for c in 0..C {
            for r in 0..R {
                matrix.columns[r][c] = self.columns[c][r];
            }
        }

        matrix
    }
}

impl<T: Zero + Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn zero() -> Self {
        Self {
            columns: [(); C].map(|_| Vector::<T, R>::zero()),
        }
    }
}

impl<T: Zero + One + Copy, const S: usize> Matrix<T, S, S> {
    pub fn identity() -> Self {
        let mut matrix = Self::zero();
        for i in 0..S {
            matrix.columns[i][i] = T::one();
        }
        matrix
    }
}

// TODO: find a way to not use zero and copy
impl<
        T: Zero + Copy + Mul<Output = T> + AddAssign<T> + Debug,
        const R: usize,
        const C: usize,
        const S: usize,
    > Mul<&Matrix<T, C, S>> for &Matrix<T, R, C>
{
    type Output = Matrix<T, R, S>;

    fn mul(self, other: &Matrix<T, C, S>) -> Self::Output {
        let mut matrix = Matrix::<T, R, S>::zero();
        for r in 0..R {
            for s in 0..S {
                let mut result = T::zero();

                for c in 0..C {
                    result += self.columns[c][r] * other.columns[s][c];
                }

                matrix[s][r] = result;
            }
        }

        matrix
    }
}

///
/// PartialEq implementation using approx_eq
///
impl<T: ApproxEq, const R: usize, const C: usize> PartialEq for Matrix<T, R, C> {
    fn eq(&self, other: &Self) -> bool {
        self.columns
            .iter()
            .zip(other.columns.iter())
            .all(|(a, b)| a == b)
    }
}

///
/// Index accessor
///
impl<T, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
    type Output = Vector<T, R>;

    fn index(&self, i: usize) -> &Self::Output {
        debug_assert!(i < C);
        &self.columns[i]
    }
}

///
/// Mutable index accessor
///
impl<T, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        debug_assert!(i < C);
        &mut self.columns[i]
    }
}

impl<T: Debug, const R: usize, const C: usize> Debug for Matrix<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix")
            .field("columns", &C)
            .field("rows", &R)
            .field("elements", &self.columns)
            .finish()
    }
}
