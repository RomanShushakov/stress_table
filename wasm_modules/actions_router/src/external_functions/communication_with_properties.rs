use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/interface_to_communicate_with_properties.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addMaterialToProperties, catch)]
    pub fn add_material_to_properties(action_id: u32, name: &str, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateMaterialInProperties, catch)]
    pub fn update_material_in_properties(action_id: u32, name: &str, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteMaterialFromProperties, catch)]
    pub fn delete_material_from_properties(action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = restoreMaterialInProperties, catch)]
    pub fn restore_material_in_properties(action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addTrussSectionToProperties, catch)]
    pub fn add_truss_section_to_properties(action_id: u32, name: &str, area: f64,
        area2: Option<f64>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateTrussSectionInProperties, catch)]
    pub fn update_truss_section_in_properties(action_id: u32, name: &str, area: f64,
        area2: Option<f64>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteTrussSectionFromProperties, catch)]
    pub fn delete_truss_section_from_properties(action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = restoreTrussSectionInProperties, catch)]
    pub fn restore_truss_section_in_properties(action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addBeamSectionToProperties, catch)]
    pub fn add_beam_section_to_properties(action_id: u32, name: &str,
        area: f64, i11: f64, i22: f64, i12: f64, it: f64, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateBeamSectionInProperties, catch)]
    pub fn update_beam_section_in_properties(action_id: u32, name: &str,
        area: f64, i11: f64, i22: f64, i12: f64, it: f64, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteBeamSectionFromProperties, catch)]
    pub fn delete_beam_section_from_properties(action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = restoreBeamSectionInProperties, catch)]
    pub fn restore_beam_section_in_properties(action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteLineNumbersFromProperties, catch)]
    pub fn delete_line_numbers_from_properties(action_id: u32, line_numbers: JsValue)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = clearPropertiesModuleByActionId)]
    pub fn clear_properties_module_by_action_id(action_id: u32);
}
