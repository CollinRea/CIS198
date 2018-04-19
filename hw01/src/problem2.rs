/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
  // Panic if the mat1 and mat2 are not the same size
  assert_eq!(mat1.len(), mat2.len());
  let mut return_mat: Matrix = Vec::new();
  for (i, x) in mat1.iter().enumerate() {
    // Panic if each innert matrix isn't equal in size
    assert_eq!(x.len(),mat2[i].len());
    let mut inner_vec: Vec<f32> = Vec::new();
    for (j, y) in x.iter().enumerate() {
      inner_vec.push(y * mat2[i][j]);
    }
    return_mat.push(inner_vec);
  }
  return_mat
}