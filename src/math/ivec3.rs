use std::fmt;
use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl IVec3 {
    pub fn zero() -> Self {
        IVec3 { x: 0, y: 0, z: 0 }
    }

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        IVec3 { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y + self.z * self.z) as f32).sqrt()
    }
}

impl fmt::Display for IVec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IVec3 {{x: {}, y: {}, z: {}}}", self.x, self.y, self.z)
    }
}

impl Neg for IVec3 {
    type Output = IVec3;

    fn neg(self) -> Self::Output {
        IVec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<IVec3> for IVec3 {
    type Output = IVec3;

    fn add(self, other: IVec3) -> Self::Output {
        IVec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<IVec3> for IVec3 {
    type Output = IVec3;

    fn sub(self, other: IVec3) -> Self::Output {
        IVec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<IVec3> for IVec3 {
    type Output = IVec3;

    fn mul(self, other: IVec3) -> Self::Output {
        IVec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<IVec3> for IVec3 {
    type Output = IVec3;

    fn div(self, other: IVec3) -> Self::Output {
        IVec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Mul<i32> for IVec3 {
    type Output = IVec3;

    fn mul(self, other: i32) -> Self::Output {
        IVec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<i32> for IVec3 {
    type Output = IVec3;

    fn div(self, other: i32) -> Self::Output {
        IVec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Mul<IVec3> for i32 {
    type Output = IVec3;

    fn mul(self, other: IVec3) -> Self::Output {
        IVec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl PartialEq for IVec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Index<usize> for IVec3 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Requested an invalid index on an IVec3: {}", index),
        }
    }
}

impl IndexMut<usize> for IVec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Requested an invalid index on an IVec3: {}", index),
        }
    }
}
