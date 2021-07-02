use wasm_bindgen::prelude::*;

use crate::FEModel;

use crate::types::{FEUInt, FEFloat};


#[wasm_bindgen]
impl FEModel
{
    pub fn clear_geometry_module_by_action_id(&mut self, action_id: FEUInt)
    {
        self.preprocessor.clear_geometry_module_by_action_id(action_id);
    }


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
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.preprocessor.delete_point(action_id, number, is_action_id_should_be_increased)
    }


    pub fn restore_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
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
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.preprocessor.delete_line(action_id, number, is_action_id_should_be_increased)
    }


    pub fn restore_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.preprocessor.restore_line(action_id, number, is_action_id_should_be_increased)
    }


    pub fn extract_points(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.extract_points(handler)
    }


    pub fn extract_lines(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.extract_lines(handler)
    }
}
