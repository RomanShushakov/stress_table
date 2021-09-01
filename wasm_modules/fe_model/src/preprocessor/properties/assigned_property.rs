use serde::Serialize;
use std::hash::Hash;

use crate::preprocessor::properties::beam_section_orientation::LocalAxis1Direction;

use crate::preprocessor::properties::functions::are_line_numbers_same;


#[derive(Debug, Clone, Serialize)]
pub struct RelatedLineData<T, V>
{
    line_number: T,
    local_axis_1_direction: Option<LocalAxis1Direction<V>>
}


impl<T, V> RelatedLineData<T, V>
    where T: Copy + PartialEq,
          V: Copy,
{
    fn create_initial(line_number: T) -> Self
    {
        RelatedLineData { line_number, local_axis_1_direction: None }
    }


    fn create(line_number: T, local_axis_1_direction: Option<LocalAxis1Direction<V>>) -> Self
    {
        RelatedLineData { line_number, local_axis_1_direction }
    }


    pub fn line_number(&self) -> T
    {
        self.line_number
    }


    pub fn local_axis_1_direction(&self) -> Option<LocalAxis1Direction<V>>
    {
        self.local_axis_1_direction.clone()
    }


    fn is_line_number_same(&self, line_number: T) -> bool
    {
        line_number == self.line_number
    }


    fn update_local_axis_1_direction(&mut self, local_axis_1_direction: Option<LocalAxis1Direction<V>>)
    {
        self.local_axis_1_direction = local_axis_1_direction;
    }


    fn extract_local_axis_1_direction_and_drop(self) -> Option<LocalAxis1Direction<V>>
    {
        self.local_axis_1_direction
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct AssignedPropertyToLines<T, V>
{
    related_lines_data: Vec<RelatedLineData<T, V>>,
    related_nodes_numbers: Vec<T>,
}


impl<T, V> AssignedPropertyToLines<T, V>
    where T: Copy + Hash + Eq,
          V: Copy,
{
    pub fn create_initial(line_numbers: &[T]) -> Self
    {
        let mut related_lines_data = Vec::new();
        let related_nodes_numbers = Vec::new();
        for line_number in line_numbers
        {
            let related_line_data = RelatedLineData::create_initial(*line_number);
            related_lines_data.push(related_line_data);
        }
        AssignedPropertyToLines { related_lines_data, related_nodes_numbers }
    }


    pub fn are_line_numbers_same(&self, line_numbers: &[T]) -> bool
    {
        let related_lines_numbers = self.extract_related_lines_numbers();
        are_line_numbers_same(&related_lines_numbers, line_numbers)
    }


    pub fn check_for_line_numbers_intersection(&self, line_numbers: &[T]) -> bool
    {
        for line_number in line_numbers
        {
            if self.related_lines_data.iter().position(|related_line_data|
                related_line_data.is_line_number_same(*line_number)).is_some()
            {
                return true;
            }
        }
        false
    }


    pub fn extract_related_lines_numbers(&self) -> Vec<T>
    {
        let mut related_lines_numbers = Vec::new();
        for related_line_data in self.related_lines_data.iter()
        {
            let line_number = related_line_data.line_number();
            related_lines_numbers.push(line_number);
        }
        related_lines_numbers
    }


    pub fn extract_related_lines_data(&self) -> Vec<RelatedLineData<T, V>>
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
                while let Some(position) = self.related_lines_data.iter()
                    .position(|related_line_data|
                        related_line_data.is_line_number_same(*line_number))
                {
                    self.related_lines_data.remove(position);
                }
            }
        }
        for line_number in line_numbers
        {
            if self.related_lines_data.iter().position(|related_line_data|
                related_line_data.is_line_number_same(*line_number)).is_none()
            {
                let related_line_data =
                    RelatedLineData::create_initial(*line_number);
                self.related_lines_data.push(related_line_data);
            }
        }
    }


    pub fn update_related_lines_data(&mut self, line_number: T,
        local_axis_1_direction: Option<LocalAxis1Direction<V>>)
    {
        if let Some(position) = self.related_lines_data.iter().position(|related_line_data|
            related_line_data.is_line_number_same(line_number))
        {
            self.related_lines_data[position].update_local_axis_1_direction(local_axis_1_direction);
        }
        else
        {
            let related_line_data = RelatedLineData::create(
                line_number, local_axis_1_direction);
            self.related_lines_data.push(related_line_data);
        }
    }


    pub fn length_of_related_lines_data(&self) -> usize
    {
        self.related_lines_data.len()
    }


    pub fn remove_line_number_from_related_lines_data(&mut self, line_number: &T)
        -> Option<Option<LocalAxis1Direction<V>>>
    {
        if let Some(position) = self.related_lines_data.iter().position(|related_line_data|
            related_line_data.is_line_number_same(*line_number))
        {
            let local_axis_1_direction =
                self.related_lines_data.remove(position).extract_local_axis_1_direction_and_drop();
            Some(local_axis_1_direction)
        }
        else
        {
            None
        }
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


    pub fn is_name_same(&self, name: &str) -> bool
    {
        self.name == name
    }
}
