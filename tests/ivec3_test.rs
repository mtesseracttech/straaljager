use straaljager::math::vector::*;

#[test]
fn test_ivec3_new() {
    let vec: IVec3 = IVec3::new(1, 2, 3);
    assert_eq!(vec.e[0], 1);
    assert_eq!(vec.e[1], 2);
    assert_eq!(vec.e[2], 3);
}

#[test]
fn test_ivec3_dot() {
    let vec1: IVec3 = IVec3::new(1, 2, 3);
    let vec2: IVec3 = IVec3::new(5, 6, 7);
    assert_eq!(vec1.dot(&vec2), 38);

    let vec3: IVec3 = IVec3::new(1, 1, 1);
    assert_eq!(vec3.dot(&vec3), 3);
}

#[test]
fn test_ivec3_length_squared() {
    let vec1: IVec3 = IVec3::new(1, 3, 4);
    assert_eq!(vec1.length_squared(), vec1.dot(&vec1));
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
fn test_ivec3_index() {
    let vec1: IVec3 = IVec3::new(5, -6, 7);
    assert_eq!(vec1[0], 5);
    assert_eq!(vec1[1], -6);
    assert_eq!(vec1[2], 7);
}
