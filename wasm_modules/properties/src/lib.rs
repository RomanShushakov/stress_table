use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde_json::json;
use std::collections::HashMap;


const EVENT_TARGET: &str = "fea-app";

const ADD_MATERIAL_EVENT_NAME: &str = "add_material_server_message";
const UPDATE_MATERIAL_EVENT_NAME: &str = "update_material_server_message";
const DELETE_MATERIAL_EVENT_NAME: &str = "delete_material_server_message";


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(value: &str);
}


fn dispatch_custom_event(detail: serde_json::Value, event_type: &str, query_selector: &str)
    -> Result<(), JsValue>
{
    let custom_event = web_sys::CustomEvent::new_with_event_init_dict(event_type,
        web_sys::CustomEventInit::new()
            .bubbles(true)
            .composed(true)
            .detail(&JsValue::from_serde(&detail).or(Err("Properties: Dispatch event: \
                detail could not be converted into JsValue!"))?))
            .or(Err(JsValue::from("Properties: Dispatch event: custom event could not be \
                constructed!")))?;
    web_sys::window().expect("no global `window` exists")
        .document().expect("should have a document on window")
        .query_selector(query_selector).or(Err(JsValue::from("Properties: Dispatch event: No \
            element find by current selector!")))?
        .unwrap()
        .dyn_into::<web_sys::EventTarget>()
        .unwrap()
        .dispatch_event(&custom_event)?;
    Ok(())
}


#[derive(Debug, Clone)]
struct Material
{
    young_modulus: f64,
    poisson_ratio: f64
}


impl Material
{
    fn create(young_modulus: f64, poisson_ratio: f64) -> Self
    {
        Material { young_modulus, poisson_ratio }
    }


    fn data_same(&self, young_modulus: f64, poisson_ration: f64) -> bool
    {
        self.young_modulus == young_modulus && self.poisson_ratio == poisson_ration
    }


    fn update(&mut self, young_modulus: f64, poisson_ration: f64)
    {
        self.young_modulus = young_modulus;
        self.poisson_ratio = poisson_ration;
    }


    fn extract_data(&self) -> (f64, f64)
    {
        (self.young_modulus, self.poisson_ratio)
    }
}


#[derive(Debug, Clone)]
struct DeletedMaterial
{
    name: String,
    material: Material,
}


impl DeletedMaterial
{
    fn create(name: String, material: Material) -> Self
    {
        DeletedMaterial { name, material }
    }


    fn extract_name_and_data(&self) -> (String, f64, f64)
    {
        let (young_modulus, poisson_ratio) = self.material.extract_data();
        (self.name.clone(), young_modulus, poisson_ratio)
    }
}


struct Property
{
    material_name: String,
}


impl Property
{
    fn create(material_name: String) -> Self
    {
        Property { material_name }
    }
}


#[wasm_bindgen]
pub struct Properties
{
    properties: HashMap<String, Property>,  // { property_name: Property }
    materials: HashMap<String, Material>,   // { material_name: Material }
    deleted_materials: HashMap<u32, DeletedMaterial>,   // { action_id: DeletedMaterial }
}


#[wasm_bindgen]
impl Properties
{
    pub fn create() -> Properties
    {
        let properties = HashMap::new();
        let materials = HashMap::new();
        let deleted_materials = HashMap::new();
        Properties { properties, materials, deleted_materials }
    }


    fn clear_deleted_materials_by_action_id(&mut self, action_id: u32)
    {
        for action_id in self.deleted_materials.clone()
            .keys()
            .filter(|deletion_action_id| **deletion_action_id >= action_id)
            .collect::<Vec<&u32>>()
            .iter()
        {
            let _ = self.deleted_materials.remove(&action_id);
        }
    }


    pub fn add_material(&mut self, action_id: u32, name: String, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        if self.materials.contains_key(&name)
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
        self.materials.insert(name.clone(), material);
        let detail = json!({ "material_data": { "name": name, "young_modulus": young_modulus,
            "poisson_ratio": poisson_ratio },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_MATERIAL_EVENT_NAME, EVENT_TARGET)?;
        log(&format!("Properties: Materials: {:?}, deleted materials: {:?}",
            self.materials, self.deleted_materials));
        Ok(())
    }


    pub fn update_material(&mut self, action_id: u32, name: String, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        if self.materials.values().position(|material|
            material.data_same(young_modulus, poisson_ratio)).is_some()
        {
            let error_message = &format!("Properties: Update material action: Material with \
                Young's modulus {} and Poisson's ratio {} does already exist!",
                    young_modulus, poisson_ratio);
            return Err(JsValue::from(error_message));
        }
        if let Some(material) = self.materials.get_mut(&name)
        {
            material.update(young_modulus, poisson_ratio);
            let detail = json!({ "material_data": { "name": name,
                "young_modulus": young_modulus, "poisson_ratio": poisson_ratio },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_MATERIAL_EVENT_NAME, EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}",
                self.materials, self.deleted_materials));
            Ok(())
        }
        else
        {
            let error_message = format!("Properties: Update material action: \
                The material with name {} could not be updated because it does not exist!", name);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_material(&mut self, action_id: u32, name: String,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        if let Some((material_name, material)) = self.materials.remove_entry(&name)
        {
            let deleted_material = DeletedMaterial::create(material_name, material);
            self.deleted_materials.insert(action_id, deleted_material);
            let detail = json!({ "material_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_MATERIAL_EVENT_NAME, EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}",
                self.materials, self.deleted_materials));
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Delete material action: Material with \
                name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn undo_delete_material(&mut self, action_id: u32, name: String,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_material) = self.deleted_materials.remove(&action_id)
        {
            let (deleted_material_name, young_modulus, poisson_ratio) =
                deleted_material.extract_name_and_data();
            if deleted_material_name != name
            {
                let error_message = &format!("Properties: Undo delete material action: \
                    Material with name {} does not exist!", name);
                return Err(JsValue::from(error_message));
            }
            let detail = json!({ "material_data": { "name": deleted_material_name,
                    "young_modulus": young_modulus, "poisson_ratio": poisson_ratio },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_MATERIAL_EVENT_NAME, EVENT_TARGET)?;
            self.materials.insert(deleted_material_name, Material::create(
                young_modulus, poisson_ratio));
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}",
                self.materials, self.deleted_materials));
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Undo delete material action: \
                Material with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }
}
