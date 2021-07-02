use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::geometry::geometry::Geometry;
use crate::preprocessor::geometry::line::{Line, DeletedLine};
use crate::preprocessor::geometry::consts::
{
    ADD_LINE_EVENT_NAME, UPDATE_LINE_EVENT_NAME, DELETE_LINE_EVENT_NAME,
    DELETED_LINE_NUMBERS_MESSAGE_HEADER, RESTORED_LINE_NUMBERS_MESSAGE_HEADER
};

use crate::types::{FEUInt};

use crate::consts::EVENT_TARGET;

use crate::functions::{log, dispatch_custom_event};


impl Geometry
{
    pub fn add_line(&mut self, action_id: FEUInt, number: FEUInt, start_point_number: FEUInt,
        end_point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(action_id);

        if self.lines.contains_key(&number)
        {
            let error_message = &format!("Geometry: Add line action: Line with \
                number {} does already exist!", number);
            return Err(JsValue::from(error_message));
        }

        if self.lines.values().position(|line|  line.start_and_end_points_same(
            start_point_number, end_point_number)).is_some()
        {
            let error_message = &format!("Geometry: Add line action: Line with \
                point number {} and {} does already exist!", start_point_number, end_point_number);
            return Err(JsValue::from(error_message));
        }
        let start_point_number =
            {
                if self.points.contains_key(&start_point_number)
                {
                    Ok(start_point_number)
                }
                else
                {
                    let error_message = &format!("Geometry: Add line action: Point with \
                        number {} does not exist!", start_point_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        let end_point_number =
            {
                if self.points.contains_key(&end_point_number)
                {
                    Ok(end_point_number)
                }
                else
                {
                    let error_message = &format!("Geometry: Add line action: Point with \
                        number {} does not exist!", end_point_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        let line = Line::create( start_point_number, end_point_number);
        self.lines.insert(number, line);
        let detail = json!({ "line_data": { "number": number,
            "start_point_number": start_point_number, "end_point_number": end_point_number },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_LINE_EVENT_NAME, EVENT_TARGET)?;
        log(&format!("Geometry: Points: {:?}, Deleted points: {:?}, Lines: {:?}, \
            Deleted lines {:?}", self.points, self.deleted_points, self.lines, self.deleted_lines));
        Ok(())
    }


    pub fn update_line(&mut self, action_id: FEUInt, number: FEUInt, start_point_number: FEUInt,
        end_point_number: FEUInt, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(action_id);

        if self.lines.values().position(|line| line.start_and_end_points_same(
            start_point_number, end_point_number)).is_some()
        {
            let error_message = &format!("Geometry: Update line action: Line with \
                point number {} and {} does already exist!", start_point_number, end_point_number);
            return Err(JsValue::from(error_message));
        }
        if !self.points.contains_key(&start_point_number)
        {
            let error_message = &format!("Geometry: Update line action: Point with \
                number {} does not exist!", start_point_number);
            return Err(JsValue::from(error_message));
        }
        if !self.points.contains_key(&end_point_number)
        {
            let error_message = &format!("Geometry: Update line action: Point with \
                number {} does not exist!", end_point_number);
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
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
        log(&format!("Geometry: Points: {:?}, Deleted points: {:?}, Lines: {:?}, \
            Deleted lines {:?}", self.points, self.deleted_points, self.lines, self.deleted_lines));
        Ok(())
    }


    pub fn delete_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        self.clear_deleted_lines_by_action_id(action_id);

        if let Some((line_number, line)) = self.lines.remove_entry(&number)
        {
            let deleted_line = DeletedLine::create(line_number, line);
            self.deleted_lines.insert(action_id, vec![deleted_line]);
            let detail = json!({ "line_data": { "number": number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_LINE_EVENT_NAME, EVENT_TARGET)?;
            let deleted_line_numbers = vec![number];
            let composed_deleted_line_numbers =
                json!({ DELETED_LINE_NUMBERS_MESSAGE_HEADER: deleted_line_numbers });
            let converted_line_numbers = JsValue::from_serde(&composed_deleted_line_numbers)
                .or(Err(JsValue::from("Geometry: Delete line info: Deleted line numbers \
                    could not be composed!")))?;
            log(&format!("Geometry: Points: {:?}, Deleted points: {:?}, Lines: {:?}, \
                Deleted lines {:?}", self.points, self.deleted_points, self.lines,
                self.deleted_lines));
            Ok(converted_line_numbers)
        }
        else
        {
            let error_message = &format!("Geometry: Delete line action: Line with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_line(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
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
            let restored_line_numbers = vec![number];
            let composed_restored_line_numbers =
                json!({ RESTORED_LINE_NUMBERS_MESSAGE_HEADER: restored_line_numbers });
            let converted_line_numbers = JsValue::from_serde(&composed_restored_line_numbers)
                .or(Err(JsValue::from("Geometry: Restore line info: Restored line numbers \
                    could not be composed!")))?;
            log(&format!("Geometry: Points: {:?}, Deleted points: {:?}, Lines: {:?}, \
                Deleted lines {:?}", self.points, self.deleted_points, self.lines,
                self.deleted_lines));
            Ok(converted_line_numbers)
        }
        else
        {
            let error_message = &format!("Geometry: Restore line action: Line with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }
}
