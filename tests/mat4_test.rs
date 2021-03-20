use straaljager::math::mat4::*;
use straaljager::math::vec4::*;

#[test]
fn test_mat4_zero() {
    let matrix = Mat4::zero();

    assert_eq!(matrix[0], Vec4::zero());
    assert_eq!(matrix[1], Vec4::zero());
    assert_eq!(matrix[2], Vec4::zero());
    assert_eq!(matrix[3], Vec4::zero());
}

#[test]
fn test_mat4_identity() {
    let matrix = Mat4::identity();

    assert_eq!(matrix[0], Vec4::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(matrix[1], Vec4::new(0.0, 1.0, 0.0, 0.0));
    assert_eq!(matrix[2], Vec4::new(0.0, 0.0, 1.0, 0.0));
    assert_eq!(matrix[3], Vec4::new(0.0, 0.0, 0.0, 1.0));
}

#[test]
fn test_mat4_eq_identity() {
    let mat1 = Mat4::identity();
    let mat2 = Mat4::identity();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat4_transposed_identity() {
    let mat1 = Mat4::identity();
    let mat2 = Mat4::identity().transposed();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat4_transposed_self_twice() {
    let mat1 = Mat4::new([
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
    ]);
    let mat2 = mat1.transposed().transposed();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat4_transposed() {
    let mat1 = Mat4::new([
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
    ])
    .transposed();
    let mat2 = Mat4::new([
        0.0, 4.0, 8.0, 12.0, 1.0, 5.0, 9.0, 13.0, 2.0, 6.0, 10.0, 14.0, 3.0, 7.0, 11.0, 15.0,
    ]);

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
    let mat1 = Mat4::new([
        1f32, 64f32, 16f32, 25f32, 4f32, -4f32, 5f32, -4f32, 2f32, 54f32, -4f32, 43f32, -59f32,
        23f32, 59f32, -14f32,
    ]);
    let mat2 = Mat4::new([
        12f32, -3f32, 54f32, 34f32, -23f32, 0f32, -12f32, 43f32, 67f32, -89f32, 19f32, 76f32,
        49f32, 56f32, 34f32, 53f32,
    ]);

    let result = mat1 * mat2;

    assert_eq!(
        result,
        Mat4::new([
            837f32, -27f32, 440f32, 5327f32, 279f32, -681f32, 223f32, 132f32, 621f32, 2758f32,
            846f32, 4365f32, 2030f32, -5858f32, -2817f32, 2725f32
        ])
    );
}