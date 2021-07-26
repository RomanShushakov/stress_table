use wasm_bindgen::prelude::*;
use std::collections::HashMap;

use crate::preprocessor::traits::ClearByActionIdTrait;

use crate::preprocessor::geometry::point::{Point, DeletedPoint};
use crate::preprocessor::geometry::line::{Line, DeletedLine};

use crate::functions::log;

use crate::types::FEUInt;


pub struct Geometry
{
    pub points: HashMap<FEUInt, Point>,    // { point_number: Point }
    pub lines: HashMap<FEUInt, Line>,  // { line_number: Line }
    pub deleted_points: HashMap<FEUInt, DeletedPoint>, // { action_id: DeletedPoint }
    pub deleted_lines: HashMap<FEUInt, Vec<DeletedLine>>,  // { action_id: Vec<DeletedLine> }
}



impl Geometry
{
    pub fn create() -> Geometry
    {
        let points = HashMap::new();
        let lines = HashMap::new();
        let deleted_points = HashMap::new();
        let deleted_lines = HashMap::new();
        Geometry { points, lines, deleted_points, deleted_lines }
    }


    pub fn clear_deleted_lines_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.deleted_lines.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.deleted_lines.remove(action_id);
        }
    }


    pub fn clear_deleted_points_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.deleted_points.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.deleted_points.remove(action_id);
        }
    }


    pub fn clear_geometry_module_by_action_id(&mut self, action_id: FEUInt)
    {
        self.clear_deleted_lines_by_action_id(action_id);
        self.clear_deleted_points_by_action_id(action_id);
    }


    pub fn check_for_line_numbers_existence(&self, line_numbers: &[FEUInt],
        error_message_header: &str) -> Result<(), JsValue>
    {
        for line_number in line_numbers
        {
            if !self.lines.contains_key(line_number)
            {
                let error_message = format!("{}: At least one line number from selected \
                    line numbers does not exist!", error_message_header);
                return Err(JsValue::from(error_message));
            }
        }
        Ok(())

    }


    pub fn logging(&self)
    {
        log(&format!("Geometry: \n
            Points: {:?}, Deleted points: {:?}, \n
            Lines: {:?}, Deleted lines {:?} \n", self.points, self.deleted_points, self.lines,
            self.deleted_lines));
    }
}


impl ClearByActionIdTrait for Geometry
{
    fn clear_by_action_id(&mut self, action_id: FEUInt)
    {
        self.clear_deleted_lines_by_action_id(action_id);
        self.clear_deleted_points_by_action_id(action_id);
    }
}
