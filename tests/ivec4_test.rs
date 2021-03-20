use straaljager::math::ivec4::*;

#[test]
fn test_ivec4_new() {
    let vec: IVec4 = IVec4::new(1, 2, 3, 4);
    assert_eq!(vec.x, 1);
    assert_eq!(vec.y, 2);
    assert_eq!(vec.z, 3);
    assert_eq!(vec.w, 4);
}

#[test]
fn test_ivec4_dot() {
    let vec1: IVec4 = IVec4::new(1, 2, 3, 4);
    let vec2: IVec4 = IVec4::new(5, 6, 7, 8);
    assert_eq!(vec1.dot(&vec2), 70);

    let vec3: IVec4 = IVec4::new(1, 1, 1, 1);
    assert_eq!(vec3.dot(&vec3), 4);
}

#[test]
fn test_ivec4_length_squared() {
    let vec1: IVec4 = IVec4::new(1, 3, 4, -4);
    assert_eq!(vec1.length_squared(), vec1.dot(&vec1));
}

#[test]
fn test_ivec4_length() {
    let vec1: IVec4 = IVec4::new(1, 0, 0, 0);
    assert_eq!(vec1.length(), 1.0);

    let vec2: IVec4 = IVec4::new(1, -1, 1, -1);
    assert_eq!(vec2.length(), 2.0);

    let vec2: IVec4 = IVec4::new(2, 4, 5, -6);
    assert_eq!(vec2.length(), 9.0);
}

#[test]
fn test_ivec4_negate() {
    let vec: IVec4 = -IVec4::new(1, -2, 3, -4);

    assert_eq!(vec, IVec4::new(-1, 2, -3, 4));
}

#[test]
fn test_ivec4_add_ivec4() {
    let vec1: IVec4 = IVec4::new(1, -2, 3, -4);
    let vec2: IVec4 = IVec4::new(-5, 6, -7, 8);
    let result: IVec4 = vec1 + vec2;

    assert_eq!(result, IVec4::new(-4, 4, -4, 4));
}

#[test]
fn test_ivec4_sub_ivec4() {
    let vec1: IVec4 = IVec4::new(1, -2, 3, -4);
    let vec2: IVec4 = IVec4::new(-5, 6, -7, 8);
    let result: IVec4 = vec1 - vec2;

    assert_eq!(result, IVec4::new(6, -8, 10, -12));
}

#[test]
fn test_ivec4_mul_ivec4() {
    let vec1: IVec4 = IVec4::new(1, -2, 3, -4);
    let vec2: IVec4 = IVec4::new(-5, 6, -7, 8);
    let result: IVec4 = vec1 * vec2;

    assert_eq!(result, IVec4::new(-5, -12, -21, -32));
}

#[test]
fn test_ivec4_div_ivec4() {
    let vec1: IVec4 = IVec4::new(6, -6, 8, -8);
    let vec2: IVec4 = IVec4::new(-2, 3, -2, 2);
    let result: IVec4 = vec1 / vec2;

    assert_eq!(result, IVec4::new(-3, -2, -4, -4));
}

#[test]
fn test_ivec4_mul_scalar() {
    let vec1: IVec4 = IVec4::new(5, -6, 7, -8);
    let result: IVec4 = vec1 * -2;

    assert_eq!(result, IVec4::new(-10, 12, -14, 16));
}

#[test]
fn test_ivec4_div_scalar() {
    let vec1: IVec4 = IVec4::new(6, -6, 8, -8);
    let result: IVec4 = vec1 / -2;

    assert_eq!(result, IVec4::new(-3, 3, -4, 4));
}

#[test]
fn test_ivec4_mul_scalar_reversed() {
    let vec1: IVec4 = IVec4::new(5, -6, 7, -8);
    let result: IVec4 = -2 * vec1;

    assert_eq!(result, IVec4::new(-10, 12, -14, 16));
}

#[test]
fn test_ivec4_index() {
    let vec1: IVec4 = IVec4::new(5, -6, 7, -8);
    assert_eq!(vec1[0], 5);
    assert_eq!(vec1[1], -6);
    assert_eq!(vec1[2], 7);
    assert_eq!(vec1[3], -8);
}
