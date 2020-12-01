use crate::fe::node::Node;
use crate::math::math_aux_traits::{One, FloatNum};
use crate::math::matrix::Matrix;
use crate::math::vector::{Vector, GlobalCoordinateAxis, GlobalCoordinatePlane};
use crate::NUMBER_OF_DOF;
use crate::fe::fe_aux_structs::
    {
        SubMatrixIndexes, compose_stiffness_submatrices_and_displacements,
        Stiffness, Displacement, StrainStressComponent, StrainStress, Strain, Stress
    };
use crate::fe::elements::element::{Element};
use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign};
use std::fmt::{Display, Debug};
use std::collections::HashMap;
use std::hash::Hash;


#[derive(Debug)]
struct IntegrationPoint<V>
{
    sampling_point: V,
    weight: V,
}


#[derive(Debug)]
struct State<T, V>
{
    jacobian: Option<V>,
    strain_displacement_matrix: Option<Matrix<V>>,
    strain_stress_indexes: Option<HashMap<usize, StrainStressComponent>>,
    integration_points: Vec<IntegrationPoint<V>>,
    rotation_matrix: Option<Matrix<V>>,
    displacements_indexes: HashMap<Displacement<T>, usize>,
    stiffness_submatrices_indexes: HashMap<Stiffness<T>, SubMatrixIndexes>,
}


#[derive(Debug)]
pub struct Truss2n2ip<T, V, W>
    where T: Hash + Eq + Copy
{
    pub number: T,
    pub node_1: Node<T, V>,
    pub node_2: Node<T, V>,
    pub young_modulus: W,
    pub area: W,
    pub area_2: Option<W>,
    state: State<T, V>,
}


impl<T, V, W> Truss2n2ip<T, V, W>
    where T: Display + Hash + Eq + Copy,
          V: FloatNum + Copy + One + Default + From<f64> +
             Add<Output = V> + Sub<Output = V> +
             Mul<Output = V> + Div<Output = V> +
             AddAssign + MulAssign + Debug,
          W: Copy + Mul<Output = W> + Into<V>
{
    pub fn create(
        number: T, node_1: Node<T, V>, node_2: Node<T, V>,
        young_modulus: W, area: W, area_2: Option<W>)
        -> Truss2n2ip<T, V, W>
    {
        let integration_point_1 = IntegrationPoint
            {
                sampling_point: V::from(-1f64 / 3f64.sqrt()),
                weight: One::one(),
            };
        let integration_point_2 = IntegrationPoint
            {
                sampling_point: V::from(1f64 / 3f64.sqrt()),
                weight: One::one(),
            };

        let nodes = vec![&node_1, &node_2];
        let (displacements_indexes, _, stiffness_submatrices_indexes) =
            compose_stiffness_submatrices_and_displacements
                (
                    NUMBER_OF_DOF as usize, nodes
                );
        Truss2n2ip
            {
                number, node_1, node_2, young_modulus, area, area_2,
                state: State
                    {
                        jacobian: None, strain_displacement_matrix: None,
                        strain_stress_indexes: None,
                        integration_points: vec![integration_point_1, integration_point_2],
                        rotation_matrix: None,
                        displacements_indexes,
                        stiffness_submatrices_indexes
                    },
            }
    }


    fn _calculate_jacobian(&mut self)
    {
        let element_length =
            ((self.node_2.coordinates.x - self.node_1.coordinates.x) *
            (self.node_2.coordinates.x - self.node_1.coordinates.x) +
            (self.node_2.coordinates.y - self.node_1.coordinates.y) *
            (self.node_2.coordinates.y - self.node_1.coordinates.y) +
            (self.node_2.coordinates.z - self.node_1.coordinates.z) *
            (self.node_2.coordinates.z - self.node_1.coordinates.z)).sqrt();
        self.state.jacobian = Some(element_length / V::from(2f64));
    }


    fn _calculate_inverse_jacobian(&mut self) -> V
    {
        if let None = self.state.jacobian
        {
            self._calculate_jacobian();
        }
        let one: V = One::one();
        one / self.state.jacobian.unwrap()
    }


    fn _calculate_determinant_of_jacobian(&mut self) -> V
    {
        if let None = self.state.jacobian
        {
            self._calculate_jacobian();
        }
        let determinant_of_jacobian = self.state.jacobian.unwrap();
        determinant_of_jacobian
    }


    fn _compose_strain_displacement_matrix(&mut self)
    {
        let inverse_jacobian = self._calculate_inverse_jacobian();
        let dh_dr_elements = vec!
            [
                vec!
                    [
                        V::from(-0.5f64), Default::default(), Default::default(),
                        Default::default(), Default::default(), Default::default(),
                        V::from(0.5f64), Default::default(), Default::default(),
                        Default::default(), Default::default(), Default::default(),
                    ],
            ];
        let dh_dr_matrix = Matrix { elements: dh_dr_elements };
        let mut strain_stress_indexes = HashMap::new();
        for i in 0..dh_dr_matrix.elements.len()
        {
            if i == 0
            {
                strain_stress_indexes.insert(i, StrainStressComponent::XX);
            }
        }
        self.state.strain_displacement_matrix = Some(
            dh_dr_matrix.multiply_by_number(inverse_jacobian));
        self.state.strain_stress_indexes = Some(strain_stress_indexes);
    }


    fn _compose_rotation_matrix(&mut self)
    {
        let element_length =
            ((self.node_2.coordinates.x - self.node_1.coordinates.x) *
            (self.node_2.coordinates.x - self.node_1.coordinates.x) +
            (self.node_2.coordinates.y - self.node_1.coordinates.y) *
            (self.node_2.coordinates.y - self.node_1.coordinates.y) +
            (self.node_2.coordinates.z - self.node_1.coordinates.z) *
            (self.node_2.coordinates.z - self.node_1.coordinates.z)).sqrt();
        let c_x = (self.node_2.coordinates.x - self.node_1.coordinates.x) / element_length;
        let c_y = (self.node_2.coordinates.y - self.node_1.coordinates.y) / element_length;
        let c_z = (self.node_2.coordinates.z - self.node_1.coordinates.z) / element_length;
        let elements = vec!
            [
                vec![c_x, c_y, c_z],
                vec![Default::default(), One::one(), Default::default()],
                vec![Default::default(), Default::default(), One::one()],
            ];
        let m = Matrix { elements };

        let mut rotation_matrix_elements = Vec::new();
        for i in 0..(NUMBER_OF_DOF * 2) as usize
        {
            let mut current_row = Vec::new();
            for j in 0..(NUMBER_OF_DOF * 2) as usize
            {
                if i < NUMBER_OF_DOF as usize
                {
                    if let Some(row) = m.elements.get(i)
                    {
                        if let Some(element) = row.get(j)
                        {
                            let current_element =
                                {
                                    if element.is_nan()
                                    {
                                        if i == j
                                        {
                                            One::one()
                                        }
                                        else
                                        {
                                            Default::default()
                                        }
                                    }
                                    else
                                    {
                                        *element
                                    }
                                };
                            current_row.push(current_element);
                        }
                        else
                        {
                            current_row.push(Default::default());
                        }
                    }
                    else
                    {
                        if i == j
                        {
                            current_row.push(One::one());
                        }
                        else
                        {
                            current_row.push(Default::default());
                        }
                    }
                }
                else
                {
                    if let Some(row) = m.elements.get(i - NUMBER_OF_DOF as usize)
                    {
                        if j < NUMBER_OF_DOF as usize
                        {
                            current_row.push(Default::default());
                        }
                        else if let Some(element) = row.get(j - NUMBER_OF_DOF as usize)
                        {
                            let current_element =
                                {
                                    if element.is_nan()
                                    {
                                        if i == j
                                        {
                                           One::one()
                                        }
                                        else
                                        {
                                            Default::default()
                                        }
                                    }
                                    else
                                    {
                                        *element
                                    }
                                };
                            current_row.push(current_element);
                        }
                        else
                        {
                            current_row.push(Default::default());
                        }
                    }
                    else
                    {
                        if i == j
                        {
                            current_row.push(One::one());
                        }
                        else
                        {
                            current_row.push(Default::default());
                        }
                    }
                }
            }
            rotation_matrix_elements.push(current_row);
        }
        let rotation_matrix = Matrix { elements: rotation_matrix_elements };
        self.state.rotation_matrix = Some(rotation_matrix);
    }


    fn _area_numerical_integration(&self) -> V
    {
        let mut integrated_area = Default::default();
        for integration_point in &self.state.integration_points
        {
            if let Some(area_2) = self.area_2
            {
                let one: V = One::one();
                integrated_area += integration_point.weight *
                    (self.area.into().sqrt() +
                    ((area_2.into().sqrt() - self.area.into().sqrt()) / V::from(2f64)) *
                    (one + integration_point.sampling_point)) *
                    (self.area.into().sqrt() +
                    ((area_2.into().sqrt() - self.area.into().sqrt()) / V::from(2f64)) *
                    (one + integration_point.sampling_point));
            }
            else
            {
                integrated_area += integration_point.weight * self.area.into();
            }
        }
        integrated_area
    }


    fn _compose_local_stiffness_matrix(&mut self) -> Result<Matrix<V>, String>
    {
        if let None = self.state.strain_displacement_matrix
        {
            self._compose_strain_displacement_matrix();
        }

        if let Ok(m) = self.state.strain_displacement_matrix.as_ref().unwrap()
            .transpose()
            .multiply_by_matrix(&self.state.strain_displacement_matrix.as_ref().unwrap())
        {
            let determinant_of_jacobian = self._calculate_determinant_of_jacobian();
            let integrated_area = self._area_numerical_integration();
            let local_stiffness_matrix = m.multiply_by_number
                (
                    integrated_area * determinant_of_jacobian * self.young_modulus.into())
                ;
            Ok(local_stiffness_matrix)
        }
        else
        {
            Err(format!("cannot compose local stiffness matrix for element {}!", self.number))
        }
    }
}


impl<T, V, W> Element<T, V, W> for Truss2n2ip<T, V, W>
    where T: Display + Hash + Eq + Copy + Debug,
          V: FloatNum + Copy + One + Default + From<f64> +
             Add<Output = V> + Sub<Output = V> +
             Mul<Output = V> + Div<Output = V> +
             AddAssign + MulAssign + Debug,
          W: Copy + Mul<Output = W> + Into<V>

{
    fn extract_stiffness_submatrices(&self) -> HashMap<Stiffness<T>, SubMatrixIndexes>
    {
        self.state.stiffness_submatrices_indexes.to_owned()
    }


    fn extract_stiffness_matrix(&mut self) -> Result<Matrix<V>, String>
    {
        if let None = self.state.rotation_matrix
        {
            self._compose_rotation_matrix();
        }
        let local_stiffness_matrix = self._compose_local_stiffness_matrix()?;
        let converted_stiffness_matrix =
            self.state.rotation_matrix.as_ref().unwrap()
                .transpose()
                .multiply_by_matrix(&local_stiffness_matrix)?
                .multiply_by_matrix(self.state.rotation_matrix.as_ref().unwrap())?;
        Ok(converted_stiffness_matrix)
    }


    fn calculate_strains_and_stresses(&mut self, global_displacements: &HashMap<Displacement<T>, V>)
        -> Result<HashMap<T, Vec<StrainStress<V>>>, String>
    {
        if let None = self.state.rotation_matrix
        {
            self._compose_rotation_matrix();
        }
        let mut displacements = Matrix::zeros(self.state.displacements_indexes.len(), 1);
        for (displacement, index) in &self.state.displacements_indexes
        {
            if let Some(global_displacement) = global_displacements.get(&displacement)
            {
                displacements.elements[*index][0] = *global_displacement;
            }
            else
            {
                displacements.elements[*index][0] = Default::default();
            }
        }
        let local_displacements = self.state.rotation_matrix
            .as_ref()
            .unwrap()
            .multiply_by_matrix(&displacements)?;
        if let None = self.state.strain_displacement_matrix
        {
            self._compose_strain_displacement_matrix();
        }
        let mut strains_and_stresses = Vec::new();
        let strains = self.state.strain_displacement_matrix
            .as_ref()
            .unwrap()
            .multiply_by_matrix(&local_displacements)?;
        let stresses = strains.multiply_by_number(self.young_modulus.into());
        for i in 0..strains.elements.len()
        {
            if let Some(strain_stress_component) =
            self.state.strain_stress_indexes
                .as_ref()
                .unwrap()
                .get(&i)
            {
                let current_strain = Strain
                    {
                        component: *strain_stress_component,
                        value: strains.elements[i][0]
                    };
                let current_stress = Stress
                    {
                        component: *strain_stress_component,
                        value: stresses.elements[i][0]
                    };
                let current_strain_stress = StrainStress
                    {
                        strain: current_strain,
                        stress: current_stress
                    };
                strains_and_stresses.push(current_strain_stress);
            }
        }
        let mut strains_and_stresses_data = HashMap::new();
        strains_and_stresses_data.insert(self.number, strains_and_stresses);
        Ok(strains_and_stresses_data)
    }
}
