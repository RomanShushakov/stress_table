use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

mod material;
use material::{Material, DeletedMaterial};

mod truss_section;
use truss_section::{TrussSection, DeletedTrussSection};

mod beam_section;
use beam_section::{BeamSection, DeletedBeamSection};

mod property;
use property::{Property, DeletedProperty, CrossSectionType};

mod methods_for_material_data_handle;
mod methods_for_truss_section_data_handle;
mod methods_for_beam_section_data_handle;
mod methods_for_properties_data_handle;


const EVENT_TARGET: &str = "fea-app";

const ADD_MATERIAL_EVENT_NAME: &str = "add_material_server_message";
const UPDATE_MATERIAL_EVENT_NAME: &str = "update_material_server_message";
const DELETE_MATERIAL_EVENT_NAME: &str = "delete_material_server_message";

const ADD_TRUSS_SECTION_EVENT_NAME: &str = "add_truss_section_server_message";
const UPDATE_TRUSS_SECTION_EVENT_NAME: &str = "update_truss_section_server_message";
const DELETE_TRUSS_SECTION_EVENT_NAME: &str = "delete_truss_section_server_message";

const ADD_BEAM_SECTION_EVENT_NAME: &str = "add_beam_section_server_message";
const UPDATE_BEAM_SECTION_EVENT_NAME: &str = "update_beam_section_server_message";
const DELETE_BEAM_SECTION_EVENT_NAME: &str = "delete_beam_section_server_message";

const ADD_PROPERTIES_EVENT_NAME: &str = "add_properties_server_message";
const UPDATE_PROPERTIES_EVENT_NAME: &str = "update_properties_server_message";
const DELETE_PROPERTIES_EVENT_NAME: &str = "delete_properties_server_message";

const DELETED_LINE_NUMBERS_MESSAGE_HEADER: &str = "deleted_line_numbers";

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


struct AssignedProperty
{
    line_numbers: Vec<u32>,
}


struct ChangedAssignedProperty
{
    name: String,
    line_numbers: Vec<u32>,
}


struct BeamSectionOrientationKey
{
    property_name: String,
    local_axis_1_direction: [f64; 3]
}


struct BeamSectionOrientation
{
    line_numbers: Vec<u32>,
}


struct ChangedBeamSectionOrientation
{
    beam_section_orientation_key: BeamSectionOrientationKey,
    beam_section_orientation: BeamSectionOrientation,
}


#[wasm_bindgen]
pub struct Properties
{
    materials: HashMap<String, Material>,   // { material_name: Material }
    deleted_materials: HashMap<u32, DeletedMaterial>,   // { action_id: DeletedMaterial }
    truss_sections: HashMap<String, TrussSection>,  // { truss_section_name: TrussSection }
    deleted_truss_sections: HashMap<u32, DeletedTrussSection>,  // { action_id: DeletedTrussSection }
    beam_sections: HashMap<String, BeamSection>,    // { beam_section_name: BeamSection }
    deleted_beam_sections: HashMap<u32, DeletedBeamSection>,  // { action_id: DeletedBeamSection }
    properties: HashMap<String, Property>,  // { property_name: Property }
    deleted_properties: HashMap<u32, Vec<DeletedProperty>>,  // { action_id: Vec<DeletedProperty> }

    assigned_properties: HashMap<String, AssignedProperty>, // { property_name: AssignedProperties }
    changed_assigned_properties: HashMap<u32, ChangedAssignedProperty>,   // { action_id: ChangedAssignedProperties }
    beam_sections_orientations: HashMap<BeamSectionOrientationKey, BeamSectionOrientation>,
    changed_beam_sections_orientations: HashMap<u32, ChangedBeamSectionOrientation>,    // { action_id: ChangedBeamSectionOrientation }
}


#[wasm_bindgen]
impl Properties
{
    pub fn create() -> Properties
    {
        let materials = HashMap::new();
        let deleted_materials = HashMap::new();
        let truss_sections = HashMap::new();
        let deleted_truss_sections = HashMap::new();
        let beam_sections = HashMap::new();
        let deleted_beam_sections = HashMap::new();

        let properties = HashMap::new();
        let deleted_properties = HashMap::new();
        let assigned_properties = HashMap::new();
        let changed_assigned_properties = HashMap::new();
        let beam_sections_orientations = HashMap::new();
        let changed_beam_sections_orientations = HashMap::new();
        Properties {
            materials, deleted_materials,
            truss_sections, deleted_truss_sections,
            beam_sections, deleted_beam_sections,

            properties, deleted_properties,
            assigned_properties, changed_assigned_properties,
            beam_sections_orientations, changed_beam_sections_orientations,
        }
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


    fn clear_deleted_truss_sections_by_action_id(&mut self, action_id: u32)
    {
        for action_id in self.deleted_truss_sections.clone()
            .keys()
            .filter(|deletion_action_id| **deletion_action_id >= action_id)
            .collect::<Vec<&u32>>()
            .iter()
        {
            let _ = self.deleted_truss_sections.remove(&action_id);
        }
    }


    fn clear_deleted_beam_sections_by_action_id(&mut self, action_id: u32)
    {
        for action_id in self.deleted_beam_sections.clone()
            .keys()
            .filter(|deletion_action_id| **deletion_action_id >= action_id)
            .collect::<Vec<&u32>>()
            .iter()
        {
            let _ = self.deleted_beam_sections.remove(&action_id);
        }
    }


    fn clear_deleted_properties_by_action_id(&mut self, action_id: u32)
    {
        for action_id in self.deleted_properties.clone()
            .keys()
            .filter(|deletion_action_id| **deletion_action_id >= action_id)
            .collect::<Vec<&u32>>()
            .iter()
        {
            let _ = self.deleted_properties.remove(&action_id);
        }
    }


    pub fn clear_properties_module_by_action_id(&mut self, action_id: u32)
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
    }


    pub fn delete_line_numbers(&mut self, action_id: u32, line_numbers: JsValue)
        -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);

        let serialized_deleted_line_numbers: Value = line_numbers
            .into_serde()
            .or(Err(JsValue::from(
            "Properties: Deleted line numbers could not be serialized!")))?;
        if let Some(deleted_line_numbers) = serialized_deleted_line_numbers
            .get(DELETED_LINE_NUMBERS_MESSAGE_HEADER)
        {
            if let Some(line_numbers) = deleted_line_numbers.as_array()
            {
                if !line_numbers.is_empty()
                {
                    for line_number in line_numbers
                    {
                        let parsed_line_number = line_number.to_string()
                            .parse::<u32>()
                            .or(Err(JsValue::from("Properties: Deleted line number: \
                                could not be converted to u32!")))?;
                        log(&format!("deleted line number in properties: {:?}", parsed_line_number));
                    }
                }
            }
        }
        Ok(())
        // let cross_section_type = CrossSectionType::Beam;
        // let cross_section_key = CrossSectionKey::create(
        //     name, cross_section_type);
        // if let Some((cross_section_key, cross_section)) =
        //     self.cross_sections.remove_entry(&cross_section_key)
        // {
        //     let deleted_cross_section = DeletedCrossSection::create(
        //         cross_section_key, cross_section);
        //     let detail = json!({ "beam_section_data": { "name": name },
        //         "is_action_id_should_be_increased": is_action_id_should_be_increased });
        //     self.deleted_cross_sections.insert(action_id, deleted_cross_section);
        //     dispatch_custom_event(detail, DELETE_BEAM_SECTION_EVENT_NAME,
        //         EVENT_TARGET)?;
        //     log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
        //         cross sections: {:?}, deleted cross sections: {:?}",
        //         self.materials, self.deleted_materials,
        //         self.cross_sections, self.deleted_cross_sections));
        //     Ok(())
        // }
        // else
        // {
        //     let error_message = &format!("Properties: Delete beam section action: \
        //         Beam section with name {} does not exist!", name);
        //     return Err(JsValue::from(error_message));
        // }
    }


    pub fn extract_materials(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
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


    pub fn extract_truss_sections(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
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


    pub fn extract_beam_sections(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        let extracted_beam_sections = json!(
            { "extracted_beam_sections": self.beam_sections });
        let composed_extracted_beam_sections =
            JsValue::from_serde(&extracted_beam_sections)
                .or(Err(JsValue::from("Properties: Extract beam sections: Beam sections \
                    could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_beam_sections);
        Ok(())
    }


    pub fn extract_properties(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        let extracted_properties = json!(
            { "extracted_properties": self.properties });
        let composed_extracted_properties =
            JsValue::from_serde(&extracted_properties)
                .or(Err(JsValue::from("Properties: Extract properties: Properties \
                    could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_properties);
        Ok(())
    }
}
