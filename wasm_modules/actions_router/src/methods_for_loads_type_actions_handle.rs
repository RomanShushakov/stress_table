use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::action::{Action, ActionType, LoadsActionType, ConcentratedLoad, DistributedLineLoad};

use crate::types::{FEUInt, FEFloat};


impl ActionsRouter
{
    pub(super) fn handle_add_concentrated_load_message(&mut self, concentrated_load_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = concentrated_load_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from(
                "Actions router: Add concentrated load action: Action id could not be converted \
                to FEUInt!")))?;
        let point_number = concentrated_load_data["point_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add concentrated load action: \
                Point number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add concentrated load action: \
                Point number could not be converted to FEUInt!")))?;

        if point_number > 10000 as FEUInt
        {
            return Err(JsValue::from("Actions router: Add concentrated load action: Point \
                number could not be greater than 10000!"));
        }

        let fx = concentrated_load_data["fx"].as_str()
            .ok_or(JsValue::from("Actions router: Add concentrated load action: \
                Load x component could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add concentrated load action: \
                Load x component could not be converted to FEFloat!")))?;
        let fy = concentrated_load_data["fy"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add concentrated load action: Load y component could not be \
                extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add concentrated load action: \
                Load y component could not be converted to FEFloat!")))?;
        let fz = concentrated_load_data["fz"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add concentrated load action: Load z component could not be \
                extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add concentrated load action: \
                Load z component could not be converted to FEFloat!")))?;
        let mx = concentrated_load_data["mx"].as_str()
            .ok_or(JsValue::from("Actions router: Add concentrated load action: \
                Moment x component could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add concentrated load action: \
                Moment x component could not be converted to FEFloat!")))?;
        let my = concentrated_load_data["my"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add concentrated load action: Moment y component could not be \
                extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add concentrated load action: \
                Moment y component could not be converted to FEFloat!")))?;
        let mz = concentrated_load_data["mz"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add concentrated load action: Moment z component could not be \
                extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add concentrated load action: \
                Moment z component could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let concentrated_load = ConcentratedLoad::create(fx, fy, fz, mx, my, mz);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(LoadsActionType::AddConcentratedLoad(
            point_number, concentrated_load, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_concentrated_load_message(&mut self, concentrated_load_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = concentrated_load_data["actionId"].to_string()
                .parse::<FEUInt>()
                .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                    Action id could not be converted to FEUInt!")))?;
        let point_number = concentrated_load_data["point_number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update concentrated load action: Point number could not be \
                extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Point number could not be converted to FEUInt!")))?;
        let old_fx_value = concentrated_load_data["old_concentrated_load_values"]["fx"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load old Fx value could not be converted to FEFloat!")))?;
        let old_fy_value = concentrated_load_data["old_concentrated_load_values"]["fy"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load old Fy value could not be converted to FEFloat!")))?;
        let old_fz_value = concentrated_load_data["old_concentrated_load_values"]["fz"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Actions router: Update concentrated load action: \
                Concentrated load old Fz value could not be converted to FEFloat!")))?;
        let old_mx_value = concentrated_load_data["old_concentrated_load_values"]["mx"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load old Mx value could not be converted to FEFloat!")))?;
        let old_my_value = concentrated_load_data["old_concentrated_load_values"]["my"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load old My value could not be converted to FEFloat!")))?;
        let old_mz_value = concentrated_load_data["old_concentrated_load_values"]["mz"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Actions router: Update concentrated load action: \
                Concentrated load old Mz value could not be converted to FEFloat!")))?;
        let new_fx_value = concentrated_load_data["new_concentrated_load_values"]["fx"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Fx value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Fx value could not be converted to FEFloat!")))?;
        let new_fy_value = concentrated_load_data["new_concentrated_load_values"]["fy"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Fy value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Fy value could not be converted to FEFloat!")))?;
        let new_fz_value = concentrated_load_data["new_concentrated_load_values"]["fz"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Fz value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Fz value could not be converted to FEFloat!")))?;
        let new_mx_value = concentrated_load_data["new_concentrated_load_values"]["mx"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Mx value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Mx value could not be converted to FEFloat!")))?;
        let new_my_value = concentrated_load_data["new_concentrated_load_values"]["my"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new My value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new My value could not be converted to FEFloat!")))?;
        let new_mz_value = concentrated_load_data["new_concentrated_load_values"]["mz"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Mz value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update concentrated load action: \
                Concentrated load new Mz value could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let old_concentrated_load = ConcentratedLoad::create(old_fx_value,
            old_fy_value, old_fz_value, old_mx_value, old_my_value, old_mz_value);
        let new_concentrated_load = ConcentratedLoad::create(new_fx_value,
            new_fy_value, new_fz_value, new_mx_value, new_my_value, new_mz_value);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            LoadsActionType::UpdateConcentratedLoad(point_number,
            old_concentrated_load, new_concentrated_load,
            is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_concentrated_load_message(&mut self, concentrated_load_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = concentrated_load_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from( "Actions router: Delete concentrated load action: \
                Action id could not be converted to FEUInt!")))?;
        let point_number = concentrated_load_data["point_number"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete concentrated load action: \
                Point number could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            LoadsActionType::DeleteConcentratedLoad(
                point_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_add_distributed_line_load_message(&mut self,
        distributed_line_load_data: &Value) -> Result<(), JsValue>
    {
        let action_id = distributed_line_load_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from(
                "Actions router: Add distributed line load action: Action id could not be \
                converted to FEUInt!")))?;
        let line_number = distributed_line_load_data["line_number"].as_str()
            .ok_or(JsValue::from("Actions router: Add distributed line load action: \
                Line number could not be extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add distributed line load action: \
                Line number could not be converted to FEUInt!")))?;

        if line_number > 10000 as FEUInt
        {
            return Err(JsValue::from("Actions router: Add distributed line load action: Line \
                number could not be greater than 10000!"));
        }

        let qx = distributed_line_load_data["qx"].as_str()
            .ok_or(JsValue::from("Actions router: Add distributed line load action: \
                Distributed line load x component could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add distributed line load action: \
                Distributed line load x component could not be converted to FEFloat!")))?;
        let qy = distributed_line_load_data["qy"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add distributed line load action: Distributed line load y \
                component could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add distributed line load action: \
                Distributed line load y component could not be converted to FEFloat!")))?;
        let qz = distributed_line_load_data["qz"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add distributed line load action: Distributed line load z \
                component could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add distributed line load action: \
                Distributed line load z component could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let distributed_line_load = DistributedLineLoad::create(qx, qy, qz);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            LoadsActionType::AddDistributedLineLoad(
                line_number, distributed_line_load,
                is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_distributed_line_load_message(&mut self,
        distributed_line_load_data: &Value) -> Result<(), JsValue>
    {
        let action_id = distributed_line_load_data["actionId"].to_string()
                .parse::<FEUInt>()
                .or(Err(JsValue::from("Actions router: Update distributed line load action: \
                    Action id could not be converted to FEUInt!")))?;
        let line_number = distributed_line_load_data["line_number"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Update distributed line load action: Line number could not be \
                extracted!"))?
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update distributed line load action: \
                Line number could not be converted to FEUInt!")))?;
        let old_qx_value =
            distributed_line_load_data["old_distributed_line_load_values"]["qx"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update distributed line load action: \
                Distributed line load old Qx value could not be converted to FEFloat!")))?;
        let old_qy_value =
            distributed_line_load_data["old_distributed_line_load_values"]["qy"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update distributed line load action: \
                Distributed line load old Qy value could not be converted to FEFloat!")))?;
        let old_qz_value =
            distributed_line_load_data["old_distributed_line_load_values"]["qz"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Actions router: Update distributed line load action: \
                Distributed line load old Qz value could not be converted to FEFloat!")))?;
        let new_qx_value =
            distributed_line_load_data["new_distributed_line_load_values"]["qx"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update distributed line load action: \
                Distributed line load new Qx value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update distributed line load action: \
                Distributed line load new Qx value could not be converted to FEFloat!")))?;
        let new_qy_value =
            distributed_line_load_data["new_distributed_line_load_values"]["qy"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update distributed line load action: \
                Distributed line load new Qy value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update distributed line load action: \
                Distributed line load new Qy value could not be converted to FEFloat!")))?;
        let new_qz_value =
            distributed_line_load_data["new_distributed_line_load_values"]["qz"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update distributed line load action: \
                Distributed line load new Qz value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update distributed line load action: \
                Distributed line load new Qz value could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let old_distributed_line_load = DistributedLineLoad::create(
            old_qx_value, old_qy_value, old_qz_value);
        let new_distributed_line_load = DistributedLineLoad::create(
            new_qx_value, new_qy_value, new_qz_value);
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            LoadsActionType::UpdateDistributedLineLoad(line_number,
            old_distributed_line_load, new_distributed_line_load,
            is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_distributed_line_load_message(&mut self,
        distributed_line_load_data: &Value) -> Result<(), JsValue>
    {
        let action_id = distributed_line_load_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from( "Actions router: Delete distributed line load action: \
                Action id could not be converted to FEUInt!")))?;
        let line_number = distributed_line_load_data["line_number"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete distributed line load action: \
                Line number could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            LoadsActionType::DeleteDistributedLineLoad(
                line_number, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
