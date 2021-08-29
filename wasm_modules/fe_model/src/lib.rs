use wasm_bindgen::prelude::*;

mod preprocessor;
use preprocessor::preprocessor::Preprocessor;

mod fe_solver;
use fe_solver::fe_solver::FESolver;

mod postprocessor;
use postprocessor::postprocessor::Postprocessor;

mod types;

mod consts;

mod functions;

mod methods_for_preprocessor_data_handle;

use types::{FEUInt, FEFloat};
use consts::TOLERANCE;


#[wasm_bindgen]
pub struct FEModel
{
    preprocessor: Preprocessor<FEUInt, FEFloat>,
    fe_solver: FESolver<FEUInt, FEFloat>,
    postprocessor: Postprocessor,
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
}
