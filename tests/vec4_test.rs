use straaljager::math::utils::approx_eq;
use straaljager::math::vec4::*;

#[test]
fn test_vec4_new() {
    let vec: Vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert!(approx_eq(vec.x, 1.0));
    assert!(approx_eq(vec.y, 2.0));
    assert!(approx_eq(vec.z, 3.0));
    assert!(approx_eq(vec.w, 4.0));
}

#[test]
fn test_vec4_dot() {
    let vec1: Vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let vec2: Vec4 = Vec4::new(5.0, 6.0, 7.0, 8.0);
    assert!(approx_eq(vec1.dot(&vec2), 70.0));

    let vec3: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);
    assert!(approx_eq(vec3.dot(&vec3), 4.0));
}

#[test]
fn test_vec4_length_squared() {
    let vec1: Vec4 = Vec4::new(1.0, 3.0, 4.0, -4.0);
    assert!(approx_eq(vec1.length_squared(), vec1.dot(&vec1)));
}

#[test]
fn test_vec4_length() {
    let vec1: Vec4 = Vec4::new(1.0, 0.0, 0.0, 0.0);
    assert!(approx_eq(vec1.length(), 1.0));

    let vec2: Vec4 = Vec4::new(1.0, -1.0, 1.0, -1.0);
    assert!(approx_eq(vec2.length(), 2.0));

    let vec2: Vec4 = Vec4::new(2.0, 4.0, 5.0, -6.0);
    assert!(approx_eq(vec2.length(), 9.0));
}

#[test]
fn test_vec4_normalized() {
    let vec1: Vec4 = Vec4::new(4.0, -6.0, 8.0, -12.0);
    let vec1_norm: Vec4 = vec1.normalized();

    assert!(vec1_norm.is_unit());

    let diff = vec1 / vec1_norm;
    assert!(approx_eq(diff.x, diff.y) && approx_eq(diff.y, diff.z) && approx_eq(diff.z, diff.w));
}

#[test]
fn test_vec4_normalize() {
    let mut vec1: Vec4 = Vec4::new(4.0, -6.0, 8.0, -12.0);
    vec1.normalize();

    assert!(vec1.is_unit());
}

#[test]
fn test_vec4_negate() {
    let vec: Vec4 = -Vec4::new(1.0, -2.0, 3.0, -4.0);

    assert_eq!(vec, Vec4::new(-1.0, 2.0, -3.0, 4.0));
}

#[test]
fn test_vec4_add_vec4() {
    let vec1: Vec4 = Vec4::new(1.0, -2.0, 3.0, -4.0);
    let vec2: Vec4 = Vec4::new(-5.0, 6.0, -7.0, 8.0);
    let result: Vec4 = vec1 + vec2;

    assert_eq!(result, Vec4::new(-4.0, 4.0, -4.0, 4.0));
}

#[test]
fn test_vec4_sub_vec4() {
    let vec1: Vec4 = Vec4::new(1.0, -2.0, 3.0, -4.0);
    let vec2: Vec4 = Vec4::new(-5.0, 6.0, -7.0, 8.0);
    let result: Vec4 = vec1 - vec2;

    assert_eq!(result, Vec4::new(6.0, -8.0, 10.0, -12.0));
}

#[test]
fn test_vec4_mul_vec4() {
    let vec1: Vec4 = Vec4::new(1.0, -2.0, 3.0, -4.0);
    let vec2: Vec4 = Vec4::new(-5.0, 6.0, -7.0, 8.0);
    let result: Vec4 = vec1 * vec2;

    assert_eq!(result, Vec4::new(-5.0, -12.0, -21.0, -32.0));
}


#[test]
fn test_vec4_div_vec4() {
    let vec1: Vec4 = Vec4::new(5.0, -6.0, 7.0, -8.0);
    let vec2: Vec4 = Vec4::new(-2.0, 3.0, -2.0, 2.0);
    let result: Vec4 = vec1 / vec2;

    assert_eq!(result, Vec4::new(-2.5, -2.0, -3.5, -4.0));
}

#[test]
fn test_vec4_mul_scalar() {
    let vec1: Vec4 = Vec4::new(5.0, -6.0, 7.0, -8.0);
    let result: Vec4 = vec1 * -2.0;

    assert_eq!(result, Vec4::new(-10.0, 12.0, -14.0, 16.0));
}

#[test]
fn test_vec4_div_scalar() {
    let vec1: Vec4 = Vec4::new(5.0, -6.0, 7.0, -8.0);
    let result: Vec4 = vec1 / -2.0;

    assert_eq!(result, Vec4::new(-2.5, 3.0, -3.5, 4.0));
}

#[test]
fn test_vec4_mul_scalar_reversed() {
    let vec1: Vec4 = Vec4::new(5.0, -6.0, 7.0, -8.0);
    let result: Vec4 = -2.0 * vec1;

    assert_eq!(result, Vec4::new(-10.0, 12.0, -14.0, 16.0));
}

#[test]
fn test_vec4_index() {
    let vec1: Vec4 = Vec4::new(5.0, -6.0, 7.0, -8.0);
    assert!(approx_eq(vec1[0], 5.0));
    assert!(approx_eq(vec1[1], -6.0));
    assert!(approx_eq(vec1[2], 7.0));
    assert!(approx_eq(vec1[3], -8.0));
}
