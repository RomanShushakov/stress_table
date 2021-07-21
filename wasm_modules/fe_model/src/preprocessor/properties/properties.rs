use wasm_bindgen::prelude::*;
use serde_json::json;
use std::collections::HashMap;

use crate::preprocessor::geometry::geometry::Geometry;

use crate::preprocessor::properties::material::{Material, DeletedMaterial};
use crate::preprocessor::properties::property::CrossSectionType;
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
    pub changed_beam_sections_orientations: HashMap<FEUInt, Vec<BeamSectionOrientation>>,    // { action_id: Vec<BeamSectionOrientation> }
    pub deleted_beam_sections_orientations: HashMap<FEUInt, Vec<BeamSectionOrientation>>,   // { action_id: Vec<BeamSectionOrientation> }
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
        property_names_for_delete: &[String]) -> Vec<String>
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


    pub fn extract_beam_section_orientations_for_change_by_assigned_property_names(
        &mut self, assigned_property_names_for_delete: &[String]) -> Vec<BeamSectionOrientation>
    {
        let mut beam_section_orientations_for_change: Vec<BeamSectionOrientation> = Vec::new();
        for assigned_property_name in assigned_property_names_for_delete
        {
            let (_, _, cross_section_type) = self.properties
                .get(assigned_property_name)
                .unwrap()
                .extract_data();
            {
                match cross_section_type
                {
                    CrossSectionType::Beam =>
                        {
                            for line_number in self.assigned_properties
                                .get(assigned_property_name)
                                .unwrap()
                                .extract_data()
                            {
                                if let Some(position) = self.beam_sections_orientations
                                    .iter()
                                    .position(|beam_section_orientation|
                                        {
                                            beam_section_orientation
                                                .extract_line_numbers()
                                                .contains(line_number)
                                        })
                                {
                                    let old_beam_section_orientation =
                                        self.beam_sections_orientations[position].clone();
                                    let local_axis_1_direction =
                                        old_beam_section_orientation.extract_local_axis_1_direction();
                                    self.beam_sections_orientations[position]
                                        .exclude_line_number(*line_number);
                                    if beam_section_orientations_for_change
                                        .iter()
                                        .position(|beam_section_orientation|
                                            beam_section_orientation
                                                .is_local_axis_1_direction_same(
                                                    &local_axis_1_direction))
                                        .is_none()
                                    {
                                        beam_section_orientations_for_change.push(
                                            old_beam_section_orientation);
                                    }
                                }
                            }
                        },
                    CrossSectionType::Truss => ()
                }
            }
        }
        beam_section_orientations_for_change
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
