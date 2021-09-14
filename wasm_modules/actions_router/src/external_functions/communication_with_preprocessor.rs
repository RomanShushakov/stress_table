use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/interface_to_communicate_with_fe_model.js")]
extern "C"
{
    #[wasm_bindgen(js_name = checkModel, catch)]
    pub fn check_model() -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = analyzeModel, catch)]
    pub fn analyze_model() -> Result<(), JsValue>;
}
