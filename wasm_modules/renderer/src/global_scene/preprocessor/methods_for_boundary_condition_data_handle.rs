use wasm_bindgen::prelude::*;

use crate::point_object::{PointObjectKey};
use crate::point_object::{PointObjectType};

use crate::boundary_condition::BoundaryCondition;

use crate::global_scene::preprocessor::preprocessor::Preprocessor;


impl Preprocessor
{
    pub fn add_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        let point_object_key = PointObjectKey::create(point_number,
            PointObjectType::Point);
        if !self.point_objects.contains_key(&point_object_key)
        {
            let error_message = format!("Renderer: Add boundary condition action: \
                Point with number {} does not exist!", point_number);
            return Err(JsValue::from(error_message));
        }

        let uid =
            {
                let mut current_uid = rand::random::<u32>();
                while self.point_objects.values().position(|point_object|
                        point_object.is_uid_same(current_uid)).is_some() ||
                    self.line_objects.values().position(|line_object|
                        line_object.is_uid_same(current_uid)).is_some() ||
                    self.concentrated_loads.values()
                        .position(|concentrated_load|
                            concentrated_load.is_uid_same(current_uid)).is_some() ||
                    self.distributed_line_loads.values()
                        .position(|distributed_line_load|
                            distributed_line_load.is_uid_same(current_uid)).is_some() ||
                    self.boundary_conditions.values()
                        .position(|boundary_condition|
                            boundary_condition.is_uid_same(current_uid)).is_some() ||
                    current_uid == 255
                {
                    current_uid = rand::random::<u32>();
                }
                current_uid
            };

        let boundary_condition = BoundaryCondition::create(uid);
        self.boundary_conditions.insert(point_number, boundary_condition);
        Ok(())
    }


    pub fn update_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        if !self.boundary_conditions.contains_key(&point_number)
        {
            let error_message = format!("Renderer: Update boundary condition action: \
                Boundary condition applied to point with number {} does not exist!",
                point_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn delete_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        if self.boundary_conditions.remove(&point_number).is_none()
        {
            let error_message = format!("Renderer: Delete boundary condition action: \
                Boundary condition applied to point with number {} does not exist!", point_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }
}
