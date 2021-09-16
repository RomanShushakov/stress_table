use wasm_bindgen::JsValue;
use serde_json::json;
use std::collections::HashMap;

use crate::postprocessor::consts::
{
    ADD_ANALYSIS_RESULT_EVENT_NAME, DELETE_ANALYSIS_RESULT_EVENT_NAME
};

use crate::consts::EVENT_TARGET;

use crate::functions::dispatch_custom_event;


pub struct AnalysisResult;


impl AnalysisResult
{
    pub fn create() -> Self
    {
        AnalysisResult
    }
}


pub struct Postprocessor
{
    analysis_results: HashMap<String, AnalysisResult>
}


impl Postprocessor
{
    pub fn create() -> Self
    {
        let analysis_results = HashMap::new();
        Postprocessor { analysis_results }
    }


    pub fn add_analysis_result(&mut self, job_name: &str, analysis_result: AnalysisResult)
        -> Result<(), JsValue>
    {
        self.analysis_results.insert(job_name.to_owned(), analysis_result);
        let detail = json!({ "analysis_result_data": { "job_name": job_name } });
        dispatch_custom_event(detail, ADD_ANALYSIS_RESULT_EVENT_NAME,
            EVENT_TARGET)?;
        Ok(())
    }


    pub fn show_job_analysis_result(&mut self, job_name: &str) -> Result<(), JsValue>
    {
        if self.analysis_results.contains_key(job_name)
        {
            Ok(())
        }
        else
        {
            let error_message = &format!("Postprocessor: Show analysis result action: \
                Analysis result for job with name {} does not exist!", job_name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn delete_analysis_result(&mut self, job_name: &str) -> Result<(), JsValue>
    {
        if self.analysis_results.remove(job_name).is_some()
        {
            let detail = json!({ "analysis_result_data": { "job_name": job_name } });
            dispatch_custom_event(detail, DELETE_ANALYSIS_RESULT_EVENT_NAME,
                EVENT_TARGET)?;
            Ok(())
        }
        else
        {
            let error_message = &format!("Postprocessor: Delete analysis result action: \
                Analysis result for job with name {} does not exist!", job_name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn extract_job_names(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let mut job_names = Vec::new();
        for job_name in self.analysis_results.keys()
        {
            job_names.push(job_name);
        }
        let extracted_job_names = json!({ "extracted_job_names": job_names });
        let composed_extracted_job_names =
            JsValue::from_serde(&extracted_job_names)
                .or(Err(JsValue::from("Postprocessor: Extract job names: Job names could \
                    not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_job_names);
        Ok(())
    }
}
