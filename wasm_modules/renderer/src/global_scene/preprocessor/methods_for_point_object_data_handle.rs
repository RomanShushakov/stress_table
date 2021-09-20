use wasm_bindgen::prelude::*;

use crate::global_scene::preprocessor::preprocessor::Preprocessor;
use crate::global_scene::point_object::{PointObjectKey, Coordinates, PointObject};
use crate::global_scene::point_object::PointObjectType;

use crate::functions::{normalize_point_objects_coordinates, log};


impl Preprocessor
{
    fn update_point_objects_normalized_coordinates(&mut self, canvas_width: f32, canvas_height: f32)
    {
        normalize_point_objects_coordinates(&mut self.point_objects,
            &self.line_objects,
            &self.concentrated_loads,
            &self.distributed_line_loads,
            canvas_width,
            canvas_height);
        log(&format!("{:?}", self.point_objects));
    }


    pub fn add_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType, canvas_width: f32, canvas_height: f32)
        -> Result<(), JsValue>
    {
        let point_object_key = PointObjectKey::create(number, point_object_type);
        let coordinates = Coordinates::create(x, y, z);
        let point_object = PointObject::create(coordinates);
        self.point_objects.insert(point_object_key, point_object);
        self.update_point_objects_normalized_coordinates(canvas_width, canvas_height);
        Ok(())
    }


    pub fn update_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType, canvas_width: f32, canvas_height: f32)
        -> Result<(), JsValue>
    {
        if let Some(point_object) = self.point_objects
            .get_mut(&PointObjectKey::create(number, point_object_type))
        {
            point_object.update_coordinates(x, y, z);
            self.update_point_objects_normalized_coordinates(canvas_width, canvas_height);
        }
        else
        {
            let error_message = format!("Renderer: Update {} action: {} with number \
                {} does not exist!", point_object_type.as_str().to_lowercase(),
                point_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn delete_point_object(&mut self, number: u32, point_object_type: PointObjectType,
        canvas_width: f32, canvas_height: f32) -> Result<(), JsValue>
    {
        if self.point_objects.remove(&PointObjectKey::create(
            number, point_object_type)).is_none()
        {
            let error_message = format!("Renderer: Delete {} action: {} with \
                number {} does not exist!", point_object_type.as_str().to_lowercase(),
                point_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        if !self.point_objects.is_empty()
        {
            self.update_point_objects_normalized_coordinates(canvas_width, canvas_height);
        }
        Ok(())
    }
}
