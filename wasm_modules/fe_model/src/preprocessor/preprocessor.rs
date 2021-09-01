use wasm_bindgen::prelude::*;
use serde_json::json;
use std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign};
use std::hash::Hash;
use std::fmt::Debug;
use serde::Serialize;

use finite_element_method::my_float::MyFloatTrait;

use crate::preprocessor::geometry::geometry::Geometry;
use crate::preprocessor::properties::properties::Properties;

use crate::preprocessor::functions::get_line_points_coordinates;


pub struct Preprocessor<T, V>
{
    pub geometry: Geometry<T, V>,
    pub properties: Properties<T, V>,
    pub tolerance: V,
}


impl<T, V> Preprocessor<T, V>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> +
             Rem<Output = T> + AddAssign + Eq + From<u8> + Hash + Debug + Serialize + PartialOrd +
             SubAssign + 'static,
          V: Copy + Add<Output = V> + Sub<Output = V> + Mul<Output = V> + Div<Output = V> +
             Debug + Serialize + From<f32> + Into<f64> + MyFloatTrait + PartialEq + MulAssign +
             AddAssign + SubAssign + 'static,
{
    pub fn create(tolerance: V) -> Self
    {
        let geometry = Geometry::<T, V>::create();
        let properties = Properties::<T, V>::create();
        Preprocessor { geometry, properties, tolerance }
    }


    pub fn show_line_info(&mut self, number: T, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let (start_point_number, end_point_number) =
            self.geometry.extract_line_info_from_geometry(number)?;
        if let Some((material_name, cross_section_name, cross_section_type)) =
            self.properties.extract_line_info_from_properties(number)
        {
            let line_info_json = json!({ "line_data_with_props": { "number": number,
                "start_point_number": start_point_number, "end_point_number": end_point_number,
                "material_name": material_name, "cross_section_name": cross_section_name,
                "cross_section_type": cross_section_type } });
            let line_info = JsValue::from_serde(&line_info_json)
                .or(Err(JsValue::from("Geometry: Show line info: Line info could not be \
                    composed!")))?;
            let this = JsValue::null();
            let _ = handler.call1(&this, &line_info)?;
            Ok(())
        }
        else
        {
            let line_info_json = json!({ "line_data": { "number": number,
                "start_point_number": start_point_number, "end_point_number": end_point_number } });
            let line_info = JsValue::from_serde(&line_info_json)
                .or(Err(JsValue::from("Geometry: Show line info: Line info could not be \
                    composed!")))?;
            let this = JsValue::null();
            let _ = handler.call1(&this, &line_info)?;
            Ok(())
        }
    }


    pub fn update_point(&mut self, action_id: T, number: T, x: V, y: V,
        z: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let line_numbers_for_update =
            self.geometry.extract_line_numbers_for_update_or_delete(number);

        self.geometry.update_point(action_id, number, x, y, z,
            is_action_id_should_be_increased)?;

        self.properties.update_lines_in_properties(action_id, line_numbers_for_update,
            &self.geometry, get_line_points_coordinates, self.tolerance)?;

        Ok(())
    }


    pub fn delete_point(&mut self, action_id: T, number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let line_numbers_for_delete =
            self.geometry.extract_line_numbers_for_update_or_delete(number);

        self.properties.delete_line_numbers_from_properties(action_id,
            &line_numbers_for_delete)?;

        self.geometry.delete_point(action_id, number, &line_numbers_for_delete,
            is_action_id_should_be_increased)?;
        Ok(())
    }


    pub fn restore_point(&mut self, action_id: T, number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let restored_line_numbers =
            self.geometry.restore_point(action_id, number, is_action_id_should_be_increased)?;

        self.properties.restore_line_numbers_in_properties(action_id, restored_line_numbers)?;
        Ok(())
    }


    pub fn update_line(&mut self, action_id: T, number: T, start_point_number: T,
        end_point_number: T, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.update_line(action_id, number, start_point_number, end_point_number,
            is_action_id_should_be_increased)?;

        self.properties.update_line_in_properties(action_id, number, &self.geometry,
            get_line_points_coordinates, self.tolerance)?;

        Ok(())
    }


    pub fn delete_line(&mut self, action_id: T, number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.delete_line_numbers_from_properties(action_id, &vec![number])?;

        self.geometry.delete_line(action_id, number, is_action_id_should_be_increased)?;
        Ok(())
    }


    pub fn restore_line(&mut self, action_id: T, number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.restore_line(action_id, number, is_action_id_should_be_increased)?;

        self.properties.restore_line_numbers_in_properties(action_id,
            vec![number])?;
        Ok(())
    }
}
