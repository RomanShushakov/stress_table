use wasm_bindgen::prelude::*;

mod preprocessor;
use preprocessor::preprocessor::Preprocessor;

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


    pub fn add_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.add_point(action_id, number, x, y, z, is_action_id_should_be_increased)?;
        self.fe_solver.add_node(action_id, number, x, y, z)?;
        Ok(())
    }


    pub fn update_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.update_point(action_id, number, x, y, z, is_action_id_should_be_increased)?;
        self.fe_solver.update_node(action_id, number, x, y, z)?;
        Ok(())
    }


    pub fn delete_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.delete_point(action_id, number, is_action_id_should_be_increased)?;
        self.fe_solver.delete_node(action_id, number)?;
        Ok(())
    }


    pub fn restore_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.restore_point(action_id, number, is_action_id_should_be_increased)?;
        self.fe_solver.restore_node(action_id, number)?;
        Ok(())
    }
}
