use wasm_bindgen::prelude::*;
use serde_json::{Value};
use self::ActionType::*;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


const ADD_POINT_EVENT: &str = "add_point";
const UPDATE_POINT_EVENT: &str = "update_point";
const DELETE_POINT_EVENT: &str = "delete_point";
const ADD_LINE_EVENT: &str = "add_line";
const UPDATE_LINE_EVENT: &str = "update_line";
const DELETE_LINE_EVENT: &str = "delete_line";


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

    #[wasm_bindgen(js_name = updatePointInGeometry, catch)]
    fn update_point_in_geometry(action_id: u32, number: u32, x: f64, y: f64, z: f64)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addWholeGeometryToPreprocessor)]
    fn add_whole_geometry_to_preprocessor();

    #[wasm_bindgen(js_name = deletePointFromGeometry, catch)]
    fn delete_point_from_geometry(action_id: u32, number: u32)
        -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addLineToGeometry, catch)]
    fn add_line_to_geometry(action_id: u32, number: u32, start_point_number: u32,
        end_point_number: u32) -> Result<(), JsValue>;
}


#[derive(Clone, Debug)]
struct ObjectNumber(u32);


impl ObjectNumber
{
    fn create(number: u32) -> ObjectNumber
    {
        ObjectNumber(number)
    }


    fn get_number(&self) -> u32
    {
        self.0
    }
}


#[derive(Clone, Debug)]
struct Coordinates
{
    x: f64,
    y: f64,
    z: f64,
}


impl Coordinates
{
    fn create(x: f64, y: f64, z: f64) -> Coordinates
    {
        Coordinates { x, y, z }
    }


    fn get_x(&self) -> f64
    {
        self.x
    }


    fn get_y(&self) -> f64
    {
        self.y
    }


    fn get_z(&self) -> f64
    {
        self.z
    }
}


#[derive(Debug, Clone)]
enum ActionType
{
    AddPoint(ObjectNumber, Coordinates),
    UpdatePoint(ObjectNumber, Coordinates, Coordinates),
    DeletePoint(ObjectNumber),
    AddLine(ObjectNumber, ObjectNumber, ObjectNumber),
    UpdateLine,
    DeleteLine,
}


#[derive(Clone)]
struct Action
{
    action_id: u32,
    action_type: ActionType,
}


impl Action
{
    fn create(action_id: u32, action_type: ActionType) -> Action
    {
        Action { action_id, action_type }
    }
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
        let point_number = ObjectNumber::create(number);
        let coordinates = Coordinates::create(x, y, z);
        let action_type = ActionType::AddPoint(point_number, coordinates);
        let action = Action::create(action_id, action_type);
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
        let point_number = ObjectNumber::create(number);
        let old_coordinates =
            Coordinates::create(old_x_value, old_y_value, old_z_value);
        let new_coordinates =
            Coordinates::create(new_x_value, new_y_value, new_z_value);
        let action_type = ActionType::UpdatePoint(point_number, old_coordinates, new_coordinates);
        let action = Action::create(action_id, action_type);
        self.current_action = Some(action);
        Ok(())
    }


    fn handle_delete_point_message(&mut self, message: &str) -> Result<(), JsValue>
    {
        let delete_point_message: Value = serde_json::from_str(message)
            .or(Err(JsValue::from("Actions router: \
            Delete point action: Message could not be parsed!")))?;
        let action_id = delete_point_message["delete_point"]["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete point action: \
                Action id could not be converted to u32!")))?;
        let number = delete_point_message["delete_point"]["number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Delete point action: \
                Point number could not be converted to u32!")))?;
        let x = delete_point_message["delete_point"]["x"].to_string()
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Delete point action: \
                Point x coordinate could not be converted to f64!")))?;
        let y = delete_point_message["delete_point"]["y"].to_string()
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Delete point action: \
                Point y coordinate could not be converted to f64!")))?;
        let z = delete_point_message["delete_point"]["z"].to_string()
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Delete point action: \
                Point z coordinate could not be converted to f64!")))?;
        let point_number = ObjectNumber::create(number);
        let action_type = ActionType::DeletePoint(point_number);
        let action = Action::create(action_id, action_type);
        self.current_action = Some(action);
        Ok(())
    }


    fn handle_add_line_message(&mut self, message: &str) -> Result<(), JsValue>
    {
        let add_line_message: Value = serde_json::from_str(message)
            .or(Err(JsValue::from("Actions router: \
            Add line action: Message could not be parsed!")))?;
        let action_id = add_line_message["add_line"]["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: Action id could not be converted to u32!")))?;
        let number = add_line_message["add_line"]["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add line action: Line number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add line action: \
                Line number could not be converted to u32!")))?;
        let start_point_number = add_line_message["add_line"]["start_point_number"]
            .as_str().ok_or(JsValue::from("Actions router: Add line action: \
                Line start point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add line action: \
                Line start point number could not be converted to u32!")))?;
        let end_point_number = add_line_message["add_line"]["end_point_number"]
            .as_str().ok_or(JsValue::from("Actions router: Add line action: \
                Line end point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add line action: \
                Line end point number could not be converted to u32!")))?;
        let line_number = ObjectNumber::create(number);
        let start_point_number = ObjectNumber::create(start_point_number);
        let end_point_number = ObjectNumber::create(end_point_number);
        let action_type = ActionType::AddLine(line_number, start_point_number, end_point_number);
        let action = Action::create(action_id, action_type);
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
                ActionType::AddPoint(point_number, coordinates) =>
                    {
                        let number = point_number.get_number();
                        let x = coordinates.get_x();
                        let y = coordinates.get_y();
                        let z = coordinates.get_z();
                        add_point_to_geometry(action_id, number, x, y, z)?;
                        self.active_actions.push(action.clone());
                    },
                ActionType::UpdatePoint(point_number, _, new_coordinates) =>
                    {
                        let number = point_number.get_number();
                        let x = new_coordinates.get_x();
                        let y = new_coordinates.get_y();
                        let z = new_coordinates.get_z();
                        update_point_in_geometry(action_id, number, x, y, z)?;
                        self.active_actions.push(action.clone());
                    },
                ActionType::DeletePoint(point_number) =>
                    {
                        let number = point_number.get_number();
                        delete_point_from_geometry(action_id, number)?;
                        self.active_actions.push(action.clone());
                    },
                ActionType::AddLine(line_number, start_point_number,
                    end_point_number) =>
                    {
                        let number = line_number.get_number();
                        let start_point_number = start_point_number.get_number();
                        let end_point_number = end_point_number.get_number();
                        add_line_to_geometry(action_id, number, start_point_number,
                            end_point_number)?;
                        self.active_actions.push(action.clone());
                    }
                _ => (),
            }
            self.current_action = None;
        }
        Ok(())
    }


    pub fn handle_message(&mut self, message: String) -> Result<(), JsValue>
    {
        if message.contains(ADD_POINT_EVENT)
        {
            self.handle_add_point_message(&message)?;
        }
        else if message.contains(UPDATE_POINT_EVENT)
        {
            self.handle_update_point_message(&message)?;
        }
        else if message.contains(DELETE_POINT_EVENT)
        {
            self.handle_delete_point_message(&message)?;
        }
        else if message.contains(ADD_LINE_EVENT)
        {
            self.handle_add_line_message(&message)?;
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
            log(&format!("Actions router: Action id: {:?}, action type: {:?}",
                action_id, action_type));
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
