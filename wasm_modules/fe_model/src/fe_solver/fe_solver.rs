use std::ops::{Rem, Div, AddAssign, Add, Sub, Mul, SubAssign, DivAssign, MulAssign};
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_json::json;

use finite_element_method::my_float::MyFloatTrait;
use finite_element_method::fem::fe_model::FEModel;
use finite_element_method::fem::finite_elements::fe_node::DeletedFENodeData;
use finite_element_method::fem::finite_elements::finite_element::{DeletedFEData, FEType};
use finite_element_method::fem::global_analysis::fe_boundary_condition::DeletedBCData;


use crate::preprocessor::preprocessor::Preprocessor;
use crate::preprocessor::properties::property::CrossSectionType;

use crate::postprocessor::analysis_result::AnalysisResult;

use crate::fe_solver::consts::ELEMENTS_AT_LINE;

use crate::functions::log;


pub struct FESolver<T, V>
{
    fem: FEModel<T, V>,
}


impl<T, V> FESolver<T, V>
    where T: Copy + Debug + Add<Output = T> + Rem<Output = T> + Div<Output = T> + AddAssign +
             From<u8> + Sub<Output = T> + Mul<Output = T> + PartialOrd + SubAssign + Hash + Eq +
             Serialize + 'static,
          V: Copy + Debug + From<f32> + Into<f64> + DivAssign + MyFloatTrait + PartialOrd +
             SubAssign + MulAssign + AddAssign + Div<Output = V> + Add<Output = V> +
             Mul<Output = V> + Sub<Output = V> + MyFloatTrait<Other = V> + Serialize + 'static,
{
    pub fn create(tolerance: V) -> Self
    {
        let fem = FEModel::create(tolerance);
        FESolver { fem }
    }


    pub fn submit_job(&mut self, preprocessor: &Preprocessor<T, V>)
        -> Result<AnalysisResult<T, V>, JsValue>
    {
        let beam_elements_number_at_line = 10u8;

        self.fem.reset();

        for (property_name, assigned_property_to_lines) in
            preprocessor.properties.assigned_properties_to_lines.iter()
        {
            let (material_name, cross_section_name, cross_section_type) =
                preprocessor.properties.properties.get(property_name).unwrap().clone_data();

            for (line_number, local_axis_1_direction) in
                assigned_property_to_lines.copy_related_lines_numbers().iter().zip(
                    assigned_property_to_lines.copy_related_lines_local_axis_1_directions())
            {
                if cross_section_type == CrossSectionType::Beam && local_axis_1_direction.is_none()
                {
                    let error_message = &format!("FESolver: Submit job action: The local \
                        axis 1 direction for line {:?} with section {:?} was not assigned!",
                        line_number, cross_section_name);
                    return Err(JsValue::from(error_message));
                }

                let (start_point_number, end_point_number) =
                    preprocessor.geometry.lines.get(line_number).unwrap().copy_points_numbers();

                let (x1, y1, z1) = preprocessor.geometry.points
                    .get(&start_point_number).unwrap().copy_coordinates();
                if !self.fem.is_node_number_exist(start_point_number)
                {
                    self.fem.add_node(start_point_number, x1, y1, z1)
                        .map_err(|e| JsValue::from(e))?;
                }

                let (x2, y2, z2) = preprocessor.geometry.points
                    .get(&end_point_number).unwrap().copy_coordinates();
                if !self.fem.is_node_number_exist(end_point_number)
                {
                    self.fem.add_node(end_point_number, x2, y2, z2)
                        .map_err(|e| JsValue::from(e))?;
                }

                let (young_modulus, poisson_ratio) = preprocessor.properties.materials
                    .get(material_name).unwrap().copy_data();

                match cross_section_type
                {
                    CrossSectionType::Truss =>
                        {
                            let (area, area2) = preprocessor.properties.truss_sections
                                .get(cross_section_name).unwrap().copy_data();
                            let truss_element_properties =
                                {
                                    if let Some(area_2) = area2
                                    {
                                        vec![young_modulus, area, area_2]
                                    }
                                    else
                                    {
                                        vec![young_modulus, area]
                                    }
                                };
                            self.fem.add_element(*line_number,
                                FEType::Truss2n1ip,
                                vec![start_point_number, end_point_number],
                                truss_element_properties)?;
                        },
                    CrossSectionType::Beam =>
                        {
                            let (area, i11, i22, i12, it, shear_factor) =
                                preprocessor.properties.beam_sections.get(cross_section_name)
                                    .unwrap().copy_data();
                            let loc_axis_1_direction =
                                local_axis_1_direction.unwrap().copy_direction();
                            let beam_element_properties = vec![young_modulus, poisson_ratio,
                                area, i11, i22, i12, it, shear_factor, loc_axis_1_direction[0],
                                loc_axis_1_direction[1], loc_axis_1_direction[2]];

                            let step_x =
                                (x2 - x1) / V::from(beam_elements_number_at_line as f32);
                            let step_y =
                                (y2 - y1) / V::from(beam_elements_number_at_line as f32);
                            let step_z =
                                (z2 - z1) / V::from(beam_elements_number_at_line as f32);
                            let mut number =
                                (*line_number + T::from(100u8) * T::from(100u8)) * T::from(100u8);
                            let mut i = 1u8;
                            while i < beam_elements_number_at_line
                            {
                                let x = x1 + step_x * V::from(i as f32);
                                let y = y1 + step_y * V::from(i as f32);
                                let z = z1 + step_z * V::from(i as f32);
                                number += T::from(1u8);
                                self.fem.add_node(number, x, y, z)?;
                                let nodes_numbers =
                                    {
                                        if i == 1u8
                                        {
                                            vec![start_point_number, number]
                                        }
                                        else
                                        {
                                            vec![number, number - T::from(1u8)]
                                        }
                                    };
                                self.fem.add_element(number,
                                    FEType::Beam2n1ipT, nodes_numbers,
                                    beam_element_properties.clone())?;
                                i += 1u8;
                            }
                            self.fem.add_element(number + T::from(1u8),
                                FEType::Beam2n1ipT,
                                vec![number, end_point_number],
                                beam_element_properties)?;
                        },
                }

            }
        }

        let nodes_numbers = self.fem.extract_all_nodes_numbers();
        log(&format!("Nodes numbers: {:?}", nodes_numbers));
        for node_number in nodes_numbers.iter()
        {
            log(&format!("Node number: {:?}, coordinates: {:?}", node_number, self.fem.copy_node_coordinates(node_number)));
        }

        let elements_numbers = self.fem.extract_all_elements_numbers();
        log(&format!("Elements numbers: {:?}", elements_numbers));
        for element_number in elements_numbers.iter()
        {
            log(&format!("Element number: {:?}, nodes: {:?}", element_number,
                self.fem.copy_element_nodes_numbers(element_number)));
        }


        Err(JsValue::from("Error!!!"))
    }
}

