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
    pub fn add_boundary_condition(&mut self, action_id: T, point_number: T,
        optional_ux: Option<V>, optional_uy: Option<V>, optional_uz: Option<V>,
        optional_rx: Option<V>, optional_ry: Option<V>, optional_rz: Option<V>,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.properties.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);

        self.boundary_conditions.add_boundary_condition(action_id, point_number,
            optional_ux, optional_uy, optional_uz, optional_rx, optional_ry, optional_rz,
            is_action_id_should_be_increased)
    }


    pub fn update_boundary_condition(&mut self, action_id: T, point_number: T,
        optional_ux: Option<V>, optional_uy: Option<V>, optional_uz: Option<V>,
        optional_rx: Option<V>, optional_ry: Option<V>, optional_rz: Option<V>,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.properties.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);

        self.boundary_conditions.update_boundary_condition(action_id, point_number,
            optional_ux, optional_uy, optional_uz, optional_rx, optional_ry, optional_rz,
            is_action_id_should_be_increased)
    }


    pub fn delete_boundary_condition(&mut self, action_id: T, point_number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.geometry.clear_by_action_id(action_id);
        self.properties.clear_by_action_id(action_id);
        self.loads.clear_by_action_id(action_id);

        self.boundary_conditions.delete_boundary_condition(action_id, point_number,
            is_action_id_should_be_increased)
    }


    pub fn restore_boundary_condition(&mut self, action_id: T, point_number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.boundary_conditions.restore_boundary_condition(action_id, point_number,
            is_action_id_should_be_increased)
    }


    pub fn show_boundary_condition_info(&mut self, point_number: T, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        self.boundary_conditions.show_boundary_condition_info(point_number, handler)
    }


    pub fn extract_boundary_conditions(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.boundary_conditions.extract_boundary_conditions(handler)
    }
}
