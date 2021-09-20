use wasm_bindgen::prelude::*;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use serde_json::json;

use crate::drawn_object::scene_adapter::SceneAdapter;
use crate::drawn_object::scene_adapter::GLMode;
use crate::drawn_object::consts::
{
    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER, DRAWN_LINE_OBJECTS_BASE_RADIUS,
    DRAWN_CONCENTRATED_LOADS_LINE_LENGTH, DRAWN_CONCENTRATED_LOADS_CAPS_HEIGHT,
    DRAWN_CONCENTRATED_LOADS_CAPS_WIDTH, DRAWN_DISTRIBUTED_LINE_LOADS_LINE_LENGTH,
    DRAWN_DISTRIBUTED_LINE_LOADS_CAPS_HEIGHT, DRAWN_DISTRIBUTED_LINE_LOADS_CAPS_WIDTH,
    DRAWN_BOUNDARY_CONDITION_CAPS_BASE_POINTS_NUMBER, DRAWN_BOUNDARY_CONDITION_CAPS_HEIGHT,
    DRAWN_BOUNDARY_CONDITION_CAPS_WIDTH, DRAWN_BEAM_SECTION_ORIENTATION_LINE_LENGTH,
    DRAWN_BEAM_SECTION_ORIENTATION_CAPS_BASE_POINTS_NUMBER,
    DRAWN_BEAM_SECTION_ORIENTATION_CAPS_HEIGHT, DRAWN_BEAM_SECTION_ORIENTATION_CAPS_WIDTH
};

use crate::global_scene::point_object::{PointObjectKey, PointObject};
use crate::global_scene::point_object::{PointObjectType};
use crate::global_scene::line_object::
{
    LineObjectKey, LineObject, BeamSectionOrientation, LineObjectNumbers
};
use crate::global_scene::line_object::{LineObjectType};
use crate::global_scene::preprocessor::concentrated_load::ConcentratedLoad;
use crate::global_scene::preprocessor::distributed_line_load::DistributedLineLoad;
use crate::global_scene::preprocessor::boundary_condition::BoundaryCondition;

use crate::consts::
{
    EVENT_TARGET, SELECTED_POINTS_EVENT_MAME, SELECTED_NODES_EVENT_MAME, SELECTED_LINES_EVENT_MAME,
    SELECTED_LINE_ELEMENTS_EVENT_MAME, SELECTED_CONCENTRATED_LOADS_POINTS_NUMBERS_EVENT_MAME,
    SELECTED_DISTRIBUTED_LINE_LOADS_LINES_NUMBERS_EVENT_MAME,
    SELECTED_BOUNDARY_CONDITIONS_POINTS_NUMBERS_EVENT_MAME,
};
use crate::functions::{dispatch_custom_event, transform_u32_to_array_of_u8};


pub struct Preprocessor
{
    optional_scene_for_selection: Option<SceneAdapter>,
    optional_scene_visible: Option<SceneAdapter>,
    pub point_objects: HashMap<PointObjectKey, PointObject>,
    pub line_objects: HashMap<LineObjectKey, LineObject>,
    beam_section_orientation_for_preview: Option<BeamSectionOrientation>,
    pub concentrated_loads: HashMap<u32, ConcentratedLoad>,
    pub distributed_line_loads: HashMap<u32, DistributedLineLoad>,
    pub boundary_conditions: HashMap<u32, BoundaryCondition>,
}


impl Preprocessor
{
    pub fn create() -> Self
    {
        Preprocessor
            {
                optional_scene_for_selection: None,
                optional_scene_visible: None,
                point_objects: HashMap::new(),
                line_objects: HashMap::new(),
                beam_section_orientation_for_preview: None,
                concentrated_loads: HashMap::new(),
                distributed_line_loads: HashMap::new(),
                boundary_conditions: HashMap::new(),
            }
    }


    pub fn update_scene_for_selection(&mut self, under_selection_box_colors: &Vec<u8>,
        selected_colors: &HashSet<[u8; 4]>, d_scale: f32, is_geometry_visible: &bool,
        is_load_visible: &bool, is_boundary_condition_visible: &bool, is_mesh_visible: &bool)
        -> Result<(), JsValue>
    {
        if !self.point_objects.is_empty()
        {
            let mut scene_for_selection = SceneAdapter::create();
            scene_for_selection.add_point_objects(
                &self.point_objects,
                GLMode::Selection,
                under_selection_box_colors,
                selected_colors,
                is_geometry_visible,
                is_mesh_visible)?;
            if !self.line_objects.is_empty()
            {
                scene_for_selection.add_line_objects(
                    &self.point_objects,
                    &self.line_objects,
                    GLMode::Selection,
                    under_selection_box_colors,
                    selected_colors,
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_LINE_OBJECTS_BASE_RADIUS / (1.0 + d_scale),
                    is_geometry_visible,
                    is_mesh_visible)?;
            }
            if !self.concentrated_loads.is_empty()
            {
                scene_for_selection.add_concentrated_loads(
                    &self.point_objects,
                    &mut self.concentrated_loads,
                    GLMode::Selection,
                    under_selection_box_colors,
                    selected_colors,
                    DRAWN_CONCENTRATED_LOADS_LINE_LENGTH / (1.0 + d_scale),
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_CONCENTRATED_LOADS_CAPS_HEIGHT / (1.0 + d_scale),
                    DRAWN_CONCENTRATED_LOADS_CAPS_WIDTH / (1.0 + d_scale),
                    is_load_visible)?;
            }
            if !self.distributed_line_loads.is_empty()
            {
                scene_for_selection.add_distributed_line_loads(
                    &self.point_objects,
                    &self.line_objects,
                    &mut self.distributed_line_loads,
                    GLMode::Selection,
                    under_selection_box_colors,
                    selected_colors,
                    DRAWN_DISTRIBUTED_LINE_LOADS_LINE_LENGTH / (1.0 + d_scale),
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_DISTRIBUTED_LINE_LOADS_CAPS_HEIGHT / (1.0 + d_scale),
                    DRAWN_DISTRIBUTED_LINE_LOADS_CAPS_WIDTH / (1.0 + d_scale),
                    is_load_visible)?;
            }
            if !self.boundary_conditions.is_empty()
            {
                scene_for_selection.add_boundary_conditions(&self.point_objects,
                    &self.boundary_conditions, GLMode::Selection,
                    under_selection_box_colors, selected_colors,
                    DRAWN_BOUNDARY_CONDITION_CAPS_BASE_POINTS_NUMBER,
                    DRAWN_BOUNDARY_CONDITION_CAPS_HEIGHT / (1.0 + d_scale),
                    DRAWN_BOUNDARY_CONDITION_CAPS_WIDTH / (1.0 + d_scale),
                    is_boundary_condition_visible)?;
            }
            self.optional_scene_for_selection = Some(scene_for_selection);
        }
        else
        {
            self.optional_scene_for_selection = None;
        }
        Ok(())
    }


    pub fn update_scene_visible(&mut self, under_selection_box_colors: &Vec<u8>,
        selected_colors: &HashSet<[u8; 4]>, d_scale: f32, is_geometry_visible: &bool,
        is_load_visible: &bool, is_boundary_condition_visible: &bool, is_mesh_visible: &bool)
        -> Result<(), JsValue>
    {
        if !self.point_objects.is_empty()
        {
            let mut scene_visible = SceneAdapter::create();
            scene_visible.add_point_objects(
                &self.point_objects,
                GLMode::Visible,
                under_selection_box_colors,
                selected_colors,
                is_geometry_visible,
                is_mesh_visible)?;
            if !self.line_objects.is_empty()
            {
                scene_visible.add_line_objects(
                    &self.point_objects,
                    &self.line_objects,
                    GLMode::Visible,
                    under_selection_box_colors,
                    selected_colors,
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_LINE_OBJECTS_BASE_RADIUS / (1.0 + d_scale),
                    is_geometry_visible,
                    is_mesh_visible)?;
                if let Some(beam_section_orientation) =
                    &self.beam_section_orientation_for_preview
                {
                    scene_visible.add_beam_section_orientation_for_preview(
                        &self.point_objects,
                        &self.line_objects,
                        beam_section_orientation,
                        DRAWN_BEAM_SECTION_ORIENTATION_LINE_LENGTH / (1.0 + d_scale),
                        DRAWN_BEAM_SECTION_ORIENTATION_CAPS_BASE_POINTS_NUMBER,
                        DRAWN_BEAM_SECTION_ORIENTATION_CAPS_HEIGHT / (1.0 + d_scale),
                        DRAWN_BEAM_SECTION_ORIENTATION_CAPS_WIDTH / (1.0 + d_scale),
                    )?;
                }
            }
            if !self.concentrated_loads.is_empty()
            {
                scene_visible.add_concentrated_loads(
                    &self.point_objects,
                    &self.concentrated_loads,
                    GLMode::Visible,
                    under_selection_box_colors,
                    selected_colors,
                    DRAWN_CONCENTRATED_LOADS_LINE_LENGTH / (1.0 + d_scale),
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_CONCENTRATED_LOADS_CAPS_HEIGHT / (1.0 + d_scale),
                    DRAWN_CONCENTRATED_LOADS_CAPS_WIDTH / (1.0 + d_scale),
                    is_load_visible)?;
            }
            if !self.distributed_line_loads.is_empty()
            {
                scene_visible.add_distributed_line_loads(
                    &self.point_objects,
                    &self.line_objects,
                    &mut self.distributed_line_loads,
                    GLMode::Visible,
                    under_selection_box_colors,
                    selected_colors,
                    DRAWN_DISTRIBUTED_LINE_LOADS_LINE_LENGTH / (1.0 + d_scale),
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_DISTRIBUTED_LINE_LOADS_CAPS_HEIGHT / (1.0 + d_scale),
                    DRAWN_DISTRIBUTED_LINE_LOADS_CAPS_WIDTH / (1.0 + d_scale),
                    is_load_visible)?;
            }
            if !self.boundary_conditions.is_empty()
            {
                scene_visible.add_boundary_conditions(&self.point_objects,
                    &self.boundary_conditions, GLMode::Visible,
                    under_selection_box_colors, selected_colors,
                    DRAWN_BOUNDARY_CONDITION_CAPS_BASE_POINTS_NUMBER,
                    DRAWN_BOUNDARY_CONDITION_CAPS_HEIGHT / (1.0 + d_scale),
                    DRAWN_BOUNDARY_CONDITION_CAPS_WIDTH / (1.0 + d_scale),
                    is_boundary_condition_visible)?;
            }
            self.optional_scene_visible = Some(scene_visible);
        }
        else
        {
            self.optional_scene_visible = None;
        }
        Ok(())
    }


    pub fn select_objects(&mut self, selected_colors: &HashSet<[u8; 4]>,
        drop_selection: &js_sys::Function) -> Result<(), JsValue>
    {
        let mut selected_point_numbers = Vec::new();
        let mut selected_node_numbers = Vec::new();
        let mut selected_line_numbers = Vec::new();
        let mut selected_line_element_numbers = Vec::new();
        let mut selected_concentrated_loads_points_numbers = Vec::new();
        let mut selected_distributed_line_loads_lines_numbers = Vec::new();
        let mut selected_boundary_conditions_points_numbers = Vec::new();
        let mut is_object_selected = false;
        for selected_color in selected_colors.iter()
        {
            for (point_object_key, point_object) in
                self.point_objects.iter()
            {
                if point_object.is_uid_same(u32::from_be_bytes(*selected_color))
                {
                    let selected_point_object_number = point_object_key.copy_number();
                    let selected_point_object_type =
                        point_object_key.copy_object_type();
                    match selected_point_object_type
                    {
                        PointObjectType::Point =>
                                selected_point_numbers.push(selected_point_object_number),
                        PointObjectType::Node =>
                            selected_node_numbers.push(selected_point_object_number),
                    }
                }
            }

            for (line_object_key, line_object) in
                self.line_objects.iter()
            {
                if line_object.is_uid_same(u32::from_be_bytes(*selected_color))
                {
                    let selected_line_object_number = line_object_key.get_number();
                    let selected_line_object_type = line_object_key.get_object_type();
                    match selected_line_object_type
                    {
                        LineObjectType::Line =>
                            selected_line_numbers.push(selected_line_object_number),
                        LineObjectType::Element =>
                            selected_line_element_numbers.push(selected_line_object_number),
                    }
                }
            }

            for (point_number, concentrated_load) in
                self.concentrated_loads.iter()
            {
                if concentrated_load.is_uid_same(u32::from_be_bytes(*selected_color))
                {
                    selected_concentrated_loads_points_numbers.push(point_number);
                }
            }

            for (line_number, distributed_line_load) in
                self.distributed_line_loads.iter()
            {
                if distributed_line_load.is_uid_same(u32::from_be_bytes(*selected_color))
                {
                    selected_distributed_line_loads_lines_numbers.push(line_number);
                }
            }

            for (point_number, boundary_condition) in
                self.boundary_conditions.iter()
            {
                if boundary_condition.is_uid_same(u32::from_be_bytes(*selected_color))
                {
                    selected_boundary_conditions_points_numbers.push(point_number);
                }
            }
        }

        if !selected_point_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({ "point_numbers": selected_point_numbers });
            dispatch_custom_event(detail, SELECTED_POINTS_EVENT_MAME,
                EVENT_TARGET)?;
        }
        else if !selected_node_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({ "node_numbers": selected_node_numbers });
            dispatch_custom_event(detail, SELECTED_NODES_EVENT_MAME,
                EVENT_TARGET)?;
        }
        else if !selected_line_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({ "line_numbers": selected_line_numbers });
            dispatch_custom_event(detail, SELECTED_LINES_EVENT_MAME,
                EVENT_TARGET)?;
        }
        else if !selected_line_element_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({ "line_element_numbers": selected_line_element_numbers });
            dispatch_custom_event(detail, SELECTED_LINE_ELEMENTS_EVENT_MAME,
                EVENT_TARGET)?;
        }
        else if !selected_concentrated_loads_points_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({
                "concentrated_loads_points_numbers": selected_concentrated_loads_points_numbers });
            dispatch_custom_event(detail,
                SELECTED_CONCENTRATED_LOADS_POINTS_NUMBERS_EVENT_MAME,
                EVENT_TARGET)?;
        }
        else if !selected_distributed_line_loads_lines_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({
                "distributed_line_loads_lines_numbers":
                selected_distributed_line_loads_lines_numbers });
            dispatch_custom_event(detail,
                SELECTED_DISTRIBUTED_LINE_LOADS_LINES_NUMBERS_EVENT_MAME,
                EVENT_TARGET)?;
        }
        else if !selected_boundary_conditions_points_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({
                "boundary_conditions_points_numbers": selected_boundary_conditions_points_numbers });
            dispatch_custom_event(detail,
                SELECTED_BOUNDARY_CONDITIONS_POINTS_NUMBERS_EVENT_MAME,
                EVENT_TARGET)?;
        }
        else
        {
            is_object_selected = false;
        }

        self.beam_section_orientation_for_preview = None;
        if is_object_selected
        {
            Ok(())
        }
        else
        {
            let this = JsValue::null();
            let _ = drop_selection.call0(&this);

            Ok(())
        }
    }


    pub fn preview_selected_line_objects(&mut self, selected_line_object_numbers: JsValue,
        line_object_type: LineObjectType, selected_colors: &mut HashSet<[u8; 4]>)
        -> Result<(), JsValue>
    {
        selected_colors.clear();
        for line_object_number in selected_line_object_numbers
            .into_serde::<LineObjectNumbers>()
            .or(Err(JsValue::from("Renderer: Preview selected line object numbers action: \
                Line object numbers could not be serialized!")))?
            .ref_line_numbers()
        {
            let current_line_object_key =
                LineObjectKey::create(*line_object_number, line_object_type);
            if let Some(line_object) =
                self.line_objects.get(&current_line_object_key)
            {
                let current_uid = line_object.copy_uid();
                let current_color = transform_u32_to_array_of_u8(current_uid);
                selected_colors.insert(current_color);
            }
            else
            {
                let error_message = format!("Renderer: Preview selected line objects \
                    action: {} with number {} does not exist!",
                    line_object_type.as_str(), line_object_number);
                return Err(JsValue::from(error_message));
            }
        }
        Ok(())
    }


    pub fn preview_beam_section_orientation(&mut self, beam_section_orientation: JsValue,
        line_object_type: LineObjectType, selected_colors: &mut HashSet<[u8; 4]>)
        -> Result<(), JsValue>
    {
        selected_colors.clear();
        let beam_section_orientation_for_preview =
            beam_section_orientation
                .into_serde::<BeamSectionOrientation>()
                .or(Err(JsValue::from("Renderer: Preview beam section orientation action: \
                    Beam section orientation could not be serialized!")))?;
        for line_object_number in beam_section_orientation_for_preview.ref_line_numbers()
        {
            let current_line_object_key =
                LineObjectKey::create(*line_object_number, line_object_type);
            if let Some(line_object) =
                self.line_objects.get(&current_line_object_key)
            {
                let current_uid = line_object.copy_uid();
                let current_color = transform_u32_to_array_of_u8(current_uid);
                selected_colors.insert(current_color);
            }
            else
            {
                let error_message = format!("Renderer: Preview beam section orientation \
                    action: {} with number {} does not exist!",
                    line_object_type.as_str(), line_object_number);
                return Err(JsValue::from(error_message));
            }
        }
        self.beam_section_orientation_for_preview = Some(beam_section_orientation_for_preview);
        Ok(())
    }


    pub fn ref_optional_scene_for_selection(&self) -> &Option<SceneAdapter>
    {
        &self.optional_scene_for_selection
    }


    pub fn ref_optional_scene_visible(&self) -> &Option<SceneAdapter>
    {
        &self.optional_scene_visible
    }
}
