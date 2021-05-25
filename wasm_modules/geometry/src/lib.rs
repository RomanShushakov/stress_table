use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const EVENT_TARGET: &str = "fea-app";

const ADD_POINT_EVENT_NAME: &str = "add_point_server_message";
const UPDATE_POINT_EVENT_NAME: &str = "update_point_server_message";
const DELETE_POINT_EVENT_NAME: &str = "delete_point_server_message";
const ADD_LINE_EVENT_NAME: &str = "add_line_server_message";
const UPDATE_LINE_EVENT_NAME: &str = "update_line_server_message";
const DELETE_LINE_EVENT_NAME: &str = "delete_line_server_message";


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}


fn dispatch_custom_event(detail: serde_json::Value, event_type: &str, query_selector: &str)
    -> Result<(), JsValue>
{
    let custom_event = web_sys::CustomEvent::new_with_event_init_dict(event_type,
        web_sys::CustomEventInit::new()
            .bubbles(true)
            .composed(true)
            .detail(&JsValue::from_serde(&detail).or(Err("Geometry: Dispatch event: \
                detail could not be converted into JsValue!"))?))
            .or(Err(JsValue::from("Geometry: Dispatch event: custom event could not be \
                constructed!")))?;
    web_sys::window().expect("no global `window` exists")
        .document().expect("should have a document on window")
        .query_selector(query_selector).or(Err(JsValue::from("Geometry: Dispatch event: No \
            element find by current selector!")))?
        .unwrap()
        .dyn_into::<web_sys::EventTarget>()
        .unwrap()
        .dispatch_event(&custom_event)?;
    Ok(())
}


#[derive(Debug, Copy, Clone)]
struct Point
{
    x: f64,
    y: f64,
    z: f64,
}


impl Point
{
    fn create(x: f64, y: f64, z: f64) -> Self
    {
        Point { x, y, z }
    }


    fn coordinates_same(&self, x: f64, y: f64, z: f64) -> bool
    {
        self.x == x && self.y == y && self.z == z
    }


    fn update(&mut self, x: f64, y: f64, z: f64)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }


    fn extract_coordinates(&self) -> (f64, f64, f64)
    {
        (self.x, self.y, self.z)
    }
}


#[derive(Debug, Copy, Clone)]
struct Line
{
    start_point_number: u32,
    end_point_number: u32,
}


impl Line
{
    fn create(start_point_number: u32, end_point_number: u32) -> Self
    {
        Line { start_point_number, end_point_number, }
    }


    fn start_and_end_points_same(&self, start_point_number: u32, end_point_number: u32) -> bool
    {
        (self.start_point_number == start_point_number &&
        self.end_point_number == end_point_number) ||
        (self.start_point_number == end_point_number &&
        self.end_point_number == start_point_number)
    }


    fn update(&mut self, start_point_number: u32, end_point_number: u32)
    {
        self.start_point_number = start_point_number;
        self.end_point_number = end_point_number;
    }


    fn extract_points_numbers(&self) -> (u32, u32)
    {
        (self.start_point_number, self.end_point_number)
    }
}


#[derive(Debug, Copy, Clone)]
struct DeletedPoint
{
    number: u32,
    point: Point,
}


impl DeletedPoint
{
    fn create(number: u32, point: Point) -> Self
    {
        DeletedPoint { number, point }
    }


    fn extract_number_and_coordinates(&self) -> (u32, f64, f64, f64)
    {
        let (x, y, z) = self.point.extract_coordinates();
        (self.number, x, y, z)
    }
}


#[derive(Debug, Copy, Clone)]
struct DeletedLine
{
    number: u32,
    line: Line,
}


impl DeletedLine
{
    fn create(number: u32, line: Line) -> Self
    {
        DeletedLine { number, line }
    }


    fn extract_number_and_points_numbers(&self) -> (u32, u32, u32)
    {
        let (start_point_number, end_point_number) = self.line.extract_points_numbers();
        (self.number, start_point_number, end_point_number)
    }


    fn extract_number(&self) -> u32
    {
        self.number
    }


    fn number_same(&self, number: u32) -> bool
    {
        self.number == number
    }
}



#[wasm_bindgen]
pub struct Geometry
{
    points: HashMap<u32, Point>,    // { point_number: Point }
    lines: HashMap<u32, Line>,  // { line_number: Line }
    points_in_lines: HashMap<u32, Vec<u32>>,    // { point_number: Vec<line_numbers> }
    deleted_points: HashMap<u32, DeletedPoint>, // { action_id: DeletedPoint }
    deleted_lines: HashMap<u32, Vec<DeletedLine>>,  // { action_id: DeletedLine }
}


#[wasm_bindgen]
impl Geometry
{
    pub fn create() -> Geometry
    {
        let points = HashMap::new();
        let lines = HashMap::new();
        let points_in_lines = HashMap::new();
        let deleted_points = HashMap::new();
        let deleted_lines = HashMap::new();
        Geometry { points, lines, points_in_lines, deleted_points, deleted_lines }
    }


    fn clear_deleted_lines_by_action_id(&mut self, action_id: u32)
    {
        for action_id in self.deleted_lines.clone()
            .keys()
            .filter(|deletion_action_id| **deletion_action_id >= action_id)
            .collect::<Vec<&u32>>()
            .iter()
        {
            let _ = self.deleted_lines.remove(action_id);
        }
    }


    fn clear_deleted_points_by_action_id(&mut self, action_id: u32)
    {
        for action_id in self.deleted_points.clone()
            .keys()
            .filter(|deletion_action_id| **deletion_action_id >= action_id)
            .collect::<Vec<&u32>>()
            .iter()
        {
            let _ = self.deleted_points.remove(action_id);
        }
    }


    pub fn add_point(&mut self, action_id: u32, number: u32, x: f64, y: f64, z: f64,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
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
        log(&format!("Geometry: Points: {:?}, Lines: {:?}, Points in \
            lines: {:?}, Deleted points: {:?}, Deleted lines {:?}", self.points,
            self.lines, self.points_in_lines, self.deleted_points, self.deleted_lines));
        Ok(())
    }


    pub fn update_point(&mut self, action_id: u32, number: u32, x: f64, y: f64, z: f64,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
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
            log(&format!("Geometry: Points: {:?}, Lines: {:?}, Points in \
                lines: {:?}, Deleted points: {:?}, Deleted lines {:?}", self.points,
                self.lines, self.points_in_lines, self.deleted_points, self.deleted_lines));
            Ok(())
        }
        else
        {
            let error_message = format!("Geometry: Update point action: \
                The point with number {} could not be updated because it does not exist!", number);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_point(&mut self, action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(action_id);
        self.clear_deleted_points_by_action_id(action_id);
        if let Some(lines_numbers) = self.points_in_lines.remove(&number)
        {
            let mut current_deleted_lines = Vec::new();
            for line_number in lines_numbers.iter()
            {
                let line = self.lines.remove(&line_number).unwrap();
                let deleted_line = DeletedLine::create(*line_number, line);
                current_deleted_lines.push(deleted_line);
                let detail = json!({ "line_data": { "number": line_number },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail, DELETE_LINE_EVENT_NAME, EVENT_TARGET)?;
            }
            if !current_deleted_lines.is_empty()
            {
                self.deleted_lines.insert(action_id, current_deleted_lines);
            }
        }
        if let Some((point_number, point)) = self.points.remove_entry(&number)
        {
            let deleted_point = DeletedPoint::create(point_number, point);
            self.deleted_points.insert(action_id, deleted_point);
            let detail = json!({ "point_data": { "number": number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_POINT_EVENT_NAME, EVENT_TARGET)?;
            log(&format!("Geometry: Points: {:?}, Lines: {:?}, Points in \
                lines: {:?}, Deleted points: {:?}, Deleted lines {:?}", self.points,
                self.lines, self.points_in_lines, self.deleted_points, self.deleted_lines));
            Ok(())
        }
        else
        {
            let error_message = &format!("Geometry: Delete point action: Point with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn undo_delete_point(&mut self, action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_point) = self.deleted_points.remove(&action_id)
        {
            let (deleted_point_number, x, y, z) =
                deleted_point.extract_number_and_coordinates();
            if deleted_point_number != number
            {
                let error_message = &format!("Geometry: Undo delete point action: Point with \
                    number {} does not exist!", number);
                return Err(JsValue::from(error_message));
            }
            let detail = json!({ "point_data": { "number": deleted_point_number,
                    "x": x, "y": y, "z": z },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_POINT_EVENT_NAME, EVENT_TARGET)?;
            self.points.insert(deleted_point_number, Point::create(x, y, z));
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
                    if let Some(line_numbers) = self.points_in_lines
                        .get_mut(&start_point_number)
                    {
                        if line_numbers.iter().position(|line_number| *line_number == number)
                            .is_none()
                        {
                            line_numbers.push(number);
                        }
                    }
                    else
                    {
                        self.points_in_lines.insert(start_point_number, vec![number]);
                    }
                    if let Some(line_numbers) = self.points_in_lines
                        .get_mut(&end_point_number)
                    {
                        if line_numbers.iter().position(|line_number| *line_number == number)
                            .is_none()
                        {
                            line_numbers.push(number);
                        }
                    }
                    else
                    {
                        self.points_in_lines.insert(end_point_number, vec![number]);
                    }
                }
            }
            log(&format!("Geometry: Points: {:?}, Lines: {:?}, Points in \
                lines: {:?}, Deleted points: {:?}, Deleted lines {:?}", self.points,
                self.lines, self.points_in_lines, self.deleted_points, self.deleted_lines));
            Ok(())
        }
        else
        {
            let error_message = &format!("Geometry: Undo delete point action: Point with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }


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


    pub fn show_point_info(&mut self, number: u32) -> Result<JsValue, JsValue>
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


    pub fn show_line_info(&mut self, number: u32) -> Result<JsValue, JsValue>
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


    pub fn add_whole_geometry_to_preprocessor(&self, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        for (point_number, point) in self.points.iter()
        {
            let (x, y, z) = point.extract_coordinates();
            let detail = json!({ "point_data":
                { "number": point_number, "x": x, "y": y, "z": z },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_POINT_EVENT_NAME, EVENT_TARGET)?;
        }
        for (line_number, line) in self.lines.iter()
        {
            let (start_point_number, end_point_number) = line.extract_points_numbers();
            let detail = json!({ "line_data": { "number": line_number,
                "start_point_number": start_point_number, "end_point_number": end_point_number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_LINE_EVENT_NAME, EVENT_TARGET)?;
        }
        Ok(())
    }
}
