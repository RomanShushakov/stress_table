use std::ops::Range;
use crate::fe::node::Node;
use std::collections::HashMap;
use std::hash::Hash;


#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum DisplacementComponent
{
    U,
    V,
    W,
    ThetaU,
    ThetaV,
    ThetaW,
}


#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Displacement<T>
{
    pub node_number: T,
    pub component: DisplacementComponent,
}


#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum ForceComponent
{
    RU,
    RV,
    RW,
    RThetaU,
    RThetaV,
    RThetaW,
}


#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Force<T>
{
    pub node_number: T,
    pub component: ForceComponent,
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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
                    let displacement_component = match k
                        {
                            0 => Some(DisplacementComponent::U),
                            1 => Some(DisplacementComponent::V),
                            2 => Some(DisplacementComponent::W),
                            3 => Some(DisplacementComponent::ThetaU),
                            4 => Some(DisplacementComponent::ThetaV),
                            5 => Some(DisplacementComponent::ThetaW),
                            _ => None
                        };
                    if let Some(displacement_component) = displacement_component
                    {
                        displacements_indexes.insert(
                            Displacement
                                {
                                    node_number: nodes[i].number,
                                    component: displacement_component
                                },
                            i * number_of_dof + k);
                    }

                    let force_component = match k
                        {
                            0 => Some(ForceComponent::RU),
                            1 => Some(ForceComponent::RV),
                            2 => Some(ForceComponent::RW),
                            3 => Some(ForceComponent::RThetaU),
                            4 => Some(ForceComponent::RThetaV),
                            5 => Some(ForceComponent::RThetaW),
                            _ => None
                        };
                    if let Some(force_component) = force_component
                    {
                        forces_indexes.insert(
                            Force
                                {
                                    node_number: nodes[i].number,
                                    component: force_component
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
