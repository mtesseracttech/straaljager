use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct IVecN {
    pub data: Vec<i32>,
}

impl IVecN {
    pub fn new(data: &[i32]) -> IVecN {
        IVecN {
            data: data.to_vec(),
        }
    }

    pub fn zero(size: usize) -> IVecN {
        IVecN {
            data: vec![0; size],
        }
    }

    pub fn dot(&self, other: &Self) -> i32 {
        assert_eq!(self.size(), other.size());

        self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum()
    }

    pub fn length_squared(&self) -> i32 {
        self.data.iter().map(|x| x * x).sum()
    }

    pub fn length(&self) -> f32 {
        self.data.iter().map(|x| (x * x) as f32).sum::<f32>().sqrt()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_unit(&self) -> bool {
        self.length_squared() == 1
    }
}

impl fmt::Display for IVecN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IVecN {{size: {}, data: {:?}}}", self.size(), self.data)
    }
}

impl Neg for IVecN {
    type Output = IVecN;

    fn neg(self) -> Self::Output {
        IVecN {
            data: self.data.iter().map(|x| -x).collect(),
        }
    }
}

///
/// Vector addition:
/// v1 + v2
///
impl Add<IVecN> for IVecN {
    type Output = IVecN;

    fn add(self, other: IVecN) -> Self::Output {
        assert_eq!(self.size(), other.size());

        IVecN {
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a + b).collect(),
        }
    }
}

///
/// Vector subtraction:
/// v1 - v2
///
impl Sub<IVecN> for IVecN {
    type Output = IVecN;

    fn sub(self, other: IVecN) -> Self::Output {
        assert_eq!(self.size(), other.size());

        IVecN {
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a - b).collect(),
        }
    }
}

///
/// Member-wise vector * vector implementation
/// v1 - v2
///
impl Mul<IVecN> for IVecN {
    type Output = IVecN;

    fn mul(self, other: IVecN) -> Self::Output {
        assert_eq!(self.size(), other.size());

        IVecN {
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).collect(),
        }
    }
}

///
/// Member-wise vector division
/// v1 / v2
///
impl Div<IVecN> for IVecN {
    type Output = IVecN;

    fn div(self, other: IVecN) -> Self::Output {
        assert_eq!(self.size(), other.size());

        IVecN {
            data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a / b).collect(),
        }
    }
}

///
/// Vector-scalar multiplication
/// v * s
///
impl Mul<i32> for IVecN {
    type Output = IVecN;

    fn mul(self, other: i32) -> Self::Output {
        IVecN {
            data: self.data.iter().map(|x| x * other).collect(),
        }
    }
}

///
/// Vector-scalar division
/// v / s
///
impl Div<i32> for IVecN {
    type Output = IVecN;

    fn div(self, other: i32) -> Self::Output {
        IVecN {
            data: self.data.iter().map(|x| x / other).collect(),
        }
    }
}

///
/// Reversed vector-scalar multiplication
/// s * v
///
impl Mul<IVecN> for i32 {
    type Output = IVecN;

    fn mul(self, other: IVecN) -> Self::Output {
        IVecN {
            data: other.data.iter().map(|x| x * self).collect(),
        }
    }
}

///
/// Partial equality
/// Only returns true if v1 == v2 for every element
///
impl PartialEq for IVecN {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }

        self.data.iter().zip(other.data.iter()).all(|(a, b)| *a ==  *b)
    }
}

///
/// Index Accessor
///
impl Index<usize> for IVecN {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size());

        &self.data[index]
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for IVecN {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size());

        &mut self.data[index]
    }
}
