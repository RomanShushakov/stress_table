use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct TrussSection
{
    area: f64,
    area2: Option<f64>,
}


impl TrussSection
{
    pub fn create(area: f64, area2: Option<f64>) -> Self
    {
        TrussSection { area, area2 }
    }


    pub fn data_same(&self, area: f64, area2: Option<f64>) -> bool
    {
        self.area == area && self.area2 == area2
    }


    pub fn update(&mut self, area: f64, area2: Option<f64>)
    {
        self.area = area;
        self.area2 = area2;
    }


    pub fn extract_data(&self) -> (f64, Option<f64>)
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


    pub fn extract_name_and_data(&self) -> (&str, f64, Option<f64>)
    {
        let (area, area2) = self.truss_section.extract_data();
        (&self.name, area, area2)
    }
}
