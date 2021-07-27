use serde::Serialize;
use std::collections::{HashMap, HashSet};

use crate::preprocessor::properties::beam_section_orientation::LocalAxis1Direction;

use crate::preprocessor::properties::functions::line_numbers_same;

use crate::types::FEUInt;


// #[derive(Debug, Clone, Serialize)]
// pub struct AssignedProperty
// {
//     line_numbers: Vec<FEUInt>,
// }
//
//
// impl AssignedProperty
// {
//     pub fn create(line_numbers: &[FEUInt]) -> Self
//     {
//         AssignedProperty { line_numbers: line_numbers.to_vec() }
//     }
//
//
//     pub fn data_same(&self, line_numbers: &[FEUInt]) -> bool
//     {
//         self.line_numbers == line_numbers
//     }
//
//
//     pub fn update(&mut self, line_numbers: &[FEUInt])
//     {
//         self.line_numbers = line_numbers.to_vec()
//     }
//
//
//     pub fn is_contain_any_provided_line_number(&self, line_numbers: &[FEUInt]) -> bool
//     {
//         if self.line_numbers.iter().position(|existed_line_number|
//             line_numbers.contains(existed_line_number)).is_some()
//         {
//             return true;
//         }
//         false
//     }
//
//
//     pub fn extract_data(&self) -> &[FEUInt]
//     {
//         self.line_numbers.as_slice()
//     }
// }
//
//
// #[derive(Debug, Clone)]
// pub struct ChangedAssignedProperty
// {
//     name: String,
//     assigned_property: AssignedProperty,
// }
//
//
// impl ChangedAssignedProperty
// {
//     pub fn create(name: &str, assigned_property: AssignedProperty) -> Self
//     {
//         ChangedAssignedProperty { name: String::from(name), assigned_property }
//     }
//
//
//     pub fn extract_name_and_data(&self) -> (&str, &[FEUInt])
//     {
//         let line_numbers = self.assigned_property.extract_data();
//         (&self.name, line_numbers)
//     }
// }
//
//
// #[derive(Debug, Clone)]
// pub struct DeletedAssignedProperty
// {
//     name: String,
//     assigned_property: AssignedProperty,
// }
//
//
// impl DeletedAssignedProperty
// {
//     pub fn create(name: &str, assigned_property: AssignedProperty) -> Self
//     {
//         DeletedAssignedProperty { name: String::from(name), assigned_property }
//     }
//
//
//     pub fn extract_name_and_data(&self) -> (&str, &[FEUInt])
//     {
//         let line_numbers = self.assigned_property.extract_data();
//         (&self.name, line_numbers)
//     }
// }


#[derive(Debug, Clone, Serialize)]
pub struct AssignedPropertyToLines
{
    related_lines_data: HashMap<FEUInt, Option<LocalAxis1Direction>>,
    related_line_elements_numbers: HashSet<FEUInt>,
}


impl AssignedPropertyToLines
{
    pub fn create_initial(line_numbers: &[FEUInt]) -> Self
    {
        let mut related_lines_data = HashMap::new();
        let related_line_elements_numbers = HashSet::new();
        for line_number in line_numbers
        {
            related_lines_data.insert(*line_number, Some(LocalAxis1Direction::create(&[0.0, 0.0, 1.0]).unwrap()));
        }
        AssignedPropertyToLines { related_lines_data, related_line_elements_numbers }
    }


    pub fn line_numbers_same(&self, line_numbers: &[FEUInt]) -> bool
    {
        let related_lines_numbers = self.extract_related_lines_numbers();
        line_numbers_same(&related_lines_numbers, line_numbers)
    }


    pub fn check_for_line_numbers_intersection(&self, line_numbers: &[FEUInt]) -> bool
    {
        for line_number in line_numbers
        {
            if self.related_lines_data.contains_key(line_number)
            {
                return true;
            }
        }
        false
    }


    pub fn extract_related_lines_numbers(&self) -> Vec<FEUInt>
    {
        let mut related_lines_numbers = Vec::new();
        for line_number in self.related_lines_data.keys()
        {
            related_lines_numbers.push(*line_number);
        }
        related_lines_numbers
    }


    pub fn extract_related_lines_data(&self) -> HashMap<FEUInt, Option<LocalAxis1Direction>>
    {
        self.related_lines_data.clone()
    }


    pub fn fit_related_lines_data_by_line_numbers(&mut self, line_numbers: &[FEUInt])
    {
        let related_lines_numbers = self.extract_related_lines_numbers();
        for line_number in related_lines_numbers.iter()
        {
            if !line_numbers.contains(line_number)
            {
                let _ = self.related_lines_data.remove(line_number);
            }
        }
        for line_number in line_numbers
        {
            if !self.related_lines_data.contains_key(line_number)
            {
                self.related_lines_data.insert(*line_number, None);
            }
        }
    }


    pub fn update_related_lines_data(&mut self, line_number: FEUInt,
        local_axis_1_direction: Option<LocalAxis1Direction>)
    {
        self.related_lines_data.insert(line_number, local_axis_1_direction);
    }


    pub fn length_of_related_lines_data(&self) -> usize
    {
        self.related_lines_data.len()
    }


    pub fn remove_line_number_from_related_lines_data(&mut self, line_number: &FEUInt)
        -> Option<Option<LocalAxis1Direction>>
    {
        self.related_lines_data.remove(line_number)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedAssignedPropertyToLines
{
    name: String,
    assigned_property_to_lines: AssignedPropertyToLines,
}


impl DeletedAssignedPropertyToLines
{
    pub fn create(name: &str, assigned_property_to_lines: AssignedPropertyToLines) -> Self
    {
        DeletedAssignedPropertyToLines { name: String::from(name), assigned_property_to_lines }
    }


    pub fn extract_name(&self) -> &str
    {
        &self.name
    }


    pub fn extract_name_and_related_lines_numbers(&self) -> (&str, Vec<FEUInt>)
    {
        let line_numbers = self.assigned_property_to_lines.extract_related_lines_numbers();
        (&self.name, line_numbers)
    }


    pub fn extract_and_drop(self) -> (String, AssignedPropertyToLines)
    {
        (self.name, self.assigned_property_to_lines)
    }
}


#[derive(Debug, Clone)]
pub struct ChangedAssignedPropertyToLines
{
    name: String,
    assigned_property_to_lines: AssignedPropertyToLines,
}


impl ChangedAssignedPropertyToLines
{
    pub fn create(name: &str, assigned_property_to_lines: AssignedPropertyToLines) -> Self
    {
        ChangedAssignedPropertyToLines { name: String::from(name), assigned_property_to_lines }
    }


    pub fn extract_name_and_related_lines_numbers(&self) -> (&str, Vec<FEUInt>)
    {
        let line_numbers = self.assigned_property_to_lines.extract_related_lines_numbers();
        (&self.name, line_numbers)
    }


    pub fn extract_and_drop(self) -> (String, AssignedPropertyToLines)
    {
        (self.name, self.assigned_property_to_lines)
    }


    pub fn name_same(&self, name: &str) -> bool
    {
        self.name == name
    }
}
