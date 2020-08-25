use std::fmt;
use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec2 {
    pub fn zero() -> Self {
        IVec2 { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Self {
        IVec2 { x, y }
    }

    pub fn dot(&self, other: &Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }
}

impl fmt::Display for IVec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IVec2 {{x: {}, y: {}}}", self.x, self.y)
    }
}

impl Neg for IVec2 {
    type Output = IVec2;

    fn neg(self) -> Self::Output {
        IVec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add<IVec2> for IVec2 {
    type Output = IVec2;

    fn add(self, other: IVec2) -> Self::Output {
        IVec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<IVec2> for IVec2 {
    type Output = IVec2;

    fn sub(self, other: IVec2) -> Self::Output {
        IVec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<IVec2> for IVec2 {
    type Output = IVec2;

    fn mul(self, other: IVec2) -> Self::Output {
        IVec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div<IVec2> for IVec2 {
    type Output = IVec2;

    fn div(self, other: IVec2) -> Self::Output {
        IVec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Mul<i32> for IVec2 {
    type Output = IVec2;

    fn mul(self, other: i32) -> Self::Output {
        IVec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<i32> for IVec2 {
    type Output = IVec2;

    fn div(self, other: i32) -> Self::Output {
        IVec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Mul<IVec2> for i32 {
    type Output = IVec2;

    fn mul(self, other: IVec2) -> Self::Output {
        IVec2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl PartialEq for IVec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Index<usize> for IVec2 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Requested an invalid index on an IVec2: {}", index),
        }
    }
}

impl IndexMut<usize> for IVec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Requested an invalid index on an IVec2: {}", index),
        }
    }
}
