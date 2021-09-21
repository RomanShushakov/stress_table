use serde::Serialize;
use wasm_bindgen::JsValue;
use std::fmt::Debug;


#[derive(Debug, Clone, Serialize)]
pub struct Material<V>
{
    young_modulus: V,
    poisson_ratio: V,
}


impl<V> Material<V>
    where V: Copy + Debug + PartialEq + PartialOrd + From<f32>,
{
    pub fn create(young_modulus: V, poisson_ratio: V) -> Result<Self, JsValue>
    {
        let error_message_header = "Material: Create material action";

        if young_modulus <= V::from(0f32)
        {
            let error_message = &format!("{}: Young modulus {:?} is less or equal to zero!",
                error_message_header, young_modulus);
            return Err(JsValue::from(error_message));
        }

        if poisson_ratio <= V::from(0f32)
        {
            let error_message = &format!("{}: Poisson ratio {:?} is less or equal to zero!",
                error_message_header, poisson_ratio);
            return Err(JsValue::from(error_message));
        }

        Ok(Material { young_modulus, poisson_ratio })
    }


    pub fn is_data_same(&self, young_modulus: V, poisson_ratio: V) -> bool
    {
        self.young_modulus == young_modulus && self.poisson_ratio == poisson_ratio
    }


    pub fn update(&mut self, young_modulus: V, poisson_ratio: V) -> Result<(), JsValue>
    {
        let error_message_header = "Material: Update material action";

        if young_modulus <= V::from(0f32)
        {
            let error_message = &format!("{}: Young modulus {:?} is less or equal to zero!",
                error_message_header, young_modulus);
            return Err(JsValue::from(error_message));
        }

        if poisson_ratio <= V::from(0f32)
        {
            let error_message = &format!("{}: Poisson ratio {:?} is less or equal to zero!",
                error_message_header, poisson_ratio);
            return Err(JsValue::from(error_message));
        }

        self.young_modulus = young_modulus;
        self.poisson_ratio = poisson_ratio;

        Ok(())
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
    where V: Copy + Debug + PartialEq + PartialOrd + From<f32>,
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
