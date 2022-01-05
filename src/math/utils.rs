pub fn approx_eq(a: f32, b: f32) -> bool {
    a.approx_eq(&b)
}

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(&self, other: &Rhs) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq(&self, other: &f32) -> bool {
        let abs_a = self.abs();
        let abs_b = other.abs();
        let diff = (self - other).abs();
        if self == other {
            true
        } else if *self == 0.0 || *other == 0.0 || diff < std::f32::MIN_POSITIVE {
            diff < 1e-5
        } else {
            diff / (abs_a + abs_b).min(std::f32::MAX) < 1e-5
        }
    }
}

impl ApproxEq for i32 {
    fn approx_eq(&self, other: &i32) -> bool {
        self == other
    }
}

///
/// Clamps the value of input in between lower and upper
/// if input > upper -> upper
/// if input < lower -> lower
/// else -> input
///
pub fn clamp(input: f32, lower: f32, upper: f32) -> f32 {
    assert!(!upper.lt(&lower));
    if input < lower {
        lower
    } else if input > upper {
        upper
    } else {
        input
    }
}

///
/// Linearly interpolates between a and b, uses t as the alpha value
/// t = 0.0 is x
/// t = 0.5 is in the middle between x and y
/// t = 1.0 is y
///
pub fn mix(a: f32, b: f32, t: f32) -> f32 {
    (b - a) * t
}

///
/// Linearly interpolates between a and b
/// and does not allow values of t more than 1 and less than 0
///
pub fn mix_clamped(a: f32, b: f32, t: f32) -> f32 {
    let t = clamp(t, 0.0, 1.0);
    mix(a, b, t)
}

///
/// Cubic Hermite interpolation between upper and lower edges
/// https://en.wikipedia.org/wiki/Smoothstep
///
pub fn smooth_mix(x: f32, upper: f32, lower: f32) -> f32 {
    assert!(!upper.lt(&lower));
    let t = clamp((x - lower) / (upper - lower), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

///
/// Quintic Hermite interpolation between upper and lower edges
/// https://en.wikipedia.org/wiki/Smoothstep
///
pub fn smoother_mix(x: f32, upper: f32, lower: f32) -> f32 {
    let t = clamp((x - lower) / (upper - lower), 0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
