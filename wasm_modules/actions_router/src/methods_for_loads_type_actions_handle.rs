use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::action::{Action, ActionType, LoadsActionType, ConcentratedLoad};

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
}
