pub use utils::*;
pub use vec2::*;
pub use vec3::*;

pub mod utils;
pub mod vec2;
pub mod vec3;

pub trait PhysicsVector: Sized {
    ///
    /// Normalizes n and reflects incident vector i over normal n
    ///
    /// i - 2.0 * n⋅i * n
    ///
    fn reflect(i: &Self, n: &Self) -> Self;

    ///
    /// Normalizes n and refracts incident vector i over normal n
    /// Refractions don't always exist, so returns None when it does not.
    /// η is the ratio of indices of refraction
    ///
    /// k = 1.0 - η^2 * (1.0 - (n⋅i)^2);
    ///
    /// if k < 0.0 => None
    ///
    /// else => Some(η * i - (η * (n⋅i) + k.sqrt()) * n)
    ///
    fn refract(i: &Self, n: &Self, eta: f32) -> Option<Self>;
}
