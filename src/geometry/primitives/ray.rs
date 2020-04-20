use crate::math::Vec3;

#[derive(debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
