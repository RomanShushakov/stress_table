use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;

mod point;
use point::{Point, DeletedPoint};

mod line;
use line::{Line, DeletedLine};

mod methods_for_point_data_handle;

mod methods_for_line_data_handle;

pub const EVENT_TARGET: &str = "fea-app";

pub const ADD_POINT_EVENT_NAME: &str = "add_point_server_message";
pub const UPDATE_POINT_EVENT_NAME: &str = "update_point_server_message";
pub const DELETE_POINT_EVENT_NAME: &str = "delete_point_server_message";

const ADD_LINE_EVENT_NAME: &str = "add_line_server_message";
const UPDATE_LINE_EVENT_NAME: &str = "update_line_server_message";
const DELETE_LINE_EVENT_NAME: &str = "delete_line_server_message";

pub const DELETED_LINE_NUMBERS_MESSAGE_HEADER: &str = "deleted_line_numbers";

#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(value: &str);
}


pub fn dispatch_custom_event(detail: serde_json::Value, event_type: &str, query_selector: &str)
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


    pub fn clear_geometry_module_by_action_id(&mut self, action_id: u32)
    {
        self.clear_deleted_lines_by_action_id(action_id);
        self.clear_deleted_points_by_action_id(action_id);
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


    pub fn extract_geometry(&self, is_action_id_should_be_increased: bool)
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
