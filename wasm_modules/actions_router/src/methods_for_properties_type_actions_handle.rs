use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::{ObjectF64Number, ObjectName, Action, IsActionShouldBeAddedToActiveActions};
use crate::{ActionType, PropertiesActionType};
use crate::action::IsActionIdShouldBeIncreased;


impl ActionsRouter
{
    pub(super) fn handle_add_material_message(&mut self, material_data: &Value) -> Result<(), JsValue>
    {
        let action_id = material_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Actions router: Add material action: Action id could not be converted to u32!")))?;
        let name = material_data["name"].to_string();
        let young_modulus = material_data["young_modulus"].as_str()
            .ok_or(JsValue::from("Actions router: Add material action: \
                Young's modulus could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add material action: \
                Young's modulus could not be converted to f64!")))?;
        let poisson_ratio = material_data["poisson_ratio"].as_str()
            .ok_or(JsValue::from(
                "Actions router: Add material action: Poisson's ratio could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add material action: \
                Poisson's ratio could not be converted to f64!")))?;
        self.undo_actions.clear();
        let material_name = ObjectName::create(name);
        let young_modulus = ObjectF64Number::create(young_modulus);
        let poisson_ratio = ObjectF64Number::create(poisson_ratio);
        let is_action_id_should_be_increased =
            IsActionIdShouldBeIncreased::create(true);
        let action_type =
            ActionType::PropertiesActionType(PropertiesActionType::AddMaterial(
                material_name, young_modulus, poisson_ratio, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions =
            IsActionShouldBeAddedToActiveActions::create(true);
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}