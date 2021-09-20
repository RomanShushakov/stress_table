use wasm_bindgen::prelude::*;

use crate::line_object::{LineObjectKey};
use crate::line_object::{LineObjectType};

use crate::distributed_line_load::DistributedLineLoad;

use crate::global_scene::preprocessor::preprocessor::Preprocessor;


impl Preprocessor
{
    pub fn add_distributed_line_load(&mut self, line_number: u32, qx: f32, qy: f32, qz: f32)
        -> Result<(), JsValue>
    {
        let line_object_key = LineObjectKey::create(line_number,
            LineObjectType::Line);
        if !self.line_objects.contains_key(&line_object_key)
        {
            let error_message = format!("Renderer: Add distributed line load action: \
                Line with number {} does not exist!", line_number);
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
                    current_uid == 255
                {
                    current_uid = rand::random::<u32>();
                }
                current_uid
            };

        let distributed_line_load = DistributedLineLoad::create(
            qx, qy, qz, uid);
        self.distributed_line_loads.insert(line_number, distributed_line_load);
        Ok(())
    }


    pub fn update_distributed_line_load(&mut self, line_number: u32, qx: f32, qy: f32, qz: f32)
        -> Result<(), JsValue>
    {
        if let Some(distributed_line_load) =
            self.distributed_line_loads.get_mut(&line_number)
        {
            distributed_line_load.update_load_components(qx, qy, qz);
        }
        else
        {
            let error_message = format!("Renderer: Update distributed line load action: \
                Distributed line load applied to line with number {} does not exist!",
                line_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn delete_distributed_line_load(&mut self, line_number: u32) -> Result<(), JsValue>
    {
        if self.distributed_line_loads.remove(&line_number).is_none()
        {
            let error_message = format!("Renderer: Delete distributed line load action: \
                Distributed line load applied to line with number {} does not exist!", line_number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }
}
