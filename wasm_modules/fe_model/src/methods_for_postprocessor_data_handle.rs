use wasm_bindgen::prelude::*;

use crate::FEModel;

use crate::types::{FEUInt, FEFloat};


#[wasm_bindgen]
impl FEModel
{
    pub fn show_job_analysis_result(&mut self, job_name: &str, job_id: u32) -> Result<(), JsValue>
    {
        self.postprocessor.show_job_analysis_result(job_name, job_id)
    }


    pub fn delete_job(&mut self, job_name: &str) -> Result<(), JsValue>
    {
        self.postprocessor.delete_analysis_result(job_name)
    }


    pub fn extract_job_ids(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.postprocessor.extract_job_ids(handler)
    }
}