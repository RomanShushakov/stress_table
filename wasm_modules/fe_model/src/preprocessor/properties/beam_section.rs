use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct BeamSection<V>
{
    area: V,
    i11: V,
    i22: V,
    i12: V,
    it: V,
    shear_factor: V,
}


impl<V> BeamSection<V>
    where V: Copy + PartialEq,
{
    pub fn create(area: V, i11: V, i22: V, i12: V, it: V, shear_factor: V) -> Self
    {
        BeamSection { area, i11, i22, i12, it, shear_factor }
    }


    pub fn data_same(&self, area: V, i11: V, i22: V, i12: V, it: V, shear_factor: V)
        -> bool
    {
        self.area == area && self.i11 == i11 && self.i22 == i22 && self.i12 == i12 &&
        self.it == it && self.shear_factor == shear_factor
    }


    pub fn update(&mut self, area: V, i11: V, i22: V, i12: V, it: V, shear_factor: V)
    {
        self.area = area;
        self.i11 = i11;
        self.i22 = i22;
        self.i12 = i12;
        self.it = it;
        self.shear_factor = shear_factor;
    }


    pub fn copy_data(&self) -> (V, V, V, V, V, V)
    {
        (self.area, self.i11, self.i22, self.i12, self.it, self.shear_factor)
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


    pub fn copy_name_and_data(&self) -> (&str, V, V, V, V, V, V)
    {
        let (area, i11, i22, i12, it, shear_factor) =
            self.beam_section.copy_data();
        (&self.name, area, i11, i22, i12, it, shear_factor)
    }
}
