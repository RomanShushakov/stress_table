use std::collections::HashMap;
use crate::fe::fe_aux_structs::
    {
        Stiffness, SubMatrixIndexes, Displacement, StrainStress
    };
use crate::math::matrix::Matrix;


pub struct ElementInfo<T, W>
{
    pub number: T,
    pub nodes_numbers: Vec<T>,
    pub stiffness_properties: Vec<W>
}


pub trait FElement<T, V, W>
{
    fn extract_stiffness_submatrices(&self) -> HashMap<Stiffness<T>, SubMatrixIndexes>;
    fn extract_stiffness_matrix(&mut self) -> Result<Matrix<V>, String>;
    fn calculate_strains_and_stresses(&mut self, global_displacements: &HashMap<Displacement<T>, V>)
        -> Result<HashMap<T, Vec<StrainStress<V>>>, String>;
    fn show_info(&self) -> ElementInfo<T, W>;
}
