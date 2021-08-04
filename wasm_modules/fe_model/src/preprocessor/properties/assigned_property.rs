use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::preprocessor::properties::beam_section_orientation::LocalAxis1Direction;

use crate::preprocessor::properties::functions::line_numbers_same;


#[derive(Debug, Clone, Serialize)]
pub struct AssignedPropertyToLines<T, V>
{
    related_lines_data: HashMap<T, Option<LocalAxis1Direction<V>>>,
    related_line_elements_numbers: HashSet<T>,
}


impl<T, V> AssignedPropertyToLines<T, V>
    where T: Copy + Hash + Eq,
          V: Copy,
{
    pub fn create_initial(line_numbers: &[T]) -> Self
    {
        let mut related_lines_data = HashMap::new();
        let related_line_elements_numbers = HashSet::new();
        for line_number in line_numbers
        {
            related_lines_data.insert(*line_number, None);
        }
        AssignedPropertyToLines { related_lines_data, related_line_elements_numbers }
    }


    pub fn line_numbers_same(&self, line_numbers: &[T]) -> bool
    {
        let related_lines_numbers = self.extract_related_lines_numbers();
        line_numbers_same(&related_lines_numbers, line_numbers)
    }


    pub fn check_for_line_numbers_intersection(&self, line_numbers: &[T]) -> bool
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


    pub fn extract_related_lines_numbers(&self) -> Vec<T>
    {
        let mut related_lines_numbers = Vec::new();
        for line_number in self.related_lines_data.keys()
        {
            related_lines_numbers.push(*line_number);
        }
        related_lines_numbers
    }


    pub fn extract_related_lines_data(&self) -> HashMap<T, Option<LocalAxis1Direction<V>>>
    {
        self.related_lines_data.clone()
    }


    pub fn fit_related_lines_data_by_line_numbers(&mut self, line_numbers: &[T])
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


    pub fn update_related_lines_data(&mut self, line_number: T,
        local_axis_1_direction: Option<LocalAxis1Direction<V>>)
    {
        self.related_lines_data.insert(line_number, local_axis_1_direction);
    }


    pub fn length_of_related_lines_data(&self) -> usize
    {
        self.related_lines_data.len()
    }


    pub fn remove_line_number_from_related_lines_data(&mut self, line_number: &T)
        -> Option<Option<LocalAxis1Direction<V>>>
    {
        self.related_lines_data.remove(line_number)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedAssignedPropertyToLines<T, V>
{
    name: String,
    assigned_property_to_lines: AssignedPropertyToLines<T, V>,
}


impl<T, V> DeletedAssignedPropertyToLines<T, V>
    where T: Copy + Hash + Eq,
          V: Copy,
{
    pub fn create(name: &str, assigned_property_to_lines: AssignedPropertyToLines<T, V>) -> Self
    {
        DeletedAssignedPropertyToLines { name: String::from(name), assigned_property_to_lines }
    }


    pub fn extract_name(&self) -> &str
    {
        &self.name
    }


    pub fn extract_name_and_related_lines_numbers(&self) -> (&str, Vec<T>)
    {
        let line_numbers = self.assigned_property_to_lines.extract_related_lines_numbers();
        (&self.name, line_numbers)
    }


    pub fn extract_and_drop(self) -> (String, AssignedPropertyToLines<T, V>)
    {
        (self.name, self.assigned_property_to_lines)
    }
}


#[derive(Debug, Clone)]
pub struct ChangedAssignedPropertyToLines<T, V>
{
    name: String,
    assigned_property_to_lines: AssignedPropertyToLines<T, V>,
}


impl<T, V> ChangedAssignedPropertyToLines<T, V>
    where T: Copy + Hash + Eq,
          V: Copy,
{
    pub fn create(name: &str, assigned_property_to_lines: AssignedPropertyToLines<T, V>) -> Self
    {
        ChangedAssignedPropertyToLines { name: String::from(name), assigned_property_to_lines }
    }


    pub fn extract_name_and_related_lines_numbers(&self) -> (&str, Vec<T>)
    {
        let line_numbers = self.assigned_property_to_lines.extract_related_lines_numbers();
        (&self.name, line_numbers)
    }


    pub fn extract_and_drop(self) -> (String, AssignedPropertyToLines<T, V>)
    {
        (self.name, self.assigned_property_to_lines)
    }


    pub fn name_same(&self, name: &str) -> bool
    {
        self.name == name
    }
}
