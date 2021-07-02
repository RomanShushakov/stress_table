use crate::preprocessor::geometry::geometry::Geometry;
use crate::preprocessor::properties::properties::Properties;


pub struct Preprocessor
{
    pub geometry: Geometry,
    pub properties: Properties,
}


impl Preprocessor
{
    pub fn create() -> Self
    {
        let geometry = Geometry::create();
        let properties = Properties::create();
        Preprocessor { geometry, properties }
    }
}
