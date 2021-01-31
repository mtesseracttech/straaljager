mod geometry;
mod math;

use math::*;
use crate::geometry::primitives::Triangle;

fn main() {
    let vec = Vec3::new(0.0, 1.0, 1.0) - Vec3::new(1.0, 1.0, 1.0);

    println!("{}", vec);
    println!("{}", vec.length());

    let tri = Triangle::new(
        &Vec3::new(0.0,0.0,0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        &Vec3::new(1.0,0.0,0.0)
    );

    println!("{:?}", tri.to_plane());
}
