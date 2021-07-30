use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::traits::ClearByActionIdTrait;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::assigned_property::
{
    ChangedAssignedPropertyToLines, DeletedAssignedPropertyToLines, AssignedPropertyToLines
};
use crate::preprocessor::properties::consts::
{
    ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME, UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
};

use crate::types::{FEUInt};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};


impl Properties
{
    pub fn delete_line_numbers_from_properties(&mut self, action_id: FEUInt,
        line_numbers: &[FEUInt]) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        let mut changed_assigned_properties_to_lines = Vec::new();

        let mut deleted_assigned_properties_to_lines = Vec::new();

        for (assigned_property_to_lines_name, assigned_property_to_lines) in
            self.assigned_properties_to_lines.iter_mut()
        {
            let current_line_numbers_for_delete = assigned_property_to_lines
                .extract_related_lines_numbers().into_iter().filter(|line_number|
                    line_numbers.contains(line_number))
                .collect::<Vec<FEUInt>>();

            if current_line_numbers_for_delete.len() == assigned_property_to_lines
                .length_of_related_lines_data()
            {
                let deleted_assigned_property_to_lines =
                    DeletedAssignedPropertyToLines::create(assigned_property_to_lines_name,
                    assigned_property_to_lines.clone());
                deleted_assigned_properties_to_lines.push(deleted_assigned_property_to_lines);

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "line_numbers": assigned_property_to_lines.extract_related_lines_numbers(),
                    },
                    "is_action_id_should_be_increased": false });

                dispatch_custom_event(detail,
                    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
            else
            {
                let changed_assigned_property_to_lines =
                    ChangedAssignedPropertyToLines::create(assigned_property_to_lines_name,
                    assigned_property_to_lines.clone());
                changed_assigned_properties_to_lines.push(changed_assigned_property_to_lines);

                let old_line_numbers =
                    assigned_property_to_lines.extract_related_lines_numbers();

                for line_number_for_delete in current_line_numbers_for_delete.iter()
                {
                    let _ = assigned_property_to_lines.remove_line_number_from_related_lines_data(
                        line_number_for_delete);
                }

                let related_lines_data =
                    assigned_property_to_lines.extract_related_lines_data();

                let (_, _, cross_section_type) =
                    self.properties.get(assigned_property_to_lines_name).unwrap().extract_data();

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": assigned_property_to_lines.extract_related_lines_numbers(),
                        "old_line_numbers": old_line_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
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
            for deleted_assigned_property_to_lines_name in deleted_assigned_properties_to_lines
                .iter()
                .map(|deleted_assigned_property_to_lines|
                    deleted_assigned_property_to_lines.extract_name())
            {
                let _ = self.assigned_properties_to_lines.remove(
                    deleted_assigned_property_to_lines_name);
            }

            self.deleted_assigned_properties_to_lines.insert(action_id,
                deleted_assigned_properties_to_lines);
        }

        self.logging();
        Ok(())
    }


    fn check_for_all_restored_line_numbers_contain(&self, action_id: FEUInt,
        restored_line_numbers: &[FEUInt]) -> Result<(), JsValue>
    {
        let mut line_numbers_for_check = restored_line_numbers.to_vec();

        if let Some(changed_assigned_properties_to_lines) =
            self.changed_assigned_properties_to_lines.get(&action_id)
        {
            for changed_assigned_property_to_lines in
                changed_assigned_properties_to_lines
            {
                let (_, related_lines_numbers) =
                    changed_assigned_property_to_lines.extract_name_and_related_lines_numbers();

                line_numbers_for_check = line_numbers_for_check
                    .into_iter()
                    .filter(|line_number| !related_lines_numbers.contains(line_number))
                    .collect::<Vec<FEUInt>>();
            }
        }

        if let Some(deleted_assigned_properties_to_lines) =
            self.deleted_assigned_properties_to_lines.get(&action_id)
        {
            for deleted_assigned_property_to_lines in deleted_assigned_properties_to_lines
            {
                let (_, related_lines_numbers) =
                    deleted_assigned_property_to_lines.extract_name_and_related_lines_numbers();

                line_numbers_for_check = line_numbers_for_check
                    .into_iter()
                    .filter(|line_number| !related_lines_numbers.contains(line_number))
                    .collect::<Vec<FEUInt>>();
            }
        }

        if !line_numbers_for_check.is_empty()
        {
            let error_message = &format!("Properties: Restore line numbers action: \
                The line numbers {:?} do not contain neither in changed assigned properties \
                nor in deleted assigned properties for action id {}", line_numbers_for_check,
                action_id);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn restore_line_numbers_in_properties(&mut self, action_id: FEUInt,
        restored_line_numbers: &[FEUInt]) -> Result<(), JsValue>
    {
        self.check_for_all_restored_line_numbers_contain(action_id, restored_line_numbers)?;

        if let Some(changed_assigned_properties_to_lines) =
            self.changed_assigned_properties_to_lines.remove(&action_id)
        {
            for changed_assigned_property_to_lines in
                changed_assigned_properties_to_lines.into_iter()
            {
                let (assigned_property_to_lines_name, assigned_property_to_lines) =
                    changed_assigned_property_to_lines.extract_and_drop();

                let related_lines_data =
                    assigned_property_to_lines.extract_related_lines_data();

                let line_numbers = assigned_property_to_lines
                    .extract_related_lines_numbers();

                let old_line_numbers = self.assigned_properties_to_lines
                    .get(&assigned_property_to_lines_name).unwrap()
                    .extract_related_lines_numbers();

                let (_, _, cross_section_type) =
                        self.properties.get(&assigned_property_to_lines_name)
                            .unwrap().extract_data();

                self.assigned_properties_to_lines.insert(assigned_property_to_lines_name.clone(),
                    assigned_property_to_lines);

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
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
        }

        if let Some(deleted_assigned_properties_to_lines) =
            self.deleted_assigned_properties_to_lines.remove(&action_id)
        {
            for deleted_assigned_property_to_lines in
                deleted_assigned_properties_to_lines.into_iter()
            {
                let (assigned_property_to_lines_name, assigned_property_to_lines) =
                    deleted_assigned_property_to_lines.extract_and_drop();

                let related_lines_data =
                    assigned_property_to_lines.extract_related_lines_data();

                let line_numbers =
                    assigned_property_to_lines.extract_related_lines_numbers();

                let (_, _, cross_section_type) =
                        self.properties.get(&assigned_property_to_lines_name)
                            .unwrap().extract_data();

                self.assigned_properties_to_lines.insert(assigned_property_to_lines_name.clone(),
                    assigned_property_to_lines);

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
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
