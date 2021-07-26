use wasm_bindgen::prelude::*;

use crate::preprocessor::traits::ClearByActionIdTrait;

use crate::Preprocessor;

use crate::preprocessor::functions::get_line_points_coordinates;

use crate::types::{FEUInt, FEFloat};


impl Preprocessor
{
    pub fn clear_properties_module_by_action_id(&mut self, action_id: FEUInt)
    {
        self.properties.clear_properties_module_by_action_id(action_id)
    }


    pub fn add_material(&mut self, action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.add_material(action_id, name, young_modulus, poisson_ratio,
            is_action_id_should_be_increased)
    }


    pub fn update_material(&mut self, action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.update_material(action_id, name, young_modulus, poisson_ratio,
            is_action_id_should_be_increased)
    }


    pub fn delete_material(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.delete_material(action_id, name, is_action_id_should_be_increased)
    }


    pub fn restore_material(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_material(action_id, name, is_action_id_should_be_increased)
    }


    pub fn add_truss_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        area2: Option<FEFloat>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.add_truss_section(action_id, name, area, area2,
            is_action_id_should_be_increased)
    }


    pub fn update_truss_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        area2: Option<FEFloat>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.update_truss_section(action_id, name, area, area2,
            is_action_id_should_be_increased)
    }


    pub fn delete_truss_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.delete_truss_section(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn restore_truss_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_truss_section(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn add_beam_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.add_beam_section(action_id, name, area, i11, i22, i12, it,
            is_action_id_should_be_increased)
    }


    pub fn update_beam_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.update_beam_section(action_id, name, area, i11, i22, i12, it,
            is_action_id_should_be_increased)
    }


    pub fn delete_beam_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.delete_beam_section(action_id, name, is_action_id_should_be_increased)
    }


    pub fn restore_beam_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_beam_section(action_id, name, is_action_id_should_be_increased)
    }


    pub fn add_properties(&mut self, action_id: FEUInt, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.add_properties(action_id, name, material_name, cross_section_name,
            cross_section_type, is_action_id_should_be_increased)
    }


    pub fn update_properties(&mut self, action_id: FEUInt, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.update_properties(action_id, name, material_name, cross_section_name,
            cross_section_type, is_action_id_should_be_increased)
    }


    pub fn delete_properties(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.delete_properties(action_id, name, is_action_id_should_be_increased)
    }


    pub fn restore_properties(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_properties(action_id, name, is_action_id_should_be_increased)
    }


    pub fn add_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Add assigned properties to lines action";
        self.geometry.check_for_line_numbers_existence(line_numbers, error_message_header)?;

        self.geometry.clear_by_action_id(action_id);

        self.properties.add_assigned_properties_to_lines(action_id, name, line_numbers,
            is_action_id_should_be_increased)
    }


    pub fn update_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Update assigned properties to lines action";
        self.geometry.check_for_line_numbers_existence(line_numbers, error_message_header)?;

        self.geometry.clear_by_action_id(action_id);

        self.properties.update_assigned_properties_to_lines(action_id, name, line_numbers,
            is_action_id_should_be_increased)
    }


    pub fn delete_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.delete_assigned_properties_to_lines(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn restore_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_assigned_properties_to_lines(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn add_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.add_beam_section_local_axis_1_direction(action_id, local_axis_1_direction,
            is_action_id_should_be_increased)
    }


    pub fn remove_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);

        self.properties.remove_beam_section_local_axis_1_direction(action_id,
            local_axis_1_direction, is_action_id_should_be_increased)
    }


    pub fn restore_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.properties.restore_beam_section_local_axis_1_direction(action_id,
            local_axis_1_direction, is_action_id_should_be_increased)
    }


    // pub fn update_beam_section_orientation_data(&mut self, action_id: FEUInt,
    //     local_axis_1_direction: &[FEFloat], line_numbers: &[FEUInt],
    //     is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    // {
    //     self.properties.update_beam_section_orientation_data(action_id, local_axis_1_direction,
    //         line_numbers, is_action_id_should_be_increased, &self.geometry,
    //         get_line_points_coordinates)
    // }


    pub fn extract_materials(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.properties.extract_materials(handler)
    }


    pub fn extract_truss_sections(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.properties.extract_truss_sections(handler)
    }


    pub fn extract_beam_sections(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.properties.extract_beam_sections(handler)
    }


    pub fn extract_properties(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.properties.extract_properties(handler)
    }


    pub fn extract_assigned_properties_to_lines(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.properties.extract_assigned_properties_to_lines(handler)
    }


    pub fn extract_beam_sections_local_axis_1_directions(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        self.properties.extract_beam_sections_local_axis_1_directions(handler)
    }
}
