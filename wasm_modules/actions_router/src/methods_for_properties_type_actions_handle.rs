use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::ActionsRouter;
use crate::{Action};
use crate::{ActionType, PropertiesActionType};
use crate::external_functions::common::log;


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


    pub(super) fn handle_add_truss_section_message(&mut self, truss_section_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = truss_section_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add truss section action: Action id could \
                not be converted to u32!")))?;
        let name = truss_section_data["name"].to_string();
        let area = truss_section_data["area"].as_str()
            .ok_or(JsValue::from("Actions router: Add truss section action: \
                Area value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add truss section action: \
                Area value could not be converted to f64!")))?;
        let area2_value = truss_section_data["area2"].as_str()
            .ok_or(JsValue::from("Actions router: Add truss section action: Area 2 value \
                could not be extracted!"))?;
        let area2 =
            {
                if area2_value.is_empty()
                {
                    None
                }
                else
                {
                    let converted_area2 = area2_value.parse::<f64>()
                        .or(Err(JsValue::from("Actions router: Add truss section action: \
                            Area 2 value could not be converted to f64!")))?;
                    Some(converted_area2)
                }
            };
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::PropertiesActionType(PropertiesActionType::AddTrussSection(
                name, area, area2, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_truss_section_message(&mut self, truss_section_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = truss_section_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update truss section action: \
                Action id could not be converted to u32!")))?;
        let name = truss_section_data["name"].to_string();
        let old_area = truss_section_data["old_truss_section_values"]["area"]
            .to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update truss section action: \
                Truss section old Area value could not be converted to f64!")))?;
        let old_area2 =
            {
                if truss_section_data["old_truss_section_values"]["area2"].is_null()
                {
                    None
                }
                else
                {
                    let converted_area2 =
                        truss_section_data["old_truss_section_values"]["area2"]
                            .to_string()
                            .parse::<f64>()
                            .or(Err(JsValue::from("Actions router: Update material action: \
                                Truss section old Area 2 value could not be converted to f64!")))?;
                    Some(converted_area2)
                }
            };
        let new_area = truss_section_data["new_truss_section_values"]["area"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update truss section action: \
                Truss section new Area value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update truss section action: \
                Truss section new Area value could not be converted to f64!")))?;
        let new_area2 =
            {
                let new_area2_value = truss_section_data["new_truss_section_values"]["area2"]
                    .as_str()
                    .ok_or(JsValue::from("Actions router: Update truss section \
                        action: Truss section new Area 2 value could not be extracted!"))?;
                if new_area2_value.is_empty()
                {
                    None
                }
                else
                {
                    let converted_area2 = new_area2_value.parse::<f64>()
                        .or(Err(JsValue::from("Actions router: Update truss section \
                            action: Truss section new Area 2 value could not be converted \
                            to f64!")))?;
                    Some(converted_area2)
                }
            };
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::PropertiesActionType(PropertiesActionType::UpdateTrussSection(
            name, old_area, old_area2, new_area, new_area2, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_truss_section_message(&mut self, truss_section_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = truss_section_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete truss section action: \
                Action id could not be converted to u32!")))?;
        self.undo_actions.clear();
        let name = truss_section_data["name"].to_string();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::PropertiesActionType(PropertiesActionType::DeleteTrussSection(
            name, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_add_beam_section_message(&mut self, beam_section_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = beam_section_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Add beam section action: Action id could \
                not be converted to u32!")))?;
        let name = beam_section_data["name"].to_string();
        let area = beam_section_data["area"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                Area value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                Area value could not be converted to f64!")))?;
        let i11 = beam_section_data["I11"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                I11 value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                I11 value could not be converted to f64!")))?;
        let i22 = beam_section_data["I22"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                I22 value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                I22 value could not be converted to f64!")))?;
        let i12 = beam_section_data["I12"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                I12 value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                I12 value could not be converted to f64!")))?;
        let it = beam_section_data["It"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                It value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                It value could not be converted to f64!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::PropertiesActionType(PropertiesActionType::AddBeamSection(
                name, area, i11, i22, i12, it, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_beam_section_message(&mut self, beam_section_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = beam_section_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Action id could not be converted to u32!")))?;
        let name = beam_section_data["name"].to_string();
        let old_area = beam_section_data["old_beam_section_values"]["area"]
            .to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old Area value could not be converted to f64!")))?;
        let old_i11 = beam_section_data["old_beam_section_values"]["I11"]
            .to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old I11 value could not be converted to f64!")))?;
        let old_i22 = beam_section_data["old_beam_section_values"]["I22"]
            .to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old I22 value could not be converted to f64!")))?;
        let old_i12 = beam_section_data["old_beam_section_values"]["I12"]
            .to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old I12 value could not be converted to f64!")))?;
        let old_it = beam_section_data["old_beam_section_values"]["It"]
            .to_string()
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old It value could not be converted to f64!")))?;
        let new_area = beam_section_data["new_beam_section_values"]["area"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new Area value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new Area value could not be converted to f64!")))?;
        let new_i11 = beam_section_data["new_beam_section_values"]["I11"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new I11 value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new I11 value could not be converted to f64!")))?;
        let new_i22 = beam_section_data["new_beam_section_values"]["I22"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new I22 value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new I22 value could not be converted to f64!")))?;
        let new_i12 = beam_section_data["new_beam_section_values"]["I12"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new I12 value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new I12 value could not be converted to f64!")))?;
        let new_it = beam_section_data["new_beam_section_values"]["It"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new It value could not be extracted!"))?
            .parse::<f64>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new It value could not be converted to f64!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::PropertiesActionType(PropertiesActionType::UpdateBeamSection(
            name, old_area, old_i11, old_i22, old_i12, old_it, new_area,
            new_i11, new_i22, new_i12, new_it, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_beam_section_message(&mut self, beam_section_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = beam_section_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Delete beam section action: \
                Action id could not be converted to u32!")))?;
        self.undo_actions.clear();
        let name = beam_section_data["name"].to_string();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::PropertiesActionType(PropertiesActionType::DeleteBeamSection(
            name, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
