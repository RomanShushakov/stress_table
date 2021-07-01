use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::{Properties, AssignedProperty, DeletedAssignedProperty};
use crate::{log, dispatch_custom_event};
use crate::
{
    EVENT_TARGET, ADD_ASSIGNED_PROPERTIES_EVENT_NAME, UPDATE_ASSIGNED_PROPERTIES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
};


#[wasm_bindgen]
impl Properties
{
    pub fn add_assigned_properties(&mut self, action_id: u32, name: &str, line_numbers: &[u32],
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

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
        let detail = json!({ "assigned_properties_data": { "name": name,
            "line_numbers": line_numbers },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_ASSIGNED_PROPERTIES_EVENT_NAME,
            EVENT_TARGET)?;
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


    pub fn update_assigned_properties(&mut self, action_id: u32, name: &str, line_numbers: &[u32],
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

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

        if let Some(assigned_property) = self.assigned_properties.get_mut(name)
        {
            assigned_property.update(line_numbers);
            let detail = json!({ "assigned_properties_data": { "name": name,
                "line_numbers": line_numbers },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_ASSIGNED_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
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
        else
        {
             let error_message = format!("Properties: Update assigned properties action: \
                The assigned properties with name {} could not be updated because it does not \
                exist!", name);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_assigned_properties(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        if let Some((assigned_property_name, assigned_property)) =
            self.assigned_properties.remove_entry(&name.to_owned())
        {
            let deleted_assigned_property =
                DeletedAssignedProperty::create(&assigned_property_name, assigned_property);
            self.deleted_assigned_properties.insert(action_id, vec![deleted_assigned_property]);
            let detail = json!({ "assigned_properties_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
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
        else
        {
            let error_message = &format!("Properties: Delete assigned properties action: \
                Assigned properties with name {} do not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_assigned_properties(&mut self, action_id: u32, name: &str,
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
            let detail = json!({ "assigned_properties_data": { "name": name,
                "line_numbers": line_numbers },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_ASSIGNED_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
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
        else
        {
            let error_message = &format!("Properties: Restore assigned properties action: \
                Assigned properties with name {} do not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }
}
