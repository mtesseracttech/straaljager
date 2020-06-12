use straaljager::assert_approx_eq;
use straaljager::math::mat4::*;
use straaljager::math::utils::approx_eq;

#[test]
fn test_mat4_zero() {
    let matrix = Mat4::zero();

    for i in 0..15 {
        assert_approx_eq!(matrix[i], 0.0f32);
    }
}

#[test]
fn test_mat4_identity() {
    let matrix = Mat4::identity();

    assert_approx_eq!(matrix[0], 1.0f32);
    assert_approx_eq!(matrix[1], 0.0f32);
    assert_approx_eq!(matrix[2], 0.0f32);
    assert_approx_eq!(matrix[3], 0.0f32);

    assert_approx_eq!(matrix[4], 0.0f32);
    assert_approx_eq!(matrix[5], 1.0f32);
    assert_approx_eq!(matrix[6], 0.0f32);
    assert_approx_eq!(matrix[7], 0.0f32);

    assert_approx_eq!(matrix[8], 0.0f32);
    assert_approx_eq!(matrix[9], 0.0f32);
    assert_approx_eq!(matrix[10], 1.0f32);
    assert_approx_eq!(matrix[11], 0.0f32);

    assert_approx_eq!(matrix[12], 0.0f32);
    assert_approx_eq!(matrix[13], 0.0f32);
    assert_approx_eq!(matrix[14], 0.0f32);
    assert_approx_eq!(matrix[15], 1.0f32);
}

#[test]
fn test_mat4_eq_identity() {
    let mat1 = Mat4::identity();
    let mat2 = Mat4::identity();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat4_mul_identity() {
    let mat1 = Mat4::identity();
    let mat2 = Mat4::identity();

    let result = mat1 * mat2;

    assert_eq!(result, Mat4::identity());
}

#[test]
fn test_mat4_mul_identity_zero() {
    let mat1 = Mat4::identity();
    let mat2 = Mat4::zero();

    let result = mat1 * mat2;

    assert_eq!(result, Mat4::zero());
}

#[test]
fn test_mat4_mul() {
    let mat1 = Mat4::new([1f32, 64f32, 16f32, 25f32, 4f32, -4f32, 5f32, -4f32, 2f32, 54f32, -4f32, 43f32, -59f32, 23f32, 59f32, -14f32]);
    let mat2 = Mat4::new([12f32, -3f32, 54f32, 34f32, -23f32, 0f32, -12f32, 43f32, 67f32, -89f32, 19f32, 76f32, 49f32, 56f32, 34f32, 53f32]);

    let result = mat1 * mat2;

    assert_eq!(result, Mat4::new([837f32, -27f32, 440f32, 5327f32, 279f32, -681f32, 223f32, 132f32, 621f32, 2758f32, 846f32, 4365f32, 2030f32, -5858f32, -2817f32, 2725f32]));
}

