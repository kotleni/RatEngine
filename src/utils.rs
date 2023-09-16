use nalgebra::{Matrix4, Rotation3, Vector3};

pub fn get_rotation_from_matrix(matrix: &Matrix4<f32>) -> Rotation3<f32> {
    // Extract the top-left 3x3 portion of the matrix
    let rotation_matrix = matrix.fixed_slice::<3, 3>(0, 0).into_owned();

    // Convert the 3x3 matrix into a Rotation3
    Rotation3::from_matrix_unchecked(rotation_matrix)
}

pub fn get_position_from_matrix(matrix: &Matrix4<f32>) -> Vector3<f32> {
    // Extract the 4th column of the matrix
    let position = matrix.fixed_slice::<3, 1>(0, 3).into_owned();

    position
}