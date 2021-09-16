use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/interface_to_communicate_with_fe_model.js")]
extern "C"
{
    #[wasm_bindgen(js_name = submitJob, catch)]
    pub fn submit_job(job_name: &str) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = showJobAnalysisResult, catch)]
    pub fn show_job_analysis_result(job_name: &str) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = deleteJob, catch)]
    pub fn delete_job(job_name: &str) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = extractJobNames)]
    pub fn extract_job_names(handler: js_sys::Function);
}
