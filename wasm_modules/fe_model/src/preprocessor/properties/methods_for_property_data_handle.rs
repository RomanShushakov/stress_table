use wasm_bindgen::prelude::*;
use serde_json::json;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;

use crate::traits::ClearByActionIdTrait;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::property::{Property, DeletedProperty};
use crate::preprocessor::properties::property::{CrossSectionType};
use crate::preprocessor::properties::consts::
{
    ADD_PROPERTIES_EVENT_NAME, UPDATE_PROPERTIES_EVENT_NAME, DELETE_PROPERTIES_EVENT_NAME,
    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
};

use crate::preprocessor::properties::functions::are_line_numbers_same;

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};
use crate::preprocessor::properties::assigned_property::ChangedAssignedPropertyToLines;


impl<T, V> Properties<T, V>
    where T: Copy + Debug + Eq + Hash + Serialize + PartialOrd,
          V: Copy + Debug + Serialize,
{
    pub fn add_properties(&mut self, action_id: T, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Add properties action";

        self.clear_by_action_id(action_id);

        if self.properties.contains_key(&name.to_owned())
        {
            let error_message = &format!("{}: Property with name {} does already exist!",
                error_message_header, name);
            return Err(JsValue::from(error_message));
        }

        self.check_for_material_existence_by_name(material_name, error_message_header)?;

        let converted_cross_section_type =
            CrossSectionType::create(cross_section_type)?;

        self.check_for_cross_section_existence(cross_section_name, &converted_cross_section_type,
            error_message_header)?;

        self.check_for_property_with_similar_data_existence(material_name, cross_section_name,
            &converted_cross_section_type, error_message_header)?;

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


    pub fn update_properties(&mut self, action_id: T, name: &str, material_name: &str,
        cross_section_name: &str, cross_section_type: &str, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Update properties action";

        self.check_for_material_existence_by_name(material_name, error_message_header)?;

        let converted_cross_section_type =
            CrossSectionType::create(cross_section_type)?;

        self.check_for_cross_section_existence(cross_section_name, &converted_cross_section_type,
            error_message_header)?;

        self.check_for_property_with_similar_data_existence(material_name, cross_section_name,
            &converted_cross_section_type, error_message_header)?;

        self.check_for_property_existence_by_name(name, error_message_header)?;

        let property = self.properties.get_mut(name).unwrap();

        let (_, _, previous_cross_section_type) = property.extract_data();

        property.update(material_name, cross_section_name,
            converted_cross_section_type.clone());

        if previous_cross_section_type != converted_cross_section_type
        {
            if self.assigned_properties_to_lines.contains_key(name)
            {
                if let Some(mut changed_assigned_properties_to_lines) =
                    self.changed_assigned_properties_to_lines.remove(&action_id)
                {
                    self.clear_by_action_id(action_id);

                    if changed_assigned_properties_to_lines.len() != 1
                    {
                        let error_message = &format!("{}: Incorrect number of assigned \
                            properties!", error_message_header);
                        return Err(JsValue::from(error_message));
                    }

                    let (changed_assigned_property_to_lines_name,
                        changed_assigned_property_to_lines) =
                        changed_assigned_properties_to_lines.remove(0)
                            .extract_and_drop();

                    if name != changed_assigned_property_to_lines_name
                    {
                        let error_message = &format!("{}: Previously changed assigned \
                            property to lines name {} does not match with {}!",
                            error_message_header, changed_assigned_property_to_lines_name, name);
                        return Err(JsValue::from(error_message));
                    }

                    let changed_assigned_property_to_lines_line_numbers =
                        changed_assigned_property_to_lines.extract_related_lines_numbers();

                    let assigned_property_to_lines_line_numbers =
                        self.assigned_properties_to_lines.get(name).unwrap()
                            .extract_related_lines_numbers();

                    if !are_line_numbers_same(
                        changed_assigned_property_to_lines_line_numbers.as_slice(),
                        assigned_property_to_lines_line_numbers.as_slice())
                    {
                        let error_message = format!("{}: The line numbers {:?} in changed \
                            assigned property to lines do not match with {:?}!",
                            error_message_header,
                            assigned_property_to_lines_line_numbers.as_slice(),
                            changed_assigned_property_to_lines_line_numbers.as_slice());
                        return Err(JsValue::from(&error_message));
                    }

                    let related_lines_data =
                        changed_assigned_property_to_lines.clone_related_lines_data();

                    self.assigned_properties_to_lines.insert(
                        changed_assigned_property_to_lines_name,
                        changed_assigned_property_to_lines);

                    let detail = json!({ "assigned_properties_to_lines_data":
                        {
                            "name": name,
                            "related_lines_data": related_lines_data,
                            "line_numbers": changed_assigned_property_to_lines_line_numbers,
                            "cross_section_type": converted_cross_section_type.as_str().to_lowercase(),
                        },
                        "is_action_id_should_be_increased": false });
                    dispatch_custom_event(detail,
                        UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                        EVENT_TARGET)?;
                }
                else
                {
                    self.clear_by_action_id(action_id);

                    let assigned_property_to_lines =
                        self.assigned_properties_to_lines.get_mut(name).unwrap();

                    let changed_assigned_property_to_lines =
                        ChangedAssignedPropertyToLines::create(name,
                       assigned_property_to_lines.clone());

                    self.changed_assigned_properties_to_lines.insert(action_id,
                        vec![changed_assigned_property_to_lines]);

                    let line_numbers =
                        assigned_property_to_lines.extract_related_lines_numbers();

                    match converted_cross_section_type
                    {
                        CrossSectionType::Truss =>
                            {
                                for line_number in line_numbers.iter()
                                {
                                    assigned_property_to_lines.update_related_lines_data(
                                        *line_number, None);
                                }
                            },
                        _ => ()
                    }

                    let related_lines_data =
                        assigned_property_to_lines.clone_related_lines_data();

                    let detail = json!({ "assigned_properties_to_lines_data":
                        {
                            "name": name,
                            "related_lines_data": related_lines_data,
                            "line_numbers": line_numbers,
                            "cross_section_type": converted_cross_section_type.as_str().to_lowercase(),
                        },
                        "is_action_id_should_be_increased": false });
                    dispatch_custom_event(detail,
                        UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                        EVENT_TARGET)?;
                }
            }
            else
            {
                self.clear_by_action_id(action_id);
            }
        }
        else
        {
            self.clear_by_action_id(action_id);
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


    pub fn delete_properties(&mut self, action_id: T, name: &str,
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
                Property with name {} do not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn delete_properties_by_names(&mut self, action_id: T,
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


    pub fn restore_properties(&mut self, action_id: T, name: &str,
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
                    action: Property with name {} do not exist!", name);
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
                Property with name {} do not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_properties_by_action_id(&mut self, action_id: T)
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
