use crate::types::{FEUInt, FEFloat};


#[derive(Debug)]
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
}

