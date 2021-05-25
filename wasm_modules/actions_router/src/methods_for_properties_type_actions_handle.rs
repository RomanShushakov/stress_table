use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::{Action};
use crate::{ActionType, PropertiesActionType};


impl ActionsRouter
{
    pub(super) fn handle_add_material_message(&mut self, material_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = material_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add material action: Action id could \
                not be converted to u32!")))?;
        let name = material_data["name"].to_string();
        let young_modulus = material_data["young_modulus"].as_str()
            .ok_or(JsValue::from("Actions router: Add material action: \
                Young's modulus value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add material action: \
                Young's modulus value could not be converted to f64!")))?;
        let poisson_ratio = material_data["poisson_ratio"].as_str()
            .ok_or(JsValue::from("Actions router: Add material action: Poisson's ratio value \
                could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add material action: \
                Poisson's ratio value could not be converted to f64!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::PropertiesActionType(PropertiesActionType::AddMaterial(
                name, young_modulus, poisson_ratio, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_material_message(&mut self, material_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = material_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update material action: Action id could \
                not be converted to u32!")))?;
        let name = material_data["name"].to_string();
        let old_young_modulus = material_data["old_material_values"]["young_modulus"]
            .to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update material action: \
                Material old Young's modulus value could not be converted to f64!")))?;
        let old_poisson_ratio = material_data["old_material_values"]["poisson_ratio"]
            .to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update material action: \
                Material old Poisson's ratio value could not be converted to f64!")))?;
        let new_young_modulus = material_data["new_material_values"]["young_modulus"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update material action: \
                Material new Young's modulus value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update material action: \
                Material new Young's modulus value could not be converted to f64!")))?;
        let new_poisson_ratio = material_data["new_material_values"]["poisson_ratio"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update material action: \
                Material new Poisson's ratio value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update material action: \
                Material new Poisson's ratio value could not be converted to f64!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::PropertiesActionType(PropertiesActionType::UpdateMaterial(
            name, old_young_modulus, old_poisson_ratio, new_young_modulus,
            new_poisson_ratio, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_material_message(&mut self, material_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = material_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete material action: \
                Action id could not be converted to u32!")))?;
        self.undo_actions.clear();
        let name = material_data["name"].to_string();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::PropertiesActionType(PropertiesActionType::DeleteMaterial(
            name, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
