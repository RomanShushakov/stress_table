use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::traits::ClearByActionIdTrait;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::assigned_property::
{
    AssignedProperty, ChangedAssignedProperty, DeletedAssignedProperty,
    ChangedAssignedPropertyToLines, DeletedAssignedPropertyToLines, AssignedPropertyToLines
};
use crate::preprocessor::properties::consts::
{
    ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME, UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
};

use crate::types::{FEUInt};

use crate::consts::EVENT_TARGET;

use crate::functions::{log, dispatch_custom_event};


impl Properties
{
    fn extract_assigned_properties_to_lines_names_for_change_or_delete_by_line_numbers(&self,
        line_numbers: &[FEUInt]) -> Vec<String>
    {
        let mut assigned_properties_to_lines_names_for_change_or_delete = Vec::new();
        for (assigned_property_to_lines_name, assigned_property_to_lines) in
            self.assigned_properties_to_lines.iter()
        {
            if assigned_property_to_lines.any_of_provided_line_numbers_contains(line_numbers)
            {
                assigned_properties_to_lines_names_for_change_or_delete.push(
                    assigned_property_to_lines_name.to_owned())
            }
        }
        assigned_properties_to_lines_names_for_change_or_delete
    }


    pub fn delete_line_numbers_from_properties(&mut self, action_id: FEUInt,
        line_numbers: &[FEUInt]) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        let mut changed_assigned_properties_to_lines = Vec::new();

        let mut deleted_assigned_properties_to_lines = Vec::new();

        let assigned_properties_to_lines_names_for_change_or_delete =
            self.extract_assigned_properties_to_lines_names_for_change_or_delete_by_line_numbers(
                line_numbers
            );

        for assigned_property_to_lines_name in
            assigned_properties_to_lines_names_for_change_or_delete.iter()
        {
            let assigned_property_to_lines =
                self.assigned_properties_to_lines.get_mut(assigned_property_to_lines_name)
                    .unwrap();
            let old_assigned_property_to_lines =
                assigned_property_to_lines.clone();

            let obsolete_assigned_property_to_lines =
                assigned_property_to_lines.clone();

            let old_line_numbers = obsolete_assigned_property_to_lines
                .extract_related_lines_numbers();

            let mut new_assigned_property_to_lines_line_numbers =
                assigned_property_to_lines.extract_related_lines_numbers();

            while let Some(position) = new_assigned_property_to_lines_line_numbers.iter()
                .position(|number| line_numbers.contains(number))
            {
                new_assigned_property_to_lines_line_numbers.remove(position);
            }
            if new_assigned_property_to_lines_line_numbers.len() > 0
            {
                let changed_assigned_property_to_lines =
                    ChangedAssignedPropertyToLines::create(assigned_property_to_lines_name,
                        old_assigned_property_to_lines);
                changed_assigned_properties_to_lines.push(changed_assigned_property_to_lines);
                assigned_property_to_lines.fit_related_lines_data_by_line_numbers(
                    new_assigned_property_to_lines_line_numbers.as_slice());
                let (_, _, cross_section_type) =
                    self.properties.get(assigned_property_to_lines_name).unwrap().extract_data();
                let related_lines_data =
                    self.assigned_properties_to_lines.get(assigned_property_to_lines_name)
                        .unwrap()
                        .extract_related_lines_data();
                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": new_assigned_property_to_lines_line_numbers.as_slice(),
                        "old_line_numbers": old_line_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
            else
            {
                let assigned_property_to_lines_for_delete =
                    self.assigned_properties_to_lines.remove(assigned_property_to_lines_name)
                        .unwrap();
                let deleted_assigned_property_to_lines =
                    DeletedAssignedPropertyToLines::create(assigned_property_to_lines_name,
                        old_assigned_property_to_lines);
                deleted_assigned_properties_to_lines.push(deleted_assigned_property_to_lines);
                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "line_numbers": assigned_property_to_lines_for_delete
                            .extract_related_lines_numbers(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }
        if !changed_assigned_properties_to_lines.is_empty()
        {
            self.changed_assigned_properties_to_lines.insert(action_id,
                changed_assigned_properties_to_lines);
        }
        if !deleted_assigned_properties_to_lines.is_empty()
        {
            self.deleted_assigned_properties_to_lines.insert(action_id,
                deleted_assigned_properties_to_lines);
        }
        self.logging();
        Ok(())
    }


    pub fn restore_line_numbers_in_properties(&mut self, action_id: FEUInt,
        restored_line_numbers: &[FEUInt]) -> Result<(), JsValue>
    {
        if let Some(changed_assigned_properties_to_lines) =
            self.changed_assigned_properties_to_lines.remove(&action_id)
        {
            for changed_assigned_property_to_lines in
                changed_assigned_properties_to_lines.iter()
            {
                let (name, line_numbers) =
                    changed_assigned_property_to_lines.extract_name_and_related_lines_numbers();
                if restored_line_numbers.iter().position(|restored_line_number|
                    line_numbers.contains(restored_line_number)).is_none()
                {
                    return Err(JsValue::from("Properties: Restore line \
                        numbers: No line number from restored line numbers does \
                        contain in changed assigned properties to lines for appropriate \
                        action id!"));
                }
                if let Some(assigned_property_to_lines_for_update) =
                    self.assigned_properties_to_lines.get_mut(name)
                {
                    let old_assigned_property_to_lines =
                        assigned_property_to_lines_for_update.clone();
                    let old_line_numbers =
                        old_assigned_property_to_lines.extract_related_lines_numbers();
                    assigned_property_to_lines_for_update.fit_related_lines_data_by_line_numbers(
                        line_numbers.as_slice());
                    let (_, _, cross_section_type) =
                        self.properties.get(name).unwrap().extract_data();
                    let related_lines_data =
                        self.assigned_properties_to_lines.get(name).unwrap()
                            .extract_related_lines_data();
                    let detail = json!({ "assigned_properties_to_lines_data":
                        {
                            "name": name,
                            "related_lines_data": related_lines_data,
                            "line_numbers": line_numbers,
                            "old_line_numbers": old_line_numbers,
                            "cross_section_type": cross_section_type.as_str().to_lowercase(),
                        },
                        "is_action_id_should_be_increased": false });
                    dispatch_custom_event(detail,
                        UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                        EVENT_TARGET)?;
                }
                else
                {
                    let error_message = &format!("Properties: Restore line \
                        numbers: Assigned properties with name {} do not exist!", name);
                    return Err(JsValue::from(error_message));
                }
            }
        }
        if let Some(deleted_assigned_properties_to_lines) =
            self.deleted_assigned_properties_to_lines.remove(&action_id)
        {
            for deleted_assigned_property_to_lines in
                deleted_assigned_properties_to_lines.iter()
            {
                let (name, line_numbers) =
                    deleted_assigned_property_to_lines.extract_name_and_related_lines_numbers();
                if restored_line_numbers.iter().position(|restored_line_number|
                    line_numbers.contains(restored_line_number)).is_none()
                {
                    return Err(JsValue::from("Properties: Restore line \
                        numbers: No line number from restored line numbers does \
                        contain in deleted assigned properties for appropriate \
                        action id!"));
                }
                self.assigned_properties_to_lines.insert(name.to_owned(),
                    AssignedPropertyToLines::create_initial(line_numbers.as_slice()));
                let (_, _, cross_section_type) =
                    self.properties.get(name).unwrap().extract_data();
                let related_lines_data =
                        self.assigned_properties_to_lines.get(name).unwrap()
                            .extract_related_lines_data();
                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": line_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }
        self.logging();
        Ok(())
    }
}
