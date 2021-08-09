use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::geometry::geometry::Geometry;
use crate::preprocessor::properties::properties::Properties;

use crate::preprocessor::functions::get_line_points_coordinates;

use crate::types::{FEUInt, FEFloat};
use crate::consts::TOLERANCE;


pub struct Preprocessor
{
    pub geometry: Geometry<FEUInt, FEFloat>,
    pub properties: Properties<FEUInt, FEFloat>,
    pub tolerance: FEFloat,
}


impl Preprocessor
{
    pub fn create() -> Self
    {
        let geometry = Geometry::create();
        let properties = Properties::create();
        Preprocessor { geometry, properties, tolerance: TOLERANCE }
    }


    pub fn show_line_info(&mut self, number: FEUInt, handler: js_sys::Function) -> Result<(), JsValue>
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


    pub fn update_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let line_numbers_for_update =
            self.geometry.extract_line_numbers_for_update_or_delete(number);

        self.geometry.update_point(action_id, number, x, y, z,
            is_action_id_should_be_increased)?;

        self.properties.update_lines_in_properties(action_id, line_numbers_for_update,
            &self.geometry, get_line_points_coordinates, self.tolerance)?;

        Ok(())
    }


    pub fn delete_point(&mut self, action_id: FEUInt, number: FEUInt,
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


    pub fn restore_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let restored_line_numbers =
            self.geometry.restore_point(action_id, number, is_action_id_should_be_increased)?;

        self.properties.restore_line_numbers_in_properties(action_id, &restored_line_numbers)?;
        Ok(())
    }


    pub fn update_line(&mut self, action_id: FEUInt, number: FEUInt, start_point_number: FEUInt,
        end_point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.update_line(action_id, number, start_point_number, end_point_number,
            is_action_id_should_be_increased)?;

        self.properties.update_line_in_properties(action_id, number, &self.geometry,
            get_line_points_coordinates, self.tolerance)?;

        Ok(())
    }


    pub fn delete_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.delete_line_numbers_from_properties(action_id, &vec![number])?;

        self.geometry.delete_line(action_id, number, is_action_id_should_be_increased)?;
        Ok(())
    }


    pub fn restore_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.restore_line(action_id, number, is_action_id_should_be_increased)?;

        self.properties.restore_line_numbers_in_properties(action_id,
            &vec![number])?;
        Ok(())
    }
}
