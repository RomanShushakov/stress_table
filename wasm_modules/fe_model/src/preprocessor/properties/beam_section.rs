use serde::Serialize;

use crate::types::FEFloat;


#[derive(Debug, Clone, Serialize)]
pub struct BeamSection
{
    area: FEFloat,
    i11: FEFloat,
    i22: FEFloat,
    i12: FEFloat,
    it: FEFloat,
}


impl BeamSection
{
    pub fn create(area: FEFloat, i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat) -> Self
    {
        BeamSection { area, i11, i22, i12, it }
    }


    pub fn data_same(&self, area: FEFloat, i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat)
        -> bool
    {
        self.area == area && self.i11 == i11 && self.i22 == i22 && self.i12 == i12 && self.it == it
    }


    pub fn update(&mut self, area: FEFloat, i11: FEFloat, i22: FEFloat, i12: FEFloat, it: FEFloat)
    {
        self.area = area;
        self.i11 = i11;
        self.i22 = i22;
        self.i12 = i12;
        self.it = it;
    }


    pub fn extract_data(&self) -> (FEFloat, FEFloat, FEFloat, FEFloat, FEFloat)
    {
        (self.area, self.i11, self.i22, self.i12, self.it)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedBeamSection
{
    name: String,
    beam_section: BeamSection,
}


impl DeletedBeamSection
{
    pub fn create(name: &str, beam_section: BeamSection) -> Self
    {
        DeletedBeamSection { name: String::from(name), beam_section }
    }


    pub fn extract_name_and_data(&self) -> (&str, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat)
    {
        let (area, i11, i22, i12, it) = self.beam_section.extract_data();
        (&self.name, area, i11, i22, i12, it)
    }
}
