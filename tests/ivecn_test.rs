use straaljager::math::vector::*;

#[test]
fn test_ivecn_new() {
    let vec: IVecN = IVecN::from_slice([1, 2, 3, 4, 5]);
    assert_eq!(vec.size(), 5);
    assert_eq!(vec.data[0], 1);
    assert_eq!(vec.data[1], 2);
    assert_eq!(vec.data[2], 3);
    assert_eq!(vec.data[3], 4);
    assert_eq!(vec.data[4], 5);
}

#[test]
fn test_ivecn_dot() {
    let vec1: IVecN = IVecN::from_slice([1, 2, 3, 4, 5]);
    let vec2: IVecN = IVecN::from_slice([6, 7, 8, 9, 10]);
    assert_eq!(vec1.dot(&vec2), 130);

    let vec3: IVecN = IVecN::from_slice([1, 1, 1, 1, 1, 1]);
    assert_eq!(vec3.dot(&vec3), 6);
}

#[test]
fn test_ivecn_length_squared() {
    let vec1: IVecN = IVecN::from_slice([1, 3, 4, -4, 6, -8]);
    assert_eq!(vec1.length_squared(), vec1.dot(&vec1));
}

#[test]
fn test_ivecn_length() {
    let vec1: IVecN = IVecN::from_slice([1, 0, 0, 0, 0, 0]);
    assert_eq!(vec1.length(), 1.0);

    let vec2: IVecN = IVecN::from_slice([1, -1, 1, -1, 1, -1]);
    assert_eq!(vec2.length(), 6_f32.sqrt());

    let vec2: IVecN = IVecN::from_slice([2, 4, 5, -6, 7, -8]);
    assert_eq!(vec2.length(), 194_f32.sqrt());
}

#[test]
fn test_ivecn_negate() {
    let vec: IVecN = -IVecN::from_slice([1, -2, 3, -4, 5, -6]);

    assert_eq!(vec, IVecN::from_slice([-1, 2, -3, 4, -5, 6]));
}

#[test]
fn test_ivecn_add_ivecn() {
    let vec1: IVecN = IVecN::from_slice([1, -2, 3, -4, 5, -6]);
    let vec2: IVecN = IVecN::from_slice([-7, 8, -9, 10, -11, 12]);
    let result: IVecN = vec1 + vec2;

    assert_eq!(result, IVecN::from_slice([-6, 6, -6, 6, -6, 6]));
}

#[test]
fn test_ivecn_sub_ivecn() {
    let vec1: IVecN = IVecN::from_slice([1, -2, 3, -4, 5, -6]);
    let vec2: IVecN = IVecN::from_slice([-7, 8, -9, 10, -11, 12]);
    let result: IVecN = vec1 - vec2;

    assert_eq!(result, IVecN::from_slice([8, -10, 12, -14, 16, -18]));
}

#[test]
fn test_ivecn_mul_ivecn() {
    let vec1: IVecN = IVecN::from_slice([1, -2, 3, -4, 5, -6]);
    let vec2: IVecN = IVecN::from_slice([7, -8, 9, -10, 11, -12]);
    let result: IVecN = vec1 * vec2;

    assert_eq!(result, IVecN::from_slice([7, 16, 27, 40, 55, 72]));
}

#[test]
fn test_ivecn_div_ivecn() {
    let vec1: IVecN = IVecN::from_slice([10, -6, 14, -8]);
    let vec2: IVecN = IVecN::from_slice([-2, 3, -2, 2]);
    let result: IVecN = vec1 / vec2;

    assert_eq!(result, IVecN::from_slice([-5, -2, -7, -4]));
}

#[test]
fn test_ivecn_mul_scalar() {
    let vec1: IVecN = IVecN::from_slice([5, -6, 7, -8]);
    let result: IVecN = vec1 * -2;

    assert_eq!(result, IVecN::from_slice([-10, 12, -14, 16]));
}

#[test]
fn test_ivecn_div_scalar() {
    let vec1: IVecN = IVecN::from_slice([10, -6, 4, -8]);
    let result: IVecN = vec1 / -2;

    assert_eq!(result, IVecN::from_slice([-5, 3, -2, 4]));
}

#[test]
fn test_ivecn_mul_scalar_reversed() {
    let vec1: IVecN = IVecN::from_slice([5, -6, 7, -8]);
    let result: IVecN = -2 * vec1;

    assert_eq!(result, IVecN::from_slice([-10, 12, -14, 16]));
}

#[test]
fn test_ivecn_index() {
    let vec1: IVecN = IVecN::from_slice([5, -6, 7, -8, 9, -10]);
    assert_eq!(vec1[0], 5);
    assert_eq!(vec1[1], -6);
    assert_eq!(vec1[2], 7);
    assert_eq!(vec1[3], -8);
    assert_eq!(vec1[4], 9);
    assert_eq!(vec1[5], -10);
}
