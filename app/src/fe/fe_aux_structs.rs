use std::ops::Range;
use crate::fe::node::Node;
use std::collections::HashMap;
use std::hash::Hash;


#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Component
{
    U,
    V,
    W,
    ThetaU,
    ThetaV,
    ThetaW,
}


#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Displacement<T>
{
    pub node_number: T,
    pub component: Component,
}


#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Force<T>
{
    pub node_number: T,
    pub component: Component,
}


#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Stiffness<T>
{
    pub first_index: T,
    pub second_index: T,
}


#[derive(Debug, Clone)]
pub struct SubMatrixIndexes
{
    pub row_indexes: Range<usize>,
    pub column_indexes: Range<usize>,
}


pub fn compose_stiffness_submatrices_and_displacements<T, V>(number_of_dof: usize, nodes: Vec<&Node<T, V>>)
    -> (HashMap<Displacement<T>, usize>, HashMap<Force<T>, usize> ,HashMap<Stiffness<T>, SubMatrixIndexes>)
        where T: PartialEq + Eq + Hash + Copy
{
    let mut displacements_indexes = HashMap::new();
    let mut forces_indexes = HashMap::new();
    let mut stiffness_submatrices_indexes = HashMap::new();
    for i in 0..nodes.len()
    {
        for j in i..nodes.len()
        {
            if i == j
            {
                for k in 0..number_of_dof
                {

                    let component = match k
                        {
                            0 => Some(Component::U),
                            1 => Some(Component::V),
                            2 => Some(Component::W),
                            3 => Some(Component::ThetaU),
                            4 => Some(Component::ThetaV),
                            5 => Some(Component::ThetaW),
                            _ => None
                        };
                    if let Some(comp) = component
                    {
                        displacements_indexes.insert(
                            Displacement
                                {
                                    node_number: nodes[i].number,
                                    component: comp.to_owned()
                                },
                            i * number_of_dof + k);
                        forces_indexes.insert(
                            Force
                                {
                                    node_number: nodes[i].number,
                                    component: comp
                                },
                            i * number_of_dof + k);
                    }
                }
                stiffness_submatrices_indexes.insert
                    (
                        Stiffness { first_index: nodes[i].number, second_index: nodes[i].number },
                        SubMatrixIndexes
                            {
                                row_indexes: (i * number_of_dof)..(i * number_of_dof + number_of_dof),
                                column_indexes: (i * number_of_dof)..(i * number_of_dof + number_of_dof),
                            }
                    );
            }
            else
            {
                stiffness_submatrices_indexes.insert
                    (
                        Stiffness { first_index: nodes[i].number, second_index: nodes[j].number },
                        SubMatrixIndexes
                            {
                                row_indexes: (i * number_of_dof)..(i * number_of_dof + number_of_dof),
                                column_indexes: (j * number_of_dof)..(j * number_of_dof + number_of_dof),
                            }
                    );
                stiffness_submatrices_indexes.insert
                    (
                        Stiffness { first_index: nodes[j].number, second_index: nodes[i].number },
                        SubMatrixIndexes
                            {
                                row_indexes: (j * number_of_dof)..(j * number_of_dof + number_of_dof),
                                column_indexes: (i * number_of_dof)..(i * number_of_dof + number_of_dof),
                            }
                    );
            }
        }
    }
    (displacements_indexes, forces_indexes, stiffness_submatrices_indexes)
}
