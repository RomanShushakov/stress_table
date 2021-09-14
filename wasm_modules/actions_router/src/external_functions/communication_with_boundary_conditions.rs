use wasm_bindgen::prelude::*;

use crate::types::{FEUInt, FEFloat};


#[wasm_bindgen(module = "/js/interface_to_communicate_with_fe_model.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addBoundaryConditionToBoundaryConditions, catch)]
    pub fn add_boundary_condition_to_boundary_conditions(action_id: FEUInt, point_number: FEUInt,
        optional_ux: Option<FEFloat>, optional_uy: Option<FEFloat>, optional_uz: Option<FEFloat>,
        optional_rx: Option<FEFloat>, optional_ry: Option<FEFloat>, optional_rz: Option<FEFloat>,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = updateBoundaryConditionInBoundaryConditions, catch)]
    pub fn update_boundary_condition_in_boundary_conditions(action_id: FEUInt, point_number: FEUInt,
        optional_ux: Option<FEFloat>, optional_uy: Option<FEFloat>, optional_uz: Option<FEFloat>,
        optional_rx: Option<FEFloat>, optional_ry: Option<FEFloat>, optional_rz: Option<FEFloat>,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = deleteBoundaryConditionFromBoundaryConditions, catch)]
    pub fn delete_boundary_condition_from_boundary_conditions(action_id: FEUInt,
        point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = restoreBoundaryConditionInBoundaryConditions, catch)]
    pub fn restore_boundary_condition_in_boundary_conditions(action_id: FEUInt,
        point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = extractBoundaryConditions)]
    pub fn extract_boundary_conditions(handler: js_sys::Function);


    #[wasm_bindgen(js_name = showBoundaryConditionInfo, catch)]
    pub fn show_boundary_condition_info(point_number: FEUInt, handler: js_sys::Function)
        -> Result<(), JsValue>;
}
