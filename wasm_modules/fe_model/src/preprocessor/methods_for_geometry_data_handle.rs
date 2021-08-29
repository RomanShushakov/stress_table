use wasm_bindgen::prelude::*;
use std::fmt::Debug;
use serde::Serialize;
use std::hash::Hash;

use crate::preprocessor::traits::ClearByActionIdTrait;

use crate::Preprocessor;



impl<T, V> Preprocessor<T, V>
    where T: Copy + Debug + Serialize + Hash + Eq + PartialOrd,
          V: Copy + Debug + Serialize + PartialEq,
{
    pub fn add_point(&mut self, action_id: T, number: T, x: V, y: V,
        z: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.clear_by_action_id(action_id);

        self.geometry.add_point(action_id, number, x, y, z, is_action_id_should_be_increased)
    }


    pub fn add_line(&mut self, action_id: T, number: T, start_point_number: T,
        end_point_number: T, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.properties.clear_by_action_id(action_id);

        self.geometry.add_line(action_id, number, start_point_number, end_point_number,
            is_action_id_should_be_increased)
    }


    pub fn show_point_info(&mut self, number: T, handler: js_sys::Function) -> Result<(), JsValue>
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
