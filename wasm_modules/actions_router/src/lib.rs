use serde_json::Value;
use wasm_bindgen::prelude::*;

mod external_functions;
use external_functions::common::log;
use external_functions::communication_with_geometry::
{
    add_point_to_geometry, update_point_in_geometry,
    delete_point_from_geometry, undo_delete_point_from_geometry,
    add_line_to_geometry, update_line_in_geometry,
    delete_line_from_geometry, undo_delete_line_from_geometry,
    show_point_info, show_line_info_from_geometry,
    add_whole_geometry_to_preprocessor
};

mod action;
use action::{Action, Coordinates, ObjectNumber, IsActionIdShouldBeIncreased, Handle, ObjectName};
use action::{GeometryActionType, ActionType};

mod methods_for_geometry_type_actions_handle;



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

const CHANGE_VIEW_EVENT: &str = "change_view";


pub struct IsActionShouldBeAddedToActiveActions(bool);


impl IsActionShouldBeAddedToActiveActions
{
    pub fn create(add_to_active_actions: bool) -> IsActionShouldBeAddedToActiveActions
    {
        IsActionShouldBeAddedToActiveActions(add_to_active_actions)
    }


    pub fn get_value(&self) -> bool
    {
        self.0
    }
}


#[wasm_bindgen]
pub struct ActionsRouter
{
    current_action: Option<(Action, IsActionShouldBeAddedToActiveActions)>,
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


    fn handle_undo_message(&mut self, undo_data: &Value) -> Result<(), JsValue> {
        let action_id = undo_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Redo action: \
                Action id could not be converted to u32!")))?;
        if let Some(position) = self.active_actions.iter().position(|action|
            action.action_id_same(action_id))
        {
            let undo_action = self.active_actions.remove(position);
            match &undo_action.get_action_type()
            {
                ActionType::GeometryActionType(geometry_action_type) =>
                    {
                        match geometry_action_type
                        {
                            GeometryActionType::AddPoint(point_number, _, _) =>
                                {
                                    let is_action_id_should_be_increased =
                                        IsActionIdShouldBeIncreased::create(false);
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::DeletePoint(point_number.clone(),
                                        is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions =
                                        IsActionShouldBeAddedToActiveActions::create(false);
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::UpdatePoint(point_number, old_coordinates,
                                new_coordinates, _) =>
                                {
                                    let is_action_id_should_be_increased =
                                        IsActionIdShouldBeIncreased::create(false);
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::UpdatePoint(point_number.clone(),
                                        new_coordinates.clone(), old_coordinates.clone(),
                                        is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions =
                                        IsActionShouldBeAddedToActiveActions::create(false);
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::DeletePoint(point_number, _) =>
                                {
                                    let is_action_id_should_be_increased =
                                        IsActionIdShouldBeIncreased::create(false);
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::UndoDeletePoint(point_number.clone(),
                                        is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions =
                                        IsActionShouldBeAddedToActiveActions::create(false);
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::AddLine(line_number, _, _, _) =>
                                {
                                    let is_action_id_should_be_increased =
                                        IsActionIdShouldBeIncreased::create(false);
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::DeleteLine(line_number.clone(),
                                        is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions =
                                        IsActionShouldBeAddedToActiveActions::create(false);
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::UpdateLine(line_number, old_start_point_number,
                                old_end_point_number, new_start_point_number,
                                new_end_point_number, _) =>
                                {
                                    let is_action_id_should_be_increased =
                                        IsActionIdShouldBeIncreased::create(false);
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::UpdateLine(line_number.clone(),
                                        new_start_point_number.clone(), new_end_point_number.clone(),
                                        old_start_point_number.clone(), old_end_point_number.clone(),
                                        is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions =
                                        IsActionShouldBeAddedToActiveActions::create(false);
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::DeleteLine(line_number, _) =>
                                {
                                    let is_action_id_should_be_increased =
                                        IsActionIdShouldBeIncreased::create(false);
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::UndoDeleteLine(line_number.clone(),
                                        is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions =
                                        IsActionShouldBeAddedToActiveActions::create(false);
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            _ => (),
                        }
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
            let add_to_active_actions =
                IsActionShouldBeAddedToActiveActions::create(true);
            self.current_action = Some((redo_action, add_to_active_actions));
        }
        Ok(())
    }


    fn handle_selected_point_number_message(&mut self, selected_point_number: &Value,
        show_object_info_handle: &js_sys::Function)
        -> Result<(), JsValue>
    {
        let point_number = selected_point_number.to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Show point info action: \
                Point number could not be converted to u32!")))?;
        let action_id = 0;
        let action_type = ActionType::ShowPointInfo(ObjectNumber::create(point_number),
            Handle::create(show_object_info_handle.clone()));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(false);
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    fn handle_selected_line_number_message(&mut self, selected_line_number: &Value,
        show_object_info_handle: &js_sys::Function) -> Result<(), JsValue>
    {
        let line_number = selected_line_number.to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Show line info action: \
                Line number could not be converted to u32!")))?;
        let action_id = 0;
        let action_type = ActionType::ShowLineInfo(ObjectNumber::create(line_number),
            Handle::create(show_object_info_handle.clone()));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(false);
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    fn handle_change_view_message(&mut self, view: &Value, change_view_handle: &js_sys::Function)
    {
        let selected_view = view["selectedView"].to_string();
        let action_id = 0;
        let action_type = ActionType::ChangeView(ObjectName::create(selected_view),
            Handle::create(change_view_handle.clone()));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(false);
        self.current_action = Some((action, add_to_active_actions));
    }


    fn handle_current_action(&mut self) -> Result<(), JsValue>
    {
        if let Some((action, add_to_active_actions)) =
            &self.current_action
        {
            let action_id = action.get_action_id();
            let action_type = &action.get_action_type();
            match action_type
            {
                ActionType::GeometryActionType(GeometryActionType::AddPoint(
                    point_number, coordinates,
                    is_action_id_should_be_increased)) =>
                    {
                        let number = point_number.get_number();
                        let x = coordinates.get_x();
                        let y = coordinates.get_y();
                        let z = coordinates.get_z();
                        add_point_to_geometry(action_id, number, x, y, z,
                            is_action_id_should_be_increased.get_value())?;
                        if add_to_active_actions.get_value() == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::GeometryActionType(GeometryActionType::UpdatePoint(
                    point_number, _, new_coordinates,
                    is_action_id_should_be_increased)) =>
                    {
                        let number = point_number.get_number();
                        let x = new_coordinates.get_x();
                        let y = new_coordinates.get_y();
                        let z = new_coordinates.get_z();
                        update_point_in_geometry(action_id, number, x, y, z,
                            is_action_id_should_be_increased.get_value())?;
                        if add_to_active_actions.get_value() == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::GeometryActionType(GeometryActionType::DeletePoint(
                    point_number, is_action_id_should_be_increased)) =>
                    {
                        let number = point_number.get_number();
                        delete_point_from_geometry(action_id, number,
                            is_action_id_should_be_increased.get_value())?;
                        if add_to_active_actions.get_value()
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::GeometryActionType(GeometryActionType::UndoDeletePoint(
                    point_number, is_action_id_should_be_increased)) =>
                    {
                        let number = point_number.get_number();
                        undo_delete_point_from_geometry(action_id, number,
                            is_action_id_should_be_increased.get_value())?;
                        if add_to_active_actions.get_value() == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::GeometryActionType(GeometryActionType::AddLine(
                    line_number, start_point_number, end_point_number,
                    is_action_id_should_be_increased)) =>
                    {
                        let number = line_number.get_number();
                        let start_point_number = start_point_number.get_number();
                        let end_point_number = end_point_number.get_number();
                        add_line_to_geometry(action_id, number, start_point_number,
                            end_point_number, is_action_id_should_be_increased.get_value())?;
                        if add_to_active_actions.get_value() == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::GeometryActionType(GeometryActionType::UpdateLine(
                    line_number, _, _, start_point_number,
                    end_point_number, is_action_id_should_be_increased)) =>
                    {
                        let number = line_number.get_number();
                        let start_point_number = start_point_number.get_number();
                        let end_point_number = end_point_number.get_number();
                        update_line_in_geometry(action_id, number, start_point_number,
                            end_point_number, is_action_id_should_be_increased.get_value())?;
                        if add_to_active_actions.get_value() == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::GeometryActionType(GeometryActionType::DeleteLine(
                    line_number, is_action_id_should_be_increased)) =>
                    {
                        let number = line_number.get_number();
                        delete_line_from_geometry(action_id, number,
                            is_action_id_should_be_increased.get_value())?;
                        if add_to_active_actions.get_value() == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::GeometryActionType(GeometryActionType::UndoDeleteLine(
                    line_number, is_action_id_should_be_increased)) =>
                    {
                        let number = line_number.get_number();
                        undo_delete_line_from_geometry(action_id, number,
                            is_action_id_should_be_increased.get_value())?;
                        if add_to_active_actions.get_value() == true
                        {
                            self.active_actions.push(action.clone());
                        }
                    },
                ActionType::ShowPointInfo(point_number, show_object_info_handle) =>
                    {
                        let number = point_number.get_number();
                        let point_info = show_point_info(number)?;
                        let this = JsValue::null();
                        let _ = show_object_info_handle.get_handle().call1(&this, &point_info)?;
                    },
                ActionType::ShowLineInfo(line_number, show_object_info_handle) =>
                    {
                        let number = line_number.get_number();
                        let line_info_from_geometry = show_line_info_from_geometry(number)?;
                        let this = JsValue::null();
                        let _ = show_object_info_handle.get_handle().call1(&this, &line_info_from_geometry)?;
                    },
               ActionType::ChangeView(selected_view_name, change_view_handle) =>
                   {
                       let view_name = selected_view_name.get_name();
                       let this = JsValue::null();
                       let _ = change_view_handle.get_handle().call1(&this, &JsValue::from(view_name))?;
                   }
            }
            self.current_action = None;
        }
        Ok(())
    }


    pub fn handle_message(&mut self, message: JsValue,
        show_object_info_handle: &js_sys::Function,
        change_view_handle: &js_sys::Function)
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
            self.handle_selected_point_number_message(&selected_point_number, show_object_info_handle)?;
        }
        else if let Some(selected_line_number) =
            serialized_message.get(SELECTED_LINE_NUMBER_EVENT)
        {
            self.handle_selected_line_number_message(&selected_line_number, show_object_info_handle)?;
        }
        else if let Some(view) = serialized_message.get(CHANGE_VIEW_EVENT)
        {
            self.handle_change_view_message(&view, change_view_handle);
        }
        else
        {
            let error_message = "Actions router: Message could not be handled!";
            return Err(JsValue::from(error_message));
        }
        self.handle_current_action()?;

        for action in &self.active_actions
        {
            let action_id = &action.get_action_id();
            let action_type = &action.get_action_type();
            log(&format!("Actions router active actions: Action id: {:?}, action type: {:?}",
                action_id, action_type));
        }
        log(&format!("Actions router: The number of active actions: {}",
            self.active_actions.len()));

        for action in &self.undo_actions
        {
            let action_id = &action.get_action_id();
            let action_type = &action.get_action_type();
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
