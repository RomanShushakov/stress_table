use wasm_bindgen::prelude::*;

use crate::global_scene::point_object::{PointObjectKey};
use crate::global_scene::point_object::{PointObjectType};
use crate::global_scene::line_object::{LineObject, LineObjectKey};
use crate::global_scene::line_object::{LineObjectType, LineObjectColorScheme};
use crate::global_scene::preprocessor::preprocessor::Preprocessor;


impl Preprocessor
{
    pub fn add_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        let point_object_type = match line_object_type
            {
                LineObjectType::Line => PointObjectType::Point,
                LineObjectType::Element => PointObjectType::Node,
            };
        let start_point_object_key = PointObjectKey::create(
            start_point_object_number, point_object_type);
        let end_point_object_key = PointObjectKey::create(
            end_point_object_number, point_object_type);
        if !self.point_objects.contains_key(&start_point_object_key)
        {
            let error_message = format!("Renderer: Add {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
            point_object_type.as_str(), start_point_object_number);
            return Err(JsValue::from(error_message));
        }
        if !self.point_objects.contains_key(&end_point_object_key)
        {
            let error_message = format!("Renderer: Add {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
            point_object_type.as_str(), end_point_object_number);
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
        let line_object_key = LineObjectKey::create(number, line_object_type);
        let line_object = LineObject::create(start_point_object_key,
            end_point_object_key, uid);
        self.line_objects.insert(line_object_key, line_object);
        Ok(())
    }


    pub fn update_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        let point_object_type = match line_object_type
            {
                LineObjectType::Line => PointObjectType::Point,
                LineObjectType::Element => PointObjectType::Node,
            };
        let start_point_object_key = PointObjectKey::create(
            start_point_object_number, point_object_type);
        let end_point_object_key = PointObjectKey::create(
            end_point_object_number, point_object_type);
        if !self.point_objects.contains_key(&start_point_object_key)
        {
            let error_message = format!("Renderer: Update {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
            point_object_type.as_str(), start_point_object_number);
            return Err(JsValue::from(error_message));
        }
        if !self.point_objects.contains_key(&end_point_object_key)
        {
            let error_message = format!("Renderer: Update {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
            point_object_type.as_str(), end_point_object_number);
            return Err(JsValue::from(error_message));
        }

        if let Some(line_object) = self.line_objects.get_mut(
            &LineObjectKey::create(number, line_object_type))
        {
            line_object.update(start_point_object_key, end_point_object_key);
        }
        else
        {
            let error_message = format!("Renderer: Update {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
                line_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn delete_line_object(&mut self, number: u32, line_object_type: LineObjectType)
        -> Result<(), JsValue>
    {
        if self.line_objects.remove(&LineObjectKey::create(number, line_object_type))
            .is_none()
        {
            let error_message = format!("Renderer: Delete {} action: {} with \
                number {} does not exist!", line_object_type.as_str().to_lowercase(),
                line_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn update_line_objects_color_scheme(&mut self, line_object_numbers: &[u32],
        line_object_type: LineObjectType, line_object_color_scheme: LineObjectColorScheme)
        -> Result<(), JsValue>
    {
        for line_object_number in line_object_numbers
        {
            let line_object_key = LineObjectKey::create(
                *line_object_number, line_object_type);
            if let Some(line_object) = self.line_objects
                .get_mut(&line_object_key)
            {
                line_object.update_color_scheme(line_object_color_scheme);
            }
            else
            {
                let error_message = format!("Renderer: Update {} color scheme action: {} \
                    with number {} does not exist!", line_object_type.as_str().to_lowercase(),
                    line_object_type.as_str(), line_object_number);
                return Err(JsValue::from(error_message));
            }
        }
        Ok(())
    }
}
