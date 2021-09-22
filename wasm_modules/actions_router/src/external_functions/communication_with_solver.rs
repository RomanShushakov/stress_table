use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/interface_to_communicate_with_fe_model.js")]
extern "C"
{
    #[wasm_bindgen(js_name = submitJob, catch)]
    pub fn submit_job(job_name: &str) -> Result<(), JsValue>;
}
