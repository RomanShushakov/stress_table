use wasm_bindgen::prelude::*;
use std::collections::HashMap;

mod preprocessor;
use preprocessor::preprocessor::Preprocessor;
use preprocessor::properties::assigned_property::AssignedPropertyToLines;

mod fe_solver;
use fe_solver::fe_solver::FESolver;

mod postprocessor;
use postprocessor::postprocessor::Postprocessor;

mod traits;

mod types;
use types::{FEUInt, FEFloat};

mod consts;
use consts::TOLERANCE;

mod functions;

mod methods_for_preprocessor_data_handle;

mod methods_for_postprocessor_data_handle;


#[wasm_bindgen]
pub struct FEModel
{
    preprocessor: Preprocessor<FEUInt, FEFloat>,
    fe_solver: FESolver<FEUInt, FEFloat>,
    postprocessor: Postprocessor<FEUInt, FEFloat>,
}


#[wasm_bindgen]
impl FEModel
{
    pub fn create() -> Self
    {
        let preprocessor = Preprocessor::create(TOLERANCE);
        let fe_solver = FESolver::create(TOLERANCE);
        let postprocessor = Postprocessor::create();
        FEModel { preprocessor, fe_solver, postprocessor }
    }


    pub fn submit_job(&mut self, job_name: &str) -> Result<(), JsValue>
    {
        // let analysis_result =
        //     self.fe_solver.submit_job(&self.preprocessor)?;
        // self.postprocessor.add_analysis_result(job_name, analysis_result)
        self.postprocessor.add_analysis_result(job_name)
    }
}
