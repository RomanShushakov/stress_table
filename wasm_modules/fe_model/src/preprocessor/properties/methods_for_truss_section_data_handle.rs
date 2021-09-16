use wasm_bindgen::prelude::*;
use serde_json::json;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;

use crate::traits::ClearByActionIdTrait;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::truss_section::{TrussSection, DeletedTrussSection};
use crate::preprocessor::properties::property::{CrossSectionType};
use crate::preprocessor::properties::consts::
{
    ADD_TRUSS_SECTION_EVENT_NAME, UPDATE_TRUSS_SECTION_EVENT_NAME,
    DELETE_TRUSS_SECTION_EVENT_NAME
};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};


impl<T, V> Properties<T, V>
    where T: Copy + Debug + Eq + Hash + Serialize + PartialOrd,
          V: Copy + Debug + Serialize + PartialEq,
{
    pub fn add_truss_section(&mut self, action_id: T, name: &str, area: V,
        area2: Option<V>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Add truss section action";

        self.clear_by_action_id(action_id);

        if self.truss_sections.contains_key(&name.to_owned())
        {
            let error_message = &format!("{}: Truss section with name {} does already \
                exist!", error_message_header, name);
            return Err(JsValue::from(error_message));
        }

        if self.truss_sections.values().position(|truss_section|
            truss_section.is_data_same(area, area2)).is_some()
        {
            let error_message = &format!("{}: Truss section with Area {:?} and Area 2 {:?} \
                does already exist!", error_message_header, area, area2);
            return Err(JsValue::from(error_message));
        }

        let truss_section = TrussSection::create(area, area2);
        self.truss_sections.insert(name.to_owned(), truss_section);
        let detail = json!({ "truss_section_data": { "name": name, "area": area,
            "area2": area2 },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_TRUSS_SECTION_EVENT_NAME,
            EVENT_TARGET)?;
        self.logging();
        Ok(())
    }


    pub fn update_truss_section(&mut self, action_id: T, name: &str, area: V,
        area2: Option<V>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Update truss section action";

        self.clear_by_action_id(action_id);

        if self.truss_sections.values().position(|truss_section|
            truss_section.is_data_same(area, area2)).is_some()
        {
            let error_message = &format!("{}:  Truss section with Area {:?} and Area 2 {:?} \
                does already exist!", error_message_header, area, area2);
            return Err(JsValue::from(error_message));
        }

        if let Some(truss_section) = self.truss_sections.get_mut(name)
        {
            truss_section.update(area, area2);
            let detail = json!({ "truss_section_data": { "name": name,
                "area": area, "area2": area2 },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_TRUSS_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
             let error_message = format!("{}: The truss section with name {} does not exist!",
                error_message_header, name);
            Err(JsValue::from(&error_message))
        }
    }


    fn extract_property_names_for_delete_by_truss_section_name(&self, truss_section_name: &str)
        -> Vec<String>
    {
        let mut property_names_for_delete = Vec::new();
        for (property_name, property) in self.properties.iter()
        {
            let (_extracted_material_name, extracted_cross_section_name,
                extracted_cross_section_type) = property.clone_data();
            if extracted_cross_section_name == truss_section_name &&
                extracted_cross_section_type == CrossSectionType::Truss
            {
                property_names_for_delete.push(property_name.clone());
            }
        }
        property_names_for_delete
    }


    pub fn delete_truss_section(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        let property_names_for_delete =
            self.extract_property_names_for_delete_by_truss_section_name(name);

        self.delete_assigned_properties_to_lines_by_names(action_id, &property_names_for_delete)?;

        self.delete_properties_by_names(action_id, &property_names_for_delete)?;

        if let Some((truss_section_name, truss_section)) =
            self.truss_sections.remove_entry(&name.to_owned())
        {
            let deleted_truss_section =
                DeletedTrussSection::create(&truss_section_name, truss_section);
            self.deleted_truss_sections.insert(action_id, deleted_truss_section);
            let detail = json!({ "truss_section_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_TRUSS_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Delete truss section action: \
                Truss section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_truss_section(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_truss_section) =
            self.deleted_truss_sections.remove(&action_id)
        {
            let (deleted_truss_section_name, area, area2) =
                deleted_truss_section.copy_name_and_data();
            if deleted_truss_section_name != name
            {
                let error_message = &format!("Properties: Restore truss section \
                    action: Truss section with name {} does not exist!", name);
                return Err(JsValue::from(error_message));
            }
            self.truss_sections.insert(deleted_truss_section_name.to_owned(),
               TrussSection::create(area, area2));
            let detail = json!({ "truss_section_data": { "name": deleted_truss_section_name,
                    "area": area, "area2": area2 },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_TRUSS_SECTION_EVENT_NAME,
                EVENT_TARGET)?;

            self.restore_properties_by_action_id(action_id)?;

            self.restore_assigned_properties_to_lines_by_action_id(action_id)?;

            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Restore truss section action: \
                Truss section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn extract_truss_sections(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_truss_sections = json!(
            { "extracted_truss_sections": self.truss_sections });
        let composed_extracted_truss_sections =
            JsValue::from_serde(&extracted_truss_sections)
                .or(Err(JsValue::from("Properties: Extract truss sections: Truss sections \
                    could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_truss_sections);
        Ok(())
    }
}
