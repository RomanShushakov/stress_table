use wasm_bindgen::prelude::*;

use crate::global_scene::point_object::{PointObjectKey, PointObject, Coordinates};
use crate::global_scene::point_object::{PointObjectType};
use crate::global_scene::line_object::{LineObject, LineObjectKey};
use crate::global_scene::line_object::{LineObjectType, LineObjectColorScheme};
use crate::global_scene::preprocessor::concentrated_load::ConcentratedLoad;
use crate::global_scene::preprocessor::distributed_line_load::DistributedLineLoad;
use crate::global_scene::preprocessor::boundary_condition::BoundaryCondition;

use crate::functions::{log, normalize_point_objects_coordinates};

use crate::Renderer;


#[wasm_bindgen]
impl Renderer
{
    pub fn add_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType) -> Result<(), JsValue>
    {
        self.state.global_scene.add_point_object(number, x, y, z, point_object_type,
            self.props.canvas_gl.width() as f32,
            self.props.canvas_gl.height() as f32)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn update_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType) -> Result<(), JsValue>
    {
        self.state.global_scene.update_point_object(number, x, y, z, point_object_type,
            self.props.canvas_gl.width() as f32,
            self.props.canvas_gl.height() as f32)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn delete_point_object(&mut self, number: u32, point_object_type: PointObjectType)
        -> Result<(), JsValue>
    {
        self.state.global_scene.delete_point_object(number, point_object_type,
            self.props.canvas_gl.width() as f32,
            self.props.canvas_gl.height() as f32)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn add_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        self.state.global_scene.add_line_object(number, start_point_object_number,
            end_point_object_number, line_object_type)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn update_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        self.state.global_scene.update_line_object(number, start_point_object_number,
            end_point_object_number, line_object_type)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn delete_line_object(&mut self, number: u32, line_object_type: LineObjectType)
        -> Result<(), JsValue>
    {
        self.state.global_scene.delete_line_object(number, line_object_type)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn update_line_objects_color_scheme(&mut self, line_object_numbers: &[u32],
        line_object_type: LineObjectType, line_object_color_scheme: LineObjectColorScheme)
        -> Result<(), JsValue>
    {
        self.state.global_scene.update_line_objects_color_scheme(line_object_numbers,
            line_object_type, line_object_color_scheme)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn add_concentrated_load(&mut self, point_number: u32, fx: f32, fy: f32, fz: f32,
        mx: f32, my: f32, mz: f32) -> Result<(), JsValue>
    {
        self.state.global_scene.add_concentrated_load(point_number, fx, fy, fz, mx, my, mz)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn update_concentrated_load(&mut self, point_number: u32, fx: f32, fy: f32, fz: f32,
        mx: f32, my: f32, mz: f32) -> Result<(), JsValue>
    {
        self.state.global_scene.update_concentrated_load(point_number, fx, fy, fz, mx, my, mz)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn delete_concentrated_load(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        self.state.global_scene.delete_concentrated_load(point_number)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn add_distributed_line_load(&mut self, line_number: u32, qx: f32, qy: f32, qz: f32)
        -> Result<(), JsValue>
    {
        self.state.global_scene.add_distributed_line_load(line_number, qx, qy, qz)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn update_distributed_line_load(&mut self, line_number: u32, qx: f32, qy: f32, qz: f32)
        -> Result<(), JsValue>
    {
        self.state.global_scene.update_distributed_line_load(line_number, qx, qy, qz)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn delete_distributed_line_load(&mut self, line_number: u32) -> Result<(), JsValue>
    {
        self.state.global_scene.delete_distributed_line_load(line_number)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn add_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        self.state.global_scene.add_boundary_condition(point_number)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn update_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        self.state.global_scene.update_boundary_condition(point_number)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn delete_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        self.state.global_scene.delete_boundary_condition(point_number)?;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }
}
