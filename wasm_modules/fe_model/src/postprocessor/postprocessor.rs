use wasm_bindgen::JsValue;
use serde_json::json;
use std::collections::HashMap;
use rand;

use crate::postprocessor::analysis_result::AnalysisResult;

use crate::postprocessor::consts::
{
    ADD_ANALYSIS_RESULT_EVENT_NAME, DELETE_ANALYSIS_RESULT_EVENT_NAME
};

use crate::consts::EVENT_TARGET;

use crate::functions::dispatch_custom_event;


pub struct Postprocessor<T, V>
{
    analysis_results: HashMap<String, AnalysisResult<T, V>>,    // { job_name, AnalysisResult }
    job_ids: HashMap<String, u32>,                              // { job_name, job_id }
}


impl<T, V> Postprocessor<T, V>
{
    pub fn create() -> Self
    {
        let analysis_results = HashMap::new();
        let job_ids = HashMap::new();
        Postprocessor { analysis_results, job_ids }
    }


    // pub fn add_analysis_result(&mut self, job_name: &str, analysis_result: AnalysisResult<T, V>)
    //     -> Result<(), JsValue>
    pub fn add_analysis_result(&mut self, job_name: &str)
        -> Result<(), JsValue>
    {
        let job_id = rand::random::<u32>();
        // self.analysis_results.insert(job_name.to_owned(), analysis_result);
        self.job_ids.insert(job_name.to_owned(), job_id);
        let detail = json!({ "analysis_result_data":
            { "job_name": job_name, "job_id": job_id } });
        dispatch_custom_event(detail, ADD_ANALYSIS_RESULT_EVENT_NAME,
            EVENT_TARGET)?;
        Ok(())
    }


    pub fn show_job_analysis_result(&mut self, job_name: &str, job_id: u32) -> Result<(), JsValue>
    {
        if let Some(id) = self.job_ids.get(job_name)
        {
            if *id != job_id
            {
                let error_message = &format!("Postprocessor: Show analysis result action: \
                Job id's for job name {} do not match!", job_name);
                return Err(JsValue::from(error_message));
            }

            // if self.analysis_results.contains_key(job_name)
            // {
            //     Ok(())
            // }
            // else
            // {
            //      let error_message = &format!("Postprocessor: Show analysis result action: \
            //         Analysis result for job with name {} does not exist!", job_name);
            //     return Err(JsValue::from(error_message));
            // }
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
        // if self.analysis_results.remove(job_name).is_some() &&
        //     self.job_ids.remove(job_name).is_some()
        if self.job_ids.remove(job_name).is_some()
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


    pub fn extract_job_ids(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_job_ids = json!({ "extracted_job_ids": self.job_ids });
        let composed_extracted_job_ids =
            JsValue::from_serde(&extracted_job_ids)
                .or(Err(JsValue::from("Postprocessor: Extract job ids: Job ids could \
                    not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_job_ids);
        Ok(())
    }
}
