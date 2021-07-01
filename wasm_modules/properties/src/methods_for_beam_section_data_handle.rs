use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::
{
    Properties, BeamSection, DeletedBeamSection, DeletedProperty, Property, AssignedProperty,
    DeletedAssignedProperty,
};
use crate::CrossSectionType;
use crate::{log, dispatch_custom_event};
use crate::
{
    EVENT_TARGET, ADD_BEAM_SECTION_EVENT_NAME, UPDATE_BEAM_SECTION_EVENT_NAME,
    DELETE_BEAM_SECTION_EVENT_NAME, DELETE_PROPERTIES_EVENT_NAME, ADD_PROPERTIES_EVENT_NAME,
    ADD_ASSIGNED_PROPERTIES_EVENT_NAME, DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
};


#[wasm_bindgen]
impl Properties
{
    pub fn add_beam_section(&mut self, action_id: u32, name: &str, area: f64,
        i11: f64, i22: f64, i12: f64, it: f64, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        if self.beam_sections.contains_key(&name.to_owned())
        {
            let error_message = &format!("Properties: Add beam section action: \
                Beam section with name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }
        if self.beam_sections.values().position(|beam_section|
            beam_section.data_same(area, i11, i22, i12, it)).is_some()
        {
            let error_message = &format!("Properties: Add cross section action: \
                Cross section with Area {}, I11 {}, I22 {}, I12 {}, It {} does already exist!",
                area, i11, i22, i12, it);
            return Err(JsValue::from(error_message));
        }
        let beam_section = BeamSection::create(area, i11, i22, i12, it);
        self.beam_sections.insert(name.to_owned(), beam_section);
        let detail = json!({ "beam_section_data": { "name": name, "area": area,
            "i11": i11, "i22": i22, "i12": i12, "it": it },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_BEAM_SECTION_EVENT_NAME,
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


    pub fn update_beam_section(&mut self, action_id: u32, name: &str, area: f64,
        i11: f64, i22: f64, i12: f64, it: f64, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        if self.beam_sections.values().position(|beam_section|
            beam_section.data_same(area, i11, i22, i12, it)).is_some()
        {
            let error_message = &format!("Properties: Update beam section action: \
                Beam section with Area {}, I11 {}, I22 {}, I12 {} and It {} does already exist!",
                    area, i11, i22, i12, it);
            return Err(JsValue::from(error_message));
        }
        if let Some(beam_section) = self.beam_sections.get_mut(name)
        {
            beam_section.update(area, i11, i22, i12, it);
            let detail = json!({ "beam_section_data": { "name": name,
                "area": area, "i11": i11, "i22": i22, "i12": i12, "it": it },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_BEAM_SECTION_EVENT_NAME,
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
             let error_message = format!("Properties: Update beam section action: \
                The beam section with name {} could not be updated because it does not exist!",
                name);
            Err(JsValue::from(&error_message))
        }
    }


    fn extract_property_names_for_delete_by_beam_section_name(&self, beam_section_name: &str)
        -> Vec<String>
    {
        let mut property_names_for_delete = Vec::new();
        for (property_name, property) in self.properties.iter()
        {
            let (_extracted_material_name, extracted_cross_section_name,
                extracted_cross_section_type) = property.extract_data();
            if extracted_cross_section_name == beam_section_name &&
                extracted_cross_section_type == CrossSectionType::Beam
            {
                property_names_for_delete.push(property_name.clone());
            }
        }
        property_names_for_delete
    }


    pub fn delete_beam_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        let deleted_property_names =
            self.extract_property_names_for_delete_by_beam_section_name(name);
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

        if let Some((beam_section_name, beam_section)) =
            self.beam_sections.remove_entry(&name.to_owned())
        {
            let deleted_beam_section =
                DeletedBeamSection::create(&beam_section_name, beam_section);
            self.deleted_beam_sections.insert(action_id, deleted_beam_section);
            let detail = json!({ "beam_section_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_BEAM_SECTION_EVENT_NAME,
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
            let error_message = &format!("Properties: Delete beam section action: \
                Beam section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_beam_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_beam_section) =
            self.deleted_beam_sections.remove(&action_id)
        {
            let (deleted_beam_section_name, area, i11, i22, i12, it) =
                deleted_beam_section.extract_name_and_data();
            if deleted_beam_section_name != name
            {
                let error_message = &format!("Properties: Restore beam section \
                    action: Beam section with name {} does not exist!", name);
                return Err(JsValue::from(error_message));
            }
            self.beam_sections.insert(deleted_beam_section_name.to_owned(),
               BeamSection::create(area, i11, i22, i12, it));
            let detail = json!({ "beam_section_data": {
                    "name": deleted_beam_section_name,
                    "area": area,
                    "i11": i11,
                    "i22": i22,
                    "i12": i12,
                    "it": it },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_BEAM_SECTION_EVENT_NAME,
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
            let error_message = &format!("Properties: Restore beam section action: \
                Beam section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }
}
