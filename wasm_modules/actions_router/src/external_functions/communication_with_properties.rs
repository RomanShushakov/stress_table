use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/interface_to_communicate_with_properties.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addMaterialToProperties, catch)]
    pub fn add_material_to_properties(action_id: u32, name: String, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;
}
