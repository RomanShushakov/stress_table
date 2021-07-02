use crate::preprocessor::geometry::geometry::Geometry;


pub struct Preprocessor
{
    pub geometry: Geometry,
}


impl Preprocessor
{
    pub fn create() -> Self
    {
        let geometry = Geometry::create();
        Preprocessor { geometry }
    }
}
