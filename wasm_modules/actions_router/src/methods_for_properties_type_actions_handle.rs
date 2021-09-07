use serde_json::Value;
use wasm_bindgen::prelude::*;
use std::convert::TryFrom;

use crate::ActionsRouter;
use crate::action::{Action, ActionType, PropertiesActionType};
use crate::external_functions::common::log;

use crate::types::{FEUInt, FEFloat};


impl ActionsRouter
{
    pub(super) fn handle_add_material_message(&mut self, material_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = material_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add material action: Action id could \
                not be converted to FEUInt!")))?;
        let name = material_data["name"].to_string();
        let young_modulus = material_data["young_modulus"].as_str()
            .ok_or(JsValue::from("Actions router: Add material action: \
                Young's modulus value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add material action: \
                Young's modulus value could not be converted to FEFloat!")))?;
        let poisson_ratio = material_data["poisson_ratio"].as_str()
            .ok_or(JsValue::from("Actions router: Add material action: Poisson's ratio value \
                could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add material action: \
                Poisson's ratio value could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::AddMaterial(
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
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update material action: Action id could \
                not be converted to FEUInt!")))?;
        let name = material_data["name"].to_string();
        let old_young_modulus = material_data["old_material_values"]["young_modulus"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update material action: \
                Material old Young's modulus value could not be converted to FEFloat!")))?;
        let old_poisson_ratio = material_data["old_material_values"]["poisson_ratio"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update material action: \
                Material old Poisson's ratio value could not be converted to FEFloat!")))?;
        let new_young_modulus = material_data["new_material_values"]["young_modulus"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update material action: \
                Material new Young's modulus value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update material action: \
                Material new Young's modulus value could not be converted to FEFloat!")))?;
        let new_poisson_ratio = material_data["new_material_values"]["poisson_ratio"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update material action: \
                Material new Poisson's ratio value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update material action: \
                Material new Poisson's ratio value could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::UpdateMaterial(
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
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete material action: \
                Action id could not be converted to FEUInt!")))?;
        let name = material_data["name"].to_string();
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::DeleteMaterial(
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
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add truss section action: Action id could \
                not be converted to FEUInt!")))?;
        let name = truss_section_data["name"].to_string();
        let area = truss_section_data["area"].as_str()
            .ok_or(JsValue::from("Actions router: Add truss section action: \
                Area value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add truss section action: \
                Area value could not be converted to FEFloat!")))?;
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
                    let converted_area2 = area2_value.parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Add truss section action: \
                            Area 2 value could not be converted to FEFloat!")))?;
                    Some(converted_area2)
                }
            };
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::AddTrussSection(
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
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update truss section action: \
                Action id could not be converted to FEUInt!")))?;
        let name = truss_section_data["name"].to_string();
        let old_area = truss_section_data["old_truss_section_values"]["area"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update truss section action: \
                Truss section old Area value could not be converted to FEFloat!")))?;
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
                            .parse::<FEFloat>()
                            .or(Err(JsValue::from("Actions router: Update material action: \
                                Truss section old Area 2 value could not be converted to FEFloat!")))?;
                    Some(converted_area2)
                }
            };
        let new_area = truss_section_data["new_truss_section_values"]["area"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update truss section action: \
                Truss section new Area value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update truss section action: \
                Truss section new Area value could not be converted to FEFloat!")))?;
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
                    let converted_area2 = new_area2_value.parse::<FEFloat>()
                        .or(Err(JsValue::from("Actions router: Update truss section \
                            action: Truss section new Area 2 value could not be converted \
                            to FEFloat!")))?;
                    Some(converted_area2)
                }
            };
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::UpdateTrussSection(
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
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete truss section action: \
                Action id could not be converted to FEUInt!")))?;
        let name = truss_section_data["name"].to_string();
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::DeleteTrussSection(
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
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add beam section action: Action id could \
                not be converted to FEUInt!")))?;
        let name = beam_section_data["name"].to_string();
        let area = beam_section_data["area"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                Area value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                Area value could not be converted to FEFloat!")))?;
        let i11 = beam_section_data["i11"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                I11 value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                I11 value could not be converted to FEFloat!")))?;
        let i22 = beam_section_data["i22"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                I22 value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                I22 value could not be converted to FEFloat!")))?;
        let i12 = beam_section_data["i12"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                I12 value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                I12 value could not be converted to FEFloat!")))?;
        let it = beam_section_data["it"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                It value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                It value could not be converted to FEFloat!")))?;
        let shear_factor = beam_section_data["shear_factor"].as_str()
            .ok_or(JsValue::from("Actions router: Add beam section action: \
                Shear factor value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Add beam section action: \
                Shear factor value could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::AddBeamSection(
                name, area, i11, i22, i12, it,
                shear_factor, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_beam_section_message(&mut self, beam_section_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = beam_section_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Action id could not be converted to FEUInt!")))?;
        let name = beam_section_data["name"].to_string();
        let old_area = beam_section_data["old_beam_section_values"]["area"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old Area value could not be converted to FEFloat!")))?;
        let old_i11 = beam_section_data["old_beam_section_values"]["i11"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old I11 value could not be converted to FEFloat!")))?;
        let old_i22 = beam_section_data["old_beam_section_values"]["i22"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old I22 value could not be converted to FEFloat!")))?;
        let old_i12 = beam_section_data["old_beam_section_values"]["i12"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old I12 value could not be converted to FEFloat!")))?;
        let old_it = beam_section_data["old_beam_section_values"]["it"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old It value could not be converted to FEFloat!")))?;
        let old_shear_factor = beam_section_data["old_beam_section_values"]["shear_factor"]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section old shear factor value could not be converted to FEFloat!")))?;
        let new_area = beam_section_data["new_beam_section_values"]["area"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new Area value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new Area value could not be converted to FEFloat!")))?;
        let new_i11 = beam_section_data["new_beam_section_values"]["i11"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new I11 value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new I11 value could not be converted to FEFloat!")))?;
        let new_i22 = beam_section_data["new_beam_section_values"]["i22"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new I22 value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new I22 value could not be converted to FEFloat!")))?;
        let new_i12 = beam_section_data["new_beam_section_values"]["i12"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new I12 value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new I12 value could not be converted to FEFloat!")))?;
        let new_it = beam_section_data["new_beam_section_values"]["it"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new It value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new It value could not be converted to FEFloat!")))?;
        let new_shear_factor = beam_section_data["new_beam_section_values"]["shear_factor"]
            .as_str()
            .ok_or(JsValue::from("Actions router: Update beam section action: \
                Beam section new shear factor value could not be extracted!"))?
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Actions router: Update beam section action: \
                Beam section new It value could not be converted to FEFloat!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::UpdateBeamSection(
            name, old_area, old_i11, old_i22, old_i12,
            old_it, old_shear_factor, new_area, new_i11,
            new_i22, new_i12, new_it, new_shear_factor,
            is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_beam_section_message(&mut self, beam_section_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = beam_section_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete beam section action: \
                Action id could not be converted to FEUInt!")))?;
        let name = beam_section_data["name"].to_string();
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::DeleteBeamSection(
            name, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_add_properties_message(&mut self, properties_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = properties_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add properties action: Action id could \
                not be converted to FEUInt!")))?;
        let name = properties_data["name"].to_string();
        let material_name = properties_data["material_name"].to_string();
        let cross_section_name = properties_data["cross_section_name"].to_string();
        let cross_section_type = properties_data["cross_section_type"].to_string();
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::AddProperties(
                name, material_name, cross_section_name, cross_section_type,
                is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_properties_message(&mut self, properties_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = properties_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update properties action: \
                Action id could not be converted to FEUInt!")))?;
        let name = properties_data["name"].to_string();
        let old_material_name = properties_data["old_properties_values"]
            ["material_name"].to_string();
        let old_cross_section_name = properties_data["old_properties_values"]
            ["cross_section_name"].to_string();
        let old_cross_section_type = properties_data["old_properties_values"]
            ["cross_section_type"].to_string();
        let new_material_name = properties_data["new_properties_values"]
            ["material_name"].to_string();
        let new_cross_section_name = properties_data["new_properties_values"]
            ["cross_section_name"].to_string();
        let new_cross_section_type = properties_data["new_properties_values"]
            ["cross_section_type"].to_string();
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::UpdateProperties(
            name, old_material_name, old_cross_section_name, old_cross_section_type,
            new_material_name, new_cross_section_name, new_cross_section_type,
            is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_properties_message(&mut self, properties_data: &Value)
        -> Result<(), JsValue>
    {
        let action_id = properties_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete properties action: \
                Action id could not be converted to FEUInt!")))?;
        let name = properties_data["name"].to_string();
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(PropertiesActionType::DeleteProperties(
            name, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_add_assigned_properties_to_lines_message(&mut self,
        assigned_properties_to_lines_data: &Value) -> Result<(), JsValue>
    {
        let action_id = assigned_properties_to_lines_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add assigned properties to lines action: \
                Action id could not be converted to FEUInt!")))?;
        let name = assigned_properties_to_lines_data["name"].to_string();
        let line_numbers = serde_json::from_value::<Vec<FEUInt>>(
            assigned_properties_to_lines_data["line_numbers"].clone())
            .or(Err(JsValue::from("Actions router: Add assigned properties to lines action: \
                Line numbers could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            PropertiesActionType::AddAssignedPropertiesToLines(name,
            line_numbers, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_assigned_properties_to_lines_message(&mut self,
        assigned_properties_to_lines_data: &Value) -> Result<(), JsValue>
    {
        let action_id = assigned_properties_to_lines_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update assigned properties to lines action: \
                Action id could not be converted to FEUInt!")))?;
        let name = assigned_properties_to_lines_data["name"].to_string();

        let old_line_numbers = serde_json::from_value::<Vec<FEUInt>>(
            assigned_properties_to_lines_data["old_assigned_properties_to_lines_values"]
                ["line_numbers"].clone())
            .or(Err(JsValue::from("Actions router: Update assigned properties to lines action: \
                Old line numbers could not be converted to FEUInt!")))?;
        let new_line_numbers = serde_json::from_value::<Vec<FEUInt>>(
            assigned_properties_to_lines_data["new_assigned_properties_to_lines_values"]
                ["line_numbers"].clone())
            .or(Err(JsValue::from("Actions router: Update assigned properties to lines action: \
                New line numbers could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            PropertiesActionType::UpdateAssignedPropertiesToLines(name,
            old_line_numbers, new_line_numbers,
            is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_delete_assigned_properties_to_lines_message(&mut self,
        assigned_properties_to_lines_data: &Value) -> Result<(), JsValue>
    {
        let action_id = assigned_properties_to_lines_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Delete assigned properties to lines action: \
                Action id could not be converted to FEUInt!")))?;
        let name = assigned_properties_to_lines_data["name"].to_string();
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            PropertiesActionType::DeleteAssignedPropertiesToLines(name,
            is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_add_beam_section_local_axis_1_direction_message(&mut self,
        local_axis_1_direction_data: &Value) -> Result<(), JsValue>
    {
        let action_id = local_axis_1_direction_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Add beam section local axis 1 direction \
                action: Action id could not be converted to FEUInt!")))?;
        let local_axis_1_direction = local_axis_1_direction_data["local_axis_1_direction"]
            .to_string()
            .replace("[", "")
            .replace("]", "")
            .split(",")
            .map(|num|
                {
                    match num.parse::<FEFloat>()
                    {
                        Ok(n) => Ok(n),
                        Err(_e) =>
                            {
                                Err(JsValue::from("Actions router: Add beam section local axis 1 \
                                    direction action: Local axis 1 direction coordinate could not \
                                    be converted to FEFloat!"))
                            }
                    }
                })
            .collect::<Result<Vec<FEFloat>, JsValue>>()?;
        if local_axis_1_direction.len() != 3
        {
            let error_message = "Actions router: Add beam section local axis 1 direction \
                action: Incorrect number of coordinates for local axis 1 direction!";
            return Err(JsValue::from(error_message));
        }
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            PropertiesActionType::AddBeamSectionLocalAxis1Direction(
            local_axis_1_direction, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_remove_beam_section_local_axis_1_direction_message(&mut self,
        local_axis_1_direction_data: &Value) -> Result<(), JsValue>
    {
        let action_id = local_axis_1_direction_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Remove beam section local axis 1 \
                direction action: Action id could not be converted to FEUInt!")))?;
        let local_axis_1_direction = local_axis_1_direction_data["local_axis_1_direction"]
            .to_string()
            .replace("[", "")
            .replace("]", "")
            .split(",")
            .map(|num|
                {
                    match num.parse::<FEFloat>()
                    {
                        Ok(n) => Ok(n),
                        Err(_e) =>
                            {
                                Err(JsValue::from("Actions router: Remove beam section local \
                                    axis 1 direction action: Local axis 1 direction coordinate \
                                    could not be converted to FEFloat!"))
                            }
                    }
                })
            .collect::<Result<Vec<FEFloat>, JsValue>>()?;
        if local_axis_1_direction.len() != 3
        {
            let error_message = "Actions router: Remove beam section local axis 1 direction \
                action: Incorrect number of coordinates for local axis 1 direction!";
            return Err(JsValue::from(error_message));
        }
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            PropertiesActionType::RemoveBeamSectionLocalAxis1Direction(
            local_axis_1_direction, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    pub(super) fn handle_update_beam_section_orientation_data_message(&mut self,
        beam_section_orientation_data: &Value) -> Result<(), JsValue>
    {
        let action_id = beam_section_orientation_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Update beam section orientation data \
                action: Action id could not be converted to FEUInt!")))?;
        let local_axis_1_direction = beam_section_orientation_data["local_axis_1_direction"]
            .to_string()
            .replace("[", "")
            .replace("]", "")
            .split(",")
            .map(|num|
                {
                    match num.parse::<FEFloat>()
                    {
                        Ok(n) => Ok(n),
                        Err(_e) =>
                            {
                                Err(JsValue::from("Actions router: Update beam section \
                                    orientation data action: Local axis 1 direction coordinate \
                                    could not be converted to FEFloat!"))
                            }
                    }
                })
            .collect::<Result<Vec<FEFloat>, JsValue>>()?;
        let old_line_numbers = serde_json::from_value::<Vec<FEUInt>>(
            beam_section_orientation_data["old_beam_section_orientation_values"]
                ["line_numbers"].clone())
            .or(Err(JsValue::from("Actions router: Update beam section orientation data \
                action: Old line numbers could not be converted to FEUInt!")))?;
        let new_line_numbers = serde_json::from_value::<Vec<FEUInt>>(
            beam_section_orientation_data["new_beam_section_orientation_values"]
                ["line_numbers"].clone())
            .or(Err(JsValue::from("Actions router: Update beam section orientation data \
                action: New line numbers could not be converted to FEUInt!")))?;
        self.undo_actions.clear();
        let is_action_id_should_be_increased = true;
        let action_type = ActionType::from(
            PropertiesActionType::UpdateBeamSectionOrientationData(local_axis_1_direction,
                old_line_numbers, new_line_numbers, is_action_id_should_be_increased));
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = true;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }
}
