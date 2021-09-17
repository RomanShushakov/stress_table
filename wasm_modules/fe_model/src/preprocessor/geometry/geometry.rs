use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::traits::ClearByActionIdTrait;

use crate::preprocessor::geometry::point::{Point, DeletedPoint};
use crate::preprocessor::geometry::line::{Line, DeletedLine};

use crate::functions::log;


pub struct Geometry<T, V>
{
    pub points: HashMap<T, Point<V>>,                       // { point_number: Point }
    pub lines: HashMap<T, Line<T>>,                         // { line_number: Line }
    pub deleted_points: HashMap<T, DeletedPoint<T, V>>,     // { action_id: DeletedPoint }
    pub deleted_lines: HashMap<T, Vec<DeletedLine<T>>>,     // { action_id: Vec<DeletedLine> }
}



impl<T, V> Geometry<T, V>
    where T: Debug + Eq + Hash + Copy + PartialOrd,
          V: Debug + Copy,
{
    pub fn create() -> Self
    {
        let points = HashMap::new();
        let lines = HashMap::new();
        let deleted_points = HashMap::new();
        let deleted_lines = HashMap::new();
        Geometry { points, lines, deleted_points, deleted_lines }
    }


    pub fn clear_deleted_lines_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_lines.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_lines.remove(action_id);
        }
    }


    pub fn clear_deleted_points_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_points.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_points.remove(action_id);
        }
    }


    pub fn check_for_line_numbers_existence(&self, line_numbers: &[T],
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
            Points: {:?}, \n
            Deleted points: {:?}, \n
            Lines: {:?}, \n
            Deleted lines {:?} \n",
            self.points,
            self.deleted_points,
            self.lines,
            self.deleted_lines));
    }
}


impl<T, V> ClearByActionIdTrait<T> for Geometry<T, V>
    where T: Debug + Copy + Eq + Hash + PartialOrd,
          V: Debug + Copy,
{
    fn clear_by_action_id(&mut self, action_id: T)
    {
        self.clear_deleted_lines_by_action_id(action_id);
        self.clear_deleted_points_by_action_id(action_id);
    }
}
