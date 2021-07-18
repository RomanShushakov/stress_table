use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::assigned_property::{AssignedProperty, DeletedAssignedProperty};
use crate::preprocessor::properties::consts::
{
    ADD_ASSIGNED_PROPERTIES_EVENT_NAME, UPDATE_ASSIGNED_PROPERTIES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_EVENT_NAME, UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
};

use crate::types::{FEUInt};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};


impl Properties
{
    pub fn add_assigned_properties(&mut self, action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        if self.assigned_properties.contains_key(&name.to_owned())
        {
            let error_message = &format!("Properties: Add assigned properties action: \
                Assigned properties with name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }

        if self.assigned_properties.values().position(|assigned_property|
            assigned_property.data_same(line_numbers)).is_some()
        {
            let error_message = &format!("Properties: Add assigned properties action: \
                Assigned properties with Line numbers {:?} does already exist!", line_numbers);
            return Err(JsValue::from(error_message));
        }

        if self.assigned_properties.iter()
            .position(|(assigned_property_name, assigned_property)|
                assigned_property_name != name &&
                assigned_property.is_contain_any_provided_line_number(line_numbers)).is_some()
        {
            let error_message = &format!("Properties: Add assigned properties action: \
                At least one line number from {:?} is already used in assigned properties!",
                line_numbers);
            return Err(JsValue::from(error_message));
        }
        let assigned_property = AssignedProperty::create(line_numbers);
        self.assigned_properties.insert(name.to_owned(), assigned_property);
        let (_, _, cross_section_type) =
            self.properties.get(name).unwrap().extract_data();
        let detail = json!({ "assigned_properties_data":
            {
                "name": name,
                "line_numbers": line_numbers,
                "cross_section_type": cross_section_type.as_str().to_lowercase(),
            },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_ASSIGNED_PROPERTIES_EVENT_NAME,
            EVENT_TARGET)?;
        self.logging();
        Ok(())
    }


    pub fn update_assigned_properties(&mut self, action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        if self.assigned_properties.values().position(|assigned_property|
            assigned_property.data_same(line_numbers)).is_some()
        {
            let error_message = &format!("Properties: Update assigned properties action: \
                Assigned properties with line numbers {:?} does already exist!",
                    line_numbers);
            return Err(JsValue::from(error_message));
        }

        if self.assigned_properties.iter()
            .position(|(assigned_property_name, assigned_property)|
                assigned_property_name != name &&
                assigned_property.is_contain_any_provided_line_number(line_numbers)).is_some()
        {
            let error_message = &format!("Properties: Update assigned properties action: \
                At least one line number from {:?} is already used in assigned properties!",
                line_numbers);
            return Err(JsValue::from(error_message));
        }

        if let Some(assigned_property) =
            self.assigned_properties.get_mut(name)
        {
            let old_assigned_property = assigned_property.clone();
            let old_line_numbers = old_assigned_property.extract_data();
            assigned_property.update(line_numbers);
            let (_, _, cross_section_type) =
                self.properties.get(name).unwrap().extract_data();
            let detail = json!({ "assigned_properties_data":
                {
                    "name": name,
                    "line_numbers": line_numbers,
                    "old_line_numbers": old_line_numbers,
                    "cross_section_type": cross_section_type.as_str().to_lowercase(),
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_ASSIGNED_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
             let error_message = format!("Properties: Update assigned properties action: \
                The assigned properties with name {} could not be updated because it does not \
                exist!", name);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_assigned_properties(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        let changed_beam_sections_orientations =
            self.extract_beam_section_orientations_for_change_by_assigned_property_names(
                &vec![name.to_string()]
            );

        for changed_beam_section_orientation in &changed_beam_sections_orientations
        {
            let local_axis_1_direction =
                changed_beam_section_orientation.extract_local_axis_1_direction();
            if let Some(position) = self.beam_sections_orientations
                .iter()
                .position(|beam_section_orientation|
                    beam_section_orientation
                        .is_local_axis_1_direction_same(&local_axis_1_direction))
            {
                let line_numbers = self.beam_sections_orientations[position]
                    .extract_line_numbers();
                let detail = json!({ "beam_section_orientation_data":
                    {
                        "local_axis_1_direction": local_axis_1_direction,
                        "line_numbers": line_numbers,
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }
        self.changed_beam_sections_orientations.insert(action_id,
            changed_beam_sections_orientations);

        if let Some((assigned_property_name, assigned_property)) =
            self.assigned_properties.remove_entry(&name.to_owned())
        {
            let deleted_assigned_property =
                DeletedAssignedProperty::create(&assigned_property_name,
                assigned_property.clone());
            self.deleted_assigned_properties.insert(action_id,
                vec![deleted_assigned_property]);
            let detail = json!({ "assigned_properties_data":
                {
                    "name": name,
                    "line_numbers": assigned_property.extract_data(),
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Delete assigned properties action: \
                Assigned properties with name {} do not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_assigned_properties(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_assigned_properties) =
            self.deleted_assigned_properties.remove(&action_id)
        {
            if deleted_assigned_properties.is_empty() || deleted_assigned_properties.len() > 1
            {
                let error_message = &format!("Properties: Restore assigned properties \
                    action: Incorrect number of assigned properties!");
                return Err(JsValue::from(error_message));
            }
            let (deleted_assigned_property_name, line_numbers) =
                deleted_assigned_properties[0].extract_name_and_data();
            if deleted_assigned_property_name != name
            {
                let error_message = &format!("Properties: Restore assigned properties \
                    action: Assigned properties with name {} do not exist!", name);
                return Err(JsValue::from(error_message));
            }
            self.assigned_properties.insert(deleted_assigned_property_name.to_owned(),
               AssignedProperty::create(line_numbers));
            let (_, _, cross_section_type) =
                self.properties.get(name).unwrap().extract_data();
            let detail = json!({ "assigned_properties_data":
                {
                    "name": name,
                    "line_numbers": line_numbers,
                    "cross_section_type": cross_section_type.as_str().to_lowercase(),
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_ASSIGNED_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;

            if let Some(beam_sections_orientations) =
                self.changed_beam_sections_orientations.remove(&action_id)
            {
                for beam_section_orientation in &beam_sections_orientations
                {
                    let (local_axis_1_direction, line_numbers) =
                        beam_section_orientation.extract_direction_and_line_numbers();
                    if let Some(position) = self.beam_sections_orientations
                        .iter()
                        .position(|beam_section_orientation|
                            beam_section_orientation.is_local_axis_1_direction_same(
                                &local_axis_1_direction))
                    {
                        self.beam_sections_orientations[position].update(line_numbers);
                        let detail = json!({ "beam_section_orientation_data":
                            {
                                "local_axis_1_direction": local_axis_1_direction,
                                "line_numbers": line_numbers,
                            },
                            "is_action_id_should_be_increased": is_action_id_should_be_increased });
                        dispatch_custom_event(detail,
                            UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
                            EVENT_TARGET)?;
                    }
                }
            }
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Restore assigned properties action: \
                Assigned properties with name {} do not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }
}
