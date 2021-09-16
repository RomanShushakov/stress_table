use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Material<V>
{
    young_modulus: V,
    poisson_ratio: V,
}


impl<V> Material<V>
    where V: Copy + PartialEq,
{
    pub fn create(young_modulus: V, poisson_ratio: V) -> Self
    {
        Material { young_modulus, poisson_ratio }
    }


    pub fn is_data_same(&self, young_modulus: V, poisson_ration: V) -> bool
    {
        self.young_modulus == young_modulus && self.poisson_ratio == poisson_ration
    }


    pub fn update(&mut self, young_modulus: V, poisson_ration: V)
    {
        self.young_modulus = young_modulus;
        self.poisson_ratio = poisson_ration;
    }


    pub fn copy_data(&self) -> (V, V)
    {
        (self.young_modulus, self.poisson_ratio)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedMaterial<V>
{
    name: String,
    material: Material<V>,
}


impl<V> DeletedMaterial<V>
    where V: Copy + PartialEq,
{
    pub fn create(name: &str, material: Material<V>) -> Self
    {
        DeletedMaterial { name: String::from(name), material }
    }


    pub fn copy_name_and_data(&self) -> (&str, V, V)
    {
        let (young_modulus, poisson_ratio) = self.material.copy_data();
        (&self.name, young_modulus, poisson_ratio)
    }
}
