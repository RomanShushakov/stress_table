use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/interface_to_communicate_with_geometry.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addPointToGeometry, catch)]
    pub fn add_point_to_geometry(action_id: u32, number: u32, x: f64, y: f64, z: f64,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updatePointInGeometry, catch)]
    pub fn update_point_in_geometry(action_id: u32, number: u32, x: f64, y: f64, z: f64,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deletePointFromGeometry, catch)]
    pub fn delete_point_from_geometry(action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = restorePointInGeometry, catch)]
    pub fn restore_point_in_geometry(action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = addLineToGeometry, catch)]
    pub fn add_line_to_geometry(action_id: u32, number: u32, start_point_number: u32,
        end_point_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateLineInGeometry, catch)]
    pub fn update_line_in_geometry(action_id: u32, number: u32, start_point_number: u32,
        end_point_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteLineFromGeometry, catch)]
    pub fn delete_line_from_geometry(action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = restoreLineInGeometry, catch)]
    pub fn restore_line_in_geometry(action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = clearGeometryModuleByActionId)]
    pub fn clear_geometry_module_by_action_id(action_id: u32);

    #[wasm_bindgen(js_name = showPointInfo, catch)]
    pub fn show_point_info(number: u32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = showLineInfoFromGeometry, catch)]
    pub fn show_line_info_from_geometry(number: u32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = extractPoints)]
    pub fn extract_points(handler: js_sys::Function);

    #[wasm_bindgen(js_name = extractLines)]
    pub fn extract_lines(handler: js_sys::Function);
}
