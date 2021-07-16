use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::property::{Property, DeletedProperty};
use crate::preprocessor::properties::property::{CrossSectionType};
use crate::preprocessor::properties::assigned_property::{AssignedProperty, DeletedAssignedProperty};
use crate::preprocessor::properties::consts::
{
    ADD_PROPERTIES_EVENT_NAME, UPDATE_PROPERTIES_EVENT_NAME,
    DELETE_PROPERTIES_EVENT_NAME, ADD_ASSIGNED_PROPERTIES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_EVENT_NAME, UPDATE_LINES_COLOR_EVENT_NAME,
};

use crate::types::{FEUInt};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};


impl Properties
{
    pub fn add_properties(&mut self, action_id: FEUInt, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        if self.properties.contains_key(&name.to_owned())
        {
            let error_message = &format!("Properties: Add properties action: \
                Properties with name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }

        let converted_cross_section_type =
            CrossSectionType::create(cross_section_type)?;

        if self.properties.values().position(|property|
            property.data_same(material_name, cross_section_name, &converted_cross_section_type))
                .is_some()
        {
            let error_message = &format!("Properties: Add properties action: \
                Properties with Material name {}, Cross section name {}, Cross section type {}, \
                does already exist!", material_name, cross_section_name, cross_section_type);
            return Err(JsValue::from(error_message));
        }
        let property = Property::create(material_name, cross_section_name,
                                        converted_cross_section_type);
        self.properties.insert(name.to_owned(), property);
        let detail = json!({ "properties_data": { "name": name,
            "material_name": material_name, "cross_section_name": cross_section_name,
            "cross_section_type": cross_section_type },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_PROPERTIES_EVENT_NAME,
            EVENT_TARGET)?;
        self.logging();
        Ok(())
    }


    pub fn update_properties(&mut self, action_id: FEUInt, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        let converted_cross_section_type =
            CrossSectionType::create(cross_section_type)?;

        if self.properties.values().position(|property|
            property.data_same(material_name, cross_section_name, &converted_cross_section_type))
                .is_some()
        {
            let error_message = &format!("Properties: Update properties action: \
                Properties with Material name {}, Cross section name {}, Cross section type {} \
                does already exist!",
                    material_name, cross_section_name, cross_section_type);
            return Err(JsValue::from(error_message));
        }
        if let Some(property) = self.properties.get_mut(name)
        {
            let (_, _, previous_cross_section_type) = property.extract_data();
            property.update(material_name, cross_section_name,
                converted_cross_section_type.clone());
            if previous_cross_section_type != converted_cross_section_type
            {
                if let Some(assigned_property) = self.assigned_properties.get(name)
                {
                    let line_numbers = assigned_property.extract_data();
                let detail = json!({ "lines_data_object":
                    {
                        "line_numbers": line_numbers,
                        "cross_section_type": converted_cross_section_type.as_str().to_lowercase()
                    }});
                dispatch_custom_event(detail, UPDATE_LINES_COLOR_EVENT_NAME,
                    EVENT_TARGET)?;
                }
            }
            let detail = json!({ "properties_data": { "name": name,
                "material_name": material_name, "cross_section_name": cross_section_name,
                "cross_section_type": cross_section_type },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
             let error_message = format!("Properties: Update properties action: \
                The properties with name {} could not be updated because it does not exist!",
                name);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_properties(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        let deleted_assigned_property_names =
            self.extract_assigned_property_names_for_delete_by_property_names(
                &vec![name.to_string()]);
        let mut deleted_assigned_properties = Vec::new();

        for assigned_property_name in deleted_assigned_property_names.iter()
        {
            let assigned_property =
                self.assigned_properties.remove(assigned_property_name).unwrap();
            let deleted_assigned_property = DeletedAssignedProperty::create(
                assigned_property_name, assigned_property.clone());
            deleted_assigned_properties.push(deleted_assigned_property);

            let detail = json!({ "assigned_properties_data":
                {
                    "name": assigned_property_name,
                    "line_numbers": assigned_property.extract_data(),
                },
                "is_action_id_should_be_increased": false });
            dispatch_custom_event(detail, DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
        }
        if !deleted_assigned_properties.is_empty()
        {
            self.deleted_assigned_properties.insert(action_id, deleted_assigned_properties);
        }

        if let Some((property_name, property)) =
            self.properties.remove_entry(&name.to_owned())
        {
            let deleted_property =
                DeletedProperty::create(&property_name, property);
            self.deleted_properties.insert(action_id, vec![deleted_property]);
            let detail = json!({ "properties_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Delete properties action: \
                Properties with name {} do not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_properties(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_properties) =
            self.deleted_properties.remove(&action_id)
        {
            if deleted_properties.is_empty() || deleted_properties.len() > 1
            {
                let error_message = &format!("Properties: Restore properties action: \
                    Incorrect number of properties!");
                return Err(JsValue::from(error_message));
            }
            let (deleted_property_name, material_name, cross_section_name,
                cross_section_type) = deleted_properties[0].extract_name_and_data();
            if deleted_property_name != name
            {
                let error_message = &format!("Properties: Restore properties \
                    action: Properties with name {} do not exist!", name);
                return Err(JsValue::from(error_message));
            }
            self.properties.insert(deleted_property_name.to_owned(),
               Property::create(material_name, cross_section_name,
                    cross_section_type.clone()));
            let detail = json!({ "properties_data": {
                    "name": deleted_property_name,
                    "material_name": material_name,
                    "cross_section_name": cross_section_name,
                    "cross_section_type": cross_section_type.as_str() },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
            if let Some(deleted_assigned_properties) =
                self.deleted_assigned_properties.remove(&action_id)
            {
                for deleted_assigned_property in &deleted_assigned_properties
                {
                    let (name, line_numbers) =
                        deleted_assigned_property.extract_name_and_data();
                    self.assigned_properties.insert(name.to_owned(),
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
                }
            }
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Restore properties action: \
                Properties with name {} do not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }
}
