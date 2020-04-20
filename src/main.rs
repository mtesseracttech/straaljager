mod geometry;
mod math;

use geometry::*;
use math::*;

fn main() {
    let vec = Vec3::new(0.0, 1.0, 1.0) - Vec3::new(1.0, 1.0, 1.0);

    println!("{}", vec);
    println!("{}", vec.length());
}
