use wasm_bindgen::prelude::*;
use serde_json::json;
use std::collections::HashMap;

use crate::preprocessor::geometry::point::{Point, DeletedPoint};
use crate::preprocessor::geometry::line::{Line, DeletedLine};


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


    pub fn show_point_info(&mut self, number: FEUInt) -> Result<JsValue, JsValue>
    {
        return if let Some(point) = self.points.get(&number)
        {
            let (x, y, z) = point.extract_coordinates();
            let point_info_json = json!({ "point_data": { "number": number,
                "x": x, "y": y, "z": z } });
            let point_info = JsValue::from_serde(&point_info_json)
                .or(Err(JsValue::from("Geometry: Show point info: Point info could not be \
                    composed!")))?;
            Ok(point_info)
        }
        else
        {
            let error_message = &format!("Geometry: Show point info action: Point with \
                number {} does not exist!", number);
            Err(JsValue::from(error_message))
        }
    }


    pub fn show_line_info(&mut self, number: FEUInt) -> Result<JsValue, JsValue>
    {
        return if let Some(line) = self.lines.get(&number)
        {
            let (start_point_number, end_point_number) = line.extract_points_numbers();
            let line_info_json = json!({ "line_data": { "number": number,
                "start_point_number": start_point_number,
                "end_point_number": end_point_number } });
            let line_info = JsValue::from_serde(&line_info_json)
                .or(Err(JsValue::from("Geometry: Show line info: Line info could not be \
                    composed!")))?;
            Ok(line_info)
        }
        else
        {
            let error_message = &format!("Geometry: Show line info action: Line with \
                number {} does not exist!", number);
            Err(JsValue::from(error_message))
        }
    }


    pub fn extract_points(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_points = json!({ "extracted_points": self.points });
        let composed_extracted_points =
            JsValue::from_serde(&extracted_points)
                .or(Err(JsValue::from("Geometry: Extract points: Points could not be \
                    composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_points);
        Ok(())
    }


    pub fn extract_lines(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_lines = json!({ "extracted_lines": self.lines });
        let composed_extracted_lines =
            JsValue::from_serde(&extracted_lines)
                .or(Err(JsValue::from("Geometry: Extract lines: Lines could not be \
                    composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_lines);
        Ok(())
    }
}
