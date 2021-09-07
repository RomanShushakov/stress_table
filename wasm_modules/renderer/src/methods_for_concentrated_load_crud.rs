use wasm_bindgen::prelude::*;

use crate::point_object::{PointObjectKey};
use crate::point_object::{PointObjectType};

use crate::concentrated_load::ConcentratedLoad;

use crate::Renderer;


#[wasm_bindgen]
impl Renderer
{
    pub fn add_concentrated_load(&mut self, point_number: u32, fx: f32, fy: f32, fz: f32,
        mx: f32, my: f32, mz: f32) -> Result<(), JsValue>
    {
        let point_object_key = PointObjectKey::create(point_number,
            PointObjectType::Point);
        if !self.props.point_objects.contains_key(&point_object_key)
        {
            let error_message = format!("Renderer: Add concentrated load action: Point with \
                number {} does not exist!", point_number);
            return Err(JsValue::from(error_message));
        }

        let uid =
            {
                let mut current_uid = rand::random::<u32>();
                while self.props.point_objects.values().position(|point_object|
                        point_object.is_uid_same(current_uid)).is_some() ||
                    self.state.line_objects.values().position(|line_object|
                        line_object.is_uid_same(current_uid)).is_some() || current_uid == 255 ||
                    self.state.concentrated_loads.values()
                        .position(|concentrated_load|
                            concentrated_load.is_uid_same(current_uid)).is_some()
                {
                    current_uid = rand::random::<u32>();
                }
                current_uid
            };

        let concentrated_load = ConcentratedLoad::create(
            fx, fy, fz, mx, my, mz, uid);
        self.state.concentrated_loads.insert(point_number, concentrated_load);
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
        Ok(())
    }


    pub fn update_concentrated_load(&mut self, point_number: u32, fx: f32, fy: f32, fz: f32,
        mx: f32, my: f32, mz: f32) -> Result<(), JsValue>
    {
        if let Some(concentrated_load) =
            self.state.concentrated_loads.get_mut(&point_number)
        {
            concentrated_load.update_load_and_moment_components(fx, fy, fz, mx, my, mz);
        }
        else
        {
            let error_message = format!("Renderer: Update concentrated load action: \
                Concentrated load applied to point with number {} does not exist!",
                point_number);
            return Err(JsValue::from(error_message));
        }
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
        Ok(())
    }


    pub fn delete_concentrated_load(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        if self.state.concentrated_loads.remove(&point_number).is_none()
        {
            let error_message = format!("Renderer: Delete concentrated load action: \
                Concentrated load applied to point with number {} does not exist!", point_number);
            return Err(JsValue::from(error_message));
        }
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
        Ok(())
    }
}
