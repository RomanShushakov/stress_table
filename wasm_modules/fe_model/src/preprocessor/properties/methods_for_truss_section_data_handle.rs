use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::truss_section::{TrussSection, DeletedTrussSection};
use crate::preprocessor::properties::property::{Property, DeletedProperty};
use crate::preprocessor::properties::property::{CrossSectionType};
use crate::preprocessor::properties::assigned_property::{AssignedProperty, DeletedAssignedProperty};
use crate::preprocessor::properties::consts::
{
    ADD_TRUSS_SECTION_EVENT_NAME, UPDATE_TRUSS_SECTION_EVENT_NAME,
    DELETE_TRUSS_SECTION_EVENT_NAME, DELETE_PROPERTIES_EVENT_NAME, ADD_PROPERTIES_EVENT_NAME,
    ADD_ASSIGNED_PROPERTIES_EVENT_NAME, DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
};

use crate::types::{FEUInt, FEFloat};

use crate::consts::EVENT_TARGET;

use crate::functions::{log, dispatch_custom_event};


impl Properties
{
    pub fn add_truss_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        area2: Option<FEFloat>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        if self.truss_sections.contains_key(&name.to_owned())
        {
            let error_message = &format!("Properties: Add truss section action: \
                Truss section with name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }
        if self.truss_sections.values().position(|truss_section|
            truss_section.data_same(area, area2)).is_some()
        {
            let error_message = &format!("Properties: Add truss section action: \
                Truss section with Area {} and Area 2 {:?} does already exist!",
                    area, area2);
            return Err(JsValue::from(error_message));
        }
        let truss_section = TrussSection::create(area, area2);
        self.truss_sections.insert(name.to_owned(), truss_section);
        let detail = json!({ "truss_section_data": { "name": name, "area": area,
            "area2": area2 },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_TRUSS_SECTION_EVENT_NAME,
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


    pub fn update_truss_section(&mut self, action_id: FEUInt, name: &str, area: FEFloat,
        area2: Option<FEFloat>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        if self.truss_sections.values().position(|truss_section|
            truss_section.data_same(area, area2)).is_some()
        {
            let error_message = &format!("Properties: Update truss section action: \
                Truss section with Area {} and Area 2 {:?} does already exist!",
                    area, area2);
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
             let error_message = format!("Properties: Update truss section action: \
                The truss section with name {} could not be updated because it does not exist!",
                name);
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
                extracted_cross_section_type) = property.extract_data();
            if extracted_cross_section_name == truss_section_name &&
                extracted_cross_section_type == CrossSectionType::Truss
            {
                property_names_for_delete.push(property_name.clone());
            }
        }
        property_names_for_delete
    }


    pub fn delete_truss_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        let deleted_property_names =
            self.extract_property_names_for_delete_by_truss_section_name(name);
        let deleted_assigned_property_names =
            self.extract_assigned_property_names_for_delete_by_property_names(
                &deleted_property_names);
        let mut deleted_properties = Vec::new();
        let mut deleted_assigned_properties = Vec::new();

        for assigned_property_name in deleted_assigned_property_names.iter()
        {
            let assigned_property =
                self.assigned_properties.remove(assigned_property_name).unwrap();
            let deleted_assigned_property = DeletedAssignedProperty::create(
                assigned_property_name, assigned_property);
            deleted_assigned_properties.push(deleted_assigned_property);

            let detail = json!({ "assigned_properties_data": { "name": assigned_property_name },
                "is_action_id_should_be_increased": false });
            dispatch_custom_event(detail, DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
        }
        if !deleted_assigned_properties.is_empty()
        {
            self.deleted_assigned_properties.insert(action_id, deleted_assigned_properties);
        }

        for property_name in deleted_property_names.iter()
        {
            let property = self.properties.remove(property_name).unwrap();
            let deleted_property = DeletedProperty::create(property_name, property);
            deleted_properties.push(deleted_property);
            let detail = json!({ "properties_data": { "name": property_name },
                "is_action_id_should_be_increased": false });
            dispatch_custom_event(detail, DELETE_PROPERTIES_EVENT_NAME,
                EVENT_TARGET)?;
        }
        if !deleted_properties.is_empty()
        {
            self.deleted_properties.insert(action_id, deleted_properties);
        }

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
            let error_message = &format!("Properties: Delete truss section action: \
                Truss section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_truss_section(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_truss_section) =
            self.deleted_truss_sections.remove(&action_id)
        {
            let (deleted_truss_section_name, area, area2) =
                deleted_truss_section.extract_name_and_data();
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
                        "is_action_id_should_be_increased": is_action_id_should_be_increased });
                    dispatch_custom_event(detail, ADD_PROPERTIES_EVENT_NAME,
                        EVENT_TARGET)?;
                }
            }
            if let Some(deleted_assigned_properties) =
                self.deleted_assigned_properties.remove(&action_id)
            {
                for deleted_assigned_property in &deleted_assigned_properties
                {
                    let (name, line_numbers) =
                        deleted_assigned_property.extract_name_and_data();
                    self.assigned_properties.insert(name.to_owned(),
                        AssignedProperty::create(line_numbers));
                    let detail = json!({ "assigned_properties_data": { "name": name,
                        "line_numbers": line_numbers },
                        "is_action_id_should_be_increased": is_action_id_should_be_increased });
                    dispatch_custom_event(detail, ADD_ASSIGNED_PROPERTIES_EVENT_NAME,
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
        else
        {
            let error_message = &format!("Properties: Restore truss section action: \
                Truss section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }
}
