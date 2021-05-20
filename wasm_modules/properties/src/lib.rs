use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde_json::json;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const EVENTS_TARGET: &str = "fea-app";

const ADD_MATERIAL: &str = "add material server message";


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


#[derive(Debug)]
struct Material
{
    action_id: u32,
    name: String,
    young_modulus: f64,
    poisson_ratio: f64
}


impl Material
{
    fn create(action_id: u32, name: String, young_modulus: f64, poisson_ratio: f64) -> Material
    {
        Material { action_id, name, young_modulus, poisson_ratio }
    }

    fn action_id_greater_or_same(&self, action_id: &u32) -> bool
    {
        self.action_id >= *action_id
    }


    fn name_same(&self, name: &str) -> bool
    {
        self.name == name
    }


    fn data_same(&self, young_modulus: f64, poisson_ration: f64) -> bool
    {
        self.young_modulus == young_modulus && self.poisson_ratio == poisson_ration
    }
}


struct Property
{
    action_id: u32,
    name: String,
    material: Rc<RefCell<Material>>,
}


impl Property
{
    fn create(action_id: u32, name: String, material: Rc<RefCell<Material>>) -> Property
    {
        Property { action_id, name, material }
    }
}


#[wasm_bindgen]
pub struct Properties
{
    properties: Vec<Property>,
    materials: Vec<Rc<RefCell<Material>>>,
    deleted_materials: Vec<Rc<RefCell<Material>>>,
}


#[wasm_bindgen]
impl Properties
{
    pub fn create() -> Properties
    {
        let properties = Vec::new();
        let materials = Vec::new();
        let deleted_materials = Vec::new();
        Properties { properties, materials, deleted_materials }
    }


    fn clear_deleted_materials_by_action_id(&mut self, action_id: &u32)
    {
        while let Some(position) = self.deleted_materials.iter()
            .position(|material|
                material.borrow().action_id_greater_or_same(&action_id))
        {
            let _ = self.deleted_materials.remove(position);
        }
    }


    pub fn add_material(&mut self, action_id: u32, name: String, young_modulus: f64,
        poisson_ratio: f64, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(&action_id);
        if self.materials.iter().position(|material|
            material.borrow().name_same(&name)).is_some()
        {
            let error_message = &format!("Properties: Add material action: Material with \
                name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }
        if self.materials.iter().position(|material|
            material.borrow().data_same(young_modulus, poisson_ratio)).is_some()
        {
            let error_message = &format!("Properties: Add material action: Material with \
                Young's modulus {} and Poisson's ratio {} does already exist!",
                    young_modulus, poisson_ratio);
            return Err(JsValue::from(error_message));
        }
        let material = Material::create(action_id, name.clone(), young_modulus,
            poisson_ratio);
        self.materials.push(Rc::new(RefCell::new(material)));
        let detail = json!({ "material_data": { "name": name, "young_modulus": young_modulus,
            "poisson_ratio": poisson_ratio },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_MATERIAL, EVENTS_TARGET)?;
        log(&format!("Properties: Materials: {:?}, deleted materials: {:?}",
            self.materials, self.deleted_materials));
        Ok(())
    }
}
