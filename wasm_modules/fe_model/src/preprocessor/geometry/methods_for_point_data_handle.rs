use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::geometry::geometry::Geometry;
use crate::preprocessor::geometry::point::{Point, DeletedPoint};
use crate::preprocessor::geometry::line::{Line, DeletedLine};
use crate::preprocessor::geometry::consts::
{
    ADD_POINT_EVENT_NAME, UPDATE_POINT_EVENT_NAME, DELETE_POINT_EVENT_NAME,
    ADD_LINE_EVENT_NAME, DELETE_LINE_EVENT_NAME,
    RESTORED_LINE_NUMBERS_MESSAGE_HEADER,
};

use crate::types::{FEUInt, FEFloat};

use crate::functions::{log, dispatch_custom_event};
use crate::consts::EVENT_TARGET;


impl Geometry
{
    pub fn add_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(action_id);
        self.clear_deleted_points_by_action_id(action_id);

        if self.points.contains_key(&number)
        {
            let error_message = &format!("Geometry: Add point action: Point with \
                number {} does already exist!", number);
            return Err(JsValue::from(error_message));
        }
        if self.points.values().position(|point|
            point.coordinates_same(x, y, z)).is_some()
        {
            let error_message = &format!("Geometry: Add point action: Point with \
                coordinates {}, {}, {} does already exist!", x, y, z);
            return Err(JsValue::from(error_message));
        }
        let point = Point::create(x, y, z);
        self.points.insert(number, point);
        let detail = json!({ "point_data": { "number": number, "x": x, "y": y, "z": z },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_POINT_EVENT_NAME, EVENT_TARGET)?;
        log(&format!("Geometry: Points: {:?}, Deleted points: {:?}, Lines: {:?}, \
            Deleted lines {:?}", self.points, self.deleted_points, self.lines, self.deleted_lines));
        Ok(())
    }


    pub fn update_point(&mut self, action_id: FEUInt, number: FEUInt, x: FEFloat, y: FEFloat,
        z: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(action_id);
        self.clear_deleted_points_by_action_id(action_id);

        if self.points.values().position(|point| point.coordinates_same(x, y, z)).is_some()
        {
            let error_message = &format!("Geometry: Update point action: Point with \
                coordinates {}, {}, {} does already exist!", x, y, z);
            return Err(JsValue::from(error_message));
        }

        if let Some(point) = self.points.get_mut(&number)
        {
            point.update(x, y, z);
            let detail = json!({ "point_data": { "number": number, "x": x, "y": y, "z": z },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_POINT_EVENT_NAME, EVENT_TARGET)?;
            log(&format!("Geometry: Points: {:?}, Deleted points: {:?}, Lines: {:?}, \
                Deleted lines {:?}", self.points, self.deleted_points, self.lines,
                self.deleted_lines));
            Ok(())
        }
        else
        {
            let error_message = format!("Geometry: Update point action: \
                The point with number {} could not be updated because it does not exist!", number);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn extract_line_numbers_for_delete(&self, point_number: FEUInt) -> Vec<FEUInt>
    {
        let mut line_numbers_for_delete = Vec::new();
        for (line_number, line) in self.lines.iter()
        {
            let (start_point_number, end_point_number) = line.extract_points_numbers();
            if start_point_number == point_number || end_point_number == point_number
            {
                line_numbers_for_delete.push(*line_number);
            }
        }
        line_numbers_for_delete
    }


    pub fn delete_point(&mut self, action_id: FEUInt, number: FEUInt,
        line_numbers_for_delete: &Vec<u32>, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(action_id);
        self.clear_deleted_points_by_action_id(action_id);

        let mut deleted_lines = Vec::new();

        for line_number in line_numbers_for_delete
        {
            let line = self.lines.remove(line_number).unwrap();
            let deleted_line = DeletedLine::create(*line_number, line);
            deleted_lines.push(deleted_line);
            let detail = json!({ "line_data": { "number": line_number },
                "is_action_id_should_be_increased": false });
            dispatch_custom_event(detail, DELETE_LINE_EVENT_NAME, EVENT_TARGET)?;
        }
        if !deleted_lines.is_empty()
        {
            self.deleted_lines.insert(action_id, deleted_lines);
        }

        if let Some((point_number, point)) = self.points.remove_entry(&number)
        {
            let deleted_point = DeletedPoint::create(point_number, point);
            self.deleted_points.insert(action_id, deleted_point);
            let detail = json!({ "point_data": { "number": number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_POINT_EVENT_NAME, EVENT_TARGET)?;
            log(&format!("Geometry: Points: {:?}, Deleted points: {:?}, Lines: {:?}, \
                Deleted lines {:?}", self.points, self.deleted_points, self.lines,
                self.deleted_lines));
            Ok(())
        }
        else
        {
            let error_message = &format!("Geometry: Delete point action: Point with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_point(&mut self, action_id: FEUInt, number: FEUInt,
        is_action_id_should_be_increased: bool) -> Result<JsValue, JsValue>
    {
        if let Some(deleted_point) = self.deleted_points.remove(&action_id)
        {
            let (deleted_point_number, x, y, z) =
                deleted_point.extract_number_and_coordinates();
            if deleted_point_number != number
            {
                let error_message = &format!("Geometry: Restore point action: Point with \
                    number {} does not exist!", number);
                return Err(JsValue::from(error_message));
            }
            let detail = json!({ "point_data": { "number": deleted_point_number,
                    "x": x, "y": y, "z": z },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_POINT_EVENT_NAME, EVENT_TARGET)?;
            self.points.insert(deleted_point_number, Point::create(x, y, z));

            let mut restored_line_numbers = Vec::new();
            if let Some(deleted_lines) = self.deleted_lines.remove(&action_id)
            {
                for deleted_line in &deleted_lines
                {
                    let (number, start_point_number, end_point_number) =
                        deleted_line.extract_number_and_points_numbers();
                    let detail = json!({ "line_data": { "number": number,
                            "start_point_number": start_point_number,
                            "end_point_number": end_point_number },
                        "is_action_id_should_be_increased": is_action_id_should_be_increased });
                    dispatch_custom_event(detail, ADD_LINE_EVENT_NAME, EVENT_TARGET)?;
                    self.lines.insert(deleted_line.extract_number(),
                        Line::create(start_point_number, end_point_number));
                    restored_line_numbers.push(number);
                }
            }
            let composed_restored_line_numbers =
                json!({ RESTORED_LINE_NUMBERS_MESSAGE_HEADER: restored_line_numbers });
            let converted_line_numbers = JsValue::from_serde(&composed_restored_line_numbers)
                .or(Err(JsValue::from("Geometry: Restore point info: Restored line numbers \
                    could not be composed!")))?;
            log(&format!("Geometry: Points: {:?}, Deleted points: {:?}, Lines: {:?}, \
                Deleted lines {:?}", self.points, self.deleted_points, self.lines,
                self.deleted_lines));
            Ok(converted_line_numbers)
        }
        else
        {
            let error_message = &format!("Geometry: Restore point action: Point with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn show_point_info(&mut self, number: FEUInt, handler: js_sys::Function) -> Result<(), JsValue>
    {
        return if let Some(point) = self.points.get(&number)
        {
            let (x, y, z) = point.extract_coordinates();
            let point_info_json = json!({ "point_data": { "number": number,
                "x": x, "y": y, "z": z } });
            let point_info = JsValue::from_serde(&point_info_json)
                .or(Err(JsValue::from("Geometry: Show point info: Point info could not be \
                    composed!")))?;
            let this = JsValue::null();
            let _ = handler.call1(&this, &point_info)?;
            Ok(())
        }
        else
        {
            let error_message = &format!("Geometry: Show point info action: Point with \
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
}
