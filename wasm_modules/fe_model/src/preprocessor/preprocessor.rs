use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::geometry::geometry::Geometry;
use crate::preprocessor::properties::properties::Properties;

use crate::types::FEUInt;


pub struct Preprocessor
{
    pub geometry: Geometry,
    pub properties: Properties,
}


impl Preprocessor
{
    pub fn create() -> Self
    {
        let geometry = Geometry::create();
        let properties = Properties::create();
        Preprocessor { geometry, properties }
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


    pub fn delete_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let line_numbers_for_delete =
            self.geometry.extract_line_numbers_for_delete(number);

        self.properties.delete_line_numbers_from_properties(action_id,
            &line_numbers_for_delete)?;

        self.geometry.delete_point(action_id, number, &line_numbers_for_delete,
            is_action_id_should_be_increased)?;
        Ok(())
    }


    pub fn delete_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.delete_line_numbers_from_properties(action_id, &vec![number])?;

        self.geometry.delete_line(action_id, number, is_action_id_should_be_increased)?;
        Ok(())
    }
}
