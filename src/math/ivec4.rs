use std::fmt;
use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl IVec4 {
    pub fn zero() -> Self {
        IVec4 {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }

    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        IVec4 { x, y, z, w }
    }

    pub fn dot(&self, other: &Self) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn length_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w) as f32).sqrt()
    }

    pub fn is_unit(&self) -> bool {
        self.length_squared() == 1
    }
}

impl fmt::Display for IVec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IVec4 {{x: {}, y: {}, z: {}, w: {}}}",
            self.x, self.y, self.z, self.w
        )
    }
}

impl Neg for IVec4 {
    type Output = IVec4;

    fn neg(self) -> Self::Output {
        IVec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Add<IVec4> for IVec4 {
    type Output = IVec4;

    fn add(self, other: IVec4) -> Self::Output {
        IVec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub<IVec4> for IVec4 {
    type Output = IVec4;

    fn sub(self, other: IVec4) -> Self::Output {
        IVec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<IVec4> for IVec4 {
    type Output = IVec4;

    fn mul(self, other: IVec4) -> Self::Output {
        IVec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Div<IVec4> for IVec4 {
    type Output = IVec4;

    fn div(self, other: IVec4) -> Self::Output {
        IVec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl Mul<i32> for IVec4 {
    type Output = IVec4;

    fn mul(self, other: i32) -> Self::Output {
        IVec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<i32> for IVec4 {
    type Output = IVec4;

    fn div(self, other: i32) -> Self::Output {
        IVec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Mul<IVec4> for i32 {
    type Output = IVec4;

    fn mul(self, other: IVec4) -> Self::Output {
        IVec4 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
            w: self * other.w,
        }
    }
}

impl PartialEq for IVec4 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl Index<usize> for IVec4 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Requested an invalid index on an IVec4: {}", index),
        }
    }
}

impl IndexMut<usize> for IVec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Requested an invalid index on an IVec4: {}", index),
        }
    }
}
