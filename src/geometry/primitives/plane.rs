use crate::math::Vec3;

#[derive(debug)]
pub struct Plane {
    normal: Vec3,
    d: f32,
}
