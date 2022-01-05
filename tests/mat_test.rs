use straaljager::math::matrix::*;

#[test]
fn test_mat_transpose_non_square() {
    let matrix = Matrix::<i32, 2, 3>::new([[0, 1], [2, 3], [4, 5]]);
    let matrix_transposed = Matrix::<i32, 3, 2>::new([[0, 2, 4], [1, 3, 5]]);

    assert_eq!(matrix.transposed(), matrix_transposed);
}
