pub use utils::*;
pub use vector::*;

pub mod utils;
pub mod vector;

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
