use crate::math::{approx_eq};
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct VecN {
    pub size: usize,
    pub data: Vec<f32>,
}

impl VecN {
    pub fn new(size: usize, data: &[f32]) -> VecN {
        VecN {
            size,
            data: data.to_vec(),
        }
    }

    pub fn zero(size: usize) -> VecN {
        VecN {
            size,
            data: vec![0.0; size],
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        assert!(self.size == other.size);

        let mut result: f32 = 0.0;
        for i in 0..self.size {
            result += self.data[i] * other.data[i];
        }
        result
    }

    pub fn length_squared(&self) -> f32 {
        let mut result: f32 = 0.0;
        for i in 0..self.size {
            result += self.data[i] * self.data[i];
        }
        result
    }

    pub fn length(&self) -> f32 {
        let mut result: f32 = 0.0;
        for i in 0..self.size {
            result += self.data[i] * self.data[i];
        }
        result.sqrt()
    }

    pub fn normalized(&self) -> Self {
        let one_over_length: f32 = 1.0 / self.length();

        let mut result: VecN = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] * one_over_length;
        }
        result
    }

    pub fn normalize(&mut self) {
        let one_over_length: f32 = 1.0 / self.length();

        for i in 0..self.size {
            self.data[i] *= one_over_length;
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_unit(&self) -> bool {
        approx_eq(self.length_squared(), 1.0)
    }
}

impl fmt::Display for VecN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VecN {{size: {}, data: {:?}}}", self.size, self.data)
    }
}

impl Neg for VecN {
    type Output = VecN;

    fn neg(self) -> Self::Output {
        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = -self.data[i];
        }

        result
    }
}

///
/// Vector addition:
/// v1 + v2
///
impl Add<VecN> for VecN {
    type Output = VecN;

    fn add(self, other: VecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] + other.data[i];
        }

        result
    }
}

///
/// Vector subtraction:
/// v1 - v2
///
impl Sub<VecN> for VecN {
    type Output = VecN;

    fn sub(self, other: VecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] - other.data[i];
        }

        result
    }
}


///
/// Member-wise vector * vector implementation
/// v1 - v2
///
impl Mul<VecN> for VecN {
    type Output = VecN;

    fn mul(self, other: VecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] * other.data[i];
        }

        result
    }
}


///
/// Member-wise vector division
/// v1 / v2
///
impl Div<VecN> for VecN {
    type Output = VecN;

    fn div(self, other: VecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] / other.data[i];
        }

        result
    }
}


///
/// Vector-scalar multiplication
/// v * s
///
impl Mul<f32> for VecN {
    type Output = VecN;

    fn mul(self, other: f32) -> Self::Output {
        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] * other;
        }

        result
    }
}


///
/// Vector-scalar division
/// v / s
///
impl Div<f32> for VecN {
    type Output = VecN;

    fn div(self, other: f32) -> Self::Output {
        let mut result = VecN::zero(self.size);
        for i in 0..self.size {
            result.data[i] = self.data[i] / other;
        }

        result
    }
}

///
/// Reversed vector-scalar multiplication
/// s * v
///
impl Mul<VecN> for f32 {
    type Output = VecN;

    fn mul(self, other: VecN) -> Self::Output {
        let mut result = VecN::zero(other.size);
        for i in 0..other.size {
            result.data[i] = self * other.data[i];
        }

        result
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for VecN {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        for i in 0..self.size {
            if !approx_eq(self.data[i], other.data[i]) {
                return false;
            }
        }

        true
    }
}


///
/// Index Accessor
///
impl Index<usize> for VecN {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size);

        &self.data[index]
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for VecN {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size);

        &mut self.data[index]
    }
}
