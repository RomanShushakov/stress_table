use wasm_bindgen::prelude::*;
use std::collections::HashSet;

use crate::drawn_object::scene_adapter::SceneAdapter;

use crate::global_scene::preprocessor::preprocessor::Preprocessor;
use crate::global_scene::point_object::PointObjectType;
use crate::global_scene::line_object::{LineObjectType, LineObjectColorScheme};


pub enum GlobalScene
{
    Preprocessor(Preprocessor)
}


impl GlobalScene
{
    pub fn create_preprocessor() -> Self
    {
        let preprocessor = Preprocessor::create();
        GlobalScene::Preprocessor(preprocessor)
    }


    pub fn update_scene_for_selection(&mut self, under_selection_box_colors: &Vec<u8>,
        selected_colors: &HashSet<[u8; 4]>, d_scale: f32, is_geometry_visible: &bool,
        is_load_visible: &bool, is_boundary_condition_visible: &bool, is_mesh_visible: &bool)
        -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.update_scene_for_selection(under_selection_box_colors,
                        selected_colors, d_scale, is_geometry_visible, is_load_visible,
                        is_boundary_condition_visible, is_mesh_visible)
                }
        }
    }


    pub fn update_scene_visible(&mut self, under_selection_box_colors: &Vec<u8>,
        selected_colors: &HashSet<[u8; 4]>, d_scale: f32, is_geometry_visible: &bool,
        is_load_visible: &bool, is_boundary_condition_visible: &bool, is_mesh_visible: &bool)
        -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.update_scene_visible(under_selection_box_colors,
                        selected_colors, d_scale, is_geometry_visible, is_load_visible,
                        is_boundary_condition_visible, is_mesh_visible)
                }
        }
    }


    pub fn select_objects(&mut self, selected_colors: &HashSet<[u8; 4]>,
        drop_selection: &js_sys::Function) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.select_objects(selected_colors, drop_selection)
                }
        }
    }


     pub fn preview_selected_line_objects(&mut self, selected_line_object_numbers: JsValue,
        line_object_type: LineObjectType, selected_colors: &mut HashSet<[u8; 4]>)
         -> Result<(), JsValue>
     {
         match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.preview_selected_line_objects(selected_line_object_numbers,
                        line_object_type, selected_colors)
                }
        }
     }


    pub fn preview_beam_section_orientation(&mut self, beam_section_orientation: JsValue,
        line_object_type: LineObjectType, selected_colors: &mut HashSet<[u8; 4]>)
        -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.preview_beam_section_orientation(beam_section_orientation,
                        line_object_type, selected_colors)
                }
        }
    }


    pub fn add_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType, canvas_width: f32, canvas_height: f32)
        -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.add_point_object(number, x, y, z, point_object_type, canvas_width,
                        canvas_height)
                }
        }
    }


    pub fn update_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType, canvas_width: f32, canvas_height: f32)
        -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.update_point_object(number, x, y, z, point_object_type,
                        canvas_width, canvas_height)
                }
        }
    }


    pub fn delete_point_object(&mut self, number: u32, point_object_type: PointObjectType,
        canvas_width: f32, canvas_height: f32) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.delete_point_object(number, point_object_type, canvas_width,
                        canvas_height)
                }
        }
    }


    pub fn add_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.add_line_object(number, start_point_object_number,
                        end_point_object_number, line_object_type)
                }
        }
    }


    pub fn update_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.update_line_object(number, start_point_object_number,
                        end_point_object_number, line_object_type)
                }
        }
    }


    pub fn delete_line_object(&mut self, number: u32, line_object_type: LineObjectType)
        -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.delete_line_object(number, line_object_type)
                }
        }
    }


    pub fn update_line_objects_color_scheme(&mut self, line_object_numbers: &[u32],
        line_object_type: LineObjectType, line_object_color_scheme: LineObjectColorScheme)
        -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.update_line_objects_color_scheme(line_object_numbers,
                        line_object_type, line_object_color_scheme)
                }
        }
    }


    pub fn add_concentrated_load(&mut self, point_number: u32, fx: f32, fy: f32, fz: f32,
        mx: f32, my: f32, mz: f32) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.add_concentrated_load(point_number, fx, fy, fz, mx, my, mz)
                }
        }
    }


    pub fn update_concentrated_load(&mut self, point_number: u32, fx: f32, fy: f32, fz: f32,
        mx: f32, my: f32, mz: f32) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.update_concentrated_load(point_number, fx, fy, fz, mx, my, mz)
                }
        }
    }


    pub fn delete_concentrated_load(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.delete_concentrated_load(point_number)
                }
        }
    }


    pub fn add_distributed_line_load(&mut self, line_number: u32, qx: f32, qy: f32, qz: f32)
        -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.add_distributed_line_load(line_number, qx, qy, qz)
                }
        }
    }


    pub fn update_distributed_line_load(&mut self, line_number: u32, qx: f32, qy: f32, qz: f32)
        -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.add_distributed_line_load(line_number, qx, qy, qz)
                }
        }
    }


    pub fn delete_distributed_line_load(&mut self, line_number: u32) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.delete_distributed_line_load(line_number)
                }
        }
    }


    pub fn add_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.add_boundary_condition(point_number)
                }
        }
    }


    pub fn update_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.update_boundary_condition(point_number)
                }
        }
    }


    pub fn delete_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.delete_boundary_condition(point_number)
                }
        }
    }


    pub fn ref_optional_scene_for_selection(&self) -> &Option<SceneAdapter>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.ref_optional_scene_for_selection()
                }
        }
    }


    pub fn ref_optional_scene_visible(&self) -> &Option<SceneAdapter>
    {
        match self
        {
            GlobalScene::Preprocessor(preprocessor) =>
                {
                    preprocessor.ref_optional_scene_visible()
                }
        }
    }
}
