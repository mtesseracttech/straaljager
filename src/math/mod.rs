pub use utils::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use vecn::*;

pub use mat2::*;
pub use mat3::*;
pub use mat4::*;

pub mod utils;
pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod vecn;

pub mod mat2;
pub mod mat3;
pub mod mat4;

pub trait PhysicsVector: Sized {
    ///
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
    /// k = 1.0 - eta * eta * (1.0 - n.dot(i) * n.dot(i));
    /// if k < 0.0 => None
    /// else       => Some(eta * i - (eta * n.dot(i) + k.sqrt()) * n)
    ///
    fn refract(i: &Self, n: &Self, eta: f32) -> Option<Self>;
}
