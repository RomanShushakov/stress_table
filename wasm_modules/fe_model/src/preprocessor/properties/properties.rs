use wasm_bindgen::prelude::*;
use serde_json::json;
use std::collections::HashMap;

use crate::preprocessor::properties::material::{Material, DeletedMaterial};
use crate::preprocessor::properties::truss_section::{TrussSection, DeletedTrussSection};
use crate::preprocessor::properties::beam_section::{BeamSection, DeletedBeamSection};
use crate::preprocessor::properties::property::{Property, DeletedProperty};
use crate::preprocessor::properties::assigned_property::
{
    AssignedProperty, ChangedAssignedProperty, DeletedAssignedProperty,
};
use crate::preprocessor::properties::beam_section_orientation::
{
    BeamSectionOrientation,
};

use crate::types::{FEUInt, FEFloat};

use crate::functions::log;


pub struct Properties
{
    pub materials: HashMap<String, Material>,   // { material_name: Material }
    pub deleted_materials: HashMap<FEUInt, DeletedMaterial>,   // { action_id: DeletedMaterial }
    pub truss_sections: HashMap<String, TrussSection>,  // { truss_section_name: TrussSection }
    pub deleted_truss_sections: HashMap<FEUInt, DeletedTrussSection>,  // { action_id: DeletedTrussSection }
    pub beam_sections: HashMap<String, BeamSection>,    // { beam_section_name: BeamSection }
    pub deleted_beam_sections: HashMap<FEUInt, DeletedBeamSection>,  // { action_id: DeletedBeamSection }
    pub properties: HashMap<String, Property>,  // { property_name: Property }
    pub deleted_properties: HashMap<FEUInt, Vec<DeletedProperty>>,  // { action_id: Vec<DeletedProperty> }
    pub assigned_properties: HashMap<String, AssignedProperty>, // { property_name: AssignedProperties }
    pub changed_assigned_properties: HashMap<FEUInt, Vec<ChangedAssignedProperty>>,   // { action_id: Vec<ChangedAssignedProperty> }
    pub deleted_assigned_properties: HashMap<FEUInt, Vec<DeletedAssignedProperty>>,   // { action_id: Vec<DeletedAssignedProperty> }

    pub beam_sections_orientations: Vec<BeamSectionOrientation>,
    pub changed_beam_sections_orientations: HashMap<FEUInt, BeamSectionOrientation>,    // { action_id: BeamSectionOrientation }
    pub deleted_beam_sections_orientations: HashMap<FEUInt, Vec<BeamSectionOrientation>>,   // { action_id: BeamSectionOrientation }
}


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
        let deleted_assigned_properties = HashMap::new();
        let beam_sections_orientations = Vec::new();
        let changed_beam_sections_orientations = HashMap::new();
        let deleted_beam_sections_orientations = HashMap::new();
        Properties {
            materials, deleted_materials,
            truss_sections, deleted_truss_sections,
            beam_sections, deleted_beam_sections,
            properties, deleted_properties,
            assigned_properties, changed_assigned_properties,
            deleted_assigned_properties, beam_sections_orientations,
            changed_beam_sections_orientations, deleted_beam_sections_orientations,
        }
    }


    pub fn clear_deleted_materials_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.deleted_materials.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.deleted_materials.remove(&action_id);
        }
    }


    pub fn clear_deleted_truss_sections_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.deleted_truss_sections.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.deleted_truss_sections.remove(&action_id);
        }
    }


    pub fn clear_deleted_beam_sections_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.deleted_beam_sections.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.deleted_beam_sections.remove(&action_id);
        }
    }


    pub fn clear_deleted_properties_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.deleted_properties.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.deleted_properties.remove(&action_id);
        }
    }


    pub fn clear_deleted_assigned_properties_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.deleted_assigned_properties.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.deleted_assigned_properties.remove(&action_id);
        }
    }


    pub fn clear_changed_assigned_properties_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.changed_assigned_properties.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.changed_assigned_properties.remove(&action_id);
        }
    }


    pub fn clear_deleted_beam_sections_orientations_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.deleted_beam_sections_orientations.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.deleted_beam_sections_orientations.remove(&action_id);
        }
    }


    pub fn clear_changed_beam_sections_orientations_by_action_id(&mut self, action_id: FEUInt)
    {
        for action_id in self.changed_beam_sections_orientations.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&FEUInt>>()
            .iter()
        {
            let _ = self.changed_beam_sections_orientations.remove(&action_id);
        }
    }


    pub fn clear_properties_module_by_action_id(&mut self, action_id: FEUInt)
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);
        self.clear_deleted_beam_sections_orientations_by_action_id(action_id);
        self.clear_changed_beam_sections_orientations_by_action_id(action_id);
    }


    pub fn extract_assigned_property_names_for_delete_by_property_names(&self,
        property_names_for_delete: &Vec<String>) -> Vec<String>
    {
        let mut assigned_property_names_for_delete = Vec::new();
        for property_name in property_names_for_delete
        {
            if self.assigned_properties
                .keys()
                .position(|assigned_property_name| assigned_property_name == property_name)
                .is_some()
            {
                assigned_property_names_for_delete.push(property_name.clone())
            }
        }
        assigned_property_names_for_delete
    }


    pub fn extract_line_info_from_properties(&mut self, number: FEUInt)
        -> Option<(String, String, String)>
    {
        for (assigned_property_name, assigned_property) in
            self.assigned_properties.iter()
        {
            if assigned_property.extract_data().iter()
                .position(|line_number| *line_number == number).is_some()
            {
                let property = self.properties.get(assigned_property_name).unwrap();
                let (material_name, cross_section_name, cross_section_type) =
                    property.extract_data();
                return Some((material_name.to_owned(), cross_section_name.to_owned(),
                    cross_section_type.as_str().to_lowercase().to_owned()));
            }
        }
        None
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


    pub fn extract_truss_sections(&self, handler: js_sys::Function) -> Result<(), JsValue>
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


    pub fn extract_beam_sections(&self, handler: js_sys::Function) -> Result<(), JsValue>
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


    pub fn extract_properties(&self, handler: js_sys::Function) -> Result<(), JsValue>
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


    pub fn extract_assigned_properties(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_assigned_properties = json!(
            { "extracted_assigned_properties": self.assigned_properties });
        let composed_extracted_assigned_properties =
            JsValue::from_serde(&extracted_assigned_properties)
                .or(Err(JsValue::from("Properties: Extract assigned properties: \
                    Assigned properties could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_assigned_properties);
        Ok(())
    }


    pub fn extract_beam_sections_orientations(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        let extracted_beam_sections_orientations = json!(
            { "extracted_beam_sections_orientations": self.beam_sections_orientations });
        let composed_extracted_beam_sections_orientations =
            JsValue::from_serde(&extracted_beam_sections_orientations)
                .or(Err(JsValue::from("Properties: Extract beam sections orientations: \
                    Beam sections orientations could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_beam_sections_orientations);
        Ok(())
    }


    pub fn logging(&self)
    {
        log(&format!("Properties: \n
            materials: {:?}, deleted materials: {:?}, \n
            truss sections: {:?}, deleted truss sections: {:?}, \n
            beam sections: {:?}, deleted beam sections: {:?}, \n
            properties: {:?}, deleted properties: {:?}, \n
            assigned_properties: {:?}, changed_assigned_properties: {:?}, \n
            deleted_assigned_properties: {:?}, beam_sections_orientations: {:?}, \n
            changed_beam_sections_orientations: {:?}, deleted_beam_sections_orientations: {:?} \n",
            self.materials, self.deleted_materials,
            self.truss_sections, self.deleted_truss_sections,
            self.beam_sections, self.deleted_beam_sections,
            self.properties, self.deleted_properties,
            self.assigned_properties, self.changed_assigned_properties,
            self.deleted_assigned_properties, self.beam_sections_orientations,
            self.changed_beam_sections_orientations, self.deleted_beam_sections_orientations)
        );
    }
}
