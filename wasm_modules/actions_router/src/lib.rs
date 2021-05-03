use wasm_bindgen::prelude::*;
use serde_json::{Value};
use self::ActionType::*;


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}


#[wasm_bindgen(module = "/js/actions_router_to_wasm_modules_communicator_interface.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addPoint)]
    fn add_point();
}


enum ObjectType
{
    Point,
    Line,
}


struct ObjectData
{
    number: u32,
    point_numbers: Vec<u32>,
    point_coordinates: Vec<f64>,
}


trait ActionObjectTrait
{
    fn update(&mut self, object_data: ObjectData);
}


struct Point
{
    number: u32,
    x: f64,
    y: f64,
    z: f64,
}


impl Point
{
    fn create(number: u32, x: f64, y: f64, z: f64) -> Point
    {
        Point { number, x, y, z }
    }
}


impl ActionObjectTrait for Point
{
    fn update(&mut self, object_data: ObjectData)
    {
        self.x = object_data.point_coordinates[0];
        self.y = object_data.point_coordinates[1];
        self.z = object_data.point_coordinates[2];
    }
}


impl ActionObjectTrait for Line
{
    fn update(&mut self, object_data: ObjectData)
    {
        self.start_point = object_data.point_numbers[0];
        self.end_point = object_data.point_numbers[1];
    }
}


struct Line
{
    number: u32,
    start_point: u32,
    end_point: u32,
}


impl Line
{
    fn create(number: u32, start_point: u32, end_point: u32) -> Line
    {
        Line { number, start_point, end_point }
    }
}


struct ActionObjectCreator;


impl ActionObjectCreator
{
    fn create(object_type: &ObjectType, object_data: ObjectData) -> Box<dyn ActionObjectTrait>
    {
        match object_type
        {
            ObjectType::Point =>
                {
                    let point = Point::create(
                        object_data.number,
                        object_data.point_coordinates[0],
                        object_data.point_coordinates[1],
                        object_data.point_coordinates[2]);
                    Box::new(point)
                },
            ObjectType::Line =>
                {
                    let line = Line::create(
                        object_data.number,
                        object_data.point_numbers[0],
                        object_data.point_numbers[1]);
                    Box::new(line)
                },
        }
    }
}


struct ActionObject
{
    object_type: ObjectType,
    object: Box<dyn ActionObjectTrait>,
}

impl ActionObject
{
    fn create(object_type: ObjectType, object_data: ObjectData) -> ActionObject
    {
        let object = ActionObjectCreator::create(&object_type, object_data);
        ActionObject { object_type, object }
    }
}


enum ActionType
{
    AddPoint,
    UpdatePoint,
    DeletePoint,
    AddLine,
    UpdateLine,
    DeleteLine,
}

impl ActionType
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            ActionType::AddPoint => String::from("add_point"),
            ActionType::UpdatePoint => String::from("update_point"),
            ActionType::DeletePoint => String::from("delete_point"),
            ActionType::AddLine => String::from("add_line"),
            ActionType::UpdateLine => String::from("update_point"),
            ActionType::DeleteLine => String::from("delete_line"),
        }
    }
}


struct Action
{
    action_id: u32,
    action_type: ActionType,
    action_object: ActionObject,
    previous_object: Option<ActionObject>,
}


#[wasm_bindgen]
pub struct ActionsRouter
{
    actions_id_counter: u32,
    active_actions: Vec<Action>,
    undo_actions: Vec<Action>,
}


#[wasm_bindgen]
impl ActionsRouter
{
    pub fn create() -> ActionsRouter
    {
        let actions_id_counter = 1;
        let active_actions = Vec::new();
        let undo_actions = Vec::new();
        ActionsRouter { actions_id_counter, active_actions, undo_actions }
    }


    pub fn get_action_id(&self) -> u32
    {
        self.actions_id_counter
    }


    pub fn handle_message(&mut self, message: String) -> Result<(), JsValue>
    {
        if message.contains(&ActionType::AddPoint.as_str())
        {
            let add_point_message: Value = serde_json::from_str(&message)
                .or(Err(JsValue::from("Add point: Message could not be parsed!")))?;
            let action_type = ActionType::AddPoint;
            let action_id = add_point_message["add_point"]["actionId"].to_string()
                .parse::<u32>()
                .or(Err(JsValue::from(
                    "Add point: Action id could not be converted to u32!")))?;
            let number = add_point_message["add_point"]["number"].as_str()
                .ok_or(JsValue::from(
                    "Add point: Point number could not be extracted!"))?
                .parse::<u32>()
                .or(Err(JsValue::from(
                    "Add point: Point number could not be converted to u32!")))?;
            let x = add_point_message["add_point"]["x"].as_str()
                .ok_or(JsValue::from(
                    "Add point: Point x coordinate could not be extracted!"))?
                .parse::<f64>()
                .or(Err(JsValue::from(
                    "Add point: Point x coordinate could not be converted to f64!")))?;
            let y = add_point_message["add_point"]["y"].as_str()
                .ok_or(JsValue::from(
                    "Add point: Point y coordinate could not be extracted!"))?
                .parse::<f64>()
                .or(Err(JsValue::from(
                    "Add point: Point y coordinate could not be converted to f64!")))?;
            let z = add_point_message["add_point"]["z"].as_str()
                .ok_or(JsValue::from(
                    "Add point: Point z coordinate could not be extracted!"))?
                .parse::<f64>()
                .or(Err(JsValue::from(
                    "Add point: Point z coordinate could not be converted to f64!")))?;
            let point_coordinates = vec![x, y, z];
            let object_data = ObjectData { number, point_numbers: Vec::new(), point_coordinates };
            let action_object =
                ActionObject::create(ObjectType::Point, object_data);
            let action = Action { action_id, action_type, action_object, previous_object: None };
            self.active_actions.push(action);
            log(&format!("{:?}", self.active_actions.len()));
        }
        log(&format!("Hello from router: {}", message));
        add_point();
        Ok(())
    }
}
