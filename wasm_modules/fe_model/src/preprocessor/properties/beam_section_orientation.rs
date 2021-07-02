use crate::types::{FEUInt, FEFloat};


pub struct BeamSectionOrientation
{
    line_numbers: Vec<FEUInt>,
}


pub struct ChangedBeamSectionOrientation
{
    local_axis_1_direction: [FEFloat; 3],
    beam_section_orientation: BeamSectionOrientation,
}


pub struct DeletedBeamSectionOrientation
{
    local_axis_1_direction: [FEFloat; 3],
    beam_section_orientation: BeamSectionOrientation,
}
