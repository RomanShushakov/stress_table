use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::action::{Action, ActionType, BoundaryConditionsActionType, BoundaryCondition};

use crate::types::{FEUInt, FEFloat};


impl ActionsRouter
{
    pub(super) fn handle_add_boundary_condition_message(&mut self, boundary_condition_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = boundary_condition_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from(
                "Actions router: Add boundary condition action: Action id could not be converted \
                to FEUInt!")))?;
        let point_number = boundary_condition_data["point_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add boundary condition action: \
                Point number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add boundary condition action: \
                Point number could not be converted to FEUInt!")))?;

        if point_number > 10000 as FEUInt
        {
            return Err(JsValue::from("Actions router: Add boundary condition action: Point \
                number could not be greater than 10000!"));
        }

        let optional_ux =
            {
                if boundary_condition_data["ux"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["ux"].as_str()
                        .ok_or(JsValue::from("Actions router: Add boundary condition \
                            action: Displacement x component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add boundary condition \
                            action: Displacement x component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_uy =
            {
                if boundary_condition_data["uy"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["uy"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add boundary condition action: Displacement y \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add boundary condition \
                            action: Displacement y component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_uz =
            {
                if boundary_condition_data["uz"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["uz"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add boundary condition action: Displacement z \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add boundary condition \
                            action: Displacement z component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_rx =
            {
                if boundary_condition_data["rx"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["rx"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add boundary condition action: Rotation x \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add boundary condition \
                            action: Rotation x component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_ry =
            {
                if boundary_condition_data["ry"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["ry"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add boundary condition action: Rotation y \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add boundary condition \
                            action: Rotation y component could not be converted to \
                            FEFloat!")))?)
                }
            };
        let optional_rz =
            {
                if boundary_condition_data["rz"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["rz"].as_str()
                        .ok_or(JsValue::from(
                            "Actions router: Add boundary condition action: Rotation z \
                            component could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add boundary condition \
                            action: Rotation z component could not be converted to \
                            FEFloat!")))?)
                }
            };

        self.undo_actions.clear();
        let boundary_condition = BoundaryCondition::create(optional_ux,
            optional_uy, optional_uz, optional_rx, optional_ry, optional_rz);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            BoundaryConditionsActionType::AddBoundaryCondition(
                point_number, boundary_condition,
                is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_boundary_condition_message(&mut self,
        boundary_condition_data: &Value) -> Result<(), JsValue>
    {
        let action_id = boundary_condition_data["actionId"].to_string()
                .parse::<FEUInt>()
                .or(Err(JsValue::from("Actions router: Update boundary condition action: \
                    Action id could not be converted to FEUInt!")))?;
        let point_number = boundary_condition_data["point_number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update boundary condition action: Point number could not be \
                extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update boundary condition action: \
                Point number could not be converted to FEUInt!")))?;

        let old_optional_ux_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["ux"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["ux"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: Old displacement x component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_uy_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["uy"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["uy"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: Old displacement y component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_uz_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["uz"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["uz"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: Old displacement z component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_rx_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["rx"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["rx"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: Old rotation x component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_ry_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["ry"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["ry"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: Old rotation y component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let old_optional_rz_value =
            {
                if boundary_condition_data["old_boundary_condition_values"]["rz"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["old_boundary_condition_values"]["rz"]
                        .to_string()
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: Old rotation z component value could not be converted to \
                            FEFloat!")))?)
                }
            };

        let new_optional_ux_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["ux"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["ux"].as_str()
                        .ok_or(JsValue::from("Actions router: Update boundary condition \
                            action: New displacement x component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: New displacement x component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_uy_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["uy"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["uy"].as_str()
                        .ok_or(JsValue::from("Actions router: Update boundary condition \
                            action: New displacement y component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: New displacement y component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_uz_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["uz"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["fx"].as_str()
                        .ok_or(JsValue::from("Actions router: Update boundary condition \
                            action: New displacement z component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: New displacement z component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_rx_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["rx"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["rx"].as_str()
                        .ok_or(JsValue::from("Actions router: Update boundary condition \
                            action: New rotation x component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: New rotation x component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_ry_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["ry"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["ry"].as_str()
                        .ok_or(JsValue::from("Actions router: Update boundary condition \
                            action: New rotation y component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: New rotation y component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        let new_optional_rz_value =
            {
                if boundary_condition_data["new_boundary_condition_values"]["rz"].is_null()
                {
                    None
                }
                else
                {
                    Some(boundary_condition_data["new_boundary_condition_values"]["rz"].as_str()
                        .ok_or(JsValue::from("Actions router: Update boundary condition \
                            action: New rotation z component value could not be extracted!"))?
                        .parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update boundary condition \
                            action: New rotation z component value could not be converted to \
                            FEFloat!")))?)
                }
            };
        self.undo_actions.clear();
        let old_boundary_condition = BoundaryCondition::create(
            old_optional_ux_value, old_optional_uy_value,
            old_optional_uy_value, old_optional_rx_value,
            old_optional_ry_value, old_optional_rz_value);
        let new_boundary_condition = BoundaryCondition::create(
            new_optional_ux_value, new_optional_uy_value,
            new_optional_uz_value, new_optional_rx_value,
            new_optional_ry_value, new_optional_rz_value);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            BoundaryConditionsActionType::UpdateBoundaryCondition(point_number,
            old_boundary_condition, new_boundary_condition,
            is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_boundary_condition_message(&mut self,
        boundary_condition_data: &Value) -> Result<(), JsValue>
    {
        let action_id = boundary_condition_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from( "Actions router: Delete boundary condition action: \
                Action id could not be converted to FEUInt!")))?;
        let point_number = boundary_condition_data["point_number"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete boundary condition action: \
                Point number could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            BoundaryConditionsActionType::DeleteBoundaryCondition(
                point_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
