use straaljager::math::ivec2::*;

#[test]
fn test_ivec2_new() {
    let vec: IVec2 = IVec2::new(1, 2);
    assert!(vec.x == 1);
    assert!(vec.y == 2);
}

#[test]
fn test_ivec2_dot() {
    let vec1: IVec2 = IVec2::new(1, 2);
    let vec2: IVec2 = IVec2::new(5, 6);
    assert!(vec1.dot(&vec2) == 17);

    let vec3: IVec2 = IVec2::new(1, 1);
    assert!(vec3.dot(&vec3) == 2);
}

#[test]
fn test_ivec2_length_squared() {
    let vec1: IVec2 = IVec2::new(1, 3);
    assert!(vec1.length_squared() == vec1.dot(&vec1));
}

#[test]
fn test_ivec2_length() {
    let vec1: IVec2 = IVec2::new(1, 0);
    assert!(vec1.length() == 1.0);

    let vec2: IVec2 = IVec2::new(1, 1);
    assert!(vec2.length() == 2.0_f32.sqrt());

    let vec2: IVec2 = IVec2::new(2, 4);
    assert!(vec2.length() == 20_f32.sqrt());
}

#[test]
fn test_ivec2_negate() {
    let vec: IVec2 = -IVec2::new(1, -2);

    assert_eq!(vec, IVec2::new(-1, 2));
}

#[test]
fn test_ivec2_add_ivec2() {
    let vec1: IVec2 = IVec2::new(1, -2);
    let vec2: IVec2 = IVec2::new(-5, 6);
    let result: IVec2 = vec1 + vec2;

    assert_eq!(result, IVec2::new(-4, 4));
}

#[test]
fn test_ivec2_sub_ivec2() {
    let vec1: IVec2 = IVec2::new(1, -2);
    let vec2: IVec2 = IVec2::new(-5, 6);
    let result: IVec2 = vec1 - vec2;

    assert_eq!(result, IVec2::new(6, -8));
}

#[test]
fn test_ivec2_mul_ivec2() {
    let vec1: IVec2 = IVec2::new(1, -2);
    let vec2: IVec2 = IVec2::new(-5, 6);
    let result: IVec2 = vec1 * vec2;

    assert_eq!(result, IVec2::new(-5, -12));
}

#[test]
fn test_ivec2_div_ivec2() {
    let vec1: IVec2 = IVec2::new(6, -6);
    let vec2: IVec2 = IVec2::new(-2, 3);
    let result: IVec2 = vec1 / vec2;

    assert_eq!(result, IVec2::new(-3, -2));
}

#[test]
fn test_ivec2_mul_scalar() {
    let vec1: IVec2 = IVec2::new(5, -6);
    let result: IVec2 = vec1 * -2;

    assert_eq!(result, IVec2::new(-10, 12));
}

#[test]
fn test_ivec2_div_scalar() {
    let vec1: IVec2 = IVec2::new(6, -6);
    let result: IVec2 = vec1 / -2;

    assert_eq!(result, IVec2::new(-3, 3));
}

#[test]
fn test_ivec2_mul_scalar_reversed() {
    let vec1: IVec2 = IVec2::new(5, -6);
    let result: IVec2 = -2 * vec1;

    assert_eq!(result, IVec2::new(-10, 12));
}

#[test]
fn test_ivec2_index() {
    let vec1: IVec2 = IVec2::new(5, -6);
    assert!(vec1[0] == 5);
    assert!(vec1[1] == -6);
}
