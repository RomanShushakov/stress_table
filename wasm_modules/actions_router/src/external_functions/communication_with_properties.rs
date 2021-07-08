use wasm_bindgen::prelude::*;

use crate::types::{FEUInt, FEFloat};


#[wasm_bindgen(module = "/js/interface_to_communicate_with_fe_model.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addMaterialToProperties, catch)]
    pub fn add_material_to_properties(action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateMaterialInProperties, catch)]
    pub fn update_material_in_properties(action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteMaterialFromProperties, catch)]
    pub fn delete_material_from_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = restoreMaterialInProperties, catch)]
    pub fn restore_material_in_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addTrussSectionToProperties, catch)]
    pub fn add_truss_section_to_properties(action_id: FEUInt, name: &str, area: FEFloat,
        area2: Option<FEFloat>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateTrussSectionInProperties, catch)]
    pub fn update_truss_section_in_properties(action_id: FEUInt, name: &str, area: FEFloat,
        area2: Option<FEFloat>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteTrussSectionFromProperties, catch)]
    pub fn delete_truss_section_from_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = restoreTrussSectionInProperties, catch)]
    pub fn restore_truss_section_in_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addBeamSectionToProperties, catch)]
    pub fn add_beam_section_to_properties(action_id: FEUInt, name: &str,
        area: FEFloat, i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateBeamSectionInProperties, catch)]
    pub fn update_beam_section_in_properties(action_id: FEUInt, name: &str,
        area: FEFloat, i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteBeamSectionFromProperties, catch)]
    pub fn delete_beam_section_from_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = restoreBeamSectionInProperties, catch)]
    pub fn restore_beam_section_in_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addPropertiesToProperties, catch)]
    pub fn add_properties_to_properties(action_id: FEUInt, name: &str,
        material_name: &str, cross_section_name: &str, cross_section_type: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updatePropertiesInProperties, catch)]
    pub fn update_properties_in_properties(action_id: FEUInt, name: &str,
        material_name: &str, cross_section_name: &str, cross_section_type: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deletePropertiesFromProperties, catch)]
    pub fn delete_properties_from_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = restorePropertiesInProperties, catch)]
    pub fn restore_properties_in_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addAssignedPropertiesToProperties, catch)]
    pub fn add_assigned_properties_to_properties(action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateAssignedPropertiesInProperties, catch)]
    pub fn update_assigned_properties_in_properties(action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteAssignedPropertiesFromProperties, catch)]
    pub fn delete_assigned_properties_from_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = restoreAssignedPropertiesInProperties, catch)]
    pub fn restore_assigned_properties_in_properties(action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addBeamSectionLocalAxis1DirectionToProperties, catch)]
    pub fn add_beam_section_local_axis_1_direction_to_properties(action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = removeBeamSectionLocalAxis1DirectionToProperties, catch)]
    pub fn remove_beam_section_local_axis_1_direction_to_properties(action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = clearPropertiesModuleByActionId)]
    pub fn clear_properties_module_by_action_id(action_id: FEUInt);

    #[wasm_bindgen(js_name = extractMaterials)]
    pub fn extract_materials(handler: js_sys::Function);

    #[wasm_bindgen(js_name = extractTrussSections)]
    pub fn extract_truss_sections(handler: js_sys::Function);

    #[wasm_bindgen(js_name = extractBeamSections)]
    pub fn extract_beam_sections(handler: js_sys::Function);

    #[wasm_bindgen(js_name = extractProperties)]
    pub fn extract_properties(handler: js_sys::Function);

    #[wasm_bindgen(js_name = extractAssignedProperties)]
    pub fn extract_assigned_properties(handler: js_sys::Function);

    #[wasm_bindgen(js_name = extractBeamSectionsOrientations)]
    pub fn extract_beam_sections_orientations(handler: js_sys::Function);
}
