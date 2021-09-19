use wasm_bindgen::prelude::*;

use crate::point_object::{PointObjectKey, PointObject, Coordinates};
use crate::point_object::{PointObjectType};

use crate::Renderer;


#[wasm_bindgen]
impl Renderer
{
    pub fn add_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType) -> Result<(), JsValue>
    {
        let point_object_key = PointObjectKey::create(number, point_object_type);
        let coordinates = Coordinates::create(x, y, z);
        let point_object = PointObject::create(coordinates);
        self.state.point_objects.insert(point_object_key, point_object);
        self.update_point_objects_normalized_coordinates();
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
        Ok(())
    }


    pub fn update_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType) -> Result<(), JsValue>
    {
        if let Some(point_object) = self.state.point_objects
            .get_mut(&PointObjectKey::create(number, point_object_type))
        {
            point_object.update_coordinates(x, y, z);
            self.update_point_objects_normalized_coordinates();
        }
        else
        {
            let error_message = format!("Renderer: Update {} action: {} with number \
                {} does not exist!", point_object_type.as_str().to_lowercase(),
                point_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
        Ok(())
    }


    pub fn delete_point_object(&mut self, number: u32, point_object_type: PointObjectType)
        -> Result<(), JsValue>
    {
        if self.state.point_objects.remove(&PointObjectKey::create(
            number, point_object_type)).is_none()
        {
            let error_message = format!("Renderer: Delete {} action: {} with \
                number {} does not exist!", point_object_type.as_str().to_lowercase(),
                point_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        if !self.state.point_objects.is_empty()
        {
            self.update_point_objects_normalized_coordinates();
        }
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
        Ok(())
    }
}
