pub use utils::*;
pub use vector::*;

pub mod utils;
pub mod vector;

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