use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::{Coordinates, Action};
use crate::{ActionType, GeometryActionType};

use crate::types::{FEUInt, FEFloat};


impl ActionsRouter
{
    pub(super) fn handle_add_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
    {
        let action_id = point_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: Action id could not be converted to FEUInt!")))?;
        let number = point_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Add point action: \
                Point number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point number could not be converted to FEUInt!")))?;
        let x = point_data["x"].as_str()
            .ok_or(JsValue::from("Actions router: Add point action: \
                Point x coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point x coordinate could not be converted to FEFloat!")))?;
        let y = point_data["y"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: Point y coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point y coordinate could not be converted to FEFloat!")))?;
        let z = point_data["z"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add point action: Point z coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add point action: \
                Point z coordinate could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let coordinates = Coordinates::create(x, y, z);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::GeometryActionType(GeometryActionType::AddPoint(
            number, coordinates, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
    {
        let action_id = point_data["actionId"].to_string()
                .parse::<FEUInt>()
                .or(Err(JsValue::from("Actions router: Update point action: \
                    Action id could not be converted to FEUInt!")))?;
        let number = point_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update point action: Point number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point number could not be converted to FEUInt!")))?;
        let old_x_value = point_data["old_point_values"]["x"].to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point old x coordinate could not be converted to FEFloat!")))?;
        let old_y_value = point_data["old_point_values"]["y"].to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point old y coordinate could not be converted to FEFloat!")))?;
        let old_z_value = point_data["old_point_values"]["z"].to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Actions router: Update point action: \
                Point old z coordinate could not be converted to FEFloat!")))?;
        let new_x_value = point_data["new_point_values"]["x"].as_str()
            .ok_or(JsValue::from("Actions router: Update point action: \
                Point new x coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point new x value could not be converted to FEFloat!")))?;
        let new_y_value = point_data["new_point_values"]["y"].as_str()
            .ok_or(JsValue::from("Actions router: Update point action: \
                Point new y coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point new y value could not be converted to FEFloat!")))?;
        let new_z_value = point_data["new_point_values"]["z"].as_str()
            .ok_or(JsValue::from("Actions router: Update point action: \
                Point new z coordinate could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update point action: \
                Point new z value could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let old_coordinates = Coordinates::create(old_x_value,
            old_y_value, old_z_value);
        let new_coordinates = Coordinates::create(new_x_value,
            new_y_value, new_z_value);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::GeometryActionType(GeometryActionType::UpdatePoint(
            number, old_coordinates, new_coordinates, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_point_message(&mut self, point_data: &Value) -> Result<(), JsValue>
    {
        let action_id = point_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from( "Actions router: Delete point action: \
                Action id could not be converted to FEUInt!")))?;
        let number = point_data["number"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete point action: \
                Point number could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::GeometryActionType(GeometryActionType::DeletePoint(
            number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_add_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
    {
        let action_id = line_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from(
                "Actions router: Add point action: Action id could not be converted to FEUInt!")))?;
        let number = line_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add line action: Line number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add line action: \
                Line number could not be converted to FEUInt!")))?;
        let start_point_number = line_data["start_point_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add line action: \
                Line start point number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add line action: \
                Line start point number could not be converted to FEUInt!")))?;
        let end_point_number = line_data["end_point_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add line action: \
                Line end point number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add line action: \
                Line end point number could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::GeometryActionType(GeometryActionType::AddLine(
            number, start_point_number, end_point_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
    {
        let action_id = line_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Action id could not be converted to FEUInt!")))?;
        let number = line_data["number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update line action: Line number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line number could not be converted to FEUInt!")))?;
        let old_start_point_number = line_data["old_line_values"]["start_point"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line old start point number could not be converted to FEUInt!")))?;
        let old_end_point_number = line_data["old_line_values"]["end_point"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line old end point number could not be converted to FEUInt!")))?;
        let new_start_point_number = line_data["new_line_values"]["start_point"].as_str()
            .ok_or(JsValue::from("Actions router: Update line action: \
                Line new start point number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line new start point number could not be converted to FEUInt!")))?;
        let new_end_point_number = line_data["new_line_values"]["end_point"].as_str()
            .ok_or(JsValue::from("Actions router: Update line action: \
                Line new end point number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update line action: \
                Line new end point number could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::GeometryActionType(GeometryActionType::UpdateLine(
            number, old_start_point_number, old_end_point_number, new_start_point_number,
            new_end_point_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_line_message(&mut self, line_data: &Value) -> Result<(), JsValue>
    {
        let action_id = line_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete line action: \
                Action id could not be converted to FEUInt!")))?;
        let number = line_data["number"].as_str()
            .ok_or(JsValue::from("Actions router: Delete line action: \
                Line number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete line action: \
                Line number could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::GeometryActionType(GeometryActionType::DeleteLine(
            number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
