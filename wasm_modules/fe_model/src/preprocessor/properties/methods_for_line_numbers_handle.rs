use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::assigned_property::
{
    AssignedProperty, ChangedAssignedProperty, DeletedAssignedProperty
};
use crate::preprocessor::properties::consts::
{
    ADD_ASSIGNED_PROPERTIES_EVENT_NAME, UPDATE_ASSIGNED_PROPERTIES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
};

use crate::types::{FEUInt};

use crate::consts::EVENT_TARGET;

use crate::functions::{log, dispatch_custom_event};


impl Properties
{
    fn extract_assigned_property_names_for_change_or_delete_by_line_numbers(&self,
        line_numbers: &[FEUInt]) -> Vec<String>
    {
        let mut assigned_property_names_for_change_or_delete = Vec::new();
        for (assigned_property_name, assigned_property) in
            &self.assigned_properties
        {
            if assigned_property.is_contain_any_provided_line_number(line_numbers)
            {
                assigned_property_names_for_change_or_delete.push(assigned_property_name.to_owned())
            }
        }
        assigned_property_names_for_change_or_delete
    }


    pub fn delete_line_numbers_from_properties(&mut self, action_id: FEUInt,
        line_numbers: &[FEUInt]) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        let mut changed_assigned_properties = Vec::new();
        let mut deleted_assigned_properties = Vec::new();
        let assigned_property_names_for_change_or_delete =
            self.extract_assigned_property_names_for_change_or_delete_by_line_numbers(
                line_numbers
            );
        for assigned_property_name in &assigned_property_names_for_change_or_delete
        {
            let assigned_property = self.assigned_properties
                .get_mut(assigned_property_name).unwrap();
            let old_assigned_property = assigned_property.clone();
            let mut new_assigned_property_line_numbers = assigned_property
                .extract_data().to_vec();
            while let Some(position) = new_assigned_property_line_numbers.iter()
                .position(|number| line_numbers.contains(number))
            {
                new_assigned_property_line_numbers.remove(position);
            }
            if new_assigned_property_line_numbers.len() > 0
            {
                let changed_assigned_property =
                    ChangedAssignedProperty::create(assigned_property_name,
                        old_assigned_property);
                changed_assigned_properties.push(changed_assigned_property);
                assigned_property.update(new_assigned_property_line_numbers.as_slice());
                let detail = json!({ "assigned_properties_data":
                    {
                        "name": assigned_property_name,
                        "line_numbers": new_assigned_property_line_numbers.as_slice()
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail, UPDATE_ASSIGNED_PROPERTIES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
            else
            {
                let _ = self.assigned_properties.remove(assigned_property_name).unwrap();
                let deleted_assigned_property =
                    DeletedAssignedProperty::create(assigned_property_name,
                        old_assigned_property);
                deleted_assigned_properties.push(deleted_assigned_property);
                let detail = json!({ "assigned_properties_data":
                    {
                        "name": assigned_property_name
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }
        if !changed_assigned_properties.is_empty()
        {
            self.changed_assigned_properties.insert(action_id,
                changed_assigned_properties);
        }
        if !deleted_assigned_properties.is_empty()
        {
            self.deleted_assigned_properties.insert(action_id,
                deleted_assigned_properties);
        }

        log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
            truss sections: {:?}, deleted truss sections: {:?}, \
            beam sections: {:?}, deleted beam sections: {:?}, \
            properties: {:?}, deleted properties: {:?}, \
            assigned_properties: {:?}, changed_assigned_properties: {:?}, \
            deleted_assigned_properties: {:?}",
            self.materials, self.deleted_materials,
            self.truss_sections, self.deleted_truss_sections,
            self.beam_sections, self.deleted_beam_sections,
            self.properties, self.deleted_properties,
            self.assigned_properties, self.changed_assigned_properties,
            self.deleted_assigned_properties)
        );
        Ok(())
    }


    pub fn restore_line_numbers_in_properties(&mut self, action_id: FEUInt,
        restored_line_numbers: &[FEUInt]) -> Result<(), JsValue>
    {
        if let Some(changed_assigned_properties) =
            self.changed_assigned_properties.remove(&action_id)
        {
            for changed_assigned_property in &changed_assigned_properties
            {
                let (name, line_numbers) =
                    changed_assigned_property.extract_name_and_data();
                if restored_line_numbers.iter().position(|restored_line_number|
                    line_numbers.contains(restored_line_number)).is_none()
                {
                    return Err(JsValue::from("Properties: Restore line \
                        numbers: No line number from restored line numbers does \
                        contain in changed assigned properties for appropriate \
                        action id!"));
                }
                if let Some(assigned_property_for_update) =
                    self.assigned_properties.get_mut(name)
                {
                    assigned_property_for_update.update(line_numbers);
                    let detail = json!({ "assigned_properties_data":
                        { "name": name, "line_numbers": line_numbers },
                        "is_action_id_should_be_increased": false });
                    dispatch_custom_event(detail,
                        UPDATE_ASSIGNED_PROPERTIES_EVENT_NAME,
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
        if let Some(deleted_assigned_properties) =
            self.deleted_assigned_properties.remove(&action_id)
        {
            for deleted_assigned_property in &deleted_assigned_properties
            {
                let (name, line_numbers) =
                    deleted_assigned_property.extract_name_and_data();
                if restored_line_numbers.iter().position(|restored_line_number|
                    line_numbers.contains(restored_line_number)).is_none()
                {
                    return Err(JsValue::from("Properties: Restore line \
                        numbers: No line number from restored line numbers does \
                        contain in deleted assigned properties for appropriate \
                        action id!"));
                }
                self.assigned_properties.insert(name.to_owned(),
                    AssignedProperty::create(line_numbers));
                let detail = json!({ "assigned_properties_data": { "name": name,
                    "line_numbers": line_numbers },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    ADD_ASSIGNED_PROPERTIES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }
        log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
            truss sections: {:?}, deleted truss sections: {:?}, \
            beam sections: {:?}, deleted beam sections: {:?}, \
            properties: {:?}, deleted properties: {:?}, \
            assigned_properties: {:?}, changed_assigned_properties: {:?}, \
            deleted_assigned_properties: {:?}",
            self.materials, self.deleted_materials,
            self.truss_sections, self.deleted_truss_sections,
            self.beam_sections, self.deleted_beam_sections,
            self.properties, self.deleted_properties,
            self.assigned_properties, self.changed_assigned_properties,
            self.deleted_assigned_properties)
        );
        Ok(())
    }
}
