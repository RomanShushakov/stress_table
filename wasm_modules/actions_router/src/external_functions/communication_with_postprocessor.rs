use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/interface_to_communicate_with_fe_model.js")]
extern "C"
{
    #[wasm_bindgen(js_name = showJobAnalysisResult, catch)]
    pub fn show_job_analysis_result(job_name: &str, job_id: u32) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = deleteJob, catch)]
    pub fn delete_job(job_name: &str) -> Result<(), JsValue>;


    #[wasm_bindgen(js_name = extractJobIds)]
    pub fn extract_job_ids(handler: js_sys::Function);
}