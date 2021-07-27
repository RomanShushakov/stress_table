use wasm_bindgen::prelude::*;

use serde::Serialize;

use crate::types::{FEUInt, FEFloat};


#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct LocalAxis1Direction(FEFloat, FEFloat, FEFloat);


impl LocalAxis1Direction
{
    pub fn create(local_axis_1_direction: &[FEFloat]) -> Result<Self, JsValue>
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
}
