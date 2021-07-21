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
    DELETE_ASSIGNED_PROPERTIES_EVENT_NAME, UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
};

use crate::types::{FEUInt, FEFloat};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};



impl Properties
{
    pub fn add_material(&mut self, action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

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
        self.logging();
        Ok(())
    }


    pub fn update_material(&mut self, action_id: FEUInt, name: &str, young_modulus: FEFloat,
        poisson_ratio: FEFloat, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

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
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = format!("Properties: Update material action: \
                The material with name {} does not exist!", name);
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
        self.clear_properties_module_by_action_id(action_id);

        let deleted_property_names =
            self.extract_property_names_for_delete_by_material_name(name);
        let deleted_assigned_property_names =
            self.extract_assigned_property_names_for_delete_by_property_names(
                &deleted_property_names);
        let changed_beam_sections_orientations =
            self.extract_beam_section_orientations_for_change_by_assigned_property_names(
                &deleted_assigned_property_names
            );

        let mut deleted_properties = Vec::new();
        let mut deleted_assigned_properties = Vec::new();

        for changed_beam_section_orientation in &changed_beam_sections_orientations
        {
            let local_axis_1_direction =
                changed_beam_section_orientation.extract_local_axis_1_direction();
            if let Some(position) = self.beam_sections_orientations
                .iter()
                .position(|beam_section_orientation|
                    beam_section_orientation
                        .is_local_axis_1_direction_same(&local_axis_1_direction))
            {
                let line_numbers = self.beam_sections_orientations[position]
                    .extract_line_numbers();
                let detail = json!({ "beam_section_orientation_data":
                    {
                        "local_axis_1_direction": local_axis_1_direction,
                        "line_numbers": line_numbers,
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }
        self.changed_beam_sections_orientations.insert(action_id,
            changed_beam_sections_orientations);

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
            self.logging();
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
            if let Some(beam_sections_orientations) =
                self.changed_beam_sections_orientations.remove(&action_id)
            {
                for beam_section_orientation in &beam_sections_orientations
                {
                    let (local_axis_1_direction, line_numbers) =
                        beam_section_orientation.extract_direction_and_line_numbers();
                    if let Some(position) = self.beam_sections_orientations
                        .iter()
                        .position(|beam_section_orientation|
                            beam_section_orientation.is_local_axis_1_direction_same(
                                &local_axis_1_direction))
                    {
                        self.beam_sections_orientations[position].update(line_numbers);
                        let detail = json!({ "beam_section_orientation_data":
                            {
                                "local_axis_1_direction": local_axis_1_direction,
                                "line_numbers": line_numbers,
                            },
                            "is_action_id_should_be_increased": is_action_id_should_be_increased });
                        dispatch_custom_event(detail,
                            UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
                            EVENT_TARGET)?;
                    }
                }
            }
            self.logging();
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
