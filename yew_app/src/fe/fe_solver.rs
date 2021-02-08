use crate::{FeNode, NUMBER_OF_DOF};
use crate::fe::elements::f_element::{FElement};
use std::hash::Hash;
use crate::math::matrix::Matrix;
use crate::fe::fe_aux_structs::{compose_stiffness_submatrices_and_displacements, Displacement, Force, Stiffness, SubMatrixIndexes, AxisComponent};
use std::ops::{AddAssign, Add, Mul, MulAssign, Sub, Div, SubAssign};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use crate::math::math_aux_traits::{One, FloatNum};
use js_sys::Error;


enum DofErrorWith
{
    Force,
    Displacement,
}


#[derive(Debug, Clone)]
pub struct GlobalAnalysisResult<T, V>
{
    pub displacements: HashMap<Displacement<T>, V>,
    pub reactions: HashMap<Force<T>, V>,
}


#[derive(Debug, Clone)]
pub struct State<T, V>
{
    pub stiffness_matrix: Matrix<V>,
    pub displacements_indexes: HashMap<Displacement<T>, usize>,
    pub forces_indexes: HashMap<Force<T>, usize>,
}


pub struct FeModel<T, V, W>
    where T: Hash + Copy,
{
    pub nodes: Vec<FeNode<T, V>>,
    pub elements: Vec<Rc<RefCell<dyn FElement<T, V, W>>>>,
    pub applied_displacements: HashMap<Displacement<T>, W>,
    pub applied_forces: Option<HashMap<Force<T>, W>>,
    pub state: Option<State<T, V>>,
    pub global_analysis_result: Option<GlobalAnalysisResult<T, V>>,
}


impl<T, V, W> FeModel<T, V, W>
    where T: Hash + Copy + Eq + Debug,
          V: Default + AddAssign + Copy + One + Debug +
             PartialEq + Add + Mul + MulAssign +
             Add<Output = V> + Mul<Output = V> +
             From<W> + Sub + Sub<Output = V> + Div +
             Div<Output = V> + SubAssign + FloatNum,

          W: Default + Copy + Debug + One + Add +
             Sub + Mul + Div + AddAssign +
             MulAssign +
             Add<Output = W> + Sub<Output = W> +
             Mul<Output = W> + Div<Output = W>
{
    pub fn create(
        nodes: Vec<FeNode<T, V>>, elements: Vec<Rc<RefCell<dyn FElement<T, V, W>>>>,
        applied_displacements: HashMap<Displacement<T>, W>,
        applied_forces: Option<HashMap<Force<T>, W>>) -> FeModel<T, V, W>
    {
        FeModel { nodes, elements, applied_displacements, applied_forces, state: None, global_analysis_result: None }
    }


    fn _compose_global_stiffness_matrix_elements(&mut self,
        global_stiffness_submatrices_indexes: HashMap<Stiffness<T>, SubMatrixIndexes>)
        -> Result<Vec<Vec<V>>, String>
    {
        let mut global_stiffness_matrix_elements = Matrix::zeros(
        self.nodes.len() * NUMBER_OF_DOF as usize,
        self.nodes.len() * NUMBER_OF_DOF as usize
            ).elements;
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
        // println!("{:?}\n", global_stiffness_matrix_elements);
        Ok(global_stiffness_matrix_elements)
    }


    fn _return_dof_error(&self, reason: DofErrorWith, node_number: T, component: AxisComponent) -> Result<(), String>
    {
        let phrase = match reason
        {
            DofErrorWith::Displacement => "displacement",
            DofErrorWith::Force => "force"
        };
        Err(format!("There are no stiffness to withstand {}: {:?} applied at node: {:?}",
                phrase, component, node_number)
            )
    }


    fn _remove_zero_cells(&self, mut global_displacements_indexes: HashMap<Displacement<T>, usize>,
        mut global_forces_indexes: HashMap<Force<T>, usize>, mut global_stiffness_matrix_elements: Vec<Vec<V>>)
        -> Result<(HashMap<Displacement<T>, usize>, HashMap<Force<T>, usize>, Vec<Vec<V>>), String>
    {
        let mut i = 0;
        while i < global_stiffness_matrix_elements.len()
        {
            if global_stiffness_matrix_elements[i][i] == V::default()
            {
                global_stiffness_matrix_elements.remove(i);

                for (displacement, index) in global_displacements_indexes
                    .clone()
                    .into_iter()
                {
                    let node_number = displacement.node_number;
                    let component = displacement.component;
                    if i == index
                    {
                        if let Some(_) = self.applied_displacements.get(&Displacement { node_number, component })
                        {
                            self._return_dof_error(DofErrorWith::Displacement, node_number, component)?;
                        }
                        global_displacements_indexes.remove(&Displacement { node_number, component });
                        if let Some(applied_forces) = &self.applied_forces
                        {
                            if let Some(_) = applied_forces.get(&Force { node_number, component })
                            {
                                self._return_dof_error(DofErrorWith::Force, node_number, component)?;
                            }
                        }
                        global_forces_indexes.remove(&Force { node_number, component });

                    }
                    if i < index
                    {
                        global_displacements_indexes.insert(Displacement { node_number, component }, index - 1);
                        global_forces_indexes.insert(Force { node_number, component }, index - 1);
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
        Ok((global_displacements_indexes,
            global_forces_indexes,
            global_stiffness_matrix_elements))
    }


    pub fn update_fe_model_state(&mut self) -> Result<(), String>
    {
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
        let mut global_stiffness_matrix_elements =
            self._compose_global_stiffness_matrix_elements(global_stiffness_submatrices_indexes)?;
        let (
            updated_global_displacements_indexes,
            updated_global_forces_indexes,
            updated_global_stiffness_matrix_elements) =
            self._remove_zero_cells(
                global_displacements_indexes,
                global_forces_indexes,
                global_stiffness_matrix_elements)?;
        let model_state = State
            {
                stiffness_matrix: Matrix { elements: updated_global_stiffness_matrix_elements },
                displacements_indexes: updated_global_displacements_indexes,
                forces_indexes: updated_global_forces_indexes,
            };
        self.state = Some(model_state);
        Ok(())
    }


    pub fn calculate_reactions_and_displacements(&mut self) -> Result<(), String>
    {
        let mut result_displacements: HashMap<Displacement<T>, V> = HashMap::new();
        let mut result_reactions = HashMap::new();
        if let Some(state) = &self.state
        {
            if state.displacements_indexes.len() == self.applied_displacements.len()
            {
                return Err("All possible displacements were restrained, the structure cannot be analyzed.".to_string());
            }
            let mut k_aa_matrix: Matrix<V> = Matrix::zeros
                (
                    state.displacements_indexes.len() - self.applied_displacements.len(),
                    state.displacements_indexes.len() - self.applied_displacements.len(),
                );
            let mut k_ab_matrix: Matrix<V> = Matrix::zeros
                (
                    state.displacements_indexes.len() - self.applied_displacements.len(),
                    self.applied_displacements.len(),
                );
            let mut k_ba_matrix: Matrix<V> = Matrix::zeros
                (
                    self.applied_displacements.len(),
                    state.displacements_indexes.len() - self.applied_displacements.len(),
                );
            let mut k_bb_matrix: Matrix<V> = Matrix::zeros
                (
                    self.applied_displacements.len(),
                    self.applied_displacements.len(),
                );
            let mut k_aa_indexes = Vec::new();
            let mut k_bb_indexes = Vec::new();
            let mut r_a_indexes = HashMap::new();
            let mut r_a_elements = Vec::new();
            let mut displacements_indexes = HashMap::new();
            let mut u_b_indexes = HashMap::new();
            let mut u_b_elements = Vec::new();
            let mut reactions_indexes = HashMap::new();
            for (displacement, index) in &state.displacements_indexes
            {
                if let Some(disp) = self.applied_displacements.get(&displacement)
                {
                    k_bb_indexes.push(index);
                    result_displacements.insert(*displacement, V::from(*disp));
                    u_b_indexes.insert(index, displacement);
                }
                else
                {
                    k_aa_indexes.push(index);
                    let component = displacement.component;
                    let node_number = displacement.node_number;
                    let force = Force { component, node_number };
                    r_a_indexes.insert(index, force);
                }
            }
            k_aa_indexes.sort();
            let mut i = 0;
            while i < k_aa_indexes.len()
            {
                for j in i..k_aa_indexes.len()
                {
                    if i == j
                    {
                        k_aa_matrix.elements[i][i] =
                            state.stiffness_matrix.elements[*k_aa_indexes[i]][*k_aa_indexes[i]];
                        if let Some(force) = r_a_indexes.get(k_aa_indexes[i])
                        {
                            if let Some(applied_forces) = &self.applied_forces
                            {
                                if let Some(applied_force) = applied_forces.get(force)
                                {
                                    r_a_elements.push(vec![V::from(*applied_force)]);
                                }
                                else
                                {
                                    r_a_elements.push(vec![Default::default()]);
                                }
                            }
                            else
                            {
                                r_a_elements.push(vec![Default::default()]);
                            }
                            let component = force.component;
                            let node_number = force.node_number;
                            displacements_indexes.insert(i, Displacement { component, node_number });
                        }
                    }
                    else
                    {
                        k_aa_matrix.elements[i][j] =
                            state.stiffness_matrix.elements[*k_aa_indexes[i]][*k_aa_indexes[j]];
                        k_aa_matrix.elements[j][i] =
                            state.stiffness_matrix.elements[*k_aa_indexes[j]][*k_aa_indexes[i]];
                    }
                }
                i += 1;
            }
            let r_a_matrix = Matrix { elements: r_a_elements};
            k_bb_indexes.sort();
            let mut i = 0;
            while i < k_bb_indexes.len()
            {
                for j in i..k_bb_indexes.len()
                {
                    if i == j
                    {
                        k_bb_matrix.elements[i][i] =
                            state.stiffness_matrix.elements[*k_bb_indexes[i]][*k_bb_indexes[i]];
                        if let Some(disp) = u_b_indexes.get(k_bb_indexes[i])
                        {
                            let component = disp.component;
                            let node_number = disp.node_number;
                            reactions_indexes.insert(i, Force { component, node_number });
                            if let Some(applied_displacement) = self.applied_displacements.get(disp)
                            {
                                u_b_elements.push(vec![V::from(*applied_displacement)]);
                            }
                        }
                    }
                    else
                    {
                        k_bb_matrix.elements[i][j] =
                            state.stiffness_matrix.elements[*k_bb_indexes[i]][*k_bb_indexes[j]];
                        k_bb_matrix.elements[j][i] =
                            state.stiffness_matrix.elements[*k_bb_indexes[j]][*k_bb_indexes[i]];
                    }
                }
                i += 1;
            }
            let u_b_matrix = Matrix { elements: u_b_elements};
            for i in 0..k_aa_indexes.len()
            {
                for j in 0..k_bb_indexes.len()
                {
                    k_ab_matrix.elements[i][j] =
                        state.stiffness_matrix.elements[*k_aa_indexes[i]][*k_bb_indexes[j]];
                }
            }
            for i in 0..k_bb_indexes.len()
            {
                for j in 0..k_aa_indexes.len()
                {
                    k_ba_matrix.elements[i][j] =
                        state.stiffness_matrix.elements[*k_bb_indexes[i]][*k_aa_indexes[j]];
                }
            }
            println!("k_aa indexes: {:?}", k_aa_indexes);
            println!("k_aa matrix: {:?}", k_aa_matrix);
            println!("k_bb indexes: {:?}", k_bb_indexes);
            println!("k_bb matrix: {:?}", k_bb_matrix);
            println!("k_ab matrix: {:?}", k_ab_matrix);
            println!("k_ba matrix: {:?}", k_ba_matrix);
            println!("r_a indexes: {:?}", r_a_indexes);
            println!("r_a matrix: {:?}", r_a_matrix);
            println!("u_b indexes: {:?}", u_b_indexes);
            println!("u_b matrix: {:?}\n", u_b_matrix);


            let k_ab_u_b = k_ab_matrix.multiply_by_matrix(&u_b_matrix)?;
            let minus_k_ab = k_ab_u_b.multiply_by_number(One::minus_one());
            let r_a_minus_k_ab_u_b = r_a_matrix.sum(&minus_k_ab)?;
            let displacements = k_aa_matrix.solve_equations::<V, V>(r_a_minus_k_ab_u_b)?;
            for i in 0..displacements.elements.len()
            {
                if let Some(displacement) = displacements_indexes.get(&i)
                {
                    let node_number = displacement.node_number;
                    let component = displacement.component;
                    let displacement_value = displacements.elements[i][0];
                    if !displacement_value.is_nan()
                    {
                        result_displacements
                            .insert(Displacement { node_number, component }, displacement_value);
                    }
                    else
                    {
                        return Err("The NaN value of displacement was defined.".to_string());
                    }
                }
            }
            let k_bb_u_b = k_bb_matrix.multiply_by_matrix(&u_b_matrix)?;
            let k_ba_u_a = k_ba_matrix.multiply_by_matrix(&displacements)?;
            let reactions = k_ba_u_a.sum(&k_bb_u_b)?;
            for i in 0..reactions.elements.len()
            {
                if let Some(reaction) = reactions_indexes.get(&i)
                {
                    let reaction_value = reactions.elements[i][0];
                    if !reaction_value.is_nan()
                    {
                        result_reactions.insert(*reaction, reaction_value);
                    }
                    else
                    {
                        return Err("The NaN value of reaction was defined.".to_string());
                    }
                }
            }
            self.global_analysis_result = Some
                (
                    GlobalAnalysisResult { displacements: result_displacements, reactions: result_reactions }
                );
            Ok(())
        }
        else
        {
            return Err("Global stiffness matrix not prepared yet, the structure cannot be analyzed!".to_string())
        }
    }
}
