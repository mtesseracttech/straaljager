use crate::math::Vec4;

pub type Color = Vec4;

impl Color {

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            x: r,
            y: g,
            z: b,
            w: 1.0,
        }
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            x: r,
            y: g,
            z: b,
            w: a,
        }
    }
}
