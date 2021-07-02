use serde::Serialize;

use crate::types::FEFloat;


#[derive(Debug, Clone, Serialize)]
pub struct TrussSection
{
    area: FEFloat,
    area2: Option<FEFloat>,
}


impl TrussSection
{
    pub fn create(area: FEFloat, area2: Option<FEFloat>) -> Self
    {
        TrussSection { area, area2 }
    }


    pub fn data_same(&self, area: FEFloat, area2: Option<FEFloat>) -> bool
    {
        self.area == area && self.area2 == area2
    }


    pub fn update(&mut self, area: FEFloat, area2: Option<FEFloat>)
    {
        self.area = area;
        self.area2 = area2;
    }


    pub fn extract_data(&self) -> (FEFloat, Option<FEFloat>)
    {
        (self.area, self.area2)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedTrussSection
{
    name: String,
    truss_section: TrussSection,
}


impl DeletedTrussSection
{
    pub fn create(name: &str, truss_section: TrussSection) -> Self
    {
        DeletedTrussSection { name: String::from(name), truss_section }
    }


    pub fn extract_name_and_data(&self) -> (&str, FEFloat, Option<FEFloat>)
    {
        let (area, area2) = self.truss_section.extract_data();
        (&self.name, area, area2)
    }
}
