use wasm_bindgen::prelude::*;
use serde_json::json;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;

use crate::preprocessor::traits::ClearByActionIdTrait;

use crate::preprocessor::geometry::geometry::Geometry;
use crate::preprocessor::geometry::line::{Line, DeletedLine};
use crate::preprocessor::geometry::consts::
{
    ADD_LINE_EVENT_NAME, UPDATE_LINE_EVENT_NAME, DELETE_LINE_EVENT_NAME,
};

use crate::types::{FEUInt};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};


impl<T, V> Geometry<T, V>
    where T: Debug + Copy + Serialize + Hash + Eq + PartialOrd,
          V: Debug + Copy,
{
    pub fn add_line(&mut self, action_id: T, number: T, start_point_number: T, end_point_number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if self.lines.contains_key(&number)
        {
            let error_message = &format!("Geometry: Add line action: Line with \
                number {:?} does already exist!", number);
            return Err(JsValue::from(error_message));
        }

        if start_point_number == end_point_number
        {
            let error_message = "Geometry: Add line action: The start and end point \
                numbers should not be the same!";
            return Err(JsValue::from(error_message));
        }

        if self.lines.values().position(|line|  line.start_and_end_points_same(
            start_point_number, end_point_number)).is_some()
        {
            let error_message = &format!("Geometry: Add line action: Line with \
                point number {:?} and {:?} does already exist!", start_point_number,
                end_point_number);
            return Err(JsValue::from(error_message));
        }

        if !self.points.contains_key(&start_point_number)
        {
            let error_message = &format!("Geometry: Add line action: Point with \
                number {:?} does not exist!", start_point_number);
            return Err(JsValue::from(error_message));
        }

        if !self.points.contains_key(&end_point_number)
        {
            let error_message = &format!("Geometry: Add line action: Point with \
                number {:?} does not exist!", end_point_number);
            return Err(JsValue::from(error_message));
        }

        let line = Line::create( start_point_number, end_point_number);
        self.lines.insert(number, line);
        let detail = json!({ "line_data": { "number": number,
            "start_point_number": start_point_number, "end_point_number": end_point_number },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_LINE_EVENT_NAME, EVENT_TARGET)?;
        self.logging();
        Ok(())
    }


    pub fn update_line(&mut self, action_id: T, number: T, start_point_number: T,
        end_point_number: T, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if start_point_number == end_point_number
        {
            let error_message = "Geometry: Update line action: The start and end point \
                numbers should not be the same!";
            return Err(JsValue::from(error_message));
        }

        if self.lines.values().position(|line| line.start_and_end_points_same(
            start_point_number, end_point_number)).is_some()
        {
            let error_message = &format!("Geometry: Update line action: Line with \
                point number {:?} and {:?} does already exist!", start_point_number,
                end_point_number);
            return Err(JsValue::from(error_message));
        }
        if !self.points.contains_key(&start_point_number)
        {
            let error_message = &format!("Geometry: Update line action: Point with \
                number {:?} does not exist!", start_point_number);
            return Err(JsValue::from(error_message));
        }
        if !self.points.contains_key(&end_point_number)
        {
            let error_message = &format!("Geometry: Update line action: Point with \
                number {:?} does not exist!", end_point_number);
            return Err(JsValue::from(error_message));
        }
        if let Some(line) = self.lines.get_mut(&number)
        {
            line.update(start_point_number, end_point_number);
            let detail = json!({ "line_data": { "number": number,
                "start_point_number": start_point_number, "end_point_number": end_point_number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_LINE_EVENT_NAME, EVENT_TARGET)?;
        }
        else
        {
            let error_message = &format!("Geometry: Update line action: Line with \
                number {:?} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
        self.logging();
        Ok(())
    }


    pub fn delete_line(&mut self, action_id: T, number: T, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if let Some((line_number, line)) = self.lines.remove_entry(&number)
        {
            let deleted_line = DeletedLine::create(line_number, line);
            self.deleted_lines.insert(action_id, vec![deleted_line]);
            let detail = json!({ "line_data": { "number": number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_LINE_EVENT_NAME, EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Geometry: Delete line action: Line with \
                number {:?} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_line(&mut self, action_id: T, number: T, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        if let Some(deleted_lines) = self.deleted_lines.remove(&action_id)
        {
            if deleted_lines.is_empty() || deleted_lines.len() > 1
            {
                let error_message = &format!("Geometry: Restore line action: Incorrect \
                    number of lines");
                return Err(JsValue::from(error_message));
            }
            if !deleted_lines[0].number_same(number) || self.lines.contains_key(&number)
            {
                let error_message = &format!("Geometry: Restore line action: Incorrect \
                    line number");
                return Err(JsValue::from(error_message));
            }
            let (number, start_point_number, end_point_number) = deleted_lines[0]
                .extract_number_and_points_numbers();
            self.lines.insert(number, Line::create(start_point_number, end_point_number));
            let detail = json!({ "line_data": { "number": number,
                "start_point_number": start_point_number, "end_point_number": end_point_number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_LINE_EVENT_NAME, EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Geometry: Restore line action: Line with \
                number {:?} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn extract_line_info_from_geometry(&mut self, number: T) -> Result<(T, T), JsValue>
    {
        return if let Some(line) = self.lines.get(&number)
        {
            let (start_point_number, end_point_number) = line.extract_points_numbers();
            Ok((start_point_number, end_point_number))
        }
        else
        {
            let error_message = &format!("Geometry: Show line info action: Line with \
                number {:?} does not exist!", number);
            Err(JsValue::from(error_message))
        }
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
