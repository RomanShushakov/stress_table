use std::collections::HashMap;
use crate::fe::fe_aux_structs::{Stiffness, SubMatrixIndexes};
use crate::math::matrix::Matrix;


pub trait Element<T, V, W>
{
    fn extract_stiffness_submatrices(&self) -> HashMap<Stiffness<T>, SubMatrixIndexes>;
    fn extract_stiffness_matrix(&mut self) -> Result<Matrix<V>, String>;
}
