use straaljager::math::mat3::*;
use straaljager::math::vec3::*;

#[test]
fn test_mat3_zero() {
    let matrix = Mat3::zero();

    assert_eq!(matrix[0], Vec3::zero());
    assert_eq!(matrix[1], Vec3::zero());
    assert_eq!(matrix[2], Vec3::zero());
}

#[test]
fn test_mat3_identity() {
    let matrix = Mat3::identity();

    assert_eq!(matrix[0], Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(matrix[1], Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(matrix[2], Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn test_mat3_eq_identity() {
    let mat1 = Mat3::identity();
    let mat2 = Mat3::identity();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat3_transposed_identity() {
    let mat1 = Mat3::identity();
    let mat2 = Mat3::identity().transposed();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat3_transposed_self_twice() {
    let mat1 = Mat3::new([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let mat2 = mat1.transposed().transposed();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat3_transposed() {
    let mat1 = Mat3::new([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]).transposed();
    let mat2 = Mat3::new([0.0, 3.0, 6.0, 1.0, 4.0, 7.0, 2.0, 5.0, 8.0]);

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat3_mul_identity() {
    let mat1 = Mat3::identity();
    let mat2 = Mat3::identity();

    let result = &mat1 * &mat2;

    assert_eq!(result, Mat3::identity());
}

#[test]
fn test_mat3_mul_identity_zero() {
    let mat1 = Mat3::identity();
    let mat2 = Mat3::zero();

    let result = &mat1 * &mat2;

    assert_eq!(result, Mat3::zero());
}

#[test]
fn test_mat3_mul() {
    let mat1 = Mat3::new([1f32, 64f32, 16f32, 25f32, 4f32, -4f32, 5f32, -4f32, 2f32]);
    let mat2 = Mat3::new([
        12f32, -3f32, 54f32, 34f32, -23f32, 0f32, -12f32, 43f32, 67f32,
    ]);

    let result = &mat1 * &mat2;

    assert_eq!(
        result,
        Mat3::new([1996f32, -787f32, 1126f32, 484f32, -339f32, 1082f32, -100f32, 163f32, 404f32])
    );
}

#[test]
fn test_mat3_mul_non_commutative() {
    let mat1 = Mat3::new([1f32, 64f32, 16f32, 25f32, 4f32, -4f32, 5f32, -4f32, 2f32]);
    let mat2 = Mat3::new([
        12f32, -3f32, 54f32, 34f32, -23f32, 0f32, -12f32, 43f32, 67f32,
    ]);

    assert_ne!(&mat1 * &mat2, &mat2 * &mat1);
}

#[test]
fn test_mat3_adjoint() {
    let mat = Mat3::new([1f32, 64f32, 16f32, 25f32, 4f32, -4f32, 5f32, -4f32, 2f32]);
    let res = Mat3::new([
        -8f32, -192f32, -320f32, -70f32, -78f32, 404f32, -120f32, 324f32, -1596f32,
    ]);

    assert_eq!(mat.adjoint(), res);
}

#[test]
fn test_mat3_inverse_identity() {
    let mat = Mat3::identity();
    assert_eq!(mat, mat.inverse());
}

#[test]
fn test_mat3_inverse() {
    let mat = Mat3::new([1f32, 64f32, 16f32, 25f32, 4f32, -4f32, 5f32, -4f32, 2f32]);
    assert_eq!(&mat * &mat.inverse(), Mat3::identity());
}
