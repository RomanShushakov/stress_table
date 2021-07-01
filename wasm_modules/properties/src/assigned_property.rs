use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
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
            line_numbers.contains(existed_line_number)).is_some()
        {
            return true;
        }
        false
    }


    pub fn extract_data(&self) -> &[u32]
    {
        self.line_numbers.as_slice()
    }
}


#[derive(Debug, Clone)]
pub struct ChangedAssignedProperty
{
    name: String,
    assigned_property: AssignedProperty,
}


impl ChangedAssignedProperty
{
    pub fn create(name: &str, assigned_property: AssignedProperty) -> Self
    {
        ChangedAssignedProperty { name: String::from(name), assigned_property }
    }


    pub fn extract_name_and_data(&self) -> (&str, &[u32])
    {
        let line_numbers = self.assigned_property.extract_data();
        (&self.name, line_numbers)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedAssignedProperty
{
    name: String,
    assigned_property: AssignedProperty,
}


impl DeletedAssignedProperty
{
    pub fn create(name: &str, assigned_property: AssignedProperty) -> Self
    {
        DeletedAssignedProperty { name: String::from(name), assigned_property }
    }


    pub fn extract_name_and_data(&self) -> (&str, &[u32])
    {
        let line_numbers = self.assigned_property.extract_data();
        (&self.name, line_numbers)
    }
}
