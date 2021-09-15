use wasm_bindgen::prelude::*;

use crate::FEModel;

use crate::types::{FEUInt, FEFloat};


#[wasm_bindgen]
impl FEModel
{
    pub fn extract_job_names(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.postprocessor.extract_job_names(handler)
    }
}