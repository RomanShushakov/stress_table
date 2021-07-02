use wasm_bindgen::prelude::*;

mod preprocessor;
use preprocessor::preprocessor::Preprocessor;

mod fe_solver;
use fe_solver::fe_solver::FESolver;

mod postprocessor;
use postprocessor::postprocessor::Postprocessor;

mod types;
use types::{FEUInt, FEFloat};

mod consts;

mod functions;


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

    pub fn clear_geometry_module_by_action_id(&mut self, action_id: FEUInt)
    {
        self.preprocessor.geometry.clear_geometry_module_by_action_id(action_id);
    }


    pub fn show_point_info(&mut self, number: FEUInt) -> Result<JsValue, JsValue>
    {
        self.preprocessor.geometry.show_point_info(number)
    }


    pub fn show_line_info(&mut self, number: FEUInt) -> Result<JsValue, JsValue>
    {
        self.preprocessor.geometry.show_line_info(number)
    }


    pub fn extract_points(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.geometry.extract_points(handler)
    }


    pub fn extract_lines(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        self.preprocessor.geometry.extract_lines(handler)
    }


    pub fn add_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.geometry.add_point(action_id, number, x, y, z,
            is_action_id_should_be_increased)
    }


    pub fn update_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.geometry.update_point(action_id, number, x, y, z,
            is_action_id_should_be_increased)
    }


    pub fn delete_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.preprocessor.geometry.delete_point(action_id, number, is_action_id_should_be_increased)
    }


    pub fn restore_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.preprocessor.geometry.restore_point(action_id, number, is_action_id_should_be_increased)
    }


    pub fn add_line(&mut self, action_id: FEUInt, number: FEUInt, start_point_number: FEUInt,
        end_point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.geometry.add_line(action_id, number, start_point_number,
            end_point_number, is_action_id_should_be_increased)
    }


    pub fn update_line(&mut self, action_id: FEUInt, number: FEUInt, start_point_number: FEUInt,
        end_point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.preprocessor.geometry.update_line(action_id, number, start_point_number,
            end_point_number, is_action_id_should_be_increased)
    }


    pub fn delete_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.preprocessor.geometry.delete_line(action_id, number, is_action_id_should_be_increased)
    }


    pub fn restore_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.preprocessor.geometry.restore_line(action_id, number, is_action_id_should_be_increased)
    }
}
