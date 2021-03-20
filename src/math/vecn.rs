use crate::math::approx_eq;
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct VecN {
    pub data: Vec<f32>,
}

impl VecN {
    pub fn new(data: &[f32]) -> VecN {
        VecN {
            data: data.to_vec(),
        }
    }

    pub fn zero(size: usize) -> VecN {
        VecN {
            data: vec![0.0; size],
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        assert_eq!(self.size(), other.size());

        self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum()
    }

    pub fn length_squared(&self) -> f32 {
        self.data.iter().map(|x| x * x).sum()
    }

    pub fn length(&self) -> f32 {
        self.data.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let one_over_length: f32 = 1.0 / self.length();
        VecN {
            data: self.data.iter().map(|x| x * one_over_length).collect(),
        }
    }

    pub fn normalize(&mut self) {
        let one_over_length: f32 = 1.0 / self.length();
        self.data.iter_mut().for_each(|x| *x *= one_over_length);
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_unit(&self) -> bool {
        approx_eq(self.length_squared(), 1.0)
    }
}

impl fmt::Display for VecN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VecN {{size: {}, data: {:?}}}", self.size(), self.data)
    }
}

impl Neg for VecN {
    type Output = VecN;

    fn neg(self) -> Self::Output {
        VecN {
            data: self.data.iter().map(|x| -x).collect(),
        }
    }
}

///
/// Vector addition:
/// v1 + v2
///
impl Add<VecN> for VecN {
    type Output = VecN;

    fn add(self, other: VecN) -> Self::Output {
        assert_eq!(self.size(), other.size());

        VecN {
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a + b).collect(),
        }
    }
}

///
/// Vector subtraction:
/// v1 - v2
///
impl Sub<VecN> for VecN {
    type Output = VecN;

    fn sub(self, other: VecN) -> Self::Output {
        assert_eq!(self.size(), other.size());

        VecN {
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a - b).collect(),
        }
    }
}

///
/// Member-wise vector * vector implementation
/// v1 - v2
///
impl Mul<VecN> for VecN {
    type Output = VecN;

    fn mul(self, other: VecN) -> Self::Output {
        assert_eq!(self.size(), other.size());

        VecN {
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).collect(),
        }
    }
}

///
/// Member-wise vector division
/// v1 / v2
///
impl Div<VecN> for VecN {
    type Output = VecN;

    fn div(self, other: VecN) -> Self::Output {
        assert_eq!(self.size(), other.size());

        VecN {
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a / b).collect(),
        }
    }
}

///
/// Vector-scalar multiplication
/// v * s
///
impl Mul<f32> for VecN {
    type Output = VecN;

    fn mul(self, other: f32) -> Self::Output {
        VecN {
            data: self.data.iter().map(|x| x * other).collect(),
        }
    }
}

///
/// Vector-scalar division
/// v / s
///
impl Div<f32> for VecN {
    type Output = VecN;

    fn div(self, other: f32) -> Self::Output {
        VecN {
            data: self.data.iter().map(|x| x / other).collect(),
        }
    }
}

///
/// Reversed vector-scalar multiplication
/// s * v
///
impl Mul<VecN> for f32 {
    type Output = VecN;

    fn mul(self, other: VecN) -> Self::Output {
        VecN {
            data: other.data.iter().map(|x| x * self).collect(),
        }
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for VecN {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }

        self.data.iter().zip(other.data.iter()).all(|(a, b)| approx_eq(*a, *b))
    }
}

///
/// Index Accessor
///
impl Index<usize> for VecN {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size());

        &self.data[index]
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for VecN {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size());

        &mut self.data[index]
    }
}
