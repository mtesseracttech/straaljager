use crate::math::Vec3;
use crate::geometry::primitives::Plane;


#[derive(Debug)]
pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3
}

impl Triangle {
    pub fn new(v0 : &Vec3, v1: &Vec3, v2: &Vec3) -> Triangle{
        Triangle{
            v0: Vec3::clone(v0),
            v1: Vec3::clone(v1),
            v2: Vec3::clone(v2)
        }
    }

    pub fn to_plane(&self) -> Plane{
        let n = Vec3::normalized(&Vec3::cross(&(self.v0 - self.v1),&(self.v0 - self.v2)));
        return Plane{
            normal: n,
            distance: Vec3::dot(&n, &self.v0)
        }
    }

    pub fn point_inside(&self, p: &Vec3){
        let plane = self.to_plane();
    }

    ///Should be made generic
    pub fn interpolate(&self, i0 : &Vec3, i1: &Vec3, i2: &Vec3){

    }
}

