use std::cmp::Ordering;

///
/// Floating point equality
///
pub fn approx_eq(lhs: &f32, rhs: &f32) -> bool {
    return match lhs.partial_cmp(rhs) {
        None => false,
        Some(o) => match o {
            Ordering::Less => false,
            Ordering::Equal => true,
            Ordering::Greater => false,
        },
    };
}

///
/// Clamps the value of input in between lower and upper
/// if input > upper -> upper
/// if input < lower -> lower
/// else -> input
///
pub fn clamp(input: f32, lower: f32, upper: f32) -> f32 {
    assert!(!(upper < lower));
    return if input < lower {
        lower
    } else if input > upper {
        upper
    } else {
        input
    };
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
    assert!(!(upper < lower));
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
