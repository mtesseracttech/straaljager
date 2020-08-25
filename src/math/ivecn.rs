use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct IVecN {
    pub size: usize,
    pub data: Vec<i32>,
}

impl IVecN {
    pub fn new(data: &[i32]) -> IVecN {
        IVecN {
            size: data.len(),
            data: data.to_vec(),
        }
    }

    pub fn zero(size: usize) -> IVecN {
        IVecN {
            size,
            data: vec![0; size],
        }
    }

    pub fn dot(&self, other: &Self) -> i32 {
        assert!(self.size == other.size);

        let mut result: i32 = 0;
        for i in 0..self.size {
            result += self.data[i] * other.data[i];
        }
        result
    }

    pub fn length_squared(&self) -> i32 {
        let mut result: i32 = 0;
        for i in 0..self.size {
            result += self.data[i] * self.data[i];
        }
        result
    }

    pub fn length(&self) -> f32 {
        let mut result: i32 = 0;
        for i in 0..self.size {
            result += self.data[i] * self.data[i];
        }
        (result as f32).sqrt()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_unit(&self) -> bool {
        self.length_squared() == 1
    }
}

impl fmt::Display for IVecN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IVecN {{size: {}, data: {:?}}}", self.size, self.data)
    }
}

impl Neg for IVecN {
    type Output = IVecN;

    fn neg(self) -> Self::Output {
        let mut result = IVecN::zero(self.size);
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
impl Add<IVecN> for IVecN {
    type Output = IVecN;

    fn add(self, other: IVecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = IVecN::zero(self.size);
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
impl Sub<IVecN> for IVecN {
    type Output = IVecN;

    fn sub(self, other: IVecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = IVecN::zero(self.size);
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
impl Mul<IVecN> for IVecN {
    type Output = IVecN;

    fn mul(self, other: IVecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = IVecN::zero(self.size);
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
impl Div<IVecN> for IVecN {
    type Output = IVecN;

    fn div(self, other: IVecN) -> Self::Output {
        assert!(self.size == other.size);

        let mut result = IVecN::zero(self.size);
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
impl Mul<i32> for IVecN {
    type Output = IVecN;

    fn mul(self, other: i32) -> Self::Output {
        let mut result = IVecN::zero(self.size);
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
impl Div<i32> for IVecN {
    type Output = IVecN;

    fn div(self, other: i32) -> Self::Output {
        let mut result = IVecN::zero(self.size);
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
impl Mul<IVecN> for i32 {
    type Output = IVecN;

    fn mul(self, other: IVecN) -> Self::Output {
        let mut result = IVecN::zero(other.size);
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
impl PartialEq for IVecN {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        for i in 0..self.size {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

///
/// Index Accessor
///
impl Index<usize> for IVecN {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size);

        &self.data[index]
    }
}

///
/// Mutable Index Accessor, to assign to the vector through index and to get a mutable index
///
impl IndexMut<usize> for IVecN {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size);

        &mut self.data[index]
    }
}
