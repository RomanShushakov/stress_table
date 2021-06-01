use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde_json::json;
use std::collections::HashMap;


const EVENT_TARGET: &str = "fea-app";

const ADD_MATERIAL_EVENT_NAME: &str = "add_material_server_message";
const UPDATE_MATERIAL_EVENT_NAME: &str = "update_material_server_message";
const DELETE_MATERIAL_EVENT_NAME: &str = "delete_material_server_message";

const ADD_TRUSS_SECTION_EVENT_NAME: &str = "add_truss_section_server_message";
const UPDATE_TRUSS_SECTION_EVENT_NAME: &str = "update_truss_section_server_message";
const DELETE_TRUSS_SECTION_EVENT_NAME: &str = "delete_truss_section_server_message";


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
    fn create(name: &str, material: Material) -> Self
    {
        DeletedMaterial { name: String::from(name), material }
    }


    fn extract_name_and_data(&self) -> (&str, f64, f64)
    {
        let (young_modulus, poisson_ratio) = self.material.extract_data();
        (&self.name, young_modulus, poisson_ratio)
    }
}


#[derive(Debug, Clone, PartialEq)]
struct CrossSectionData
{
    numerical_data: Vec<f64>,
    optional_data: Vec<Option<f64>>,
}


impl CrossSectionData
{
    fn create(numerical_data: Vec<f64>, optional_data: Vec<Option<f64>>) -> Self
    {
        CrossSectionData { numerical_data, optional_data }
    }


    fn extract_numerical_data(&self) -> &[f64]
    {
        self.numerical_data.as_slice()
    }


    fn extract_optional_data(&self) -> &[Option<f64>]
    {
        self.optional_data.as_slice()
    }
}


#[derive(Debug, Clone)]
struct CrossSection
{
    cross_section_data: CrossSectionData,
}


impl CrossSection
{
    fn create(cross_section_data: CrossSectionData) -> Self
    {
        CrossSection { cross_section_data }
    }


    fn data_same(&self, cross_section_data: &CrossSectionData) -> bool
    {
        self.cross_section_data == *cross_section_data
    }


    fn update(&mut self, cross_section_data: CrossSectionData)
    {
        self.cross_section_data = cross_section_data;
    }


    fn extract_numerical_data(&self) -> &[f64]
    {
        self.cross_section_data.extract_numerical_data()
    }


    fn extract_optional_data(&self) -> &[Option<f64>]
    {
        self.cross_section_data.extract_optional_data()
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CrossSectionType
{
    Truss,
    Beam,
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct CrossSectionKey
{
    name: String,
    cross_section_type: CrossSectionType
}


impl CrossSectionKey
{
    fn create(name: &str, cross_section_type: CrossSectionType) -> Self
    {
        CrossSectionKey { name: String::from(name), cross_section_type }
    }


    fn name_same(&self, name: &str) -> bool
    {
        self.name == name
    }


    fn extract_name(&self) -> &str
    {
        &self.name
    }
}


#[derive(Debug, Clone)]
struct DeletedCrossSection
{
    cross_section_key: CrossSectionKey,
    cross_section: CrossSection,
}


impl DeletedCrossSection
{
    fn create(cross_section_key: CrossSectionKey, cross_section: CrossSection) -> Self
    {
        DeletedCrossSection { cross_section_key, cross_section }
    }


    fn extract_key_and_data(&self) -> (&CrossSectionKey, &CrossSection)
    {
        (&self.cross_section_key, &self.cross_section)
    }
}


struct Property
{
    material_name: String,
    cross_section_key: CrossSectionKey,
    cross_section_orientation: Option<[f64; 3]>
}


impl Property
{
    fn create(material_name: String, cross_section_key: CrossSectionKey,
        cross_section_orientation: Option<[f64; 3]>) -> Self
    {
        Property { material_name, cross_section_key, cross_section_orientation }
    }
}


#[wasm_bindgen]
pub struct Properties
{
    properties: HashMap<String, Property>,  // { property_name: Property }
    materials: HashMap<String, Material>,   // { material_name: Material }
    deleted_materials: HashMap<u32, DeletedMaterial>,   // { action_id: DeletedMaterial }
    cross_sections: HashMap<CrossSectionKey, CrossSection>,
    deleted_cross_sections: HashMap<u32, DeletedCrossSection>,  // { action_id: DeletedCrossSection }
}


#[wasm_bindgen]
impl Properties
{
    pub fn create() -> Properties
    {
        let properties = HashMap::new();
        let materials = HashMap::new();
        let deleted_materials = HashMap::new();
        let cross_sections = HashMap::new();
        let deleted_cross_sections = HashMap::new();
        Properties { properties, materials, deleted_materials, cross_sections,
            deleted_cross_sections }
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


    fn clear_deleted_cross_sections_by_action_id(&mut self, action_id: u32)
    {
        for action_id in self.deleted_cross_sections.clone()
            .keys()
            .filter(|deletion_action_id| **deletion_action_id >= action_id)
            .collect::<Vec<&u32>>()
            .iter()
        {
            let _ = self.deleted_cross_sections.remove(&action_id);
        }
    }


    pub fn clear_properties_module_by_action_id(&mut self, action_id: u32)
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_materials_by_action_id(action_id);
    }


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


    pub fn add_truss_section(&mut self, action_id: u32, name: &str, area: f64,
        area2: Option<f64>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
        let cross_section_type = CrossSectionType::Truss;
        let cross_section_key = CrossSectionKey::create(
            name, cross_section_type);
        if self.cross_sections.contains_key(&cross_section_key)
        {
            let error_message = &format!("Properties: Add cross section action: \
                Truss cross section with name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }
        let cross_section_numerical_data = vec![area];
        let cross_section_optional_data = vec![area2];
        let cross_section_data = CrossSectionData::create(
            cross_section_numerical_data, cross_section_optional_data);
        if self.cross_sections.values().position(|cross_section|
            cross_section.data_same(&cross_section_data)).is_some()
        {
            let error_message = &format!("Properties: Add cross section action: \
                Cross section with Area {} and Area 2 {:?} does already exist!",
                    area, area2);
            return Err(JsValue::from(error_message));
        }
        let cross_section = CrossSection::create(cross_section_data);
        self.cross_sections.insert(cross_section_key, cross_section);
        let detail = json!({ "truss_section_data": { "name": name, "area": area,
            "area2": area2 },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_TRUSS_SECTION_EVENT_NAME,
            EVENT_TARGET)?;
        log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
            cross sections: {:?}, deleted cross sections: {:?}",
            self.materials, self.deleted_materials,
            self.cross_sections, self.deleted_cross_sections));
        Ok(())
    }


    pub fn update_truss_section(&mut self, action_id: u32, name: &str, area: f64,
        area2: Option<f64>, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
        let cross_section_type = CrossSectionType::Truss;
        let cross_section_key = CrossSectionKey::create(
            name, cross_section_type);
        let cross_section_numerical_data = vec![area];
        let cross_section_optional_data = vec![area2];
        let cross_section_data = CrossSectionData::create(
            cross_section_numerical_data, cross_section_optional_data);
        if self.cross_sections.values().position(|cross_section|
            cross_section.data_same(&cross_section_data)).is_some()
        {
            let error_message = &format!("Properties: Update truss section action: \
                Truss section with Area {} and Area 2 {:?} does already exist!",
                    area, area2);
            return Err(JsValue::from(error_message));
        }
        if let Some(truss_section) = self.cross_sections.get_mut(&cross_section_key)
        {
            truss_section.update(cross_section_data);
            let detail = json!({ "truss_section_data": { "name": name,
                "area": area, "area2": area2 },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_TRUSS_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                cross sections: {:?}, deleted cross sections: {:?}",
                self.materials, self.deleted_materials,
                self.cross_sections, self.deleted_cross_sections));
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


    pub fn delete_truss_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
        let cross_section_type = CrossSectionType::Truss;
        let cross_section_key = CrossSectionKey::create(
            name, cross_section_type);
        if let Some((cross_section_key, cross_section)) =
            self.cross_sections.remove_entry(&cross_section_key)
        {
            let deleted_cross_section = DeletedCrossSection::create(
                cross_section_key, cross_section);
            let detail = json!({ "truss_section_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            self.deleted_cross_sections.insert(action_id, deleted_cross_section);
            dispatch_custom_event(detail, DELETE_TRUSS_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                cross sections: {:?}, deleted cross sections: {:?}",
                self.materials, self.deleted_materials,
                self.cross_sections, self.deleted_cross_sections));
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Delete truss section action: \
                Truss section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_truss_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_cross_section) =
            self.deleted_cross_sections.remove(&action_id)
        {
            let cross_section_type = CrossSectionType::Truss;
            let cross_section_key = CrossSectionKey::create(name,
                cross_section_type);
            let (deleted_cross_section_key, deleted_cross_section) =
                deleted_cross_section.extract_key_and_data();
            if deleted_cross_section_key != &cross_section_key
            {
                let error_message = &format!("Properties: Restore truss section \
                    action: Truss section with name {} does not exist!", name);
                return Err(JsValue::from(error_message));
            }
            let detail = json!({ "truss_section_data": {
                    "name": deleted_cross_section_key.extract_name(),
                    "area": deleted_cross_section.extract_numerical_data()[0],
                    "area2": deleted_cross_section.extract_optional_data()[0] },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            self.cross_sections.insert(deleted_cross_section_key.to_owned(),
                deleted_cross_section.to_owned());
            dispatch_custom_event(detail, ADD_TRUSS_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                cross sections: {:?}, deleted cross sections: {:?}",
                self.materials, self.deleted_materials,
                self.cross_sections, self.deleted_cross_sections));
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
