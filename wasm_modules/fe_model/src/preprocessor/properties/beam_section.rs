use serde::Serialize;
use wasm_bindgen::JsValue;
use std::fmt::Debug;


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
    where V: Copy + Debug + PartialEq + PartialOrd + From<f32>,
{
    pub fn create(area: V, i11: V, i22: V, i12: V, it: V, shear_factor: V) -> Result<Self, JsValue>
    {
        let error_message_header = "Beam section: Create beam section action";

        if area <= V::from(0f32)
        {
            let error_message = &format!("{}: Area {:?} is less or equal to zero!",
                error_message_header, area);
            return Err(JsValue::from(error_message));
        }

        if i11 <= V::from(0f32)
        {
            let error_message = &format!("{}: Moment of inertia I11 {:?} is less or \
                equal to zero!", error_message_header, i11);
            return Err(JsValue::from(error_message));
        }

        if i22 <= V::from(0f32)
        {
            let error_message = &format!("{}: Moment of inertia I22 {:?} is less or \
                equal to zero!", error_message_header, i22);
            return Err(JsValue::from(error_message));
        }

        if it <= V::from(0f32)
        {
            let error_message = &format!("{}: Torsion constant It {:?} is less or \
                equal to zero!", error_message_header, it);
            return Err(JsValue::from(error_message));
        }

        if shear_factor <= V::from(0f32)
        {
            let error_message = &format!("{}: Shear factor {:?} is less or \
                equal to zero!", error_message_header, shear_factor);
            return Err(JsValue::from(error_message));
        }

        Ok(BeamSection { area, i11, i22, i12, it, shear_factor })
    }


    pub fn data_same(&self, area: V, i11: V, i22: V, i12: V, it: V, shear_factor: V)
        -> bool
    {
        self.area == area && self.i11 == i11 && self.i22 == i22 && self.i12 == i12 &&
        self.it == it && self.shear_factor == shear_factor
    }


    pub fn update(&mut self, area: V, i11: V, i22: V, i12: V, it: V, shear_factor: V)
        -> Result<(), JsValue>
    {
        let error_message_header = "Beam section: Update beam section action";

        if area <= V::from(0f32)
        {
            let error_message = &format!("{}: Area {:?} is less or equal to zero!",
                error_message_header, area);
            return Err(JsValue::from(error_message));
        }

        if i11 <= V::from(0f32)
        {
            let error_message = &format!("{}: Moment of inertia I11 {:?} is less or \
                equal to zero!", error_message_header, i11);
            return Err(JsValue::from(error_message));
        }

        if i22 <= V::from(0f32)
        {
            let error_message = &format!("{}: Moment of inertia I22 {:?} is less or \
                equal to zero!", error_message_header, i22);
            return Err(JsValue::from(error_message));
        }

        if it <= V::from(0f32)
        {
            let error_message = &format!("{}: Torsion constant It {:?} is less or \
                equal to zero!", error_message_header, it);
            return Err(JsValue::from(error_message));
        }

        if shear_factor <= V::from(0f32)
        {
            let error_message = &format!("{}: Shear factor {:?} is less or \
                equal to zero!", error_message_header, shear_factor);
            return Err(JsValue::from(error_message));
        }

        self.area = area;
        self.i11 = i11;
        self.i22 = i22;
        self.i12 = i12;
        self.it = it;
        self.shear_factor = shear_factor;

        Ok(())
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
    where V: Copy + Debug + PartialEq + PartialOrd + From<f32>,
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
