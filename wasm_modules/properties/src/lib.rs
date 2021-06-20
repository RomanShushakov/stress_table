use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

mod material;
use material::{Material, DeletedMaterial};

mod methods_for_material_data_handle;


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


#[derive(Debug, Clone)]
struct CrossSection
{
    numerical_data: Vec<f64>,
    optional_data: Vec<Option<f64>>,
}


impl CrossSection
{
    fn create(numerical_data: Vec<f64>, optional_data: Vec<Option<f64>>) -> Self
    {
        CrossSection { numerical_data, optional_data }
    }


    fn data_same(&self, numerical_data: &Vec<f64>, optional_data: &Vec<Option<f64>>) -> bool
    {
        self.numerical_data == *numerical_data && self.optional_data == *optional_data
    }


    fn update(&mut self, numerical_data: Vec<f64>, optional_data: Vec<Option<f64>>)
    {
        self.numerical_data = numerical_data;
        self.optional_data = optional_data;
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


    fn is_truss(&self) -> bool
    {
        self.cross_section_type == CrossSectionType::Truss
    }


    fn is_beam(&self) -> bool
    {
        self.cross_section_type == CrossSectionType::Beam
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
}


impl Property
{
    fn create(material_name: String, cross_section_key: CrossSectionKey) -> Self
    {
        Property { material_name, cross_section_key }
    }
}


struct DeletedProperty
{
    name: String,
    property: Property,
}


struct AssignedProperties
{
    line_numbers: Vec<u32>,
}


struct ChangedAssignedProperties
{
    property_name: String,
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
    cross_sections: HashMap<CrossSectionKey, CrossSection>,
    deleted_cross_sections: HashMap<u32, DeletedCrossSection>,  // { action_id: DeletedCrossSection }
    properties: HashMap<String, Property>,  // { property_name: Property }
    deleted_properties: HashMap<u32, DeletedProperty>,  // { action_id: DeletedProperty }
    assigned_properties: HashMap<String, AssignedProperties>, // { property_name: AssignedProperties }
    changed_assigned_properties: HashMap<u32, ChangedAssignedProperties>,   // { action_id: ChangedAssignedProperties }
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
        let cross_sections = HashMap::new();
        let deleted_cross_sections = HashMap::new();
        let properties = HashMap::new();
        let deleted_properties = HashMap::new();
        let assigned_properties = HashMap::new();
        let changed_assigned_properties = HashMap::new();
        let beam_sections_orientations = HashMap::new();
        let changed_beam_sections_orientations = HashMap::new();
        Properties { materials, deleted_materials, cross_sections,
            deleted_cross_sections, properties, deleted_properties,
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
        self.clear_deleted_cross_sections_by_action_id(action_id);
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
        if self.cross_sections.values().position(|cross_section|
            cross_section
                .data_same(&cross_section_numerical_data, &cross_section_optional_data))
                .is_some()
        {
            let error_message = &format!("Properties: Add cross section action: \
                Cross section with Area {} and Area 2 {:?} does already exist!",
                    area, area2);
            return Err(JsValue::from(error_message));
        }
        let cross_section = CrossSection::create(cross_section_numerical_data,
            cross_section_optional_data);
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
        if self.cross_sections.values().position(|cross_section|
            cross_section
                .data_same(&cross_section_numerical_data, &cross_section_optional_data))
                .is_some()
        {
            let error_message = &format!("Properties: Update truss section action: \
                Truss section with Area {} and Area 2 {:?} does already exist!",
                    area, area2);
            return Err(JsValue::from(error_message));
        }
        if let Some(truss_section) = self.cross_sections.get_mut(&cross_section_key)
        {
            truss_section.update(cross_section_numerical_data, cross_section_optional_data);
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


    pub fn add_beam_section(&mut self, action_id: u32, name: &str, area: f64,
        i11: f64, i22: f64, i12: f64, it: f64, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
        let cross_section_type = CrossSectionType::Beam;
        let cross_section_key = CrossSectionKey::create(
            name, cross_section_type);
        if self.cross_sections.contains_key(&cross_section_key)
        {
            let error_message = &format!("Properties: Add cross section action: \
                Beam cross section with name {} does already exist!", name);
            return Err(JsValue::from(error_message));
        }
        let cross_section_numerical_data = vec![area, i11, i22, i12, it];
        let cross_section_optional_data = vec![None];
        if self.cross_sections.values().position(|cross_section|
            cross_section
                .data_same(&cross_section_numerical_data, &cross_section_optional_data))
                .is_some()
        {
            let error_message = &format!("Properties: Add cross section action: \
                Cross section with Area {}, I11 {}, I22 {}, I12 {}, It {} does already exist!",
                area, i11, i22, i12, it);
            return Err(JsValue::from(error_message));
        }
        let cross_section = CrossSection::create(cross_section_numerical_data,
            cross_section_optional_data);
        self.cross_sections.insert(cross_section_key, cross_section);
        let detail = json!({ "beam_section_data": { "name": name, "area": area,
            "i11": i11, "i22": i22, "i12": i12, "it": it },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_BEAM_SECTION_EVENT_NAME,
            EVENT_TARGET)?;
        log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
            cross sections: {:?}, deleted cross sections: {:?}",
            self.materials, self.deleted_materials,
            self.cross_sections, self.deleted_cross_sections));
        Ok(())
    }


    pub fn update_beam_section(&mut self, action_id: u32, name: &str, area: f64,
        i11: f64, i22: f64, i12: f64, it: f64, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
        let cross_section_type = CrossSectionType::Beam;
        let cross_section_key = CrossSectionKey::create(
            name, cross_section_type);
        let cross_section_numerical_data = vec![area, i11, i22, i12, it];
        let cross_section_optional_data = vec![None];
        if self.cross_sections.values().position(|cross_section|
            cross_section
                .data_same(&cross_section_numerical_data, &cross_section_optional_data))
                .is_some()
        {
            let error_message = &format!("Properties: Update beam section action: \
                Beam section with Area {}, I11 {}, I22 {}, I12 {} and It {} does already exist!",
                    area, i11, i22, i12, it);
            return Err(JsValue::from(error_message));
        }
        if let Some(beam_section) = self.cross_sections.get_mut(&cross_section_key)
        {
            beam_section.update(cross_section_numerical_data, cross_section_optional_data);
            let detail = json!({ "beam_section_data": { "name": name,
                "area": area, "i11": i11, "i22": i22, "i12": i12, "it": it },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_BEAM_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                cross sections: {:?}, deleted cross sections: {:?}",
                self.materials, self.deleted_materials,
                self.cross_sections, self.deleted_cross_sections));
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


    pub fn delete_beam_section(&mut self, action_id: u32, name: &str,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
        let cross_section_type = CrossSectionType::Beam;
        let cross_section_key = CrossSectionKey::create(
            name, cross_section_type);
        if let Some((cross_section_key, cross_section)) =
            self.cross_sections.remove_entry(&cross_section_key)
        {
            let deleted_cross_section = DeletedCrossSection::create(
                cross_section_key, cross_section);
            let detail = json!({ "beam_section_data": { "name": name },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            self.deleted_cross_sections.insert(action_id, deleted_cross_section);
            dispatch_custom_event(detail, DELETE_BEAM_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                cross sections: {:?}, deleted cross sections: {:?}",
                self.materials, self.deleted_materials,
                self.cross_sections, self.deleted_cross_sections));
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
        if let Some(deleted_cross_section) =
            self.deleted_cross_sections.remove(&action_id)
        {
            let cross_section_type = CrossSectionType::Beam;
            let cross_section_key = CrossSectionKey::create(name,
                cross_section_type);
            let (deleted_cross_section_key, deleted_cross_section) =
                deleted_cross_section.extract_key_and_data();
            if deleted_cross_section_key != &cross_section_key
            {
                let error_message = &format!("Properties: Restore beam section \
                    action: Beam section with name {} does not exist!", name);
                return Err(JsValue::from(error_message));
            }
            let detail = json!({ "beam_section_data": {
                    "name": deleted_cross_section_key.extract_name(),
                    "area": deleted_cross_section.extract_numerical_data()[0],
                    "i11": deleted_cross_section.extract_numerical_data()[1],
                    "i22": deleted_cross_section.extract_numerical_data()[2],
                    "i12": deleted_cross_section.extract_numerical_data()[3],
                    "it": deleted_cross_section.extract_numerical_data()[4] },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            self.cross_sections.insert(deleted_cross_section_key.to_owned(),
                deleted_cross_section.to_owned());
            dispatch_custom_event(detail, ADD_BEAM_SECTION_EVENT_NAME,
                EVENT_TARGET)?;
            log(&format!("Properties: Materials: {:?}, deleted materials: {:?}, \
                cross sections: {:?}, deleted cross sections: {:?}",
                self.materials, self.deleted_materials,
                self.cross_sections, self.deleted_cross_sections));
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Restore beam section action: \
                Beam section with name {} does not exist!", name);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn delete_line_numbers(&mut self, action_id: u32, line_numbers: JsValue)
        -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_cross_sections_by_action_id(action_id);
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
}
