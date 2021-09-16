use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::traits::ClearByActionIdTrait;

use crate::preprocessor::properties::material::{Material, DeletedMaterial};
use crate::preprocessor::properties::truss_section::{TrussSection, DeletedTrussSection};
use crate::preprocessor::properties::beam_section::{BeamSection, DeletedBeamSection};
use crate::preprocessor::properties::property::{Property, DeletedProperty};
use crate::preprocessor::properties::property::CrossSectionType;
use crate::preprocessor::properties::assigned_property::
{
    AssignedPropertyToLines, DeletedAssignedPropertyToLines, ChangedAssignedPropertyToLines,
};
use crate::preprocessor::properties::beam_section_orientation::{LocalAxis1Direction};

use crate::functions::log;


pub struct Properties<T, V>
{
    pub materials: HashMap<String, Material<V>>,                                                        // { material_name: Material }
    pub deleted_materials: HashMap<T, DeletedMaterial<V>>,                                              // { action_id: DeletedMaterial }
    pub truss_sections: HashMap<String, TrussSection<V>>,                                               // { truss_section_name: TrussSection }
    pub deleted_truss_sections: HashMap<T, DeletedTrussSection<V>>,                                     // { action_id: DeletedTrussSection }
    pub beam_sections: HashMap<String, BeamSection<V>>,                                                 // { beam_section_name: BeamSection }
    pub deleted_beam_sections: HashMap<T, DeletedBeamSection<V>>,                                       // { action_id: DeletedBeamSection }
    pub properties: HashMap<String, Property>,                                                          // { property_name: Property }
    pub deleted_properties: HashMap<T, Vec<DeletedProperty>>,                                           // { action_id: Vec<DeletedProperty> }
    pub assigned_properties_to_lines: HashMap<String, AssignedPropertyToLines<T, V>>,                   // { property_name: AssignedProperties }
    pub deleted_assigned_properties_to_lines: HashMap<T, Vec<DeletedAssignedPropertyToLines<T, V>>>,    // { action_id: Vec<DeletedAssignedPropertyToLines> }
    pub changed_assigned_properties_to_lines: HashMap<T, Vec<ChangedAssignedPropertyToLines<T, V>>>,    // { action_id: Vec<ChangedAssignedPropertyToLines> }
    pub beam_sections_local_axis_1_directions: Vec<LocalAxis1Direction<V>>,
    pub deleted_beam_sections_local_axis_1_directions: HashMap<T, LocalAxis1Direction<V>>,              // { action_id: LocalAxis1Direction }
}


impl<T, V> Properties<T, V>
    where T: Copy + Debug + Eq + Hash + PartialOrd,
          V: Copy + Debug,
{
    pub fn create() -> Properties<T, V>
    {
        let materials = HashMap::new();
        let deleted_materials = HashMap::new();
        let truss_sections = HashMap::new();
        let deleted_truss_sections = HashMap::new();
        let beam_sections = HashMap::new();
        let deleted_beam_sections = HashMap::new();
        let properties = HashMap::new();
        let deleted_properties = HashMap::new();
        let assigned_properties_to_lines = HashMap::new();
        let deleted_assigned_properties_to_lines = HashMap::new();
        let changed_assigned_properties_to_lines = HashMap::new();
        let beam_sections_local_axis_1_directions = Vec::new();
        let deleted_beam_sections_local_axis_1_directions = HashMap::new();
        Properties {
            materials, deleted_materials,
            truss_sections, deleted_truss_sections,
            beam_sections, deleted_beam_sections,
            properties, deleted_properties,
            assigned_properties_to_lines, deleted_assigned_properties_to_lines,
            changed_assigned_properties_to_lines,
            beam_sections_local_axis_1_directions,
            deleted_beam_sections_local_axis_1_directions,
        }
    }


    pub fn clear_deleted_materials_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_materials.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_materials.remove(&action_id);
        }
    }


    pub fn clear_deleted_truss_sections_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_truss_sections.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_truss_sections.remove(&action_id);
        }
    }


    pub fn clear_deleted_beam_sections_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_beam_sections.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_beam_sections.remove(&action_id);
        }
    }


    pub fn clear_deleted_properties_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_properties.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_properties.remove(&action_id);
        }
    }


    pub fn clear_deleted_assigned_properties_to_lines_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_assigned_properties_to_lines.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_assigned_properties_to_lines.remove(&action_id);
        }
    }


    pub fn clear_changed_assigned_properties_to_lines_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.changed_assigned_properties_to_lines.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.changed_assigned_properties_to_lines.remove(&action_id);
        }
    }


    pub fn clear_deleted_beam_local_axis_1_direction_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_beam_sections_local_axis_1_directions.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_beam_sections_local_axis_1_directions.remove(&action_id);
        }
    }


    pub fn extract_line_info_from_properties(&mut self, number: T) -> Option<(String, String, String)>
    {
        for (assigned_property_to_lines_name, assigned_property_to_lines) in
            self.assigned_properties_to_lines.iter()
        {
            if assigned_property_to_lines.copy_related_lines_numbers().iter()
                .position(|line_number| *line_number == number).is_some()
            {
                let property = self.properties.get(assigned_property_to_lines_name).unwrap();
                let (material_name, cross_section_name, cross_section_type) =
                    property.clone_data();
                return Some((material_name.to_owned(), cross_section_name.to_owned(),
                    cross_section_type.as_str().to_lowercase().to_owned()));
            }
        }
        None
    }


    pub fn check_for_property_existence_by_name(&self, property_name: &str,
        error_message_header: &str) -> Result<(), JsValue>
    {
        if !self.properties.contains_key(property_name)
        {
            let error_message = &format!("{}: Property with name {} does not exist!",
                error_message_header, property_name);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn check_for_property_with_similar_data_existence(&self, material_name: &str,
        cross_section_name: &str, cross_section_type: &CrossSectionType, error_message_header: &str)
        -> Result<(), JsValue>
    {
        if self.properties.values().position(|property|
            property.is_data_same(material_name, cross_section_name, cross_section_type))
                .is_some()
        {
            let error_message = &format!("{}: Property with Material name {}, \
                Cross section name {}, Cross section type {} does already exist!",
                error_message_header, material_name, cross_section_name, cross_section_type.as_str());
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }



    pub fn check_for_the_similar_line_numbers_in_assigned_properties_to_lines_existence(&self,
        line_numbers: &[T], error_message_header: &str) -> Result<(), JsValue>
    {
        if self.assigned_properties_to_lines.values()
            .position(|existed_assigned_property_to_lines|
                existed_assigned_property_to_lines.are_line_numbers_same(line_numbers))
            .is_some()
        {
            let error_message = &format!("{}: Assigned property to lines with line \
                numbers {:?} does already exist!", error_message_header, line_numbers);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn check_for_line_numbers_intersection_in_assigned_properties_to_lines(&self,
        assigned_property_to_lines_name: &str, line_numbers: &[T], error_message_header: &str)
        -> Result<(), JsValue>
    {
        if self.assigned_properties_to_lines.iter()
            .position(|(existed_assigned_property_to_lines_name,
                    existed_assigned_property_to_lines)|
                existed_assigned_property_to_lines_name != &assigned_property_to_lines_name &&
                existed_assigned_property_to_lines.check_for_line_numbers_intersection(
                    line_numbers))
            .is_some()
        {
            let error_message = &format!("{:?}: At least one line number from {:?} is \
                already used in another assigned property to lines!", error_message_header,
                line_numbers);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn check_for_material_existence_by_name(&self, material_name: &str,
        error_message_header: &str) -> Result<(), JsValue>
    {
        if !self.materials.contains_key(material_name)
        {
            let error_message = &format!("{}: Material with name {} does not exist!",
                error_message_header, material_name);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn check_for_cross_section_existence(&self, cross_section_name: &str,
        cross_section_type: &CrossSectionType, error_message_header: &str) -> Result<(), JsValue>
    {
        match cross_section_type
        {
            CrossSectionType::Truss =>
                {
                    if !self.truss_sections.contains_key(cross_section_name)
                    {
                        let error_message = &format!("{}: Truss section with name {} \
                            does not exist!", error_message_header, cross_section_name);
                        return Err(JsValue::from(error_message));
                    }
                },
            CrossSectionType::Beam =>
                {
                    if !self.beam_sections.contains_key(cross_section_name)
                    {
                        let error_message = &format!("{}: Beam section with name {} \
                            does not exist!", error_message_header, cross_section_name);
                        return Err(JsValue::from(error_message));
                    }
                },
        }
        Ok(())
    }


    pub fn check_the_correspondence_of_cross_section_type_to_beam(&self, line_number: &T,
        error_message_header: &str) -> Result<(), JsValue>
    {
        for (assigned_property_to_lines_name, assigned_property_to_lines) in
            self.assigned_properties_to_lines.iter()
        {
            if assigned_property_to_lines.copy_related_lines_numbers()
                .contains(line_number)
            {
                let (_, _, cross_section_type) = self.properties
                    .get(assigned_property_to_lines_name)
                    .unwrap()
                    .clone_data();
                return match cross_section_type
                {
                    CrossSectionType::Truss =>
                        {
                            let error_message = &format!("{}: Beam section orientation \
                                should be applied to 'Beam' cross section type only!",
                                error_message_header);
                            Err(JsValue::from(error_message))
                        },
                    CrossSectionType::Beam => Ok(()),
                }
            }
        }
        let error_message = &format!("{}: There are no assigned property which \
            contains line number {:?}!", error_message_header, line_number);
        Err(JsValue::from(error_message))
    }


    pub fn logging(&self)
    {
        log(&format!("Properties: \n
            materials: {:?}, \n
            deleted materials: {:?}, \n
            truss sections: {:?}, \n
            deleted truss sections: {:?}, \n
            beam sections: {:?}, \n
            deleted beam sections: {:?}, \n
            properties: {:?}, \n
            deleted properties: {:?}, \n
            assigned_properties_to_lines: {:?}, \n
            deleted_assigned_properties_to_lines: {:?}, \n
            changed_assigned_properties_to_lines: {:?}, \n
            beam_sections_local_axis_1_directions: {:?}, \n
            deleted_beam_sections_local_axis_1_directions: {:?} \n",
            self.materials,
            self.deleted_materials,
            self.truss_sections,
            self.deleted_truss_sections,
            self.beam_sections,
            self.deleted_beam_sections,
            self.properties,
            self.deleted_properties,
            self.assigned_properties_to_lines,
            self.deleted_assigned_properties_to_lines,
            self.changed_assigned_properties_to_lines,
            self.beam_sections_local_axis_1_directions,
            self.deleted_beam_sections_local_axis_1_directions)
        );
    }
}


impl<T, V> ClearByActionIdTrait<T> for Properties<T, V>
    where T: Copy + Debug + Eq + Hash + PartialOrd,
          V: Copy + Debug,
{
    fn clear_by_action_id(&mut self, action_id: T)
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_to_lines_by_action_id(action_id);
        self.clear_changed_assigned_properties_to_lines_by_action_id(action_id);
        self.clear_deleted_beam_local_axis_1_direction_by_action_id(action_id);
    }
}
