use wasm_bindgen::prelude::*;
use std::collections::{HashSet, HashMap};
use serde_json::json;

use crate::drawn_object::scene_adapter::SceneAdapter;

use crate::global_scene::preprocessor::preprocessor::Preprocessor;
use crate::global_scene::point_object::PointObjectType;
use crate::global_scene::point_object::{PointObjectKey, PointObject};
use crate::global_scene::line_object::{LineObjectKey, LineObject};
use crate::global_scene::line_object::{LineObjectType, LineObjectColorScheme};
use crate::global_scene::preprocessor::concentrated_load::ConcentratedLoad;
use crate::global_scene::preprocessor::distributed_line_load::DistributedLineLoad;
use crate::global_scene::preprocessor::boundary_condition::BoundaryCondition;

use crate::consts::{EVENT_TARGET, EXTRACT_DATA_FOR_POSTPROCESSOR_EVENT_NAME};
use crate::functions::dispatch_custom_event;

pub enum SceneState
{
    Preprocessor,
    Postprocessor,
}


pub struct Postprocessor {}


impl Postprocessor
{
    pub fn create() -> Self
    {
        Postprocessor {}
    }
}


pub struct GlobalScene
{
    scene_state: SceneState,
    preprocessor: Preprocessor,
    optional_postprocessor: Option<(u32, Postprocessor)>,
}


impl GlobalScene
{
    pub fn initialize_preprocessor_state() -> Self
    {
        let preprocessor = Preprocessor::create();
        GlobalScene { scene_state: SceneState::Preprocessor, preprocessor,
            optional_postprocessor: None }
    }


    pub fn activate_preprocessor_state(&mut self)
    {
        match self.scene_state
        {
            SceneState::Postprocessor =>
                {
                    self.scene_state = SceneState::Preprocessor;
                },
            SceneState::Preprocessor => ()
        }
    }


    pub fn activate_postprocessor_state(&mut self, postprocessor_id: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    if let Some((id, postprocessor)) =
                        self.optional_postprocessor.as_mut()
                    {
                        if *id != postprocessor_id
                        {
                            *id = postprocessor_id;
                            *postprocessor = Postprocessor::create();
                        }
                    }
                    else
                    {
                        self.optional_postprocessor =
                            Some((postprocessor_id, Postprocessor::create()));
                        let detail = json!("_data");
                        dispatch_custom_event(detail,
                            EXTRACT_DATA_FOR_POSTPROCESSOR_EVENT_NAME,
                            EVENT_TARGET)?;
                    }
                    self.scene_state = SceneState::Postprocessor;
                    Ok(())
                },
            SceneState::Postprocessor => Ok(())
        }
    }


    pub fn update_scene_for_selection(&mut self, under_selection_box_colors: &Vec<u8>,
        selected_colors: &HashSet<[u8; 4]>, d_scale: f32, is_geometry_visible: &bool,
        is_load_visible: &bool, is_boundary_condition_visible: &bool, is_mesh_visible: &bool)
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.update_scene_for_selection(under_selection_box_colors,
                        selected_colors, d_scale, is_geometry_visible, is_load_visible,
                        is_boundary_condition_visible, is_mesh_visible)
                },
            _ => Ok(())
        }
    }


    pub fn update_scene_visible(&mut self, under_selection_box_colors: &Vec<u8>,
        selected_colors: &HashSet<[u8; 4]>, d_scale: f32, is_geometry_visible: &bool,
        is_load_visible: &bool, is_boundary_condition_visible: &bool, is_mesh_visible: &bool)
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.update_scene_visible(under_selection_box_colors,
                        selected_colors, d_scale, is_geometry_visible, is_load_visible,
                        is_boundary_condition_visible, is_mesh_visible)
                },
            _ => Ok(())
        }
    }


    pub fn select_objects(&mut self, selected_colors: &HashSet<[u8; 4]>,
        drop_selection: &js_sys::Function) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.select_objects(selected_colors, drop_selection)
                },
            _ => Ok(())
        }
    }


     pub fn preview_selected_line_objects(&mut self, selected_line_object_numbers: JsValue,
        line_object_type: LineObjectType, selected_colors: &mut HashSet<[u8; 4]>)
         -> Result<(), JsValue>
     {
         match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.preview_selected_line_objects(selected_line_object_numbers,
                        line_object_type, selected_colors)
                },
            _ => Ok(())
        }
     }


    pub fn preview_beam_section_orientation(&mut self, beam_section_orientation: JsValue,
        line_object_type: LineObjectType, selected_colors: &mut HashSet<[u8; 4]>)
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.preview_beam_section_orientation(beam_section_orientation,
                        line_object_type, selected_colors)
                },
            _ => Ok(())
        }
    }


    pub fn add_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType, canvas_width: f32, canvas_height: f32)
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.add_point_object(number, x, y, z, point_object_type,
                        canvas_width, canvas_height)
                }
            _ => Ok(())
        }
    }


    pub fn update_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType, canvas_width: f32, canvas_height: f32)
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.update_point_object(number, x, y, z, point_object_type,
                        canvas_width, canvas_height)
                },
            _ => Ok(())
        }
    }


    pub fn delete_point_object(&mut self, number: u32, point_object_type: PointObjectType,
        canvas_width: f32, canvas_height: f32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.delete_point_object(number, point_object_type, canvas_width,
                        canvas_height)
                },
            _ => Ok(())
        }
    }


    pub fn add_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.add_line_object(number, start_point_object_number,
                        end_point_object_number, line_object_type)
                },
            _ => Ok(())
        }
    }


    pub fn update_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.update_line_object(number, start_point_object_number,
                        end_point_object_number, line_object_type)
                },
            _ => Ok(())
        }
    }


    pub fn delete_line_object(&mut self, number: u32, line_object_type: LineObjectType)
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.delete_line_object(number, line_object_type)
                },
            _ => Ok(())
        }
    }


    pub fn update_line_objects_color_scheme(&mut self, line_object_numbers: &[u32],
        line_object_type: LineObjectType, line_object_color_scheme: LineObjectColorScheme)
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.update_line_objects_color_scheme(line_object_numbers,
                        line_object_type, line_object_color_scheme)
                },
            _ => Ok(())
        }
    }


    pub fn add_concentrated_load(&mut self, point_number: u32, fx: f32, fy: f32, fz: f32,
        mx: f32, my: f32, mz: f32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.add_concentrated_load(point_number, fx, fy, fz, mx, my, mz)
                },
            _ => Ok(())
        }
    }


    pub fn update_concentrated_load(&mut self, point_number: u32, fx: f32, fy: f32, fz: f32,
        mx: f32, my: f32, mz: f32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.update_concentrated_load(point_number, fx, fy, fz, mx, my, mz)
                },
            _ => Ok(())
        }
    }


    pub fn delete_concentrated_load(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.delete_concentrated_load(point_number)
                },
            _ => Ok(())
        }
    }


    pub fn add_distributed_line_load(&mut self, line_number: u32, qx: f32, qy: f32, qz: f32)
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.add_distributed_line_load(line_number, qx, qy, qz)
                },
            _ => Ok(())
        }
    }


    pub fn update_distributed_line_load(&mut self, line_number: u32, qx: f32, qy: f32, qz: f32)
        -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.add_distributed_line_load(line_number, qx, qy, qz)
                },
            _ => Ok(())
        }
    }


    pub fn delete_distributed_line_load(&mut self, line_number: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.delete_distributed_line_load(line_number)
                },
            _ => Ok(())
        }
    }


    pub fn add_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.add_boundary_condition(point_number)
                },
            _ => Ok(())
        }
    }


    pub fn update_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.update_boundary_condition(point_number)
                },
            _ => Ok(())
        }
    }


    pub fn delete_boundary_condition(&mut self, point_number: u32) -> Result<(), JsValue>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.delete_boundary_condition(point_number)
                },
            _ => Ok(())
        }
    }


    pub fn ref_optional_scene_for_selection(&self) -> &Option<SceneAdapter>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.ref_optional_scene_for_selection()
                },
            _ => &None
        }
    }


    pub fn ref_optional_scene_visible(&self) -> &Option<SceneAdapter>
    {
        match self.scene_state
        {
            SceneState::Preprocessor =>
                {
                    self.preprocessor.ref_optional_scene_visible()
                }
            _ => &None
        }
    }


    pub fn ref_state(&self) -> &SceneState
    {
        &self.scene_state
    }


    pub fn ref_preprocessor_point_objects(&self) -> &HashMap<PointObjectKey, PointObject>
    {
        &self.preprocessor.ref_point_objects()
    }


    pub fn ref_preprocessor_line_objects(&self) -> &HashMap<LineObjectKey, LineObject>
    {
        &self.preprocessor.ref_line_objects()
    }
}
