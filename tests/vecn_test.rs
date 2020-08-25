use straaljager::math::utils::approx_eq;
use straaljager::math::vecn::*;

#[test]
fn test_vecn_new() {
    let vec: VecN = VecN::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(vec.size, 5);
    assert!(approx_eq(vec.data[0], 1.0));
    assert!(approx_eq(vec.data[1], 2.0));
    assert!(approx_eq(vec.data[2], 3.0));
    assert!(approx_eq(vec.data[3], 4.0));
    assert!(approx_eq(vec.data[4], 5.0));
}

#[test]
fn test_vecn_dot() {
    let vec1: VecN = VecN::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    let vec2: VecN = VecN::new(&[6.0, 7.0, 8.0, 9.0, 10.0]);
    assert!(approx_eq(vec1.dot(&vec2), 130.0));

    let vec3: VecN = VecN::new(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
    assert!(approx_eq(vec3.dot(&vec3), 6.0));
}

#[test]
fn test_vecn_length_squared() {
    let vec1: VecN = VecN::new(&[1.0, 3.0, 4.0, -4.0, 6.0, -8.0]);
    assert!(approx_eq(vec1.length_squared(), vec1.dot(&vec1)));
}

#[test]
fn test_vecn_length() {
    let vec1: VecN = VecN::new(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    assert!(approx_eq(vec1.length(), 1.0));

    let vec2: VecN = VecN::new(&[1.0, -1.0, 1.0, -1.0, 1.0, -1.0]);
    assert!(approx_eq(vec2.length(), 6.0f32.sqrt()));

    let vec2: VecN = VecN::new(&[2.0, 4.0, 5.0, -6.0, 7.0, -8.0]);
    assert!(approx_eq(vec2.length(), 194.0f32.sqrt()));
}

#[test]
fn test_vecn_normalized() {
    let vec1: VecN = VecN::new(&[4.0, -6.0, 8.0, -12.0, 14.0, -16.0]);
    let vec1_norm: VecN = vec1.normalized();

    assert!(vec1_norm.is_unit());

    let diff: VecN = vec1 / vec1_norm;
    assert!(
        approx_eq(diff[0], diff[1])
            && approx_eq(diff[1], diff[2])
            && approx_eq(diff[2], diff[3])
            && approx_eq(diff[3], diff[4])
            && approx_eq(diff[4], diff[5])
    );
}

#[test]
fn test_vecn_normalize() {
    let mut vec1: VecN = VecN::new(&[4.0, -6.0, 8.0, -12.0]);
    vec1.normalize();

    assert!(vec1.normalized() == vec1);
    assert!(vec1.is_unit());
}

#[test]
fn test_vecn_negate() {
    let vec: VecN = -VecN::new(&[1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);

    assert_eq!(vec, VecN::new(&[-1.0, 2.0, -3.0, 4.0, -5.0, 6.0]));
}

#[test]
fn test_vecn_add_vecn() {
    let vec1: VecN = VecN::new(&[1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);
    let vec2: VecN = VecN::new(&[-7.0, 8.0, -9.0, 10.0, -11.0, 12.0]);
    let result: VecN = vec1 + vec2;

    assert_eq!(result, VecN::new(&[-6.0, 6.0, -6.0, 6.0, -6.0, 6.0]));
}

#[test]
fn test_vecn_sub_vecn() {
    let vec1: VecN = VecN::new(&[1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);
    let vec2: VecN = VecN::new(&[-7.0, 8.0, -9.0, 10.0, -11.0, 12.0]);
    let result: VecN = vec1 - vec2;

    assert_eq!(result, VecN::new(&[8.0, -10.0, 12.0, -14.0, 16.0, -18.0]));
}

#[test]
fn test_vecn_mul_vecn() {
    let vec1: VecN = VecN::new(&[1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);
    let vec2: VecN = VecN::new(&[7.0, -8.0, 9.0, -10.0, 11.0, -12.0]);
    let result: VecN = vec1 * vec2;

    assert_eq!(result, VecN::new(&[7.0, 16.0, 27.0, 40.0, 55.0, 72.0]));
}

#[test]
fn test_vecn_div_vecn() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0]);
    let vec2: VecN = VecN::new(&[-2.0, 3.0, -2.0, 2.0]);
    let result: VecN = vec1 / vec2;

    assert_eq!(result, VecN::new(&[-2.5, -2.0, -3.5, -4.0]));
}

#[test]
fn test_vecn_mul_scalar() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0]);
    let result: VecN = vec1 * -2.0;

    assert_eq!(result, VecN::new(&[-10.0, 12.0, -14.0, 16.0]));
}

#[test]
fn test_vecn_div_scalar() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0]);
    let result: VecN = vec1 / -2.0;

    assert_eq!(result, VecN::new(&[-2.5, 3.0, -3.5, 4.0]));
}

#[test]
fn test_vecn_mul_scalar_reversed() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0]);
    let result: VecN = -2.0 * vec1;

    assert_eq!(result, VecN::new(&[-10.0, 12.0, -14.0, 16.0]));
}

#[test]
fn test_vecn_index() {
    let vec1: VecN = VecN::new(&[5.0, -6.0, 7.0, -8.0, 9.0, -10.0]);
    assert!(approx_eq(vec1[0], 5.0));
    assert!(approx_eq(vec1[1], -6.0));
    assert!(approx_eq(vec1[2], 7.0));
    assert!(approx_eq(vec1[3], -8.0));
    assert!(approx_eq(vec1[4], 9.0));
    assert!(approx_eq(vec1[5], -10.0));
}
