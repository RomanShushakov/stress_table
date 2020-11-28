use crate::{Node, NUMBER_OF_DOF};
use crate::fe::elements::element::{Element};
use std::hash::Hash;
use crate::math::matrix::Matrix;
use crate::fe::fe_aux_structs::compose_stiffness_submatrices_and_displacements;


pub struct Model<T, V, W>
    where T: Hash + Copy,

{
    pub nodes: Vec<Node<T, V>>,
    pub elements: Vec<Box<dyn Element<T, V, W>>>
}


impl<T, V, W> Model<T, V, W>
    where T: Hash + Copy + Eq,
          V: Default,
{
    pub fn compose_global_stiffness_matrix(&self) -> Matrix<V>
    {
        let mut elements = Vec::new();
        for _ in 0..(self.nodes.len() * NUMBER_OF_DOF as usize)
        {
            let mut current_row = Vec::new();
            for _ in 0..(self.nodes.len() * NUMBER_OF_DOF as usize)
            {
                current_row.push(Default::default());
            }
            elements.push(current_row);
        }
        let mut nodes = Vec::new();
        for node in &self.nodes
        {
            nodes.push(node);
        }
        let (mut global_displacements, global_stiffnesses) =
            compose_stiffness_submatrices_and_displacements(NUMBER_OF_DOF as usize, nodes);



        Matrix { elements }
    }
}