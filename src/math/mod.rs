pub use ivec2::*;
pub use ivec3::*;
pub use ivec4::*;
pub use ivecn::*;
pub use utils::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use vecn::*;

use std::ops::{Mul, MulAssign};

pub mod ivec2;
pub mod ivec3;
pub mod ivec4;
pub mod ivecn;
pub mod utils;
pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod vecn;

pub trait FloatVector<'a>
where
    Self: 'a + Sized + MulAssign<f32>,
    &'a Self: Mul<f32, Output = Self>,
{
    fn zero() -> Self;

    fn size() -> usize;

    fn dot(&self, other: &Self) -> f32;

    fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn normalized(&'a self) -> Self {
        let one_over_length: f32 = 1.0 / self.length();
        self * one_over_length
    }

    fn normalize(&'a mut self) {
        let one_over_length: f32 = 1.0 / self.length();
        *self *= one_over_length;
    }

    fn is_unit(&self) -> bool {
        approx_eq(self.length_squared(), 1.0)
    }
}

pub trait PhysicsVector: Sized {
    /// Normalizes n and reflects incident vector i over normal n
    ///
    /// i - 2.0 * i.dot(n) * n
    ///
    fn reflect(i: &Self, n: &Self) -> Self;

    ///
    /// Normalizes n and refracts incident vector i over normal n
    /// Refractions don't always exist, so returns None when it does not.
    /// Eta is the ratio of indices of refraction
    ///
    fn refract(i: &Self, n: &Self, eta: f32) -> Option<Self>;
}

// impl<'a, T: FloatVector<'a>> PhysicsVector for T {
//     fn reflect(i: &Self, n: &Self) -> Self {
//         debug_assert!(
//             n.is_unit(),
//             "The reflect function only works with normalized normal vectors"
//         );
//         i - 2.0 * i.dot(n) * n
//     }
//
//     fn refract(i: &Self, n: &Self, eta: f32) -> Option<Self> {
//         debug_assert!(
//             n.is_unit(),
//             "The refraction function only works with normalized normal vectors"
//         );
//         let n_dot_i: f32 = n.dot(i);
//         let k: f32 = 1.0 - eta * eta * (1.0 - n_dot_i * n_dot_i);
//         if k < 0.0 {
//             None
//         } else {
//             Some(eta * i - (eta * n_dot_i + k.sqrt()) * n)
//         }
//     }
// }
