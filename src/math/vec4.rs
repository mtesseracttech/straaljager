use std::ops;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec4 {
    pub fn zero() -> Self {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w}
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(&self) -> Vec4 {
        let one_over_length: f32 = 1.0 / self.length();
        Vec4 {
            x: self.x * one_over_length,
            y: self.y * one_over_length,
            z: self.z * one_over_length,
            w: self.w * one_over_length,
        }
    }

    pub fn normalize(&mut self) {
        let one_over_length: f32 = 1.0 / self.length();
        self.x *= one_over_length;
        self.y *= one_over_length;
        self.z *= one_over_length;
        self.w *= one_over_length,
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec4 {{x: {}, y: {}, z: {}, w: {}}}", self.x, self.y, self.z, self.w)
    }
}

impl ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// Vec4 + Vec4

impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl ops::Div<Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

// Vec4 + f32

impl ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: f32) -> Self::Output {
        Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl ops::Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, other: f32) -> Self::Output {
        Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

// f32 + f32

impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Self::Output {
        Vec4 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
            w: self * other.w,
        }
    }
}
