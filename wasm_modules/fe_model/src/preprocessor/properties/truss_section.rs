use serde::Serialize;
use wasm_bindgen::JsValue;
use std::fmt::Debug;


#[derive(Debug, Clone, Serialize)]
pub struct TrussSection<V>
{
    area: V,
    area2: Option<V>,
}


impl<V> TrussSection<V>
    where V: Copy + Debug + PartialEq + PartialOrd + From<f32>,
{
    pub fn create(area: V, area2: Option<V>) -> Result<Self, JsValue>
    {
        let error_message_header = "Truss section: Create truss section action";

        if area <= V::from(0f32)
        {
            let error_message = &format!("{}: Area {:?} is less or equal to zero!",
                error_message_header, area);
            return Err(JsValue::from(error_message));
        }

        if let Some(area) = area2
        {
            if area <= V::from(0f32)
            {
                let error_message = &format!("{}: Area {:?} is less or equal to zero!",
                    error_message_header, area);
                return Err(JsValue::from(error_message));
            }
        }

        Ok(TrussSection { area, area2 })
    }


    pub fn is_data_same(&self, area: V, area2: Option<V>) -> bool
    {
        self.area == area && self.area2 == area2
    }


    pub fn update(&mut self, area: V, area2: Option<V>) -> Result<(), JsValue>
    {
        let error_message_header = "Truss section: Update truss section action";

        if area <= V::from(0f32)
        {
            let error_message = &format!("{}: Area {:?} is less or equal to zero!",
                error_message_header, area);
            return Err(JsValue::from(error_message));
        }

        if let Some(area) = area2
        {
            if area <= V::from(0f32)
            {
                let error_message = &format!("{}: Area {:?} is less or equal to zero!",
                    error_message_header, area);
                return Err(JsValue::from(error_message));
            }
        }

        self.area = area;
        self.area2 = area2;

        Ok(())
    }


    pub fn copy_data(&self) -> (V, Option<V>)
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
    where V: Copy + Debug + PartialEq + PartialOrd + From<f32>,
{
    pub fn create(name: &str, truss_section: TrussSection<V>) -> Self
    {
        DeletedTrussSection { name: String::from(name), truss_section }
    }


    pub fn copy_name_and_data(&self) -> (&str, V, Option<V>)
    {
        let (area, area2) = self.truss_section.copy_data();
        (&self.name, area, area2)
    }
}
