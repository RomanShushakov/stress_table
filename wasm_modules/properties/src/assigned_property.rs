use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct AssignedProperty
{
    line_numbers: Vec<u32>,
}

impl AssignedProperty
{
    pub fn create(line_numbers: &[u32]) -> Self
    {
        AssignedProperty { line_numbers: line_numbers.to_vec() }
    }


    pub fn data_same(&self, line_numbers: &[u32]) -> bool
    {
        self.line_numbers == line_numbers
    }


    pub fn update(&mut self, line_numbers: &[u32])
    {
        self.line_numbers = line_numbers.to_vec()
    }


    pub fn is_contain_any_provided_line_number(&self, line_numbers: &[u32]) -> bool
    {
        if self.line_numbers.iter().position(|existed_line_number|
            line_numbers.iter().any(|line_number| existed_line_number == line_number)).is_some()
        {
            return true;
        }
        false
    }
}


#[derive(Debug)]
pub struct ChangedAssignedProperty
{
    name: String,
    line_numbers: Vec<u32>,
}


#[derive(Debug)]
pub struct DeletedAssignedProperty
{
    name: String,
    line_numbers: Vec<u32>,
}