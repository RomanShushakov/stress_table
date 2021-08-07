use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct TrussSection<V>
{
    area: V,
    area2: Option<V>,
}


impl<V> TrussSection<V>
    where V: Copy + PartialEq,
{
    pub fn create(area: V, area2: Option<V>) -> Self
    {
        TrussSection { area, area2 }
    }


    pub fn is_data_same(&self, area: V, area2: Option<V>) -> bool
    {
        self.area == area && self.area2 == area2
    }


    pub fn update(&mut self, area: V, area2: Option<V>)
    {
        self.area = area;
        self.area2 = area2;
    }


    pub fn extract_data(&self) -> (V, Option<V>)
    {
        (self.area, self.area2)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedTrussSection<V>
{
    name: String,
    truss_section: TrussSection<V>,
}


impl<V> DeletedTrussSection<V>
    where V: Copy + PartialEq,
{
    pub fn create(name: &str, truss_section: TrussSection<V>) -> Self
    {
        DeletedTrussSection { name: String::from(name), truss_section }
    }


    pub fn extract_name_and_data(&self) -> (&str, V, Option<V>)
    {
        let (area, area2) = self.truss_section.extract_data();
        (&self.name, area, area2)
    }
}
