use wasm_bindgen::prelude::*;

use serde::Serialize;

use crate::types::{FEUInt, FEFloat};


#[derive(Debug, Serialize, Clone)]
pub struct BeamSectionOrientation
{
    local_axis_1_direction: [FEFloat; 3],
    line_numbers: Vec<FEUInt>,
}


impl BeamSectionOrientation
{
    pub fn create(local_axis_1_direction: [FEFloat; 3], line_numbers: Vec<FEUInt>) -> Self
    {
        BeamSectionOrientation { local_axis_1_direction, line_numbers }
    }


    pub fn is_local_axis_1_direction_same(&self, local_axis_1_direction: &[FEFloat; 3]) -> bool
    {
        self.local_axis_1_direction == *local_axis_1_direction
    }


    pub fn is_line_numbers_same(&self, line_numbers: &[FEUInt]) -> bool
    {
        self.line_numbers == line_numbers
    }


    pub fn extract_local_axis_1_direction(&self) -> [FEFloat; 3]
    {
        self.local_axis_1_direction
    }


    pub fn extract_line_numbers(&self) -> &[FEUInt]
    {
        self.line_numbers.as_slice()
    }


    pub fn extract_direction_and_line_numbers(&self) -> ([FEFloat; 3], &[FEUInt])
    {
        (self.local_axis_1_direction, self.line_numbers.as_slice())
    }


    pub fn update(&mut self, line_numbers: &[FEUInt])
    {
        self.line_numbers = line_numbers.to_vec();
    }


    pub fn exclude_line_number(&mut self, line_number: FEUInt)
    {
        if let Some(position) = self.line_numbers
            .iter()
            .position(|number| *number == line_number)
        {
            let _ = self.line_numbers.remove(position);
        }
    }
}


#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct LocalAxis1Direction(FEFloat, FEFloat, FEFloat);


impl LocalAxis1Direction
{
    pub fn create(local_axis_1_direction: &[FEFloat]) -> Result<Self, JsValue>
    {
        if local_axis_1_direction.len() != 3
        {
            let error_message = &format!("Properties: Create beam section local axis 1 \
                direction: Incorrect number of components in {:?}!", local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }

        Ok(LocalAxis1Direction(
            local_axis_1_direction[0],
            local_axis_1_direction[1],
            local_axis_1_direction[2])
        )
    }
}
