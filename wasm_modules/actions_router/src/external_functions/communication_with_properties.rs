use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/interface_to_communicate_with_properties.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addMaterialToProperties, catch)]
    pub fn add_material_to_properties(action_id: u32, name: String, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateMaterialInProperties, catch)]
    pub fn update_material_in_properties(action_id: u32, name: String, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteMaterialFromProperties, catch)]
    pub fn delete_material_from_properties(action_id: u32, name: String,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = undoDeleteMaterialFromProperties, catch)]
    pub fn undo_delete_material_from_properties(action_id: u32, name: String,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addTrussSectionToProperties, catch)]
    pub fn add_truss_section_to_properties(action_id: u32, name: String, area: f64,
        area2: Option<f64>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateTrussSectionInProperties, catch)]
    pub fn update_truss_section_in_properties(action_id: u32, name: String, area: f64,
        area2: Option<f64>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteTrussSectionFromProperties, catch)]
    pub fn delete_truss_section_from_properties(action_id: u32, name: String,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;
}
