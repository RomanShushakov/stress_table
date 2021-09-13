use wasm_bindgen::prelude::*;
use std::ops::{Add, Rem, Sub, Mul, Div, AddAssign, SubAssign, MulAssign};
use std::fmt::Debug;
use serde::Serialize;
use std::hash::Hash;

use crate::traits::ClearByActionIdTrait;

use crate::Preprocessor;

use crate::preprocessor::functions::get_line_points_coordinates;

use finite_element_method::my_float::MyFloatTrait;


impl<T, V> Preprocessor<T, V>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> +
             Rem<Output = T> + AddAssign + Debug + Serialize + From<u8> + Eq + PartialOrd +
             SubAssign + Hash + 'static,
          V: Copy + Add<Output = V> + Sub<Output = V> + Mul<Output = V> + Div<Output = V> +
             Debug + Serialize + PartialEq + From<f32> + Into<f64> + PartialOrd + MyFloatTrait +
             AddAssign + SubAssign + MulAssign + 'static,
{
    pub fn add_material(&mut self, action_id: T, name: &str, young_modulus: V,
        poisson_ratio: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.add_material(action_id, name, young_modulus, poisson_ratio,
            is_action_id_should_be_increased)
    }


    pub fn update_material(&mut self, action_id: T, name: &str, young_modulus: V,
        poisson_ratio: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.update_material(action_id, name, young_modulus, poisson_ratio,
            is_action_id_should_be_increased)
    }


    pub fn delete_material(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.delete_material(action_id, name, is_action_id_should_be_increased)
    }


    pub fn restore_material(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_material(action_id, name, is_action_id_should_be_increased)
    }


    pub fn add_truss_section(&mut self, action_id: T, name: &str, area: V,
        area2: Option<V>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.add_truss_section(action_id, name, area, area2,
            is_action_id_should_be_increased)
    }


    pub fn update_truss_section(&mut self, action_id: T, name: &str, area: V,
        area2: Option<V>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.update_truss_section(action_id, name, area, area2,
            is_action_id_should_be_increased)
    }


    pub fn delete_truss_section(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.delete_truss_section(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn restore_truss_section(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_truss_section(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn add_beam_section(&mut self, action_id: T, name: &str, area: V,
        i11: V, i22: V, i12: V, it: V, shear_factor: V,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.add_beam_section(action_id, name, area, i11, i22, i12, it, shear_factor,
            is_action_id_should_be_increased)
    }


    pub fn update_beam_section(&mut self, action_id: T, name: &str, area: V,
        i11: V, i22: V, i12: V, it: V, shear_factor: V,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.update_beam_section(action_id, name, area, i11, i22, i12, it, shear_factor,
            is_action_id_should_be_increased)
    }


    pub fn delete_beam_section(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.delete_beam_section(action_id, name, is_action_id_should_be_increased)
    }


    pub fn restore_beam_section(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_beam_section(action_id, name, is_action_id_should_be_increased)
    }


    pub fn add_properties(&mut self, action_id: T, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.add_properties(action_id, name, material_name, cross_section_name,
            cross_section_type, is_action_id_should_be_increased)
    }


    pub fn update_properties(&mut self, action_id: T, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.update_properties(action_id, name, material_name, cross_section_name,
            cross_section_type, is_action_id_should_be_increased)
    }


    pub fn delete_properties(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.delete_properties(action_id, name, is_action_id_should_be_increased)
    }


    pub fn restore_properties(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_properties(action_id, name, is_action_id_should_be_increased)
    }


    pub fn add_assigned_properties_to_lines(&mut self, action_id: T, name: &str,
        line_numbers: &[T], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Add assigned properties to lines action";
        self.geometry.check_for_line_numbers_existence(line_numbers, error_message_header)?;

        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.add_assigned_properties_to_lines(action_id, name, line_numbers,
            is_action_id_should_be_increased)
    }


    pub fn update_assigned_properties_to_lines(&mut self, action_id: T, name: &str,
        line_numbers: &[T], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Update assigned properties to lines action";
        self.geometry.check_for_line_numbers_existence(line_numbers, error_message_header)?;

        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.update_assigned_properties_to_lines(action_id, name, line_numbers,
            is_action_id_should_be_increased)
    }


    pub fn delete_assigned_properties_to_lines(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.delete_assigned_properties_to_lines(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn restore_assigned_properties_to_lines(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.restore_assigned_properties_to_lines(action_id, name,
            is_action_id_should_be_increased)
    }


    pub fn add_beam_section_local_axis_1_direction(&mut self, action_id: T,
        local_axis_1_direction: &[V], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.add_beam_section_local_axis_1_direction(action_id, local_axis_1_direction,
            is_action_id_should_be_increased)
    }


    pub fn remove_beam_section_local_axis_1_direction(&mut self, action_id: T,
        local_axis_1_direction: &[V], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.remove_beam_section_local_axis_1_direction(action_id,
            local_axis_1_direction, is_action_id_should_be_increased)
    }


    pub fn restore_beam_section_local_axis_1_direction(&mut self, action_id: T,
        local_axis_1_direction: &[V], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.properties.restore_beam_section_local_axis_1_direction(action_id,
            local_axis_1_direction, is_action_id_should_be_increased)
    }


    pub fn update_beam_section_orientation_data(&mut self, action_id: T,
        local_axis_1_direction: &[V], line_numbers: &[T],
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);
        self.boundary_conditions.clear_by_action_id(action_id);

        self.properties.update_beam_section_orientation_data(action_id, local_axis_1_direction,
            line_numbers, is_action_id_should_be_increased, &self.geometry,
            get_line_points_coordinates, self.tolerance)
    }


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
