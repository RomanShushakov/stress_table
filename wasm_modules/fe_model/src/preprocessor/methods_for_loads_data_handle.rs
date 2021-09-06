use wasm_bindgen::prelude::*;
use std::fmt::Debug;
use serde_json::json;
use serde::Serialize;
use std::hash::Hash;

use crate::Preprocessor;
use crate::traits::ClearByActionIdTrait;


impl<T, V> Preprocessor<T, V>
    where T: Copy + Debug + Serialize + Hash + Eq + PartialOrd,
          V: Copy + Debug + Serialize + PartialEq,
{
    pub fn add_concentrated_load(&mut self, action_id: T, point_number: T, fx: V, fy: V, fz: V,
        mx: V, my: V, mz: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.properties.clear_by_action_id(action_id);

        self.loads.add_concentrated_load(action_id, point_number, fx, fy, fz, mx, my, mz,
            is_action_id_should_be_increased)
    }


    pub fn update_concentrated_load(&mut self, action_id: T, point_number: T, fx: V, fy: V, fz: V,
        mx: V, my: V, mz: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.properties.clear_by_action_id(action_id);

        self.loads.update_concentrated_load(action_id, point_number, fx, fy, fz, mx, my, mz,
            is_action_id_should_be_increased)
    }


    pub fn delete_concentrated_load(&mut self, action_id: T, point_number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.properties.clear_by_action_id(action_id);

        self.loads.delete_concentrated_load(action_id, point_number,
            is_action_id_should_be_increased)
    }


    pub fn restore_concentrated_load(&mut self, action_id: T, point_number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.loads.restore_concentrated_load(action_id, point_number,
            is_action_id_should_be_increased)
    }


    pub fn show_concentrated_load_info(&mut self, point_number: T, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        self.loads.show_concentrated_load_info(point_number, handler)
    }


    pub fn extract_concentrated_loads(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.loads.extract_concentrated_loads(handler)
    }
}
