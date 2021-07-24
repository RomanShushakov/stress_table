use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::assigned_property::
{
    AssignedProperty, DeletedAssignedProperty, AssignedPropertyToLines,
    DeletedAssignedPropertyToLines,
};
use crate::preprocessor::properties::consts::
{
    ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME, UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME, UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
};

use crate::types::{FEUInt};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};


impl Properties
{
    pub fn add_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        if !self.properties.contains_key(name)
        {
            let error_message = &format!("Properties: Add assigned properties to lines \
                action: Property with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }

        if self.assigned_properties_to_lines.contains_key(name)
        {
            let error_message = &format!("Properties: Add assigned properties to lines \
                action: Assigned property to lines with name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }

        if self.assigned_properties_to_lines.values()
            .position(|assigned_property_to_lines|
                assigned_property_to_lines.line_numbers_same(line_numbers)).is_some()
        {
            let error_message = &format!("Properties: Add assigned properties to lines \
                action: Assigned property to lines with line numbers {:?} does already exist!",
                line_numbers);
            return Err(JsValue::from(error_message));
        }

        if self.assigned_properties_to_lines.iter()
            .position(|(assigned_property_to_lines_name, assigned_property_to_lines)|
                assigned_property_to_lines_name != name &&
                assigned_property_to_lines.check_for_line_numbers_intersection(line_numbers))
            .is_some()
        {
            let error_message = &format!("Properties: Add assigned properties to lines \
                action: At least one line number from {:?} is already used in another assigned \
                property to lines!", line_numbers);
            return Err(JsValue::from(error_message));
        }
        let assigned_property_to_lines =
            AssignedPropertyToLines::create_initial(line_numbers);
        let related_lines_data =
            assigned_property_to_lines.extract_related_lines_data();
        self.assigned_properties_to_lines.insert(name.to_owned(), assigned_property_to_lines);
        let (_, _, cross_section_type) =
            self.properties.get(name).unwrap().extract_data();
        let detail = json!({ "assigned_properties_to_lines_data":
            {
                "name": name,
                "related_lines_data": related_lines_data,
                "line_numbers": line_numbers,
                "cross_section_type": cross_section_type.as_str().to_lowercase(),
            },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
            EVENT_TARGET)?;

        self.logging();
        Ok(())
    }


    pub fn update_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        if !self.properties.contains_key(name)
        {
            let error_message = &format!("Properties: Update assigned properties to lines \
                action: Property with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }

        if self.assigned_properties_to_lines.values()
            .position(|assigned_property_to_lines|
                assigned_property_to_lines.line_numbers_same(line_numbers)).is_some()
        {
            let error_message = &format!("Properties: Update assigned properties to lines \
                action: Assigned property to lines with line numbers {:?} does already exist!",
                line_numbers);
            return Err(JsValue::from(error_message));
        }

        if self.assigned_properties_to_lines.iter()
            .position(|(assigned_property_to_lines_name, assigned_property_to_lines)|
                assigned_property_to_lines_name != name &&
                assigned_property_to_lines.check_for_line_numbers_intersection(line_numbers))
            .is_some()
        {
            let error_message = &format!("Properties: Update assigned properties to lines \
                action: At least one line number from {:?} is already used in another assigned \
                property to lines!", line_numbers);
            return Err(JsValue::from(error_message));
        }

        if let Some(assigned_property_to_lines) =
            self.assigned_properties_to_lines.get_mut(name)
        {
            let old_assigned_property_to_lines = assigned_property_to_lines.clone();
            let old_related_lines_numbers =
                old_assigned_property_to_lines.extract_related_lines_numbers();
            assigned_property_to_lines.replace_related_lines_data(line_numbers);
            let (_, _, cross_section_type) =
                self.properties.get(name).unwrap().extract_data();
            let related_lines_data =
                self.assigned_properties_to_lines.get(name).unwrap().extract_related_lines_data();
            let detail = json!({ "assigned_properties_to_lines_data":
                {
                    "name": name,
                    "related_lines_data": related_lines_data,
                    "line_numbers": line_numbers,
                    "old_line_numbers": old_related_lines_numbers,
                    "cross_section_type": cross_section_type.as_str().to_lowercase(),
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
             let error_message = format!("Properties: Update assigned properties action: \
                The assigned property with name {} does not exist!", name);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        if !self.properties.contains_key(name)
        {
            let error_message = &format!("Properties: Delete assigned properties to lines \
                action: Property with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }

        if let Some((assigned_property_to_lines_name, assigned_property_to_lines)) =
            self.assigned_properties_to_lines.remove_entry(&name.to_owned())
        {
            let deleted_assigned_property_to_lines =
                DeletedAssignedPropertyToLines::create(&assigned_property_to_lines_name,
                assigned_property_to_lines.clone());
            self.deleted_assigned_properties_to_lines.insert(action_id,
                vec![deleted_assigned_property_to_lines]);
            let detail = json!({ "assigned_properties_to_lines_data":
                {
                    "name": name,
                    "line_numbers": assigned_property_to_lines.extract_related_lines_numbers(),
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Delete assigned properties to lines \
                action: Assigned property to lines with name {} do not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_assigned_property_to_lines) =
            self.deleted_assigned_properties_to_lines.remove(&action_id)
        {
            if deleted_assigned_property_to_lines.is_empty() ||
                deleted_assigned_property_to_lines.len() > 1
            {
                let error_message = &format!("Properties: Restore assigned properties \
                    to lines action: Incorrect number of assigned properties!");
                return Err(JsValue::from(error_message));
            }
            let (deleted_assigned_property_to_lines_name, related_lines_numbers) =
                deleted_assigned_property_to_lines[0].extract_name_and_related_lines_numbers();

            if !self.properties.contains_key(deleted_assigned_property_to_lines_name)
            {
                let error_message = &format!("Properties: Restore assigned properties to \
                    lines action: Property with name {} does not exist!",
                    deleted_assigned_property_to_lines_name);
                return Err(JsValue::from(error_message));
            }

            if deleted_assigned_property_to_lines_name != name
            {
                let error_message = &format!("Properties: Restore assigned properties \
                    to lines action: Assigned property to lines with name {} does not exist!", name);
                return Err(JsValue::from(error_message));
            }

            self.assigned_properties_to_lines.insert(
                deleted_assigned_property_to_lines_name.to_owned(),
                AssignedPropertyToLines::create_initial(related_lines_numbers.as_slice()));
            let (_, _, cross_section_type) =
                self.properties.get(name).unwrap().extract_data();
            let related_lines_data =
                self.assigned_properties_to_lines.get(name).unwrap().extract_related_lines_data();
            let detail = json!({ "assigned_properties_to_lines_data":
                {
                    "name": name,
                    "related_lines_data": related_lines_data,
                    "line_numbers": related_lines_numbers,
                    "cross_section_type": cross_section_type.as_str().to_lowercase(),
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                EVENT_TARGET)?;

            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Restore assigned properties to lines \
                action: Assigned property to lines with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn extract_assigned_properties_to_lines(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_assigned_properties_to_lines = json!(
            { "extracted_assigned_properties_to_lines": self.assigned_properties_to_lines });
        let composed_extracted_assigned_properties_to_lines =
            JsValue::from_serde(&extracted_assigned_properties_to_lines)
                .or(Err(JsValue::from("Properties: Extract assigned properties to lines: \
                    Assigned properties to lines could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_assigned_properties_to_lines);
        Ok(())
    }
}
