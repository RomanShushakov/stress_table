use wasm_bindgen::prelude::*;
use std::fmt::Debug;

use serde::Serialize;


#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct LocalAxis1Direction<V>(V, V, V);


impl<V> LocalAxis1Direction<V>
    where V: Copy + Debug + Into<f64>,
{
    pub fn create(local_axis_1_direction: &[V]) -> Result<Self, JsValue>
    {
        if local_axis_1_direction.len() != 3
        {
            let error_message = &format!("Properties: Create beam section local axis 1 \
                direction: Incorrect number of components in {:?}!", local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }

        Ok(LocalAxis1Direction(
            local_axis_1_direction[0],
            local_axis_1_direction[1],
            local_axis_1_direction[2])
        )
    }


    pub fn extract(&self) -> [f64; 3]
    {
        [self.0.into(), self.1.into(), self.2.into()]
    }
}
