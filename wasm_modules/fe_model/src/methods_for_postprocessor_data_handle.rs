use wasm_bindgen::prelude::*;

use crate::FEModel;

use crate::types::{FEUInt, FEFloat};


#[wasm_bindgen]
impl FEModel
{
    pub fn show_job_analysis_result(&mut self, job_name: &str) -> Result<(), JsValue>
    {
        self.postprocessor.show_job_analysis_result(job_name)
    }


    pub fn delete_job(&mut self, job_name: &str) -> Result<(), JsValue>
    {
        self.postprocessor.delete_analysis_result(job_name)
    }


    pub fn extract_job_names(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.postprocessor.extract_job_names(handler)
    }
}