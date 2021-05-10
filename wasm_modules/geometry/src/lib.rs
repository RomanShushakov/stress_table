use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const EVENTS_TARGET: &str = "fea-app";
const ADD_POINT: &str = "add point server message";
const UPDATE_POINT: &str = "update point server message";
const DELETE_POINT: &str = "delete point server message";
const ADD_LINE: &str = "add line server message";
const UPDATE_LINE: &str = "update line server message";
const DELETE_LINE: &str = "delete line server message";

const SELECTED_POINT_INFO: &str = "selected point server message";


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


#[derive(Debug)]
struct Point
{
    action_id: u32,
    number: u32,
    x: f64,
    y: f64,
    z: f64,
}


impl Point
{
    fn create(action_id: u32, number: u32, x: f64, y: f64, z: f64) -> Point
    {
        Point { action_id, number, x, y, z }
    }


    fn number_same(&self, number: u32) -> bool
    {
        self.number == number
    }


    fn action_id_same(&self, action_id: &u32) -> bool
    {
        self.action_id == *action_id
    }


    fn action_id_greater_or_same(&self, action_id: &u32) -> bool
    {
        self.action_id >= *action_id
    }


    fn coordinates_same(&self, x: f64, y: f64, z: f64) -> bool
    {
        self.x == x && self.y == y && self.z == z
    }


    fn update(&mut self, action_id: u32, x: f64, y: f64, z: f64)
    {
        self.action_id = action_id;
        self.x = x;
        self.y = y;
        self.z = z;
    }


    fn extract_number(&self) -> u32
    {
        self.number
    }


    fn extract_number_and_coordinates(&self) -> (u32, f64, f64, f64)
    {
        (self.number, self.x, self.y, self.z)
    }


    fn update_action_id(&mut self, action_id: u32)
    {
        self.action_id = action_id;
    }
}


#[derive(Debug)]
struct Line
{
    action_id: u32,
    number: u32,
    start_point: Rc<RefCell<Point>>,
    end_point: Rc<RefCell<Point>>,
}


impl Line
{
    fn create(action_id: u32, number: u32, start_point: Rc<RefCell<Point>>,
        end_point: Rc<RefCell<Point>>) -> Line
    {
        Line { action_id, number, start_point, end_point }
    }


    fn number_same(&self, number: u32) -> bool
    {
        self.number == number
    }


    fn action_id_same(&self, action_id: &u32) -> bool
    {
        self.action_id == *action_id
    }


    fn action_id_greater_or_same(&self, action_id: &u32) -> bool
    {
        self.action_id >= *action_id
    }


    fn start_and_end_points_same(&self, start_point_number: u32, end_point_number: u32) -> bool
    {
        (self.start_point.borrow().number_same(start_point_number) &&
        self.end_point.borrow().number_same(end_point_number)) ||
        (self.start_point.borrow().number_same(end_point_number) &&
        self.end_point.borrow().number_same(start_point_number))
    }


    fn update(&mut self, action_id: u32, start_point: Rc<RefCell<Point>>,
        end_point: Rc<RefCell<Point>>)
    {
        self.action_id = action_id;
        self.start_point = start_point;
        self.end_point = end_point;
    }


    fn update_action_id(&mut self, action_id: u32)
    {
        self.action_id = action_id;
    }


    fn point_belongs(&self, point_number: u32) -> bool
    {
        self.start_point.borrow().number_same(point_number) ||
        self.end_point.borrow().number_same(point_number)
    }


    fn extract_number_and_points_numbers(&self) -> (u32, u32, u32)
    {
        (self.number, self.start_point.borrow().extract_number(),
            self.end_point.borrow().extract_number())
    }


    fn extract_number(&self) -> u32
    {
        self.number
    }
}


#[wasm_bindgen]
pub struct Geometry
{
    points: Vec<Rc<RefCell<Point>>>,
    lines: Vec<Line>,
    deleted_points: Vec<Rc<RefCell<Point>>>,
    deleted_lines: Vec<Line>,
}


#[wasm_bindgen]
impl Geometry
{
    pub fn create() -> Geometry
    {
        let points = Vec::new();
        let lines = Vec::new();
        let deleted_points = Vec::new();
        let deleted_lines = Vec::new();
        Geometry { points, lines, deleted_points, deleted_lines }
    }


    fn clear_deleted_lines_by_action_id(&mut self, action_id: &u32)
    {
        while let Some(position) = self.deleted_lines.iter().position(|line|
            line.action_id_greater_or_same(&action_id))
        {
            let _ = self.deleted_lines.remove(position);
        }
    }


    fn clear_deleted_points_by_action_id(&mut self, action_id: &u32)
    {
        while let Some(position) = self.deleted_points.iter().position(|point|
            point.borrow().action_id_greater_or_same(&action_id))
        {
            let _ = self.deleted_points.remove(position);
        }
    }


    pub fn add_point(&mut self, action_id: u32, number: u32, x: f64, y: f64, z: f64,
        is_action_id_should_be_increased: bool ) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(&action_id);
        self.clear_deleted_points_by_action_id(&action_id);
        if self.points.iter().position(|point|
            point.borrow().number_same(number)).is_some()
        {
            let error_message = &format!("Geometry: Add point action: Point with \
                number {} does already exist!", number);
            return Err(JsValue::from(error_message));
        }
        if self.points.iter().position(|point|
            point.borrow().coordinates_same(x, y, z)).is_some()
        {
            let error_message = &format!("Geometry: Add point action: Point with \
                coordinates {}, {}, {} does already exist!", x, y, z);
            return Err(JsValue::from(error_message));
        }
        let point = Point::create(action_id, number, x, y, z);
        self.points.push(Rc::new(RefCell::new(point)));
        let detail = json!({ "point_data": { "number": number, "x": x, "y": y, "z": z },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_POINT, EVENTS_TARGET)?;
        log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, \
            deleted lines {:?}", self.points, self.lines, self.deleted_points, self.deleted_lines));
        Ok(())
    }


    pub fn update_point(&mut self, action_id: u32, number: u32, x: f64, y: f64, z: f64,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(&action_id);
        self.clear_deleted_points_by_action_id(&action_id);
        if self.points.iter().position(|point|
            point.borrow().coordinates_same(x, y, z)).is_some()
        {
            let error_message = &format!("Geometry: Update point action: Point with \
                coordinates {}, {}, {} does already exist!", x, y, z);
            return Err(JsValue::from(error_message));
        }

        if let Some(position) = self.points.iter().position(|point|
            point.borrow().number_same(number))
        {
            self.points[position].borrow_mut().update(action_id, x, y, z);
            let detail = json!({ "point_data": { "number": number, "x": x, "y": y, "z": z },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_POINT, EVENTS_TARGET)?;
            log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, \
                deleted lines {:?}", self.points, self.lines, self.deleted_points,
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


    pub fn delete_point(&mut self, action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(&action_id);
        self.clear_deleted_points_by_action_id(&action_id);
        if let Some(position) = self.points.iter().position(|point|
            point.borrow().number_same(number))
        {
            while let Some(position) = self.lines.iter().position(|line|
                line.point_belongs(number))
            {
                let mut deleted_line = self.lines.remove(position);
                let deleted_line_number = deleted_line.extract_number();
                deleted_line.update_action_id(action_id);
                self.deleted_lines.push(deleted_line);
                let detail = json!({ "line_data": { "number": deleted_line_number },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail, DELETE_LINE, EVENTS_TARGET)?;
            }
            let deleted_point = self.points.remove(position);
            deleted_point.borrow_mut().update_action_id(action_id);
            self.deleted_points.push(deleted_point);
            let detail = json!({ "point_data": { "number": number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_POINT, EVENTS_TARGET)?;
            log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, \
                deleted lines {:?}", self.points, self.lines, self.deleted_points,
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


    pub fn undo_delete_point(&mut self, action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(position) = self.deleted_points.iter().position(|point|
            point.borrow().action_id_same(&action_id) && point.borrow().number_same(number))
        {
            let returned_point = self.deleted_points.remove(position);
            let (number, x, y, z) = returned_point.borrow().extract_number_and_coordinates();
            let detail = json!({ "point_data": { "number": number, "x": x, "y": y, "z": z },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_POINT, EVENTS_TARGET)?;
            self.points.push(returned_point);
            log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, \
                deleted lines {:?}", self.points, self.lines, self.deleted_points,
                self.deleted_lines));
            while let Some(position) = self.deleted_lines.iter().position(|line|
                line.action_id_same(&action_id))
            {
                let returned_line = self.deleted_lines.remove(position);
                let (number, start_point_number, end_point_number) =
                    returned_line.extract_number_and_points_numbers();
                let detail = json!({ "line_data": { "number": number,
                    "start_point_number": start_point_number, "end_point_number": end_point_number },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased });
                dispatch_custom_event(detail, ADD_LINE, EVENTS_TARGET)?;
                self.lines.push(returned_line);
                log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, \
                    deleted lines {:?}", self.points, self.lines, self.deleted_points,
                    self.deleted_lines));
            }
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
        self.clear_deleted_lines_by_action_id(&action_id);
        if self.lines.iter().position(|line| line.number_same(number)).is_some()
        {
            let error_message = &format!("Geometry: Add line action: Line with \
                number {} does already exist!", number);
            return Err(JsValue::from(error_message));
        }
        if self.lines.iter().position(|line|
            line.start_and_end_points_same(start_point_number, end_point_number)).is_some()
        {
            let error_message = &format!("Geometry: Add line action: Line with \
                point number {} and {} does already exist!", start_point_number, end_point_number);
            return Err(JsValue::from(error_message));
        }
        let start_point =
            {
                if let Some(position) = self
                    .points
                    .iter()
                    .position(|point|
                        point.borrow().number_same(start_point_number))
                {
                    Ok(Rc::clone(&self.points[position]))
                }
                else
                {
                    let error_message = &format!("Geometry: Add line action: Point with \
                        number {} does not exist!", start_point_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        let end_point =
            {
                if let Some(position) = self.points.iter().position(|point|
                    point.borrow().number_same(end_point_number))
                {
                    Ok(Rc::clone(&self.points[position]))
                }
                else
                {
                    let error_message = &format!("Geometry: Add line action: Point with \
                        number {} does not exist!", end_point_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        let line = Line::create(action_id, number, start_point, end_point);
        self.lines.push(line);
        let detail = json!({ "line_data": { "number": number,
            "start_point_number": start_point_number, "end_point_number": end_point_number },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_LINE, EVENTS_TARGET)?;
        log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, \
            deleted lines {:?}", self.points, self.lines, self.deleted_points, self.deleted_lines));
        Ok(())
    }

    pub fn update_line(&mut self, action_id: u32, number: u32, start_point_number: u32,
        end_point_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(&action_id);
        if self.lines.iter().position(|line|
            line.start_and_end_points_same(start_point_number, end_point_number)).is_some()
        {
            let error_message = &format!("Geometry: Update line action: Line with \
                point number {} and {} does already exist!", start_point_number, end_point_number);
            return Err(JsValue::from(error_message));
        }
        let start_point =
            {
                if let Some(position) = self.points.iter().position(|point|
                    point.borrow().number_same(start_point_number))
                {
                    Ok(Rc::clone(&self.points[position]))
                }
                else
                {
                    let error_message = &format!("Geometry: Update line action: Point with \
                        number {} does not exist!", start_point_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        let end_point =
            {
                if let Some(position) = self.points.iter().position(|point|
                    point.borrow().number_same(end_point_number))
                {
                    Ok(Rc::clone(&self.points[position]))
                }
                else
                {
                    let error_message = &format!("Geometry: Update line action: Point with \
                        number {} does not exist!", end_point_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        if let Some(position) = self.lines.iter().position(|line|
            line.number_same(number))
        {
            self.lines[position].update(action_id, start_point, end_point);
            let detail = json!({ "line_data": { "number": number,
                "start_point_number": start_point_number, "end_point_number": end_point_number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_LINE, EVENTS_TARGET)?;
            log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, deleted \
                lines {:?}", self.points, self.lines, self.deleted_points, self.deleted_lines));
        }
        else
        {
            let error_message = &format!("Geometry: Update line action: Line with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }

    pub fn delete_line(&mut self, action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_lines_by_action_id(&action_id);
        if let Some(position) = self.lines.iter().position(|line|
            line.number_same(number))
        {
            let mut deleted_line = self.lines.remove(position);
            deleted_line.update_action_id(action_id);
            self.deleted_lines.push(deleted_line);
            let detail = json!({ "line_data": { "number": number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_LINE, EVENTS_TARGET)?;
            log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, \
                deleted lines {:?}", self.points, self.lines, self.deleted_points,
                self.deleted_lines));
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
        if let Some(position) = self.deleted_lines.iter().position(|line|
            line.action_id_same(&action_id) && line.number_same(number))
        {
            let returned_line = self.deleted_lines.remove(position);
            let (line_number, start_point_number, end_point_number) =
                returned_line.extract_number_and_points_numbers();
            let detail = json!({ "line_data": { "number": line_number,
                "start_point_number": start_point_number, "end_point_number": end_point_number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_LINE, EVENTS_TARGET)?;
            self.lines.push(returned_line);
            Ok(())
        }
        else
        {
            let error_message = &format!("Geometry: Undo delete line action: Line with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn show_point_info(&mut self, number: u32) -> Result<(), JsValue>
    {
        if let Some(position) = self.points.iter().position(|point|
            point.borrow().number_same(number))
        {
            let (number, x, y, z) = self.points[position]
                .borrow()
                .extract_number_and_coordinates();
            log(&format!("You selected point with number: {}, x: {}, y: {}, z: {}",
                number, x, y, z));
        }
        else
        {
            let error_message = &format!("Geometry: Show point info action: Point with \
                number {} does not exist!", number);
            return Err(JsValue::from(error_message));
        }

        Ok(())

        // if self.points.iter().position(|point|
        //     point.borrow().coordinates_same(x, y, z)).is_some()
        // {
        //     let error_message = &format!("Geometry: Add point action: Point with \
        //         coordinates {}, {}, {} does already exist!", x, y, z);
        //     return Err(JsValue::from(error_message));
        // }
        // let point = Point::create(action_id, number, x, y, z);
        // self.points.push(Rc::new(RefCell::new(point)));
        // let detail = json!({ "point_data": { "number": number, "x": x, "y": y, "z": z },
        //         "is_action_id_should_be_increased": is_action_id_should_be_increased });
        // dispatch_custom_event(detail, ADD_POINT, EVENTS_TARGET)?;
        // log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, \
        //     deleted lines {:?}", self.points, self.lines, self.deleted_points,
        //     self.deleted_lines));
        // Ok(())
    }

    pub fn add_whole_geometry_to_preprocessor(&self, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        for point in &self.points
        {
            let (point_number, x, y, z) =
                point.borrow().extract_number_and_coordinates();
            let detail = json!({ "point_data":
                { "number": point_number, "x": x, "y": y, "z": z },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_POINT, EVENTS_TARGET)?;
        }
        for line in &self.lines
        {
            let (line_number, start_point_number, end_point_number) =
                line.extract_number_and_points_numbers();
            let detail = json!({ "line_data": { "number": line_number,
                "start_point_number": start_point_number, "end_point_number": end_point_number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_LINE, EVENTS_TARGET)?;
        }
        Ok(())
    }
}
