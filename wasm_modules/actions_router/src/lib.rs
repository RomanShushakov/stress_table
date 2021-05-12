use serde_json::Value;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ADD_POINT_EVENT: &str = "add_point";
const UPDATE_POINT_EVENT: &str = "update_point";
const DELETE_POINT_EVENT: &str = "delete_point";
const ADD_LINE_EVENT: &str = "add_line";
const UPDATE_LINE_EVENT: &str = "update_line";
const DELETE_LINE_EVENT: &str = "delete_line";
const UNDO: &str = "undo";
const REDO: &str = "redo";

const SELECTED_POINT_NUMBER_EVENT: &str = "selected_point_number";
const SELECTED_LINE_NUMBER_EVENT: &str = "selected_line_number";

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
    fn add_point_to_geometry(action_id: u32, number: u32, x: f64, y: f64, z: f64,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updatePointInGeometry, catch)]
    fn update_point_in_geometry(action_id: u32, number: u32, x: f64, y: f64, z: f64,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addWholeGeometryToPreprocessor)]
    fn add_whole_geometry_to_preprocessor(is_action_id_should_be_increased: bool);

    #[wasm_bindgen(js_name = deletePointFromGeometry, catch)]
    fn delete_point_from_geometry(action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = undoDeletePointFromGeometry, catch)]
    fn undo_delete_point_from_geometry(action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = addLineToGeometry, catch)]
    fn add_line_to_geometry(action_id: u32, number: u32, start_point_number: u32,
        end_point_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = updateLineInGeometry, catch)]
    fn update_line_in_geometry(action_id: u32, number: u32, start_point_number: u32,
        end_point_number: u32, is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = deleteLineFromGeometry, catch)]
    fn delete_line_from_geometry(action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = undoDeleteLineFromGeometry, catch)]
    fn undo_delete_line_from_geometry(action_id: u32, number: u32,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = showPointInfo, catch)]
    fn show_point_info(number: u32) -> Result<String, JsValue>;

    #[wasm_bindgen(js_name = showLineInfoFromGeometry, catch)]
    fn show_line_info_from_geometry(number: u32) -> Result<String, JsValue>;
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
    AddPoint(ObjectNumber, Coordinates, bool),
    UpdatePoint(ObjectNumber, Coordinates, Coordinates, bool),
    DeletePoint(ObjectNumber, bool),
    UndoDeletePoint(ObjectNumber, bool),
    AddLine(ObjectNumber, ObjectNumber, ObjectNumber, bool),
    UpdateLine(ObjectNumber, ObjectNumber, ObjectNumber, ObjectNumber, ObjectNumber, bool),
    DeleteLine(ObjectNumber, bool),
    UndoDeleteLine(ObjectNumber, bool),

    ShowPointInfo(ObjectNumber, js_sys::Function),
    ShowLineInfo(ObjectNumber, js_sys::Function),
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


    fn action_id_same(&self, action_id: u32) -> bool
    {
        self.action_id == action_id
    }
}


#[wasm_bindgen]
pub struct ActionsRouter
{
    current_action: Option<(Action, bool)>,
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


    fn handle_add_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
    {
        let action_id = point_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: Action id could not be converted to u32!")))?;
        let number = point_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Add point action: \
                Point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point number could not be converted to u32!")))?;
        let x = point_data["x"].as_str()
            .ok_or(JsValue::from("Actions router: Add point action: \
                Point x coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point x coordinate could not be converted to f64!")))?;
        let y = point_data["y"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: Point y coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point y coordinate could not be converted to f64!")))?;
        let z = point_data["z"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: Point z coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point z coordinate could not be converted to f64!")))?;
        self.undo_actions.clear();
        let point_number = ObjectNumber::create(number);
        let coordinates = Coordinates::create(x, y, z);
        let is_action_id_should_be_increased = true;
        let action_type =
            ActionType::AddPoint(point_number, coordinates, is_action_id_should_be_increased);
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }

    fn handle_update_point_message(&mut self, point_data: &Value) -> Result<(), JsValue> {
        let action_id = point_data["actionId"].to_string()
                .parse::<u32>()
                .or(Err(JsValue::from("Actions router: Update point action: \
                    Action id could not be converted to u32!")))?;
        let number = point_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update point action: Point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point number could not be converted to u32!")))?;
        let old_x_value = point_data["old_point_values"]["x"].to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point old x coordinate could not be converted to f64!")))?;
        let old_y_value = point_data["old_point_values"]["y"].to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point old y coordinate could not be converted to f64!")))?;
        let old_z_value = point_data["old_point_values"]["z"].to_string()
            .parse::<f64>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point old z coordinate could not be converted to f64!")))?;
        let new_x_value = point_data["new_point_values"]["x"].as_str()
            .ok_or(JsValue::from("Actions router: Update point action: \
                Point new x coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point new x value could not be converted to f64!")))?;
        let new_y_value = point_data["new_point_values"]["y"].as_str()
            .ok_or(JsValue::from("Actions router: Update point action: \
                Point new y coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point new y value could not be converted to f64!")))?;
        let new_z_value = point_data["new_point_values"]["z"].as_str()
            .ok_or(JsValue::from("Actions router: Update point action: \
                Point new z coordinate could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point new z value could not be converted to f64!")))?;
        self.undo_actions.clear();
        let point_number = ObjectNumber::create(number);
        let old_coordinates = Coordinates::create(old_x_value,
            old_y_value, old_z_value);
        let new_coordinates = Coordinates::create(new_x_value,
            new_y_value, new_z_value);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::UpdatePoint(point_number,old_coordinates,
            new_coordinates, is_action_id_should_be_increased);
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }

    fn handle_delete_point_message(&mut self, point_data: &Value) -> Result<(), JsValue> {
        let action_id = point_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from( "Actions router: Delete point action: \
                Action id could not be converted to u32!")))?;
        let number = point_data["number"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete point action: \
                Point number could not be converted to u32!")))?;
        self.undo_actions.clear();
        let point_number = ObjectNumber::create(number);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::DeletePoint(point_number, is_action_id_should_be_increased);
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }

    fn handle_add_line_message(&mut self, line_data: &Value) -> Result<(), JsValue> {
        let action_id = line_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: Action id could not be converted to u32!")))?;
        let number = line_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add line action: Line number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add line action: \
                Line number could not be converted to u32!")))?;
        let start_point_number = line_data["start_point_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add line action: \
                Line start point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add line action: \
                Line start point number could not be converted to u32!")))?;
        let end_point_number = line_data["end_point_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add line action: \
                Line end point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add line action: \
                Line end point number could not be converted to u32!")))?;
        let line_number = ObjectNumber::create(number);
        let start_point_number = ObjectNumber::create(start_point_number);
        let end_point_number = ObjectNumber::create(end_point_number);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::AddLine(line_number, start_point_number,
            end_point_number, is_action_id_should_be_increased);
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }

    fn handle_update_line_message(&mut self, line_data: &Value) -> Result<(), JsValue> {
        let action_id = line_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Action id could not be converted to u32!")))?;
        let number = line_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update line action: Line number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line number could not be converted to u32!")))?;
        let old_start_point_number = line_data["old_line_values"]["start_point"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line old start point number could not be converted to u32!")))?;
        let old_end_point_number = line_data["old_line_values"]["end_point"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line old end point number could not be converted to u32!")))?;
        let new_start_point_number = line_data["new_line_values"]["start_point"].as_str()
            .ok_or(JsValue::from("Actions router: Update line action: \
                Line new start point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line new start point number could not be converted to u32!")))?;
        let new_end_point_number = line_data["new_line_values"]["end_point"].as_str()
            .ok_or(JsValue::from("Actions router: Update line action: \
                Line new end point number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line new end point number could not be converted to u32!")))?;
        let line_number = ObjectNumber::create(number);
        let old_start_point_number = ObjectNumber::create(old_start_point_number);
        let old_end_point_number = ObjectNumber::create(old_end_point_number);
        let new_start_point_number = ObjectNumber::create(new_start_point_number);
        let new_end_point_number = ObjectNumber::create(new_end_point_number);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::UpdateLine(line_number, old_start_point_number,
            old_end_point_number, new_start_point_number, new_end_point_number,
            is_action_id_should_be_increased);
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }

    fn handle_delete_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
    {
        let action_id = line_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete line action: \
                Action id could not be converted to u32!")))?;
        let number = line_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Delete line action: \
                Line number could not be extracted!"))?
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete line action: \
                Line number could not be converted to u32!")))?;
        let line_number = ObjectNumber::create(number);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::DeleteLine(line_number, is_action_id_should_be_increased);
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }

    fn handle_undo_message(&mut self, undo_data: &Value) -> Result<(), JsValue> {
        let action_id = undo_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Redo action: \
                Action id could not be converted to u32!")))?;
        if let Some(position) = self.active_actions.iter().position(|action|
            action.action_id_same(action_id))
        {
            let undo_action = self.active_actions.remove(position);
            match &undo_action.action_type
            {
                ActionType::AddPoint(point_number, _, _) =>
                    {
                        let is_action_id_should_be_increased = false;
                        let action_type = ActionType::DeletePoint(point_number.clone(),
                            is_action_id_should_be_increased);
                        let action = Action::create(action_id, action_type);
                        let add_to_active_actions = false;
                        self.current_action = Some((action, add_to_active_actions));
                    },
                ActionType::UpdatePoint(point_number, old_coordinates,
                    new_coordinates, _) =>
                    {
                        let is_action_id_should_be_increased = false;
                        let action_type = ActionType::UpdatePoint(point_number.clone(),
                            new_coordinates.clone(), old_coordinates.clone(),
                            is_action_id_should_be_increased);
                        let action = Action::create(action_id, action_type);
                        let add_to_active_actions = false;
                        self.current_action = Some((action, add_to_active_actions));
                    },
                ActionType::DeletePoint(point_number, _) =>
                    {
                        let is_action_id_should_be_increased = false;
                        let action_type = ActionType::UndoDeletePoint(point_number.clone(),
                            is_action_id_should_be_increased);
                        let action = Action::create(action_id, action_type);
                        let add_to_active_actions = false;
                        self.current_action = Some((action, add_to_active_actions));
                    },
                ActionType::AddLine(line_number, _, _, _) =>
                    {
                        let is_action_id_should_be_increased = false;
                        let action_type = ActionType::DeleteLine(line_number.clone(),
                            is_action_id_should_be_increased);
                        let action = Action::create(action_id, action_type);
                        let add_to_active_actions = false;
                        self.current_action = Some((action, add_to_active_actions));
                    },
                ActionType::UpdateLine(line_number, old_start_point_number,
                    old_end_point_number, new_start_point_number,
                    new_end_point_number, _) =>
                    {
                        let is_action_id_should_be_increased = false;
                        let action_type = ActionType::UpdateLine(line_number.clone(),
                            new_start_point_number.clone(), new_end_point_number.clone(),
                            old_start_point_number.clone(), old_end_point_number.clone(),
                            is_action_id_should_be_increased);
                        let action = Action::create(action_id, action_type);
                        let add_to_active_actions = false;
                        self.current_action = Some((action, add_to_active_actions));
                    },
                ActionType::DeleteLine(line_number, _) =>
                    {
                        let is_action_id_should_be_increased = false;
                        let action_type = ActionType::UndoDeleteLine(line_number.clone(),
                            is_action_id_should_be_increased);
                        let action = Action::create(action_id, action_type);
                        let add_to_active_actions = false;
                        self.current_action = Some((action, add_to_active_actions));
                    },
                _ => (),
            }
            self.undo_actions.push(undo_action);
        }
        Ok(())
    }


    fn handle_redo_message(&mut self, redo_data: &Value) -> Result<(), JsValue>
    {
        let action_id = redo_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Redo action: \
                Action id could not be converted to u32!")))?;
        if let Some(position) = self.undo_actions.iter().position(|action|
            action.action_id_same(action_id))
        {
            let redo_action = self.undo_actions.remove(position);
            let add_to_active_actions = true;
            self.current_action = Some((redo_action, add_to_active_actions));
        }
        Ok(())
    }


    fn handle_selected_point_number_message(&mut self, selected_point_number: &Value,
        show_object_info: &js_sys::Function)
        -> Result<(), JsValue>
    {
        let point_number = selected_point_number.to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Show point info action: \
                Point number could not be converted to u32!")))?;
        let action_id = 0;
        let action_type = ActionType::ShowPointInfo(ObjectNumber::create(point_number), show_object_info.clone());
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = false;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    fn handle_selected_line_number_message(&mut self, selected_line_number: &Value,
        show_object_info: &js_sys::Function) -> Result<(), JsValue>
    {
        let line_number = selected_line_number.to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Show line info action: \
                Line number could not be converted to u32!")))?;
        let action_id = 0;
        let action_type = ActionType::ShowLineInfo(ObjectNumber::create(line_number), show_object_info.clone());
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = false;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    fn handle_current_action(&mut self) -> Result<(), JsValue>
    {
        if let Some((action, add_to_active_actions)) = &self.current_action
        {
            let action_id = action.action_id;
            let action_type = &action.action_type;
            match action_type
            {
                ActionType::AddPoint(point_number, coordinates,
                    is_action_id_should_be_increased) =>
                    {
                        let number = point_number.get_number();
                        let x = coordinates.get_x();
                        let y = coordinates.get_y();
                        let z = coordinates.get_z();
                        add_point_to_geometry(action_id, number, x, y, z,
                            *is_action_id_should_be_increased)?;
                        if *add_to_active_actions == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::UpdatePoint(point_number, _, new_coordinates,
                    is_action_id_should_be_increased) =>
                    {
                        let number = point_number.get_number();
                        let x = new_coordinates.get_x();
                        let y = new_coordinates.get_y();
                        let z = new_coordinates.get_z();
                        update_point_in_geometry(action_id, number, x, y, z,
                            *is_action_id_should_be_increased)?;
                        if *add_to_active_actions == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::DeletePoint(point_number,
                    is_action_id_should_be_increased) =>
                    {
                        let number = point_number.get_number();
                        delete_point_from_geometry(action_id, number,
                            *is_action_id_should_be_increased)?;
                        if *add_to_active_actions == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::UndoDeletePoint(point_number,
                    is_action_id_should_be_increased) =>
                    {
                        let number = point_number.get_number();
                        undo_delete_point_from_geometry(action_id, number,
                            *is_action_id_should_be_increased)?;
                        if *add_to_active_actions == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::AddLine(line_number, start_point_number,
                    end_point_number, is_action_id_should_be_increased) =>
                    {
                        let number = line_number.get_number();
                        let start_point_number = start_point_number.get_number();
                        let end_point_number = end_point_number.get_number();
                        add_line_to_geometry(action_id, number, start_point_number,
                            end_point_number, *is_action_id_should_be_increased)?;
                        if *add_to_active_actions == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::UpdateLine(line_number, _, _,
                    start_point_number, end_point_number,
                    is_action_id_should_be_increased) =>
                    {
                        let number = line_number.get_number();
                        let start_point_number = start_point_number.get_number();
                        let end_point_number = end_point_number.get_number();
                        update_line_in_geometry(action_id, number, start_point_number,
                            end_point_number, *is_action_id_should_be_increased)?;
                        if *add_to_active_actions == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::DeleteLine(line_number,
                    is_action_id_should_be_increased) =>
                    {
                        let number = line_number.get_number();
                        delete_line_from_geometry(action_id, number,
                            *is_action_id_should_be_increased)?;
                        if *add_to_active_actions == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::UndoDeleteLine(line_number,
                    is_action_id_should_be_increased) =>
                    {
                        let number = line_number.get_number();
                        undo_delete_line_from_geometry(action_id, number,
                            *is_action_id_should_be_increased)?;
                        if *add_to_active_actions == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::ShowPointInfo(point_number, show_object_info) =>
                    {
                        let number = point_number.get_number();
                        let point_info = show_point_info(number)?;
                        let point_info_message = format!("Point: {}.",
                            point_info);
                        let this = JsValue::null();
                        let _ = show_object_info.call1(&this, &JsValue::from(point_info_message))?;
                    },
                ActionType::ShowLineInfo(line_number, show_object_info) =>
                    {
                        let number = line_number.get_number();
                        let line_info_from_geometry = show_line_info_from_geometry(number)?;
                        let line_info_message =
                            format!("Line: {}.", line_info_from_geometry);
                        let this = JsValue::null();
                        let _ = show_object_info.call1(&this, &JsValue::from(line_info_message))?;
                    }
            }
            self.current_action = None;
        }
        Ok(())
    }


    pub fn handle_message(&mut self, message: JsValue, show_object_info: &js_sys::Function)
        -> Result<(), JsValue>
    {
        let serialized_message: Value = message.into_serde().or(Err(JsValue::from(
            "Actions router: Message could not be serialized!")))?;
        if let Some(point_data) = serialized_message.get(ADD_POINT_EVENT)
        {
            self.handle_add_point_message(&point_data)?;
        }
        else if let Some(point_data) = serialized_message.get(UPDATE_POINT_EVENT)
        {
            self.handle_update_point_message(&point_data)?;
        }
        else if let Some(point_data) = serialized_message.get(DELETE_POINT_EVENT)
        {
            self.handle_delete_point_message(&point_data)?;
        }
        else if let Some(line_data) = serialized_message.get(ADD_LINE_EVENT)
        {
            self.handle_add_line_message(&line_data)?;
        }
        else if let Some(line_data) = serialized_message.get(UPDATE_LINE_EVENT)
        {
            self.handle_update_line_message(&line_data)?;
        }
        else if let Some(line_data) = serialized_message.get(DELETE_LINE_EVENT)
        {
            self.handle_delete_line_message(&line_data)?;
        }
        else if let Some(undo_data) = serialized_message.get(UNDO)
        {
            self.handle_undo_message(&undo_data)?;
        }
        else if let Some(redo_data) = serialized_message.get(REDO)
        {
            self.handle_redo_message(&redo_data)?;
        }
        else if let Some(selected_point_number) =
            serialized_message.get(SELECTED_POINT_NUMBER_EVENT)
        {
            self.handle_selected_point_number_message(&selected_point_number, show_object_info)?;
        }
        else if let Some(selected_line_number) =
            serialized_message.get(SELECTED_LINE_NUMBER_EVENT)
        {
            self.handle_selected_line_number_message(&selected_line_number, show_object_info)?;
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
            log(&format!("Actions router active actions: Action id: {:?}, action type: {:?}",
                action_id, action_type));
        }

        log(&format!("Actions router: The number of active actions: {}",
            self.active_actions.len()));
        for action in &self.undo_actions
        {
            let action_id = &action.action_id;
            let action_type = &action.action_type;
            log(&format!("Actions router undo actions: Action id: {:?}, action type: {:?}",
                action_id, action_type));
        }

        log(&format!("Actions router: The number of undo actions: {}", self.undo_actions.len()));
        Ok(())
    }


    pub fn add_whole_geometry_to_preprocessor(&self)
    {
        let is_action_id_should_be_increased = false;
        add_whole_geometry_to_preprocessor(is_action_id_should_be_increased);
    }
}
