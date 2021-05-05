use wasm_bindgen::prelude::*;
use serde_json::{Value};
use self::ActionType::*;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}


#[wasm_bindgen(module = "/js/interface_to_communicate_with_geometry.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addPointToGeometry, catch)]
    fn add_point_to_geometry(action_id: u32, number: u32, x: f64, y: f64, z: f64)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updatePointToGeometry, catch)]
    fn update_point_to_geometry(action_id: u32, number: u32, x: f64, y: f64, z: f64)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addWholeGeometryToPreprocessor)]
    fn add_whole_geometry_to_preprocessor();
}


#[derive(Debug, Clone)]
enum ObjectType
{
    Point,
    Line,
}


#[derive(Debug, Clone)]
pub struct ObjectData
{
    number: u32,
    properties: Vec<f64>,
    contained_objects_numbers: Vec<u32>,
    contained_objects_properties: Vec<f64>,
}



pub trait ActionObjectClone
{
    fn clone_box(&self) -> Box<dyn ActionObjectTrait>;
}


impl<T> ActionObjectClone for T
    where T: ActionObjectTrait + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn ActionObjectTrait>
    {
        Box::new(self.clone())
    }
}


impl Clone for Box<dyn ActionObjectTrait>
{
    fn clone(&self) -> Box<dyn ActionObjectTrait>
    {
        self.clone_box()
    }
}


pub trait ActionObjectTrait: ActionObjectClone
{
    fn update(&mut self, object_data: ObjectData);
    fn extract_object_data(&self) -> ObjectData;
}


#[derive(Clone)]
pub struct Point
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
        self.x = object_data.properties[0];
        self.y = object_data.properties[1];
        self.z = object_data.properties[2];
    }


    fn extract_object_data(&self) -> ObjectData
    {
        let number = self.number;
        let properties = vec![self.x, self.y, self.z];
        let contained_objects_numbers = Vec::new();
        let contained_objects_properties = Vec::new();
        ObjectData { number, properties, contained_objects_numbers, contained_objects_properties }
    }
}


#[derive(Clone)]
pub struct Line
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


impl ActionObjectTrait for Line
{
    fn update(&mut self, object_data: ObjectData)
    {
        self.start_point = object_data.contained_objects_numbers[0];
        self.end_point = object_data.contained_objects_numbers[1];
    }


    fn extract_object_data(&self) -> ObjectData
    {
        let number = self.number;
        let properties = Vec::new();
        let contained_objects_numbers = vec![self.start_point, self.end_point];
        let contained_objects_properties = Vec::new();
        ObjectData { number, properties, contained_objects_numbers, contained_objects_properties }
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
                        object_data.properties[0],
                        object_data.properties[1],
                        object_data.properties[2]);
                    Box::new(point)
                },
            ObjectType::Line =>
                {
                    let line = Line::create(
                        object_data.number,
                        object_data.contained_objects_numbers[0],
                        object_data.contained_objects_numbers[1]);
                    Box::new(line)
                },
        }
    }
}


#[derive(Clone)]
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


    fn extract_object_data(&self) -> ObjectData
    {
        self.object.extract_object_data()
    }
}

#[derive(Debug, Clone)]
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


#[derive(Clone)]
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
    current_action: Option<Action>,
    active_actions: Vec<Action>,
    undo_actions: Vec<Action>,
}


#[wasm_bindgen]
impl ActionsRouter
{
    pub fn create() -> ActionsRouter
    {
        let current_action = None;
        let active_actions = Vec::new();
        let undo_actions = Vec::new();
        ActionsRouter { current_action, active_actions, undo_actions }
    }


    fn handle_add_point_message(&mut self, message: &str) -> Result<(), JsValue>
    {
        let add_point_message: Value = serde_json::from_str(message)
            .or(Err(JsValue::from("Actions router: \
            Add point action: Message could not be parsed!")))?;
        let action_id = add_point_message["add_point"]["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: Action id could not be converted to u32!")))?;
        let number = add_point_message["add_point"]["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: Point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: \
                Point number could not be converted to u32!")))?;
        let x = add_point_message["add_point"]["x"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: \
                Point x coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: \
                Point x coordinate could not be converted to f64!")))?;
        let y = add_point_message["add_point"]["y"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: Point y coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: \
                Point y coordinate could not be converted to f64!")))?;
        let z = add_point_message["add_point"]["z"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: Point z coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: \
                Point z coordinate could not be converted to f64!")))?;
        let object_type = ObjectType::Point;
        let action_type = ActionType::AddPoint;
        let properties = vec![x, y, z];
        let contained_objects_numbers = Vec::new();
        let contained_objects_properties = Vec::new();
        let object_data = ObjectData {
            number, properties, contained_objects_numbers, contained_objects_properties };
        let action_object = ActionObject::create(object_type, object_data);
        let action = Action { action_id, action_type, action_object, previous_object: None };
        self.current_action = Some(action);
        Ok(())
    }


    fn handle_update_point_message(&mut self, message: &str) -> Result<(), JsValue>
    {
        let update_point_message: Value = serde_json::from_str(message)
            .or(Err(JsValue::from("Actions router: Update point action: \
            Message could not be parsed!")))?;
        let action_id = update_point_message["update_point"]["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Action id could not be converted to u32!")))?;
        let number = update_point_message["update_point"]["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update point action: Point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point number could not be converted to u32!")))?;
        let old_point_coordinate_values: Value =
            serde_json::from_str(&update_point_message["update_point"]["old_point_values"]
                .to_string())
                .or(Err(JsValue::from(
                    "Actions router: Update point action: \
                    Point old coordinates could not be extracted!")))?;
        let old_x_value = old_point_coordinate_values["x"].to_string()
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point old x coordinate could not be converted to f64!")))?;
        let old_y_value = old_point_coordinate_values["y"].to_string()
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point old y coordinate could not be converted to f64!")))?;
        let old_z_value = old_point_coordinate_values["z"].to_string()
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point old z coordinate could not be converted to f64!")))?;
        let new_point_coordinate_values: Value =
            serde_json::from_str(&update_point_message["update_point"]["new_point_values"]
                .to_string())
                .or(Err(JsValue::from(
                    "Actions router: Update point action: \
                    Point new coordinates could not be extracted!")))?;
        let new_x_value = new_point_coordinate_values["x"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update point action: \
                Point new x coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point new x value could not be converted to f64!")))?;
        let new_y_value = new_point_coordinate_values["y"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update point action: \
                Point new y coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point new y value could not be converted to f64!")))?;
        let new_z_value = new_point_coordinate_values["z"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update point action: \
                Point new z coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point new z value could not be converted to f64!")))?;
        let object_type = ObjectType::Point;
        let action_type = ActionType::UpdatePoint;
        let old_properties = vec![old_x_value, old_y_value, old_z_value];
        let old_contained_objects_numbers = Vec::new();
        let old_contained_objects_properties = Vec::new();
        let previous_object_data = ObjectData {
            number, properties: old_properties,
            contained_objects_numbers: old_contained_objects_numbers,
            contained_objects_properties: old_contained_objects_properties };
        let previous_action_object =
            ActionObject::create(object_type.clone(), previous_object_data);
        let properties = vec![new_x_value, new_y_value, new_z_value];
        let contained_objects_numbers = Vec::new();
        let contained_objects_properties = Vec::new();
        let object_data = ObjectData { number, properties,
            contained_objects_numbers, contained_objects_properties };
        let action_object = ActionObject::create(object_type, object_data);
        let action = Action { action_id, action_type, action_object,
            previous_object: Some(previous_action_object) };
        self.current_action = Some(action);
        Ok(())
    }


    fn handle_current_action(&mut self) -> Result<(), JsValue>
    {
        if let Some(action) = &self.current_action
        {
            let action_id = action.action_id;
            let action_type = &action.action_type;
            match action_type
            {
                ActionType::AddPoint =>
                    {
                        let action_object_data =
                            &action.action_object.extract_object_data();
                        let number = action_object_data.number;
                        let x = action_object_data.properties[0];
                        let y = action_object_data.properties[1];
                        let z = action_object_data.properties[2];
                        add_point_to_geometry(action_id, number, x, y, z)?;
                        self.active_actions.push(action.clone());
                    },
                ActionType::UpdatePoint =>
                    {
                        let action_object_data =
                            &action.action_object.extract_object_data();
                        let number = action_object_data.number;
                        let x = action_object_data.properties[0];
                        let y = action_object_data.properties[1];
                        let z = action_object_data.properties[2];
                        update_point_to_geometry(action_id, number, x, y, z)?;
                        self.active_actions.push(action.clone());
                    },
                _ => (),
            }
            self.current_action = None;
        }
        Ok(())
    }


    pub fn handle_message(&mut self, message: String) -> Result<(), JsValue>
    {
        if message.contains(&ActionType::AddPoint.as_str())
        {
            self.handle_add_point_message(&message)?;
        }
        else if message.contains(&ActionType::UpdatePoint.as_str())
        {
            self.handle_update_point_message(&message)?;
        }
        else
        {
            let error_message = "Actions router: Message could not be handled!";
            return Err(JsValue::from(error_message));
        }
        self.handle_current_action()?;

        for action in &self.active_actions
        {
            let action_id = &action.action_id;
            let action_type = &action.action_type;
            let action_object = &action.action_object;
            let object_type = &action_object.object_type;
            let object_data = &action_object.extract_object_data();
            let previous_object = &action.previous_object;
            let (previous_object_type, previous_object_data) =
                if let Some(object) = previous_object
                {
                    let object_type = object.object_type.clone();
                    let object_data = object.extract_object_data().clone();
                    (Some(object_type), Some(object_data))
                }
                else
                {
                    (None, None)
                };
            log(&format!("Actions router: Action id: {:?}, action type: {:?}, \
                object type: {:?}, object data: {:?}, previous object type: {:?}, \
                previous object data {:?}", action_id, action_type, object_type, object_data,
                previous_object_type, previous_object_data));
        }
        log(&format!("Actions router: The number of active actions: {}",
            self.active_actions.len()));

        Ok(())
    }


    pub fn add_whole_geometry_to_preprocessor(&self)
    {
        add_whole_geometry_to_preprocessor();
    }
}
