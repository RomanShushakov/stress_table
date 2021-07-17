use wasm_bindgen::prelude::*;

mod preprocessor;
use preprocessor::preprocessor::Preprocessor;

mod fe_solver;
use fe_solver::fe_solver::FESolver;

mod postprocessor;
use postprocessor::postprocessor::Postprocessor;

mod extended_matrix;

mod types;

mod consts;

mod functions;

mod methods_for_preprocessor_data_handle;


#[wasm_bindgen]
pub struct FEModel
{
    preprocessor: Preprocessor,
    fe_solver: FESolver,
    postprocessor: Postprocessor,
}


#[wasm_bindgen]
impl FEModel
{
    pub fn create() -> Self
    {
        let preprocessor = Preprocessor::create();
        let fe_solver = FESolver::create();
        let postprocessor = Postprocessor::create();
        FEModel { preprocessor, fe_solver, postprocessor }
    }
}
