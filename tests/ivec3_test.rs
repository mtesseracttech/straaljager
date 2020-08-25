use straaljager::math::ivec3::*;

#[test]
fn test_ivec3_new() {
    let vec: IVec3 = IVec3::new(1, 2, 3);
    assert!(vec.x == 1);
    assert!(vec.y == 2);
    assert!(vec.z == 3);
}

#[test]
fn test_ivec3_dot() {
    let vec1: IVec3 = IVec3::new(1, 2, 3);
    let vec2: IVec3 = IVec3::new(5, 6, 7);
    assert!(vec1.dot(&vec2) == 38);

    let vec3: IVec3 = IVec3::new(1, 1, 1);
    assert!(vec3.dot(&vec3) == 3)
}

#[test]
fn test_ivec3_length_squared() {
    let vec1: IVec3 = IVec3::new(1, 3, 4);
    assert!(vec1.length_squared() == vec1.dot(&vec1));
}

#[test]
fn test_ivec3_length() {
    let vec1: IVec3 = IVec3::new(1, 0, 0);
    assert!(vec1.length() == 1.0);

    let vec2: IVec3 = IVec3::new(1, 1, 1);
    assert!(vec2.length() == 3.0_f32.sqrt());

    let vec2: IVec3 = IVec3::new(2, 4, 5);
    assert!(vec2.length() == 45_f32.sqrt());
}

#[test]
fn test_ivec3_negate() {
    let vec: IVec3 = -IVec3::new(1, -2, 3);

    assert_eq!(vec, IVec3::new(-1, 2, -3));
}

#[test]
fn test_ivec3_add_ivec3() {
    let vec1: IVec3 = IVec3::new(1, -2, 3);
    let vec2: IVec3 = IVec3::new(-5, 6, -7);
    let result: IVec3 = vec1 + vec2;

    assert_eq!(result, IVec3::new(-4, 4, -4));
}

#[test]
fn test_ivec3_sub_ivec3() {
    let vec1: IVec3 = IVec3::new(1, -2, 3);
    let vec2: IVec3 = IVec3::new(-5, 6, -7);
    let result: IVec3 = vec1 - vec2;

    assert_eq!(result, IVec3::new(6, -8, 10));
}

#[test]
fn test_ivec3_mul_ivec3() {
    let vec1: IVec3 = IVec3::new(1, -2, 3);
    let vec2: IVec3 = IVec3::new(-5, 6, -7);
    let result: IVec3 = vec1 * vec2;

    assert_eq!(result, IVec3::new(-5, -12, -21));
}

#[test]
fn test_ivec3_div_ivec3() {
    let vec1: IVec3 = IVec3::new(6, -6, 8);
    let vec2: IVec3 = IVec3::new(-2, 3, -2);
    let result: IVec3 = vec1 / vec2;

    assert_eq!(result, IVec3::new(-3, -2, -4));
}

#[test]
fn test_ivec3_mul_scalar() {
    let vec1: IVec3 = IVec3::new(5, -6, 7);
    let result: IVec3 = vec1 * -2;

    assert_eq!(result, IVec3::new(-10, 12, -14));
}

#[test]
fn test_ivec3_div_scalar() {
    let vec1: IVec3 = IVec3::new(6, -6, 8);
    let result: IVec3 = vec1 / -2;

    assert_eq!(result, IVec3::new(-3, 3, -4));
}

#[test]
fn test_ivec3_mul_scalar_reversed() {
    let vec1: IVec3 = IVec3::new(5, -6, 7);
    let result: IVec3 = -2 * vec1;

    assert_eq!(result, IVec3::new(-10, 12, -14));
}

#[test]
fn test_ivec3_index() {
    let vec1: IVec3 = IVec3::new(5, -6, 7);
    assert!(vec1[0] == 5);
    assert!(vec1[1] == -6);
    assert!(vec1[2] == 7);
}
