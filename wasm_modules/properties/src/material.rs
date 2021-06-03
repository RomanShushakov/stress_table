#[derive(Debug, Clone)]
pub struct Material
{
    young_modulus: f64,
    poisson_ratio: f64
}


impl Material
{
    pub fn create(young_modulus: f64, poisson_ratio: f64) -> Self
    {
        Material { young_modulus, poisson_ratio }
    }


    pub fn data_same(&self, young_modulus: f64, poisson_ration: f64) -> bool
    {
        self.young_modulus == young_modulus && self.poisson_ratio == poisson_ration
    }


    pub fn update(&mut self, young_modulus: f64, poisson_ration: f64)
    {
        self.young_modulus = young_modulus;
        self.poisson_ratio = poisson_ration;
    }


    pub fn extract_data(&self) -> (f64, f64)
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


    pub fn extract_name_and_data(&self) -> (&str, f64, f64)
    {
        let (young_modulus, poisson_ratio) = self.material.extract_data();
        (&self.name, young_modulus, poisson_ratio)
    }
}
