use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::traits::ClearByActionIdTrait;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::property::{Property, DeletedProperty};
use crate::preprocessor::properties::property::{CrossSectionType};
use crate::preprocessor::properties::assigned_property::{AssignedProperty, DeletedAssignedProperty};
use crate::preprocessor::properties::consts::
{
    ADD_PROPERTIES_EVENT_NAME, UPDATE_PROPERTIES_EVENT_NAME,
    DELETE_PROPERTIES_EVENT_NAME, ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME, UPDATE_LINES_COLOR_EVENT_NAME,
    UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
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
        self.clear_by_action_id(action_id);

        if self.properties.contains_key(&name.to_owned())
        {
            let error_message = &format!("Properties: Add properties action: \
                Properties with name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }

        if !self.materials.contains_key(material_name)
        {
            let error_message = &format!("Properties: Add properties action: \
                Material with name {} does not exist!", material_name);
            return Err(JsValue::from(error_message));
        }

        let converted_cross_section_type =
            CrossSectionType::create(cross_section_type)?;

        match converted_cross_section_type
        {
            CrossSectionType::Truss =>
                {
                    if !self.truss_sections.contains_key(cross_section_name)
                    {
                        let error_message = &format!("Properties: Add properties action: \
                            Truss section with name {} does not exist!", cross_section_name);
                        return Err(JsValue::from(error_message));
                    }
                },
            CrossSectionType::Beam =>
                {
                    if !self.beam_sections.contains_key(cross_section_name)
                    {
                        let error_message = &format!("Properties: Add properties action: \
                            Beam section with name {} does not exist!", cross_section_name);
                        return Err(JsValue::from(error_message));
                    }
                },
        }

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
        self.clear_by_action_id(action_id);

        if !self.materials.contains_key(material_name)
        {
            let error_message = &format!("Properties: Update properties action: \
                Material with name {} does not exist!", material_name);
            return Err(JsValue::from(error_message));
        }

        let converted_cross_section_type =
            CrossSectionType::create(cross_section_type)?;

        match converted_cross_section_type
        {
            CrossSectionType::Truss =>
                {
                    if !self.truss_sections.contains_key(cross_section_name)
                    {
                        let error_message = &format!("Properties: Update properties action: \
                            Truss section with name {} does not exist!", cross_section_name);
                        return Err(JsValue::from(error_message));
                    }
                },
            CrossSectionType::Beam =>
                {
                    if !self.beam_sections.contains_key(cross_section_name)
                    {
                        let error_message = &format!("Properties: Update properties action: \
                            Beam section with name {} does not exist!", cross_section_name);
                        return Err(JsValue::from(error_message));
                    }
                },
        }

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
                if let Some(assigned_property_to_lines) =
                    self.assigned_properties_to_lines.get(name)
                {
                    let line_numbers =
                        assigned_property_to_lines.extract_related_lines_numbers();
                    let detail = json!({ "lines_color_scheme_data":
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
                The properties with name {} does not exist!",
                name);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_properties(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        let property_names_for_delete = vec![name.to_string()];

        self.delete_assigned_properties_to_lines_by_names(action_id, &property_names_for_delete)?;

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


    pub fn delete_properties_by_names(&mut self, action_id: FEUInt,
        property_names_for_delete: &[String]) -> Result<(), JsValue>
    {
        let mut properties_for_delete = Vec::new();

        for property_name in property_names_for_delete.iter()
        {
            let property = self.properties.remove(property_name).unwrap();
            let deleted_property = DeletedProperty::create(property_name, property);
            properties_for_delete.push(deleted_property);
            let detail = json!({ "properties_data": { "name": property_name },
                "is_action_id_should_be_increased": false });
            dispatch_custom_event(detail, DELETE_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
        }

        if !properties_for_delete.is_empty()
        {
            self.deleted_properties.insert(action_id, properties_for_delete);
        }
        Ok(())
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

            self.restore_assigned_properties_to_lines_by_action_id(action_id)?;

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


    pub fn restore_properties_by_action_id(&mut self, action_id: FEUInt)
        -> Result<(), JsValue>
    {
        if let Some(deleted_properties) =
            self.deleted_properties.remove(&action_id)
        {
            for deleted_property in &deleted_properties
            {
                let (name, material_name, cross_section_name,
                    cross_section_type) = deleted_property.extract_name_and_data();
                self.properties.insert(name.to_owned(),
                    Property::create(material_name, cross_section_name,
                        cross_section_type.clone()));
                let transformed_cross_section_type = r#"""#.to_owned() +
                    &cross_section_type.as_str().to_lowercase() + r#"""#;
                let detail = json!({ "properties_data": { "name": name,
                    "material_name": material_name, "cross_section_name": cross_section_name,
                    "cross_section_type": transformed_cross_section_type },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail, ADD_PROPERTIES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }
        Ok(())
    }


    pub fn extract_properties(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_properties = json!(
            { "extracted_properties": self.properties });
        let composed_extracted_properties =
            JsValue::from_serde(&extracted_properties)
                .or(Err(JsValue::from("Properties: Extract properties: Properties \
                    could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_properties);
        Ok(())
    }
}
