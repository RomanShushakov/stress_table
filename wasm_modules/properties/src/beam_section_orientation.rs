pub struct BeamSectionOrientation
{
    line_numbers: Vec<u32>,
}


pub struct ChangedBeamSectionOrientation
{
    local_axis_1_direction: [f64; 3],
    beam_section_orientation: BeamSectionOrientation,
}


pub struct DeletedBeamSectionOrientation
{
    local_axis_1_direction: [f64; 3],
    beam_section_orientation: BeamSectionOrientation,
}
