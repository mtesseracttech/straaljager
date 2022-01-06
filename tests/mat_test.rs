use straaljager::math::matrix::*;

#[test]
fn test_mat_transpose_non_square() {
    let matrix = Matrix::<i32, 2, 3>::new([[0, 1], [2, 3], [4, 5]]);
    let matrix_transposed = Matrix::<i32, 3, 2>::new([[0, 2, 4], [1, 3, 5]]);

    assert_eq!(matrix.transposed(), matrix_transposed);
}

#[test]
fn test_mat_mul_non_square() {
    let matrix1 = Matrix::<i32, 2, 3>::new([[0, 1], [2, 3], [4, 5]]);
    let matrix2 = Matrix::<i32, 3, 1>::new([[6, 7, 8]]);
    let result = Matrix::<i32, 2, 1>::new([[46, 67]]);

    assert_eq!(&matrix1 * &matrix2, result);
}
