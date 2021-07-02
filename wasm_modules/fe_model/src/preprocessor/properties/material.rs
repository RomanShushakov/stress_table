use serde::Serialize;

use crate::types::FEFloat;


#[derive(Debug, Clone, Serialize)]
pub struct Material
{
    young_modulus: FEFloat,
    poisson_ratio: FEFloat
}


impl Material
{
    pub fn create(young_modulus: FEFloat, poisson_ratio: FEFloat) -> Self
    {
        Material { young_modulus, poisson_ratio }
    }


    pub fn data_same(&self, young_modulus: FEFloat, poisson_ration: FEFloat) -> bool
    {
        self.young_modulus == young_modulus && self.poisson_ratio == poisson_ration
    }


    pub fn update(&mut self, young_modulus: FEFloat, poisson_ration: FEFloat)
    {
        self.young_modulus = young_modulus;
        self.poisson_ratio = poisson_ration;
    }


    pub fn extract_data(&self) -> (FEFloat, FEFloat)
    {
        (self.young_modulus, self.poisson_ratio)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedMaterial
{
    name: String,
    material: Material,
}


impl DeletedMaterial
{
    pub fn create(name: &str, material: Material) -> Self
    {
        DeletedMaterial { name: String::from(name), material }
    }


    pub fn extract_name_and_data(&self) -> (&str, FEFloat, FEFloat)
    {
        let (young_modulus, poisson_ratio) = self.material.extract_data();
        (&self.name, young_modulus, poisson_ratio)
    }
}
