use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::{Properties, Material, DeletedMaterial};
use crate::{log, dispatch_custom_event};
use crate::
{
    EVENT_TARGET, ADD_MATERIAL_EVENT_NAME, UPDATE_MATERIAL_EVENT_NAME, DELETE_MATERIAL_EVENT_NAME
};


#[wasm_bindgen]
impl Properties
{
    pub fn add_material(&mut self, action_id: u32, name: &str, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
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
            cross sections: {:?}, deleted cross sections: {:?}",
            self.materials, self.deleted_materials,
            self.cross_sections, self.deleted_cross_sections));
        Ok(())
    }


    pub fn update_material(&mut self, action_id: u32, name: &str, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
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
                cross sections: {:?}, deleted cross sections: {:?}",
                self.materials, self.deleted_materials,
                self.cross_sections, self.deleted_cross_sections));
            Ok(())
        }
        else
        {
            let error_message = format!("Properties: Update material action: \
                The material with name {} could not be updated because it does not exist!", name);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_material(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
        if let Some((material_name, material)) = self.materials.remove_entry(&name.to_owned())
        {
            let deleted_material = DeletedMaterial::create(&material_name, material);
            self.deleted_materials.insert(action_id, deleted_material);
            let detail = json!({ "material_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_MATERIAL_EVENT_NAME, EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                cross sections: {:?}, deleted cross sections: {:?}",
                self.materials, self.deleted_materials,
                self.cross_sections, self.deleted_cross_sections));
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Delete material action: Material with \
                name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_material(&mut self, action_id: u32, name: &str,
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
            dispatch_custom_event(detail, ADD_MATERIAL_EVENT_NAME, EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                cross sections: {:?}, deleted cross sections: {:?}",
                self.materials, self.deleted_materials,
                self.cross_sections, self.deleted_cross_sections));
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



