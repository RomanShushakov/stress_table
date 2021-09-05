use wasm_bindgen::prelude::*;

use crate::FEModel;

use crate::types::{FEUInt, FEFloat};
use crate::functions::log;


#[wasm_bindgen]
impl FEModel
{
    pub fn add_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.add_point(action_id, number, x, y, z, is_action_id_should_be_increased)
    }


    pub fn update_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.update_point(action_id, number, x, y, z, is_action_id_should_be_increased)
    }


    pub fn delete_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.delete_point(action_id, number, is_action_id_should_be_increased)
    }


    pub fn restore_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.restore_point(action_id, number, is_action_id_should_be_increased)
    }


    pub fn add_line(&mut self, action_id: FEUInt, number: FEUInt, start_point_number: FEUInt,
        end_point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.add_line(action_id, number, start_point_number, end_point_number,
            is_action_id_should_be_increased)
    }


    pub fn update_line(&mut self, action_id: FEUInt, number: FEUInt, start_point_number: FEUInt,
        end_point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.update_line(action_id, number, start_point_number, end_point_number,
            is_action_id_should_be_increased)
    }


    pub fn delete_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.delete_line(action_id, number, is_action_id_should_be_increased)
    }


    pub fn restore_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.restore_line(action_id, number, is_action_id_should_be_increased)
    }


    pub fn add_material(&mut self, action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.add_material(action_id, name, young_modulus, poisson_ratio,
            is_action_id_should_be_increased)
    }


    pub fn update_material(&mut self, action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.update_material(action_id, name, young_modulus, poisson_ratio,
            is_action_id_should_be_increased)
    }


    pub fn delete_material(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.delete_material(action_id, name, is_action_id_should_be_increased)
    }


    pub fn restore_material(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.restore_material(action_id, name, is_action_id_should_be_increased)
    }


    pub fn add_truss_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        area2: Option<FEFloat>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.add_truss_section(action_id, name, area, area2,
            is_action_id_should_be_increased)
    }


    pub fn update_truss_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        area2: Option<FEFloat>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.update_truss_section(action_id, name, area, area2,
            is_action_id_should_be_increased)
    }


    pub fn delete_truss_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.delete_truss_section(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn restore_truss_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.restore_truss_section(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn add_beam_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat, shear_factor: FEFloat,
        is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.preprocessor.add_beam_section(action_id, name, area, i11, i22, i12, it,
            shear_factor, is_action_id_should_be_increased)
    }


    pub fn update_beam_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat, shear_factor: FEFloat,
        is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.preprocessor.update_beam_section(action_id, name, area, i11, i22, i12, it,
            shear_factor, is_action_id_should_be_increased)
    }


    pub fn delete_beam_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.delete_beam_section(action_id, name, is_action_id_should_be_increased)
    }


    pub fn restore_beam_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.restore_beam_section(action_id, name, is_action_id_should_be_increased)
    }


    pub fn add_properties(&mut self, action_id: FEUInt, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.preprocessor.add_properties(action_id, name, material_name, cross_section_name,
            cross_section_type, is_action_id_should_be_increased)
    }


    pub fn update_properties(&mut self, action_id: FEUInt, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.preprocessor.update_properties(action_id, name, material_name, cross_section_name,
            cross_section_type, is_action_id_should_be_increased)
    }


    pub fn delete_properties(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.delete_properties(action_id, name, is_action_id_should_be_increased)
    }


    pub fn restore_properties(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.restore_properties(action_id, name, is_action_id_should_be_increased)
    }


    pub fn add_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.add_assigned_properties_to_lines(action_id, name, line_numbers,
            is_action_id_should_be_increased)
    }


    pub fn update_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.update_assigned_properties_to_lines(action_id, name, line_numbers,
            is_action_id_should_be_increased)
    }


    pub fn delete_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.delete_assigned_properties_to_lines(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn restore_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.restore_assigned_properties_to_lines(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn add_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.preprocessor.add_beam_section_local_axis_1_direction(action_id, local_axis_1_direction,
            is_action_id_should_be_increased)
    }


    pub fn remove_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.preprocessor.remove_beam_section_local_axis_1_direction(action_id,
            local_axis_1_direction, is_action_id_should_be_increased)
    }


    pub fn restore_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.preprocessor.restore_beam_section_local_axis_1_direction(action_id,
            local_axis_1_direction, is_action_id_should_be_increased)
    }


    pub fn update_beam_section_orientation_data(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], line_numbers: &[FEUInt],
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.update_beam_section_orientation_data(action_id, local_axis_1_direction,
            line_numbers, is_action_id_should_be_increased)
    }


    pub fn add_concentrated_load(&mut self, action_id: FEUInt, point_number: FEUInt,
        fx: FEFloat, fy: FEFloat, fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.add_concentrated_load(action_id, point_number, fx, fy, fz, mx, my, mz,
        is_action_id_should_be_increased)
    }


    pub fn update_concentrated_load(&mut self, action_id: FEUInt, point_number: FEUInt,
        fx: FEFloat, fy: FEFloat, fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        Ok(())
    }


    pub fn delete_concentrated_load(&mut self, action_id: FEUInt, point_number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        Ok(())
    }


    pub fn restore_concentrated_load(&mut self, action_id: FEUInt, point_number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        Ok(())
    }


    pub fn extract_points(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.extract_points(handler)
    }


    pub fn extract_lines(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.extract_lines(handler)
    }


    pub fn extract_materials(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.extract_materials(handler)
    }


    pub fn extract_truss_sections(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.extract_truss_sections(handler)
    }


    pub fn extract_beam_sections(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.extract_beam_sections(handler)
    }


    pub fn extract_properties(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.extract_properties(handler)
    }


    pub fn extract_assigned_properties_to_lines(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.extract_assigned_properties_to_lines(handler)
    }


    pub fn extract_beam_sections_local_axis_1_directions(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        self.preprocessor.extract_beam_sections_local_axis_1_directions(handler)
    }


    pub fn extract_concentrated_loads(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        self.preprocessor.extract_concentrated_loads(handler)
    }


    pub fn show_point_info(&mut self, number: FEUInt, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.show_point_info(number, handler)
    }


    pub fn show_line_info(&mut self, number: FEUInt, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.show_line_info(number, handler)
    }
}
