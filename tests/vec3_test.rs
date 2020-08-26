use straaljager::math::utils::approx_eq;
use straaljager::math::vec3::*;

#[test]
fn test_vec3_new() {
    let vec: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    assert!(approx_eq(vec.x, 1.0));
    assert!(approx_eq(vec.y, 2.0));
    assert!(approx_eq(vec.z, 3.0));
}

#[test]
fn test_vec3_dot() {
    let vec1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let vec2: Vec3 = Vec3::new(5.0, 6.0, 7.0);
    assert!(approx_eq(vec1.dot(&vec2), 38.0));

    let vec3: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    assert!(approx_eq(vec3.dot(&vec3), 3.0));
}

#[test]
fn test_vec3_cross() {
    let vec1: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    let vec2: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    assert_eq!(vec1.cross(&vec2), Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn test_vec3_length_squared() {
    let vec1: Vec3 = Vec3::new(1.0, 3.0, 4.0);
    assert!(approx_eq(vec1.length_squared(), vec1.dot(&vec1)));
}

#[test]
fn test_vec3_length() {
    let vec1: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    assert!(approx_eq(vec1.length(), 1.0));

    let vec2: Vec3 = Vec3::new(1.0, -1.0, 1.0);
    assert!(approx_eq(vec2.length(), 3.0f32.sqrt()));

    let vec2: Vec3 = Vec3::new(2.0, 4.0, 5.0);
    assert!(approx_eq(vec2.length(), 45.0f32.sqrt()));
}

#[test]
fn test_vec3_normalized() {
    let vec1: Vec3 = Vec3::new(4.0, -6.0, 8.0);
    let vec1_norm: Vec3 = vec1.normalized();

    assert!(vec1_norm.is_unit());

    let diff = vec1 / vec1_norm;
    assert!(approx_eq(diff.x, diff.y) && approx_eq(diff.y, diff.z));
}

#[test]
fn test_vec3_normalize() {
    let mut vec1: Vec3 = Vec3::new(4.0, -6.0, 8.0);
    vec1.normalize();

    assert!(vec1.is_unit());
}

#[test]
fn test_vec3_negate() {
    let vec: Vec3 = -Vec3::new(1.0, -2.0, 3.0);

    assert_eq!(vec, Vec3::new(-1.0, 2.0, -3.0));
}

#[test]
fn test_vec3_add_vec3() {
    let vec1: Vec3 = Vec3::new(1.0, -2.0, 3.0);
    let vec2: Vec3 = Vec3::new(-5.0, 6.0, -7.0);
    let result: Vec3 = vec1 + vec2;

    assert_eq!(result, Vec3::new(-4.0, 4.0, -4.0));
}

#[test]
fn test_vec3_sub_vec3() {
    let vec1: Vec3 = Vec3::new(1.0, -2.0, 3.0);
    let vec2: Vec3 = Vec3::new(-5.0, 6.0, -7.0);
    let result: Vec3 = vec1 - vec2;

    assert_eq!(result, Vec3::new(6.0, -8.0, 10.0));
}

#[test]
fn test_vec3_mul_vec3() {
    let vec1: Vec3 = Vec3::new(1.0, -2.0, 3.0);
    let vec2: Vec3 = Vec3::new(-5.0, 6.0, -7.0);
    let result: Vec3 = vec1 * vec2;

    assert_eq!(result, Vec3::new(-5.0, -12.0, -21.0));
}


#[test]
fn test_vec3_div_vec3() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    let vec2: Vec3 = Vec3::new(-2.0, 3.0, -2.0);
    let result: Vec3 = vec1 / vec2;

    assert_eq!(result, Vec3::new(-2.5, -2.0, -3.5));
}

#[test]
fn test_vec3_mul_scalar() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    let result: Vec3 = vec1 * -2.0;

    assert_eq!(result, Vec3::new(-10.0, 12.0, -14.0));
}

#[test]
fn test_vec3_div_scalar() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    let result: Vec3 = vec1 / -2.0;

    assert_eq!(result, Vec3::new(-2.5, 3.0, -3.5));
}

#[test]
fn test_vec3_mul_scalar_reversed() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    let result: Vec3 = -2.0 * vec1;

    assert_eq!(result, Vec3::new(-10.0, 12.0, -14.0));
}

#[test]
fn test_vec3_index() {
    let vec1: Vec3 = Vec3::new(5.0, -6.0, 7.0);
    assert!(approx_eq(vec1[0], 5.0));
    assert!(approx_eq(vec1[1], -6.0));
    assert!(approx_eq(vec1[2], 7.0));
}
