use crate::{Node, NUMBER_OF_DOF};
use crate::fe::elements::element::{Element};
use std::hash::Hash;
use crate::math::matrix::Matrix;
use crate::fe::fe_aux_structs::{compose_stiffness_submatrices_and_displacements, Displacement, Force};
use std::ops::AddAssign;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct State<T, V>
{
    pub stiffness_matrix: Matrix<V>,
    pub displacements_indexes: HashMap<Displacement<T>, usize>,
    pub forces_indexes: HashMap<Force<T>, usize>,
}


pub struct Model<T, V, W>
    where T: Hash + Copy,
{
    pub nodes: Vec<Node<T, V>>,
    pub elements: Vec<Rc<RefCell<dyn Element<T, V, W>>>>,
    pub displacements: Vec<HashMap<Displacement<T>, W>>,
    pub forces: Vec<HashMap<Force<T>, W>>,
    pub state: Option<State<T, V>>,
}


impl<T, V, W> Model<T, V, W>
    where T: Hash + Copy + Eq,
          V: Default + AddAssign + Copy +
             PartialEq<f64>
{
    pub fn create(
        nodes: Vec<Node<T, V>>, elements: Vec<Rc<RefCell<dyn Element<T, V, W>>>>,
        displacements: Vec<HashMap<Displacement<T>, W>>,
        forces: Vec<HashMap<Force<T>, W>>) -> Model<T, V, W>
    {
        Model { nodes, elements, displacements, forces, state: None }
    }


    pub fn compose_global_stiffness_matrix(&mut self) -> Result<(), String>
    {
        let mut global_stiffness_matrix_elements = Vec::new();
        for _ in 0..(self.nodes.len() * NUMBER_OF_DOF as usize)
        {
            let mut current_row = Vec::new();
            for _ in 0..(self.nodes.len() * NUMBER_OF_DOF as usize)
            {
                current_row.push(Default::default());
            }
            global_stiffness_matrix_elements.push(current_row);
        }
        let mut nodes = Vec::new();
        for node in &self.nodes
        {
            nodes.push(node);
        }
        let (
            mut global_displacements_indexes,
            mut global_forces_indexes,
            global_stiffness_submatrices_indexes) =
            compose_stiffness_submatrices_and_displacements(NUMBER_OF_DOF as usize, nodes);
        for element in &self.elements
        {
            let local_stiffness_matrix = element.borrow_mut().extract_stiffness_matrix()?;
            let local_stiffness_submatrices_indexes =
                element.borrow_mut().extract_stiffness_submatrices();
            for (stiffness_indexes, local_submatrix_indexes) in local_stiffness_submatrices_indexes
            {
                if let Some(global_submatrix_indexes) =
                    global_stiffness_submatrices_indexes.get(&stiffness_indexes)
                {
                    for (i_local, i_global) in local_submatrix_indexes.row_indexes
                        .to_owned()
                        .zip(global_submatrix_indexes.row_indexes.to_owned())
                    {
                        for (j_local, j_global) in local_submatrix_indexes.column_indexes
                            .to_owned()
                            .zip(global_submatrix_indexes.column_indexes.to_owned())
                        {
                            global_stiffness_matrix_elements[i_global][j_global] +=
                                local_stiffness_matrix.elements[i_local][j_local];
                        }
                    }
                }
            }
        }
        let mut i = 0;
        while i < global_stiffness_matrix_elements.len()
        {
            if global_stiffness_matrix_elements[i][i] == 0f64
            {
                global_stiffness_matrix_elements.remove(i);

                for (
                        (displacement_component, displacement_index),
                        (force_component, force_index)
                    )  in global_displacements_indexes.clone()
                    .into_iter()
                    .zip(global_forces_indexes.clone())
                {
                    if i == displacement_index
                    {
                        global_displacements_indexes.remove(&displacement_component);
                    }
                    if i < displacement_index
                    {
                        global_displacements_indexes.insert(displacement_component, displacement_index - 1);
                    }
                    if i == force_index
                    {
                        global_forces_indexes.remove(&(force_component));
                    }
                    if i < force_index
                    {
                        global_forces_indexes.insert(force_component, force_index - 1);
                    }
                }
                for j in 0..global_stiffness_matrix_elements.len()
                {
                    global_stiffness_matrix_elements[j].remove(i);
                }
                continue;
            }
            i += 1;
        }
        let model_state = State
            {
                stiffness_matrix: Matrix { elements: global_stiffness_matrix_elements },
                displacements_indexes: global_displacements_indexes,
                forces_indexes: global_forces_indexes,
            };
        self.state = Some(model_state);
        Ok(())
    }

}
