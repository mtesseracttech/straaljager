use straaljager::math::mat2::*;
use straaljager::math::vec2::*;

#[test]
fn test_mat2_zero() {
    let matrix = Mat2::zero();

    assert_eq!(matrix[0], Vec2::zero());
    assert_eq!(matrix[1], Vec2::zero());
}

#[test]
fn test_mat2_identity() {
    let matrix = Mat2::identity();

    assert_eq!(matrix[0], Vec2::new(1.0, 0.0));
    assert_eq!(matrix[1], Vec2::new(0.0, 1.0));
}

#[test]
fn test_mat2_eq_identity() {
    let mat1 = Mat2::identity();
    let mat2 = Mat2::identity();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat2_transposed_identity() {
    let mat1 = Mat2::identity();
    let mat2 = Mat2::identity().transposed();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat2_transposed_self_twice() {
    let mat1 = Mat2::new([0.0, 1.0, 2.0, 3.0]);
    let mat2 = mat1.transposed().transposed();

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat2_transposed() {
    let mat1 = Mat2::new([0.0, 1.0, 2.0, 3.0]).transposed();
    let mat2 = Mat2::new([0.0, 2.0, 1.0, 3.0]);

    assert_eq!(mat1, mat2);
}

#[test]
fn test_mat2_mul_identity() {
    let mat1 = Mat2::identity();
    let mat2 = Mat2::identity();

    let result = &mat1 * &mat2;

    assert_eq!(result, Mat2::identity());
}

#[test]
fn test_mat2_mul_identity_zero() {
    let mat1 = Mat2::identity();
    let mat2 = Mat2::zero();

    let result = &mat1 * &mat2;

    assert_eq!(result, Mat2::zero());
}

#[test]
fn test_mat2_mul() {
    let mat1 = Mat2::new([1f32, 64f32, 16f32, 25f32]);
    let mat2 = Mat2::new([12f32, -3f32, 54f32, 34f32]);

    let result = &mat1 * &mat2;

    assert_eq!(result, Mat2::new([3468f32, 2173f32, 1542f32, 802f32]));
}

#[test]
fn test_mat2_mul_non_commutative() {
    let mat1 = Mat2::new([1f32, 64f32, 16f32, 25f32]);
    let mat2 = Mat2::new([12f32, -3f32, 54f32, 34f32]);

    assert_ne!(&mat1 * &mat2, &mat2 * &mat1);
}

#[test]
fn test_mat2_adjoint() {
    let mat = Mat2::new([1f32, 64f32, 16f32, 25f32]);
    let res = Mat2::new([25f32, -64f32, -16f32, 1f32]);

    assert_eq!(mat.adjoint(), res);
}

#[test]
fn test_mat2_inverse_identity() {
    let mat = Mat2::identity();
    assert_eq!(mat, mat.inverse());
}

#[test]
fn test_mat2_inverse() {
    let mat = Mat2::new([1f32, 64f32, 16f32, 25f32]);
    assert_eq!(&mat * &mat.inverse(), Mat2::identity());
}
