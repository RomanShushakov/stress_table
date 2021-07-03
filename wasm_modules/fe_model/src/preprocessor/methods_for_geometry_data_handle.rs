use wasm_bindgen::prelude::*;

use crate::Preprocessor;

use crate::types::{FEUInt, FEFloat};


impl Preprocessor
{
    pub fn clear_geometry_module_by_action_id(&mut self, action_id: FEUInt)
    {
        self.geometry.clear_geometry_module_by_action_id(action_id);
    }


    pub fn add_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.add_point(action_id, number, x, y, z,
            is_action_id_should_be_increased)
    }


    pub fn update_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.update_point(action_id, number, x, y, z,
            is_action_id_should_be_increased)
    }


    pub fn delete_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.geometry.delete_point(action_id, number, is_action_id_should_be_increased)
    }


    pub fn restore_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.geometry.restore_point(action_id, number, is_action_id_should_be_increased)
    }


    pub fn add_line(&mut self, action_id: FEUInt, number: FEUInt, start_point_number: FEUInt,
        end_point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.add_line(action_id, number, start_point_number,
            end_point_number, is_action_id_should_be_increased)
    }


    pub fn update_line(&mut self, action_id: FEUInt, number: FEUInt, start_point_number: FEUInt,
        end_point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.update_line(action_id, number, start_point_number,
            end_point_number, is_action_id_should_be_increased)
    }


    pub fn delete_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.geometry.delete_line(action_id, number, is_action_id_should_be_increased)
    }


    pub fn restore_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.geometry.restore_line(action_id, number, is_action_id_should_be_increased)
    }


    pub fn show_point_info(&mut self, number: FEUInt, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.geometry.show_point_info(number, handler)
    }


    pub fn extract_points(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.geometry.extract_points(handler)
    }


    pub fn extract_lines(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.geometry.extract_lines(handler)
    }
}
