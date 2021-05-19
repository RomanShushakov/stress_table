use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::{ObjectNumber, Coordinates, Action, IsActionShouldBeAddedToActiveActions};
use crate::{ActionType, GeometryActionType};
use crate::action::IsActionIdShouldBeIncreased;


impl ActionsRouter
{
    pub(super) fn handle_add_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
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
        let is_action_id_should_be_increased =
            IsActionIdShouldBeIncreased::create(true);
        let action_type =
            ActionType::GeometryActionType(GeometryActionType::AddPoint(
                point_number, coordinates, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(true);
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
    {
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
        let is_action_id_should_be_increased =
            IsActionIdShouldBeIncreased::create(true);
        let action_type = ActionType::GeometryActionType(GeometryActionType::UpdatePoint(
            point_number,old_coordinates, new_coordinates, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(true);
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
    {
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
        let is_action_id_should_be_increased =
            IsActionIdShouldBeIncreased::create(true);
        let action_type = ActionType::GeometryActionType(GeometryActionType::DeletePoint(
            point_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(true);
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_add_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
    {
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
        let is_action_id_should_be_increased =
            IsActionIdShouldBeIncreased::create(true);
        let action_type = ActionType::GeometryActionType(GeometryActionType::AddLine(
            line_number, start_point_number, end_point_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(true);
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
    {
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
        let is_action_id_should_be_increased =
            IsActionIdShouldBeIncreased::create(true);
        let action_type = ActionType::GeometryActionType(GeometryActionType::UpdateLine(
            line_number, old_start_point_number, old_end_point_number, new_start_point_number,
            new_end_point_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(true);
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
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
        let is_action_id_should_be_increased =
            IsActionIdShouldBeIncreased::create(true);
        let action_type = ActionType::GeometryActionType(GeometryActionType::DeleteLine(
            line_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(true);
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
