use std::collections::HashMap;
use crate::fe::fe_aux_structs::{Stiffness, SubMatrixIndexes};


pub trait Element<T, V, W>
{
    fn extract_stiffness_submatrices(&self) -> HashMap<Stiffness<T>, SubMatrixIndexes>;
}