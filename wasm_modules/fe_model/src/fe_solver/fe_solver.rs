use std::ops::{Rem, Div, AddAssign, Add, Sub, Mul, SubAssign, DivAssign, MulAssign};
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_json::json;

use finite_element_method::my_float::MyFloatTrait;
use finite_element_method::fem::fe_model::FEModel;
use finite_element_method::fem::finite_elements::finite_element::FEType;
use finite_element_method::fem::global_analysis::fe_boundary_condition::BCType;
use finite_element_method::fem::global_analysis::fe_dof_parameter_data::GlobalDOFParameter;


use crate::preprocessor::preprocessor::Preprocessor;
use crate::preprocessor::properties::property::CrossSectionType;

use crate::preprocessor::functions::compare_with_tolerance;

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
             Serialize + MulAssign + 'static,
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

        let mut divider = 10u8;
        let mut number_of_digits = 1u8;
        while beam_elements_number_at_line / divider > 0
        {
            divider *= 10u8;
            number_of_digits += 1u8;
        }

        let mut nodes_coordinates = HashMap::new();
        let mut elements_nodes_numbers = HashMap::new();
        let mut elements_rotation_matrices_data = HashMap::new();

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

                    if let Some(concentrated_load) =
                        preprocessor.loads.concentrated_loads.get(&start_point_number)
                    {
                        let (fx, fy, fz) = concentrated_load.copy_load_components();
                        if fx != V::from(0f32)
                        {
                            let force_number = start_point_number * T::from(6u8) - T::from(5u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                start_point_number, GlobalDOFParameter::X,
                                fx)?;
                        }
                        if fy != V::from(0f32)
                        {
                            let force_number = start_point_number * T::from(6u8) - T::from(4u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                start_point_number, GlobalDOFParameter::Y,
                                fy)?;
                        }
                        if fz != V::from(0f32)
                        {
                            let force_number = start_point_number * T::from(6u8) - T::from(3u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                start_point_number, GlobalDOFParameter::Z,
                                fz)?;
                        }
                        let (mx, my, mz) = concentrated_load.copy_moment_components();
                        if mx != V::from(0f32)
                        {
                            let force_number = start_point_number * T::from(6u8) - T::from(2u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                start_point_number, GlobalDOFParameter::ThX,
                                mx)?;
                        }
                        if my != V::from(0f32)
                        {
                            let force_number = start_point_number * T::from(6u8) - T::from(1u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                start_point_number, GlobalDOFParameter::ThY,
                                my)?;
                        }
                        if mz != V::from(0f32)
                        {
                            let force_number = start_point_number * T::from(6u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                start_point_number, GlobalDOFParameter::ThZ,
                                mz)?;
                        }
                    }

                    if let Some(boundary_condition) =
                        preprocessor.boundary_conditions.boundary_conditions
                            .get(&start_point_number)
                    {
                        let (optional_ux, optional_uy, optional_uz) =
                            boundary_condition.copy_optional_displacement_components();
                        if let Some(ux) = optional_ux
                        {
                            let displacement_number =
                                start_point_number * T::from(6u8) - T::from(5u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                start_point_number, GlobalDOFParameter::X,
                                ux)?;
                        }
                        if let Some(uy) = optional_uy
                        {
                            let displacement_number =
                                start_point_number * T::from(6u8) - T::from(4u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                start_point_number, GlobalDOFParameter::Y,
                                uy)?;
                        }
                        if let Some(uz) = optional_uz
                        {
                            let displacement_number =
                                start_point_number * T::from(6u8) - T::from(3u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                start_point_number, GlobalDOFParameter::Z,
                                uz)?;
                        }
                        let (optional_rx, optional_ry, optional_rz) =
                            boundary_condition.copy_optional_rotation_components();
                        if let Some(rx) = optional_rx
                        {
                            let displacement_number =
                                start_point_number * T::from(6u8) - T::from(2u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                start_point_number, GlobalDOFParameter::ThX,
                                rx)?;
                        }
                        if let Some(ry) = optional_ry
                        {
                            let displacement_number =
                                start_point_number * T::from(6u8) - T::from(1u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                start_point_number, GlobalDOFParameter::ThY,
                                ry)?;
                        }
                        if let Some(rz) = optional_rz
                        {
                            let displacement_number =  start_point_number * T::from(6u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                start_point_number, GlobalDOFParameter::ThZ,
                                rz)?;
                        }
                    }

                    nodes_coordinates.insert(start_point_number, (x1, y1, z1));
                }

                let (x2, y2, z2) = preprocessor.geometry.points
                    .get(&end_point_number).unwrap().copy_coordinates();
                if !self.fem.is_node_number_exist(end_point_number)
                {
                    self.fem.add_node(end_point_number, x2, y2, z2)
                        .map_err(|e| JsValue::from(e))?;

                    if let Some(concentrated_load) =
                        preprocessor.loads.concentrated_loads.get(&end_point_number)
                    {
                        let (fx, fy, fz) = concentrated_load.copy_load_components();
                        if fx != V::from(0f32)
                        {
                            let force_number = end_point_number * T::from(6u8) - T::from(5u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                end_point_number, GlobalDOFParameter::X,
                                fx)?;
                        }
                        if fy != V::from(0f32)
                        {
                            let force_number = end_point_number * T::from(6u8) - T::from(4u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                end_point_number, GlobalDOFParameter::Y,
                                fy)?;
                        }
                        if fz != V::from(0f32)
                        {
                            let force_number = end_point_number * T::from(6u8) - T::from(3u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                end_point_number, GlobalDOFParameter::Z,
                                fz)?;
                        }
                        let (mx, my, mz) = concentrated_load.copy_moment_components();
                        if mx != V::from(0f32)
                        {
                            let force_number = end_point_number * T::from(6u8) - T::from(2u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                end_point_number, GlobalDOFParameter::ThX,
                                mx)?;
                        }
                        if my != V::from(0f32)
                        {
                            let force_number = end_point_number * T::from(6u8) - T::from(1u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                end_point_number, GlobalDOFParameter::ThY,
                                my)?;
                        }
                        if mz != V::from(0f32)
                        {
                            let force_number = end_point_number * T::from(6u8);
                            self.fem.add_bc(BCType::Force, force_number,
                                end_point_number, GlobalDOFParameter::ThZ,
                                mz)?;
                        }
                    }

                    if let Some(boundary_condition) =
                        preprocessor.boundary_conditions.boundary_conditions
                            .get(&end_point_number)
                    {
                        let (optional_ux, optional_uy, optional_uz) =
                            boundary_condition.copy_optional_displacement_components();
                        if let Some(ux) = optional_ux
                        {
                            let displacement_number =
                                end_point_number * T::from(6u8) - T::from(5u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                end_point_number, GlobalDOFParameter::X,
                                ux)?;
                        }
                        if let Some(uy) = optional_uy
                        {
                            let displacement_number =
                                end_point_number * T::from(6u8) - T::from(4u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                end_point_number, GlobalDOFParameter::Y,
                                uy)?;
                        }
                        if let Some(uz) = optional_uz
                        {
                            let displacement_number =
                                end_point_number * T::from(6u8) - T::from(3u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                end_point_number, GlobalDOFParameter::Z,
                                uz)?;
                        }
                        let (optional_rx, optional_ry, optional_rz) =
                            boundary_condition.copy_optional_rotation_components();
                        if let Some(rx) = optional_rx
                        {
                            let displacement_number =
                                end_point_number * T::from(6u8) - T::from(2u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                end_point_number, GlobalDOFParameter::ThX,
                                rx)?;
                        }
                        if let Some(ry) = optional_ry
                        {
                            let displacement_number =
                                end_point_number * T::from(6u8) - T::from(1u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                end_point_number, GlobalDOFParameter::ThY,
                                ry)?;
                        }
                        if let Some(rz) = optional_rz
                        {
                            let displacement_number =  end_point_number * T::from(6u8);
                            self.fem.add_bc(BCType::Displacement, displacement_number,
                                end_point_number, GlobalDOFParameter::ThZ,
                                rz)?;
                        }
                    }

                    nodes_coordinates.insert(end_point_number, (x2, y2, z2));
                }

                let (young_modulus, poisson_ratio) = preprocessor.properties.materials
                    .get(material_name).unwrap().copy_data();

                match cross_section_type
                {
                    CrossSectionType::Truss =>
                        {
                            let (area, area2) = preprocessor.properties.truss_sections
                                .get(cross_section_name).unwrap().copy_data();

                            if preprocessor.loads.distributed_line_loads
                                .contains_key(line_number)
                            {
                                let error_message = &format!("FESolver: Submit job action: \
                                    Distributed load could not be applied to line {:?} with \
                                    assigned truss section {:?}!", line_number, cross_section_name);
                                return Err(JsValue::from(error_message));
                            }

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
                            self.fem.add_element(line_number.clone(),
                                FEType::Truss2n1ip,
                                vec![start_point_number, end_point_number],
                                truss_element_properties)?;
                            elements_nodes_numbers.insert(line_number.clone(),
                                vec![start_point_number, end_point_number]);
                            let unique_elements_of_rotation_matrix =
                                self.fem.extract_unique_elements_of_rotation_matrix(
                                    line_number)?;
                            elements_rotation_matrices_data.insert(line_number.clone(),
                                unique_elements_of_rotation_matrix);
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

                            let line_projection_x_length = compare_with_tolerance(
                                (x2 - x1).my_abs(), preprocessor.tolerance);
                            let line_projection_y_length = compare_with_tolerance(
                                (y2 - y1).my_abs(), preprocessor.tolerance);
                            let line_projection_z_length = compare_with_tolerance(
                                (z2 - z1).my_abs(), preprocessor.tolerance);

                            let optional_distributed_load =
                                if let Some(distributed_load) =
                                    preprocessor.loads.distributed_line_loads.get(line_number)
                                {
                                    let (qx, qy, qz) =
                                        distributed_load.copy_load_components();

                                    let qx_interim =
                                        {
                                            if line_projection_y_length == V::from(0f32) &&
                                                line_projection_z_length == V::from(0f32) &&
                                                line_projection_x_length != V::from(0f32)
                                            {
                                                qx * line_projection_x_length /
                                                    V::from(beam_elements_number_at_line as f32)
                                            }
                                            else
                                            {
                                                ((qx * line_projection_y_length).my_powi(2) +
                                                (qx * line_projection_z_length).my_powi(2))
                                                    .my_sqrt() /
                                                    V::from(beam_elements_number_at_line as f32)
                                            }
                                        };
                                    let qx_end = qx_interim / V::from(2f32);
                                    let qy_interim =
                                        {
                                            if line_projection_x_length == V::from(0f32) &&
                                                line_projection_z_length == V::from(0f32) &&
                                                line_projection_y_length != V::from(0f32)
                                            {
                                                qy * line_projection_y_length /
                                                    V::from(beam_elements_number_at_line as f32)
                                            }
                                            else
                                            {
                                                ((qy * line_projection_x_length).my_powi(2) +
                                                (qy * line_projection_z_length).my_powi(2))
                                                    .my_sqrt() /
                                                    V::from(beam_elements_number_at_line as f32)
                                            }
                                        };
                                    let qy_end= qy_interim / V::from(2f32);
                                    let qz_interim =
                                        {
                                            if line_projection_x_length == V::from(0f32) &&
                                                line_projection_y_length == V::from(0f32) &&
                                                line_projection_z_length != V::from(0f32)
                                            {
                                                qz * line_projection_z_length /
                                                    V::from(beam_elements_number_at_line as f32)
                                            }
                                            else
                                            {
                                                ((qz * line_projection_x_length).my_powi(2) +
                                                (qz * line_projection_y_length).my_powi(2))
                                                    .my_sqrt() /
                                                    V::from(beam_elements_number_at_line as f32)
                                            }
                                        };
                                    let qz_end = qz_interim / V::from(2f32);
                                    Some((qx_end, qx_interim, qy_end, qy_interim, qz_end,
                                        qz_interim))
                                }
                                else
                                {
                                    None
                                };

                            if let Some((qx_end, _qx_interim, qy_end, _qy_interim,
                                qz_end, _qz_interim)) = optional_distributed_load
                            {
                                let start_point_qx_load_number =
                                    start_point_number * T::from(6u8) - T::from(5u8);
                                let start_point_qy_load_number =
                                    start_point_number * T::from(6u8) - T::from(4u8);
                                let start_point_qz_load_number =
                                    start_point_number * T::from(6u8) - T::from(3u8);
                                let end_point_qx_load_number =
                                    end_point_number * T::from(6u8) - T::from(5u8);
                                let end_point_qy_load_number =
                                    end_point_number * T::from(6u8) - T::from(4u8);
                                let end_point_qz_load_number =
                                    end_point_number * T::from(6u8) - T::from(3u8);

                                if qx_end != V::from(0f32)
                                {
                                    if self.fem.is_bc_key_exist(start_point_qx_load_number,
                                        BCType::Force)
                                    {
                                        let existed_fx_force_value =
                                            self.fem.copy_bc_value(BCType::Force,
                                                start_point_qx_load_number)
                                                .map_err(|e| JsValue::from(e))?;
                                        let updated_fx_force_value = existed_fx_force_value +
                                            qx_end;
                                        self.fem.update_bc(BCType::Force,
                                            start_point_qx_load_number,
                                            start_point_number,
                                            GlobalDOFParameter::X,
                                            updated_fx_force_value)?;
                                    }
                                    else
                                    {
                                        self.fem.add_bc(BCType::Force,
                                            start_point_qx_load_number,
                                            start_point_number,
                                            GlobalDOFParameter::X,
                                            qx_end)?;
                                    }

                                    if self.fem.is_bc_key_exist(end_point_qx_load_number,
                                        BCType::Force)
                                    {
                                        let existed_fx_force_value =
                                            self.fem.copy_bc_value(BCType::Force,
                                                end_point_qx_load_number)
                                                .map_err(|e| JsValue::from(e))?;
                                        let updated_fx_force_value = existed_fx_force_value +
                                            qx_end;
                                        self.fem.update_bc(BCType::Force,
                                            end_point_qx_load_number,
                                            end_point_number,
                                            GlobalDOFParameter::X,
                                            updated_fx_force_value)?;
                                    }
                                    else
                                    {
                                        self.fem.add_bc(BCType::Force,
                                            end_point_qx_load_number,
                                            end_point_number,
                                            GlobalDOFParameter::X,
                                            qx_end)?;
                                    }
                                }

                                if qy_end != V::from(0f32)
                                {
                                    if self.fem.is_bc_key_exist(start_point_qy_load_number,
                                        BCType::Force)
                                    {
                                        let existed_fy_force_value =
                                            self.fem.copy_bc_value(BCType::Force,
                                                start_point_qy_load_number)
                                                .map_err(|e| JsValue::from(e))?;
                                        let updated_fy_force_value = existed_fy_force_value +
                                            qy_end;
                                        self.fem.update_bc(BCType::Force,
                                            start_point_qy_load_number,
                                            start_point_number,
                                            GlobalDOFParameter::Y,
                                            updated_fy_force_value)?;
                                    }
                                    else
                                    {
                                        self.fem.add_bc(BCType::Force,
                                            start_point_qy_load_number,
                                            start_point_number,
                                            GlobalDOFParameter::Y,
                                            qy_end)?;
                                    }
                                    if self.fem.is_bc_key_exist(end_point_qy_load_number,
                                        BCType::Force)
                                    {
                                        let existed_fy_force_value =
                                            self.fem.copy_bc_value(BCType::Force,
                                                end_point_qy_load_number)
                                                .map_err(|e| JsValue::from(e))?;
                                        let updated_fy_force_value = existed_fy_force_value +
                                            qy_end;
                                        self.fem.update_bc(BCType::Force,
                                            end_point_qy_load_number,
                                            end_point_number,
                                            GlobalDOFParameter::Y,
                                            updated_fy_force_value)?;
                                    }
                                    else
                                    {
                                        self.fem.add_bc(BCType::Force,
                                            end_point_qy_load_number,
                                            end_point_number,
                                            GlobalDOFParameter::Y,
                                            qy_end)?;
                                    }
                                }

                                if qz_end != V::from(0f32)
                                {
                                    if self.fem.is_bc_key_exist(start_point_qz_load_number,
                                        BCType::Force)
                                    {
                                        let existed_fz_force_value =
                                            self.fem.copy_bc_value(BCType::Force,
                                                start_point_qz_load_number)
                                                .map_err(|e| JsValue::from(e))?;
                                        let updated_fz_force_value = existed_fz_force_value +
                                            qz_end;
                                        self.fem.update_bc(BCType::Force,
                                            start_point_qz_load_number,
                                            start_point_number,
                                            GlobalDOFParameter::Z,
                                            updated_fz_force_value)?;
                                    }
                                    else
                                    {
                                        self.fem.add_bc(BCType::Force,
                                            start_point_qz_load_number,
                                            start_point_number,
                                            GlobalDOFParameter::Z,
                                            qz_end)?;
                                    }
                                    if self.fem.is_bc_key_exist(end_point_qz_load_number,
                                        BCType::Force)
                                    {
                                        let existed_fz_force_value =
                                            self.fem.copy_bc_value(BCType::Force,
                                                end_point_qz_load_number)
                                                .map_err(|e| JsValue::from(e))?;
                                        let updated_fz_force_value = existed_fz_force_value +
                                            qz_end;
                                        self.fem.update_bc(BCType::Force,
                                            end_point_qz_load_number,
                                            end_point_number,
                                            GlobalDOFParameter::Z,
                                            updated_fz_force_value)?;
                                    }
                                    else
                                    {
                                        self.fem.add_bc(BCType::Force,
                                            end_point_qz_load_number,
                                            end_point_number,
                                            GlobalDOFParameter::Z,
                                            qz_end)?;
                                    }
                                }
                            }

                            let mut number = *line_number + T::from(100u8) * T::from(100u8);
                            for _ in 0..number_of_digits
                            {
                                number *= T::from(10u8);
                            }

                            let mut i = 1u8;
                            while i < beam_elements_number_at_line
                            {
                                let x = x1 + step_x * V::from(i as f32);
                                let y = y1 + step_y * V::from(i as f32);
                                let z = z1 + step_z * V::from(i as f32);
                                number += T::from(1u8);
                                self.fem.add_node(number, x, y, z)?;

                                if let Some((_qx_end, qx_interim, _qy_end, qy_interim,
                                    _qz_end, qz_interim)) = optional_distributed_load
                                {
                                    let qx_load_number = number * T::from(6u8) - T::from(5u8);
                                    let qy_load_number = number * T::from(6u8) - T::from(4u8);
                                    let qz_load_number = number * T::from(6u8) - T::from(3u8);

                                    if qx_interim != V::from(0f32)
                                    {
                                        self.fem.add_bc(BCType::Force,
                                            qx_load_number,
                                            number,
                                            GlobalDOFParameter::X,
                                            qx_interim)?;
                                    }

                                    if qy_interim != V::from(0f32)
                                    {
                                        self.fem.add_bc(BCType::Force,
                                            qy_load_number,
                                            number,
                                            GlobalDOFParameter::Y,
                                            qy_interim)?;
                                    }

                                    if qz_interim != V::from(0f32)
                                    {
                                        self.fem.add_bc(BCType::Force,
                                            qz_load_number,
                                            number,
                                            GlobalDOFParameter::Z,
                                            qz_interim)?;
                                    }
                                }

                                nodes_coordinates.insert(number, (x, y, z));
                                let nodes_numbers =
                                    {
                                        if i == 1u8
                                        {
                                            vec![start_point_number, number]
                                        }
                                        else
                                        {
                                            vec![number - T::from(1u8), number]
                                        }
                                    };
                                self.fem.add_element(number,
                                    FEType::Beam2n1ipT,
                                    nodes_numbers.clone(),
                                    beam_element_properties.clone())?;
                                elements_nodes_numbers.insert(number, nodes_numbers);
                                let unique_elements_of_rotation_matrix =
                                    self.fem.extract_unique_elements_of_rotation_matrix(
                                        &number)?;
                                elements_rotation_matrices_data.insert(number,
                                    unique_elements_of_rotation_matrix);
                                i += 1u8;
                            }
                            self.fem.add_element(number + T::from(1u8),
                                FEType::Beam2n1ipT,
                                vec![number, end_point_number],
                                beam_element_properties)?;
                            elements_nodes_numbers.insert(number + T::from(1u8),
                                vec![number, end_point_number]);
                            let unique_elements_of_rotation_matrix =
                                self.fem.extract_unique_elements_of_rotation_matrix(
                                    &(number + T::from(1u8)))?;
                            elements_rotation_matrices_data.insert(number + T::from(1u8),
                                unique_elements_of_rotation_matrix);
                        },
                }
            }
        }

        let global_analysis_result = self.fem.global_analysis()?;

        for (reaction, dof_parameter_data) in
            global_analysis_result.reactions_values().iter()
                .zip(global_analysis_result.reactions_dof_parameters_data())
        {
            log(&format!("reaction: {:?}, node: {:?}, parameter: {:?}", reaction,
                     dof_parameter_data.copy_node_number(), dof_parameter_data.copy_dof_parameter()));
        }
        log("");

        for (displacement, dof_parameter_data) in
            global_analysis_result.displacements_values().iter()
                .zip(global_analysis_result.displacements_dof_parameters_data().iter())
        {
            log(&format!("displacement: {:?}, node: {:?}, parameter: {:?}", displacement,
                     dof_parameter_data.copy_node_number(), dof_parameter_data.copy_dof_parameter()));
        }
        log("");

        let displacements = global_analysis_result.displacements();

        let elements_analysis_result = self.fem.elements_analysis(
            &displacements)?;

        for (element_number, element_analysis_data) in
            elements_analysis_result.ref_elements_analysis_data().iter()
        {
            if let (Some(forces_values), Some(forces_components)) =
                (element_analysis_data.optional_ref_forces_values(), element_analysis_data.optional_ref_forces_components())
            {
                let mut forces_values_msg = String::from("Element forces: ");
                for force_value in forces_values.iter()
                {
                    forces_values_msg += &format!("{:?}, ", force_value);
                }
                let mut forces_components_msg = String::from("Element forces components: ");
                for force_component in forces_components.iter()
                {
                    forces_components_msg += &format!("{:?}, ", force_component);
                }
                log(&format!("Element number: {:?}", element_number));
                log(&format!("{}", forces_values_msg));
                log(&format!("{}", forces_components_msg));
                log("");
            }

            if let Some(nodal_forces) = element_analysis_data.ref_optional_nodal_forces()
            {
                log(&format!("Element number: {:?}", element_number));
                for (node_number, nodal_forces) in nodal_forces
                {
                    let mut forces_values = String::from("Nodal forces: ");
                    let mut forces_components = String::from("Nodal forces components: ");
                    for force_value in nodal_forces.ref_forces_values()
                    {
                        forces_values += &format!("{:?}, ", force_value);
                    }
                    for force_component in nodal_forces.ref_forces_components()
                    {
                        forces_components += &format!("{:?}, ", force_component);
                    }

                    log(&format!("\t Node number: {:?}", node_number));
                    log(&format!("\t \t {}", forces_values));
                    log(&format!("\t \t {}", forces_components));
                    log("");
                }
            }
        }
        log(&format!("{:?}", elements_analysis_result.ref_elements_by_types()));
        log("");

        let analysis_result = AnalysisResult::create(
            nodes_coordinates,
            elements_nodes_numbers,
            elements_rotation_matrices_data,
            global_analysis_result,
            elements_analysis_result);

        Ok(analysis_result)
    }
}
