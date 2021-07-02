use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::material::{Material, DeletedMaterial};
use crate::preprocessor::properties::property::{Property, DeletedProperty};
use crate::preprocessor::properties::assigned_property::
{
    AssignedProperty, DeletedAssignedProperty
};
use crate::preprocessor::properties::consts::
{
    ADD_MATERIAL_EVENT_NAME, UPDATE_MATERIAL_EVENT_NAME, DELETE_MATERIAL_EVENT_NAME,
    DELETE_PROPERTIES_EVENT_NAME, ADD_PROPERTIES_EVENT_NAME, ADD_ASSIGNED_PROPERTIES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_EVENT_NAME,
};

use crate::types::{FEUInt, FEFloat};

use crate::consts::EVENT_TARGET;

use crate::functions::{log, dispatch_custom_event};



impl Properties
{
    pub fn add_material(&mut self, action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        if self.materials.contains_key(&name.to_owned())
        {
            let error_message = &format!("Properties: Add material action: Material with \
                name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }
        if self.materials.values().position(|material|
            material.data_same(young_modulus, poisson_ratio)).is_some()
        {
            let error_message = &format!("Properties: Add material action: Material with \
                Young's modulus {} and Poisson's ratio {} does already exist!",
                    young_modulus, poisson_ratio);
            return Err(JsValue::from(error_message));
        }
        let material = Material::create(young_modulus, poisson_ratio);
        self.materials.insert(name.to_owned(), material);
        let detail = json!({ "material_data": { "name": name, "young_modulus": young_modulus,
            "poisson_ratio": poisson_ratio },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_MATERIAL_EVENT_NAME, EVENT_TARGET)?;
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


    pub fn update_material(&mut self, action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        if self.materials.values().position(|material|
            material.data_same(young_modulus, poisson_ratio)).is_some()
        {
            let error_message = &format!("Properties: Update material action: Material with \
                Young's modulus {} and Poisson's ratio {} does already exist!",
                    young_modulus, poisson_ratio);
            return Err(JsValue::from(error_message));
        }
        if let Some(material) = self.materials.get_mut(name)
        {
            material.update(young_modulus, poisson_ratio);
            let detail = json!({ "material_data": { "name": name,
                "young_modulus": young_modulus, "poisson_ratio": poisson_ratio },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_MATERIAL_EVENT_NAME, EVENT_TARGET)?;
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
            let error_message = format!("Properties: Update material action: \
                The material with name {} could not be updated because it does not exist!", name);
            Err(JsValue::from(&error_message))
        }
    }


    fn extract_property_names_for_delete_by_material_name(&self, material_name: &str) -> Vec<String>
    {
        let mut property_names_for_delete = Vec::new();
        for (property_name, property) in self.properties.iter()
        {
            let (extracted_material_name, _extracted_cross_section_name,
                _extracted_cross_section_type) = property.extract_data();
            if extracted_material_name == material_name
            {
                property_names_for_delete.push(property_name.clone());
            }
        }
        property_names_for_delete
    }


    pub fn delete_material(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        let deleted_property_names =
            self.extract_property_names_for_delete_by_material_name(name);
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

        if let Some((material_name, material)) =
            self.materials.remove_entry(&name.to_owned())
        {
            let deleted_material = DeletedMaterial::create(&material_name, material);
            self.deleted_materials.insert(action_id, deleted_material);
            let detail = json!({ "material_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_MATERIAL_EVENT_NAME, EVENT_TARGET)?;
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
            let error_message = &format!("Properties: Delete material action: Material with \
                name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_material(&mut self, action_id: FEUInt, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_material) = self.deleted_materials.remove(&action_id)
        {
            let (deleted_material_name, young_modulus, poisson_ratio) =
                deleted_material.extract_name_and_data();
            if deleted_material_name != name
            {
                let error_message = &format!("Properties: Restore material action: \
                    Material with name {} does not exist!", name);
                return Err(JsValue::from(error_message));
            }
            self.materials.insert(deleted_material_name.to_owned(), Material::create(
                young_modulus, poisson_ratio));
            let detail = json!({ "material_data": { "name": deleted_material_name,
                    "young_modulus": young_modulus, "poisson_ratio": poisson_ratio },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_MATERIAL_EVENT_NAME,
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
            let error_message = &format!("Properties: Restore material action: \
                Material with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }
}
