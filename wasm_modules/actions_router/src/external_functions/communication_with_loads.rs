use wasm_bindgen::prelude::*;

use crate::types::{FEUInt, FEFloat};


#[wasm_bindgen(module = "/js/interface_to_communicate_with_fe_model.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addConcentratedLoadToLoads, catch)]
    pub fn add_concentrated_load_to_loads(action_id: FEUInt, point_number: FEUInt, fx: FEFloat,
        fy: FEFloat, fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = updateConcentratedLoadInLoads, catch)]
    pub fn update_concentrated_load_in_loads(action_id: FEUInt, point_number: FEUInt, fx: FEFloat,
        fy: FEFloat, fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = deleteConcentratedLoadFromLoads, catch)]
    pub fn delete_concentrated_load_from_loads(action_id: FEUInt, point_number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = restoreConcentratedLoadInLoads, catch)]
    pub fn restore_concentrated_load_in_loads(action_id: FEUInt, point_number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = showConcentratedLoadInfo, catch)]
    pub fn show_concentrated_load_info(point_number: FEUInt, handler: js_sys::Function)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = extractConcentratedLoads)]
    pub fn extract_concentrated_loads(handler: js_sys::Function);
}
