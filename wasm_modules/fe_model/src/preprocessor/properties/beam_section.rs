use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct BeamSection<V>
{
    area: V,
    i11: V,
    i22: V,
    i12: V,
    it: V,
}


impl<V> BeamSection<V>
    where V: Copy + PartialEq,
{
    pub fn create(area: V, i11: V, i22: V, i12: V, it: V) -> Self
    {
        BeamSection { area, i11, i22, i12, it }
    }


    pub fn data_same(&self, area: V, i11: V, i22: V, i12: V, it: V)
        -> bool
    {
        self.area == area && self.i11 == i11 && self.i22 == i22 && self.i12 == i12 && self.it == it
    }


    pub fn update(&mut self, area: V, i11: V, i22: V, i12: V, it: V)
    {
        self.area = area;
        self.i11 = i11;
        self.i22 = i22;
        self.i12 = i12;
        self.it = it;
    }


    pub fn extract_data(&self) -> (V, V, V, V, V)
    {
        (self.area, self.i11, self.i22, self.i12, self.it)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedBeamSection<V>
{
    name: String,
    beam_section: BeamSection<V>,
}


impl<V> DeletedBeamSection<V>
    where V: Copy + PartialEq,
{
    pub fn create(name: &str, beam_section: BeamSection<V>) -> Self
    {
        DeletedBeamSection { name: String::from(name), beam_section }
    }


    pub fn extract_name_and_data(&self) -> (&str, V, V, V, V, V)
    {
        let (area, i11, i22, i12, it) = self.beam_section.extract_data();
        (&self.name, area, i11, i22, i12, it)
    }
}
