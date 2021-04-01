use straaljager::math::utils::approx_eq;
use straaljager::math::vector::*;

#[test]
fn test_vec2_new() {
    let vec: Vec2 = Vec2::new(1.0, 2.0);
    assert!(approx_eq(vec.x, 1.0));
    assert!(approx_eq(vec.y, 2.0));
}

#[test]
fn test_vec2_dot() {
    let vec1: Vec2 = Vec2::new(1.0, 2.0);
    let vec2: Vec2 = Vec2::new(5.0, 6.0);
    assert!(approx_eq(vec1.dot(&vec2), 17.0));

    let vec2: Vec2 = Vec2::new(1.0, 1.0);
    assert!(approx_eq(vec2.dot(&vec2), 2.0));
}

#[test]
fn test_vec2_length_squared() {
    let vec1: Vec2 = Vec2::new(1.0, 3.0);
    assert!(approx_eq(vec1.length_squared(), vec1.dot(&vec1)));
}

#[test]
fn test_vec2_length() {
    let vec1: Vec2 = Vec2::new(1.0, 0.0);
    assert!(approx_eq(vec1.length(), 1.0));

    let vec2: Vec2 = Vec2::new(1.0, -1.0);
    assert!(approx_eq(vec2.length(), 2.0f32.sqrt()));

    let vec2: Vec2 = Vec2::new(2.0, 4.0);
    assert!(approx_eq(vec2.length(), 20.0f32.sqrt()));
}

#[test]
fn test_vec2_normalized() {
    let vec1: Vec2 = Vec2::new(4.0, -6.0);
    let vec1_norm: Vec2 = vec1.normalized();

    assert!(vec1_norm.is_unit());

    let diff = vec1 / vec1_norm;
    assert!(approx_eq(diff.x, diff.y));
}

#[test]
fn test_vec2_normalize() {
    let mut vec1: Vec2 = Vec2::new(4.0, -6.0);
    vec1.normalize();

    assert!(vec1.is_unit());
}

#[test]
fn test_vec2_negate() {
    let vec: Vec2 = -Vec2::new(1.0, -2.0);

    assert_eq!(vec, Vec2::new(-1.0, 2.0));
}

#[test]
fn test_vec2_add_vec2() {
    let vec1: Vec2 = Vec2::new(1.0, -2.0);
    let vec2: Vec2 = Vec2::new(-5.0, 6.0);
    let result: Vec2 = vec1 + vec2;

    assert_eq!(result, Vec2::new(-4.0, 4.0));
}

#[test]
fn test_vec2_sub_vec2() {
    let vec1: Vec2 = Vec2::new(1.0, -2.0);
    let vec2: Vec2 = Vec2::new(-5.0, 6.0);
    let result: Vec2 = vec1 - vec2;

    assert_eq!(result, Vec2::new(6.0, -8.0));
}

#[test]
fn test_vec2_mul_vec2() {
    let vec1: Vec2 = Vec2::new(1.0, -2.0);
    let vec2: Vec2 = Vec2::new(-5.0, 6.0);
    let result: Vec2 = vec1 * vec2;

    assert_eq!(result, Vec2::new(-5.0, -12.0));
}

#[test]
fn test_vec2_div_vec2() {
    let vec1: Vec2 = Vec2::new(5.0, -6.0);
    let vec2: Vec2 = Vec2::new(-2.0, 3.0);
    let result: Vec2 = vec1 / vec2;

    assert_eq!(result, Vec2::new(-2.5, -2.0));
}

#[test]
fn test_vec2_mul_scalar() {
    let vec1: Vec2 = Vec2::new(5.0, -6.0);
    let result: Vec2 = vec1 * -2.0;

    assert_eq!(result, Vec2::new(-10.0, 12.0));
}

#[test]
fn test_vec2_div_scalar() {
    let vec1: Vec2 = Vec2::new(5.0, -6.0);
    let result: Vec2 = vec1 / -2.0;

    assert_eq!(result, Vec2::new(-2.5, 3.0));
}

#[test]
fn test_vec2_mul_scalar_reversed() {
    let vec1: Vec2 = Vec2::new(5.0, -6.0);
    let result: Vec2 = -2.0 * vec1;

    assert_eq!(result, Vec2::new(-10.0, 12.0));
}

#[test]
fn test_vec2_index() {
    let vec1: Vec2 = Vec2::new(5.0, -6.0);
    assert!(approx_eq(vec1[0], 5.0));
    assert!(approx_eq(vec1[1], -6.0));
}
