use std::collections::HashMap;

use crate::point_object::PointObjectKey;
use crate::point_object::PointObject;
use crate::line_object::LineObjectKey;
use crate::line_object::{LineObject, BeamSectionOrientation};
use crate::concentrated_load::ConcentratedLoad;
use crate::distributed_line_load::DistributedLineLoad;
use crate::boundary_condition::BoundaryCondition;


pub struct Preprocessor
{
    point_objects: HashMap<PointObjectKey, PointObject>,
    line_objects: HashMap<LineObjectKey, LineObject>,
    beam_section_orientation_for_preview: Option<BeamSectionOrientation>,
    concentrated_loads: HashMap<u32, ConcentratedLoad>,
    distributed_line_loads: HashMap<u32, DistributedLineLoad>,
    boundary_conditions: HashMap<u32, BoundaryCondition>,
}


impl Preprocessor
{
    fn create() -> Self
    {
        Preprocessor
            {
                point_objects: HashMap::new(),
                line_objects: HashMap::new(),
                beam_section_orientation_for_preview: None,
                concentrated_loads: HashMap::new(),
                distributed_line_loads: HashMap::new(),
                boundary_conditions: HashMap::new(),
            }
    }
}


pub enum Scene
{
    Preprocessor(Preprocessor)
}
