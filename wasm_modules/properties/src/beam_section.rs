use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct BeamSection
{
    area: f64,
    i11: f64,
    i22: f64,
    i12: f64,
    it: f64,
}


impl BeamSection
{
    pub fn create(area: f64, i11: f64, i22: f64, i12: f64, it: f64) -> Self
    {
        BeamSection { area, i11, i22, i12, it }
    }


    pub fn data_same(&self, area: f64, i11: f64, i22: f64, i12: f64, it: f64) -> bool
    {
        self.area == area && self.i11 == i11 && self.i22 == i22 && self.i12 == i12 && self.it == it
    }


    pub fn update(&mut self, area: f64, i11: f64, i22: f64, i12: f64, it: f64)
    {
        self.area = area;
        self.i11 = i11;
        self.i22 = i22;
        self.i12 = i12;
        self.it = it;
    }


    pub fn extract_data(&self) -> (f64, f64, f64, f64, f64)
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


    pub fn extract_name_and_data(&self) -> (&str, f64, f64, f64, f64, f64)
    {
        let (area, i11, i22, i12, it) = self.beam_section.extract_data();
        (&self.name, area, i11, i22, i12, it)
    }
}
