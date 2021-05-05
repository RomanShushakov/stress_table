use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use serde_json::json;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}


#[wasm_bindgen(module = "/js/interface_to_communicate_geometry_with_app.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addPointToApp)]
    fn add_point_to_app(number: u32, x: f64, y: f64, z: f64, is_preprocessor_request: bool);

    #[wasm_bindgen(js_name = updatePointInApp)]
    fn update_point_in_app(number: u32, x: f64, y: f64, z: f64);
}


fn dispatch_custom_event(detail: serde_json::Value, event_type: &str, query_selector: &str)
    -> Result<(), JsValue>
{
    let custom_event = web_sys::CustomEvent::new_with_event_init_dict(
        event_type,
        web_sys::CustomEventInit::new()
            .bubbles(true)
            .composed(true)
            .detail(&JsValue::from_serde(&detail)
                .or(Err("Geometry: Dispatch event: detail could not be \
                converted into JsValue!"))?))
                    .or(Err(JsValue::from("Geometry: Dispatch event: \
                    custom event could not be constructed!")))?;
    web_sys::window().expect("no global `window` exists")
        .document().expect("should have a document on window")
        .query_selector(query_selector).or(Err(JsValue::from("Geometry: Dispatch event: No \
            element find by current selector!")))?.unwrap()
        .dyn_into::<web_sys::EventTarget>().unwrap()
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
    fn number_same(&self, number: u32) -> bool
    {
        self.number == number
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


    fn extract_number_and_coordinates(&self) -> (u32, f64, f64, f64)
    {
        (self.number, self.x, self.y, self.z)
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


    pub fn add_whole_geometry_to_preprocessor(&self)
    {
        for point in &self.points
        {
            let (number, x, y, z) =
                point.borrow().extract_number_and_coordinates();
            add_point_to_app(number, x, y, z, true);
        }
    }


    pub fn add_point(&mut self, action_id: u32, number: u32, x: f64, y: f64, z: f64)
        -> Result<(), JsValue>
    {
        if self.points.iter()
            .position(|point| point.borrow().number_same(number)).is_some()
        {
            let error_message = &format!("Geometry: Add point action: Point with \
                number {} does already exist!", number);
            return Err(JsValue::from(error_message));
        }
        if self.points.iter()
            .position(|point| point.borrow().coordinates_same(x, y, z)).is_some()
        {
            let error_message = &format!("Geometry: Add point action: Point with \
                coordinates {}, {}, {} does already exist!", x, y, z);
            return Err(JsValue::from(error_message));
        }
        let point = Point { action_id, number, x, y, z };
        self.points.push(Rc::new(RefCell::new(point)));
        // add_point_to_app(number, x, y, z, false);
        let detail =
            json!({"number": number, "x": x, "y": y, "z": z, "is_preprocessor_request": false});
        let event_type = "add point";
        let query_selector = "fea-app";
        dispatch_custom_event(detail, event_type, query_selector)?;
        log(&format!("Geometry: Points: {:?}, lines: {:?}, deleted points: {:?}, \
            deleted lines {:?}", self.points, self.lines, self.deleted_points, self.deleted_lines));
        Ok(())
    }


    pub fn update_point(&mut self, action_id: u32, number: u32, x: f64, y: f64, z: f64)
        -> Result<(), JsValue>
    {
        if self.points.iter()
            .position(|point| point.borrow().coordinates_same(x, y, z)).is_some()
        {
            let error_message = &format!("Geometry: Update point action: Point with \
                coordinates {}, {}, {} does already exist!", x, y, z);
            return Err(JsValue::from(error_message));
        }

        if let Some(position) = self.points.iter()
            .position(|point| point.borrow().number_same(number))
        {
            self.points[position].borrow_mut().update(action_id, x, y, z);
            update_point_in_app(number, x, y, z);
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
}
