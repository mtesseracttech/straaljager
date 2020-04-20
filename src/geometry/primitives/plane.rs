use crate::math::Vec3;

#[derive(Debug)]
pub struct Plane {
    normal: Vec3,
    distance: f32,
}
