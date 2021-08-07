use wasm_bindgen::prelude::*;
use serde_json::json;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;

use crate::preprocessor::traits::ClearByActionIdTrait;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::material::{Material, DeletedMaterial};
use crate::preprocessor::properties::consts::
{
    ADD_MATERIAL_EVENT_NAME, UPDATE_MATERIAL_EVENT_NAME, DELETE_MATERIAL_EVENT_NAME,
};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};


impl<T, V> Properties<T, V>
    where T: Copy + Debug + Eq + Hash + Serialize + PartialOrd,
          V: Copy + Debug + Serialize + PartialEq,
{
    pub fn add_material(&mut self, action_id: T, name: &str, young_modulus: V,
        poisson_ratio: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Add material action";

        self.clear_by_action_id(action_id);

        if self.materials.contains_key(&name.to_owned())
        {
            let error_message = &format!("{}: Material with name {} does already exist!",
                error_message_header, name);
            return Err(JsValue::from(error_message));
        }

        if self.materials.values().position(|material|
            material.is_data_same(young_modulus, poisson_ratio)).is_some()
        {
            let error_message = &format!("{}: Material with Young's modulus {:?} and \
                Poisson's ratio {:?} does already exist!", error_message_header,
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


    pub fn update_material(&mut self, action_id: T, name: &str, young_modulus: V,
        poisson_ratio: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Update material action";

        self.clear_by_action_id(action_id);

        if self.materials.values().position(|material|
            material.is_data_same(young_modulus, poisson_ratio)).is_some()
        {
            let error_message = &format!("{}: Material with Young's modulus {:?} and \
                Poisson's ratio {:?} does already exist!", error_message_header, young_modulus,
                poisson_ratio);
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
            let error_message = format!("{}: The material with name {} does not exist!",
                error_message_header, name);
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


    pub fn delete_material(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {

        self.clear_by_action_id(action_id);

        let property_names_for_delete =
            self.extract_property_names_for_delete_by_material_name(name);

        self.delete_assigned_properties_to_lines_by_names(action_id, &property_names_for_delete)?;

        self.delete_properties_by_names(action_id, &property_names_for_delete)?;

        if let Some((material_name, material)) =
            self.materials.remove_entry(&name.to_owned())
        {
            let deleted_material = DeletedMaterial::create(&material_name, material);
            self.deleted_materials.insert(action_id, deleted_material);
            let detail = json!({ "material_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_MATERIAL_EVENT_NAME,
                EVENT_TARGET)?;
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


    pub fn restore_material(&mut self, action_id: T, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Restore material action";

        if let Some(deleted_material) = self.deleted_materials.remove(&action_id)
        {
            let (deleted_material_name, young_modulus, poisson_ratio) =
                deleted_material.extract_name_and_data();
            if deleted_material_name != name
            {
                let error_message = &format!("{}: Material with name {} does not exist!",
                    error_message_header, name);
                return Err(JsValue::from(error_message));
            }
            self.materials.insert(deleted_material_name.to_owned(), Material::create(
                young_modulus, poisson_ratio));
            let detail = json!({ "material_data": { "name": deleted_material_name,
                    "young_modulus": young_modulus, "poisson_ratio": poisson_ratio },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_MATERIAL_EVENT_NAME,
                EVENT_TARGET)?;

            self.restore_properties_by_action_id(action_id)?;

            self.restore_assigned_properties_to_lines_by_action_id(action_id)?;

            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("{}: Material with name {} does not exist!",
                error_message_header, name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn extract_materials(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_materials = json!({ "extracted_materials": self.materials });
        let composed_extracted_materials =
            JsValue::from_serde(&extracted_materials)
                .or(Err(JsValue::from("Properties: Extract materials: Materials could not \
                    be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_materials);
        Ok(())
    }
}
