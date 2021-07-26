use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::traits::ClearByActionIdTrait;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::assigned_property::
{
    AssignedProperty, DeletedAssignedProperty, AssignedPropertyToLines,
    DeletedAssignedPropertyToLines, ChangedAssignedPropertyToLines
};
use crate::preprocessor::properties::consts::
{
    ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME, UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME, UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
};

use crate::preprocessor::properties::functions::line_numbers_same;

use crate::types::{FEUInt};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};


impl Properties
{
    pub fn add_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        line_numbers: &[FEUInt], is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Add assigned properties to lines action";

        self.clear_by_action_id(action_id);

        self.check_for_property_existence_by_name(name, error_message_header)?;

        if self.assigned_properties_to_lines.contains_key(name)
        {
            let error_message = &format!("{}: Assigned property to lines with name {} \
                does already exist!", error_message_header, name);
            return Err(JsValue::from(error_message));
        }

        self.check_for_the_similar_line_numbers_in_assigned_properties_to_lines_existence(
            line_numbers, error_message_header)?;

        self.check_for_line_numbers_intersection_in_assigned_properties_to_lines(
            name, line_numbers, error_message_header)?;

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
        let error_message_header = "Properties: Update assigned properties \
            to lines action";

        if let Some(mut previously_changed_assigned_properties_to_lines) =
            self.changed_assigned_properties_to_lines.remove(&action_id)
        {
            self.clear_by_action_id(action_id);

            if previously_changed_assigned_properties_to_lines.len() != 1
            {
                let error_message = &format!("{}: Incorrect number of assigned properties!",
                    error_message_header);
                return Err(JsValue::from(error_message));
            }

            let (assigned_property_to_lines_name, assigned_property_to_lines) =
                previously_changed_assigned_properties_to_lines.remove(0).extract_and_drop();

            if assigned_property_to_lines_name != name
            {
                let error_message = format!("{}: The changed assigned property to lines \
                    name {} does not match with {}!", error_message_header,
                    assigned_property_to_lines_name, name);
                return Err(JsValue::from(&error_message));
            }

            let related_lines_numbers = assigned_property_to_lines
                .extract_related_lines_numbers();

            if !line_numbers_same(related_lines_numbers.as_slice(),
                line_numbers)
            {
                let error_message = format!("{}: The line numbers {:?} in changed assigned \
                    property to lines do not match with {:?}!", error_message_header,
                    related_lines_numbers.as_slice(), line_numbers);
                return Err(JsValue::from(&error_message));
            }

            self.check_for_property_existence_by_name(&assigned_property_to_lines_name,
                error_message_header)?;

            self.check_for_the_similar_line_numbers_in_assigned_properties_to_lines_existence(
                related_lines_numbers.as_slice(),
                error_message_header)?;

            self.check_for_line_numbers_intersection_in_assigned_properties_to_lines(
                &assigned_property_to_lines_name, related_lines_numbers.as_slice(),
                error_message_header)?;

            if let Some(old_assigned_property_to_lines) =
                self.assigned_properties_to_lines.remove(name)
            {
                let old_related_lines_numbers =
                    old_assigned_property_to_lines.extract_related_lines_numbers();

                let changed_assigned_property_to_lines =
                    ChangedAssignedPropertyToLines::create(name, old_assigned_property_to_lines);

                self.assigned_properties_to_lines.insert(assigned_property_to_lines_name.clone(),
                    assigned_property_to_lines);

                self.changed_assigned_properties_to_lines.insert(action_id,
                    vec![changed_assigned_property_to_lines]);

                let (_, _, cross_section_type) =
                    self.properties.get(&assigned_property_to_lines_name).unwrap().extract_data();

                let related_lines_data =
                    self.assigned_properties_to_lines.get(&assigned_property_to_lines_name)
                        .unwrap().extract_related_lines_data();
                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": &assigned_property_to_lines_name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": related_lines_numbers,
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
                 let error_message = format!("{}: The assigned property to lines with \
                    name {} does not exist!", error_message_header, assigned_property_to_lines_name);
                Err(JsValue::from(&error_message))
            }
        }
        else
        {
            self.clear_by_action_id(action_id);

            self.check_for_property_existence_by_name(name, error_message_header)?;

            self.check_for_the_similar_line_numbers_in_assigned_properties_to_lines_existence(
                line_numbers, error_message_header)?;

            self.check_for_line_numbers_intersection_in_assigned_properties_to_lines(
                name, line_numbers, error_message_header)?;

            if let Some(assigned_property_to_lines) =
                self.assigned_properties_to_lines.get_mut(name)
            {
                let old_assigned_property_to_lines =
                    assigned_property_to_lines.clone();

                let old_related_lines_numbers =
                    old_assigned_property_to_lines.extract_related_lines_numbers();

                assigned_property_to_lines.fit_related_lines_data_by_line_numbers(
                    line_numbers);

                let changed_assigned_property_to_lines =
                    ChangedAssignedPropertyToLines::create(name, old_assigned_property_to_lines);

                self.changed_assigned_properties_to_lines.insert(action_id,
                    vec![changed_assigned_property_to_lines]);

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
                        "old_line_numbers": old_related_lines_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": is_action_id_should_be_increased });
                dispatch_custom_event(detail,
                    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
                self.logging();
                Ok(())
            }
            else
            {
                 let error_message = format!("{}: The assigned property to lines with \
                    name {} does not exist!", error_message_header, name);
                Err(JsValue::from(&error_message))
            }
        }
    }


    pub fn delete_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Delete assigned properties to lines action";

        self.clear_by_action_id(action_id);

        self.check_for_property_existence_by_name(name, error_message_header)?;

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
            let error_message = &format!("{}: Assigned property to lines with name {} does \
                not exist!", error_message_header, name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn delete_assigned_properties_to_lines_by_names(&mut self, action_id: FEUInt,
        property_names_for_delete: &[String]) -> Result<(), JsValue>
    {
        let mut assigned_properties_to_lines_for_delete = Vec::new();

        for property_name in property_names_for_delete.iter()
        {
            if let Some(assigned_property_to_lines) =
            self.assigned_properties_to_lines.remove(property_name)
            {
                let related_lines_numbers =
                    assigned_property_to_lines.extract_related_lines_numbers();
                let deleted_assigned_property_to_lines =
                    DeletedAssignedPropertyToLines::create(property_name,
                                                           assigned_property_to_lines);
                assigned_properties_to_lines_for_delete.push(
                    deleted_assigned_property_to_lines);
                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": property_name,
                        "line_numbers": related_lines_numbers,
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }

        if !assigned_properties_to_lines_for_delete.is_empty()
        {
            self.deleted_assigned_properties_to_lines.insert(action_id,
                assigned_properties_to_lines_for_delete);
        }

        Ok(())
    }


    pub fn restore_assigned_properties_to_lines(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Restore assigned properties to lines action";

        if let Some(mut deleted_assigned_property_to_lines) =
            self.deleted_assigned_properties_to_lines.remove(&action_id)
        {
            if deleted_assigned_property_to_lines.len() != 1
            {
                let error_message = &format!("{}: Incorrect number of assigned properties!",
                    error_message_header);
                return Err(JsValue::from(error_message));
            }
            let (deleted_assigned_property_to_lines_name, deleted_assigned_property_to_lines) =
                deleted_assigned_property_to_lines.remove(0).extract_and_drop();

            self.check_for_property_existence_by_name(
                &deleted_assigned_property_to_lines_name, error_message_header)?;

            if deleted_assigned_property_to_lines_name != name
            {
                let error_message = &format!("{}: Assigned property to lines with name {} \
                    does not exist!", error_message_header, name);
                return Err(JsValue::from(error_message));
            }

            self.assigned_properties_to_lines.insert(deleted_assigned_property_to_lines_name,
                deleted_assigned_property_to_lines);

            let (_, _, cross_section_type) =
                self.properties.get(name).unwrap().extract_data();

            let related_lines_data =
                self.assigned_properties_to_lines.get(name).unwrap().extract_related_lines_data();

            let related_lines_numbers = self.assigned_properties_to_lines.get(name)
                .unwrap().extract_related_lines_numbers();

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
            let error_message = &format!("{}: Assigned property to lines with name {} \
                does not exist!", error_message_header, name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_assigned_properties_to_lines_by_action_id(&mut self, action_id: FEUInt)
        -> Result<(), JsValue>
    {
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

                self.assigned_properties_to_lines.insert(
                    assigned_property_to_lines_name.clone(), assigned_property_to_lines);

                let (_, _, cross_section_type) = self.properties.get(
                    &assigned_property_to_lines_name).unwrap().extract_data();

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": &assigned_property_to_lines_name,
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
        Ok(())
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
