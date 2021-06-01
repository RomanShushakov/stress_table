use serde_json::json;
use wasm_bindgen::prelude::*;

use crate::{Geometry, Line, DeletedLine};
use crate::{EVENT_TARGET, ADD_LINE_EVENT_NAME, UPDATE_LINE_EVENT_NAME, DELETE_LINE_EVENT_NAME};
use crate::{log, dispatch_custom_event};


#[wasm_bindgen]
impl Geometry
{
    pub fn add_line(&mut self, action_id: u32, number: u32, start_point_number: u32,
        end_point_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
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
                    if let Some(lines_numbers) = self.points_in_lines
                        .get_mut(&start_point_number)
                    {
                        if lines_numbers.iter().position(|line_number|
                            *line_number == number).is_none()
                        {
                            lines_numbers.push(number);
                        }
                    }
                    else
                    {
                        self.points_in_lines.insert(start_point_number, vec![number]);
                    }
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
                    if let Some(lines_numbers) = self.points_in_lines
                        .get_mut(&end_point_number)
                    {
                        if lines_numbers.iter().position(|line_number|
                            *line_number == number).is_none()
                        {
                            lines_numbers.push(number);
                        }
                    }
                    else
                    {
                        self.points_in_lines.insert(end_point_number, vec![number]);
                    }
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
        log(&format!("Geometry: Points: {:?}, Lines: {:?}, Points in \
            lines: {:?}, Deleted points: {:?}, Deleted lines {:?}", self.points,
            self.lines, self.points_in_lines, self.deleted_points, self.deleted_lines));
        Ok(())
    }


    pub fn update_line(&mut self, action_id: u32, number: u32, start_point_number: u32,
        end_point_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(action_id);
        if self.lines.values().position(|line| line.start_and_end_points_same(
            start_point_number, end_point_number)).is_some()
        {
            let error_message = &format!("Geometry: Update line action: Line with \
                point number {} and {} does already exist!", start_point_number, end_point_number);
            return Err(JsValue::from(error_message));
        }
        let new_start_point_number =
            {
                if self.points.contains_key(&start_point_number)
                {

                    Ok(start_point_number)
                }
                else
                {
                    let error_message = &format!("Geometry: Update line action: Point with \
                        number {} does not exist!", start_point_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        let new_end_point_number =
            {
                if self.points.contains_key(&end_point_number)
                {
                    Ok(end_point_number)
                }
                else
                {
                    let error_message = &format!("Geometry: Update line action: Point with \
                        number {} does not exist!", end_point_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        if let Some(line) = self.lines.get_mut(&number)
        {
            let (previous_start_point_number, previous_end_point_number) =
                line.extract_points_numbers();

            if let Some(line_numbers) = self.points_in_lines
                .get_mut(&previous_start_point_number)
            {
                if let Some(position) = line_numbers.iter().position(|line_number|
                    *line_number == number)
                {
                    let _ = line_numbers.remove(position);
                }
            }
            if let Some(line_numbers) = self.points_in_lines
                .get_mut(&previous_end_point_number)
            {
                if let Some(position) = line_numbers.iter().position(|line_number|
                    *line_number == number)
                {
                    let _ = line_numbers.remove(position);
                }
            }
            if let Some(line_numbers) = self.points_in_lines
                .get_mut(&new_start_point_number)
            {
                if line_numbers.iter().position(|line_number|
                    *line_number == number).is_none()
                {
                    line_numbers.push(number);
                }
            }
            else
            {
                self.points_in_lines.insert(new_start_point_number, vec![number]);
            }
            if let Some(line_numbers) = self.points_in_lines
                .get_mut(&new_end_point_number)
            {
                if line_numbers.iter().position(|line_number|
                    *line_number == number).is_none()
                {
                    line_numbers.push(number);
                }
            }
            else
            {
                self.points_in_lines.insert(new_end_point_number, vec![number]);
            }
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
        log(&format!("Geometry: Points: {:?}, Lines: {:?}, Points in \
            lines: {:?}, Deleted points: {:?}, Deleted lines {:?}", self.points,
            self.lines, self.points_in_lines, self.deleted_points, self.deleted_lines));
        Ok(())
    }


    pub fn delete_line(&mut self, action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(action_id);
        if let Some((line_number, line)) = self.lines.remove_entry(&number)
        {
            let (start_point_number, end_point_number) = line.extract_points_numbers();
            if let Some(lines_numbers) =
                self.points_in_lines.get_mut(&start_point_number)
            {
                if let Some(position) = lines_numbers.iter().position(|line_number|
                    *line_number == number)
                {
                    lines_numbers.remove(position);
                }
            }
            if let Some(lines_numbers) =
                self.points_in_lines.get_mut(&end_point_number)
            {
                if let Some(position) = lines_numbers.iter().position(|line_number|
                    *line_number == number)
                {
                    lines_numbers.remove(position);
                }
            }
            let deleted_line = DeletedLine::create(line_number, line);
            self.deleted_lines.insert(action_id, vec![deleted_line]);
            let detail = json!({ "line_data": { "number": number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_LINE_EVENT_NAME, EVENT_TARGET)?;
            log(&format!("Geometry: Points: {:?}, Lines: {:?}, Points in \
                lines: {:?}, Deleted points: {:?}, Deleted lines {:?}", self.points,
                self.lines, self.points_in_lines, self.deleted_points, self.deleted_lines));
            Ok(())
        }
        else
        {
            let error_message = &format!("Geometry: Delete line action: Line with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn undo_delete_line(&mut self, action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_lines) = self.deleted_lines.remove(&action_id)
        {
            if deleted_lines.is_empty() || deleted_lines.len() > 1
            {
                let error_message = &format!("Geometry: Undo delete line action: Incorrect \
                    number of lines");
                return Err(JsValue::from(error_message));
            }
            if !deleted_lines[0].number_same(number) || self.lines.contains_key(&number)
            {
                let error_message = &format!("Geometry: Undo delete line action: Incorrect \
                    line number");
                return Err(JsValue::from(error_message));
            }
            let (number, start_point_number, end_point_number) = deleted_lines[0]
                .extract_number_and_points_numbers();
            if let Some(lines_numbers) = self.points_in_lines
                .get_mut(&start_point_number)
            {
                if lines_numbers.iter().position(|line_number| *line_number == number)
                    .is_none()
                {
                    lines_numbers.push(number);
                }
            }
            else
            {
                self.points_in_lines.insert(start_point_number, vec![number]);
            }
            if let Some(lines_numbers) = self.points_in_lines
                .get_mut(&end_point_number)
            {
                if lines_numbers.iter().position(|line_number| *line_number == number)
                    .is_none()
                {
                    lines_numbers.push(number);
                }
            }
            else
            {
                self.points_in_lines.insert(end_point_number, vec![number]);
            }
            let detail = json!({ "line_data": { "number": number,
                "start_point_number": start_point_number, "end_point_number": end_point_number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_LINE_EVENT_NAME, EVENT_TARGET)?;
            self.lines.insert(number, Line::create(start_point_number, end_point_number));
            Ok(())
        }
        else
        {
            let error_message = &format!("Geometry: Undo delete line action: Line with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }
}