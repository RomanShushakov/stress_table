use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext as GL};
use std::f32::consts::PI;
use std::collections::{HashMap, HashSet};

use extended_matrix::extended_matrix::ExtendedMatrix;
use extended_matrix::basic_matrix::basic_matrix::MatrixElementPosition;
use extended_matrix::functions::copy_element_value;

use crate::global_scene::point_object::{PointObjectKey, PointObject};
use crate::global_scene::point_object::{PointObjectType};
use crate::global_scene::line_object::{LineObject, LineObjectKey, BeamSectionOrientation};
use crate::global_scene::line_object::{LineObjectType, LineObjectColorScheme};
use crate::global_scene::preprocessor::concentrated_load::{ConcentratedLoad, Sign, CSAxis};
use crate::global_scene::preprocessor::distributed_line_load::DistributedLineLoad;
use crate::global_scene::preprocessor::boundary_condition::BoundaryCondition;

use crate::drawn_object::drawn_object::DrawnObject;
use crate::drawn_object::functions::{build_monochrome_cylinder_around_line, build_monochrome_cone};
use crate::drawn_object::consts::
{
    DRAWN_POINTS_COLOR, DRAWN_NODES_COLOR, DRAWN_LINES_DEFAULT_COLOR, DRAWN_LINES_TRUSS_PROPS_COLOR,
    DRAWN_LINES_BEAM_PROPS_COLOR, DRAWN_ELEMENTS_COLOR, DRAWN_BEAM_SECTION_ORIENTATION_COLOR,
    DRAWN_CONCENTRATED_LOADS_COLOR, DRAWN_DISTRIBUTED_LINE_LOADS_COLOR,
    NUMBER_OF_DISTRIBUTED_LINE_LOAD_ARROWS, DRAWN_BOUNDARY_CONDITION_COLOR,
};

use crate::consts::TOLERANCE;

use crate::functions::
{
    define_drawn_object_color, compose_rotation_matrix_for_vector, calculate_line_length
};


// pub const CANVAS_BACKGROUND_COLOR: &str = "black";

pub const DRAWN_DISPLACEMENTS_COLOR: [f32; 4] = [1.0, 0.5, 0.0, 1.0]; // orange
pub const CANVAS_DRAWN_DISPLACEMENTS_DENOTATION_COLOR: &str = "orange";

pub const DRAWN_DISPLACEMENTS_CAPS_HEIGHT: f32 = 0.015; // arrow length
pub const DRAWN_DISPLACEMENTS_CAPS_WIDTH: f32 = 0.007; // half of arrow width
pub const DRAWN_DISPLACEMENTS_CAPS_BASE_POINTS_NUMBER: u16 = 12; // the number of points in cone circular base

pub const DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_X: f32 = 0.01;
pub const DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_Y: f32 = 0.015;

pub const DRAWN_FORCES_COLOR: [f32; 4] = [1.0, 0.0, 1.0, 1.0]; // magenta
pub const CANVAS_DRAWN_FORCES_DENOTATION_COLOR: &str = "magenta";

pub const HINT_SHIFT_X: f32 = 0.05;
pub const ROTATION_HINT_SHIFT_Y: f32 = 0.85;
pub const ZOOM_HINT_SHIFT_Y: f32 = 0.9;
pub const PAN_HINT_SHIFT_Y: f32 = 0.95;

pub const DRAWN_DEFORMED_SHAPE_NODES_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; // white
pub const DRAWN_DEFORMED_SHAPE_ELEMENTS_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; // white
pub const CANVAS_DRAWN_DEFORMED_SHAPE_NODES_DENOTATION_COLOR: &str = "white";
pub const DRAWN_DEFORMED_SHAPE_NODES_DENOTATION_SHIFT: f32 = 0.02;

pub const DRAWN_OBJECT_SELECTED_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // red
pub const CANVAS_DRAWN_OBJECT_SELECTED_DENOTATION_COLOR: &str = "red";
pub const DRAWN_OBJECT_UNDER_CURSOR_COLOR: [f32; 4] =
    [0.752941, 0.752941, 0.752941, 1.0]; // grey
pub const CANVAS_DRAWN_OBJECT_UNDER_CURSOR_DENOTATION_COLOR: &str = "grey";

pub const DISPLACEMENT_SHIFT_X: f32 = 0.05;
pub const DISPLACEMENT_HEADER_SHIFT_Y: f32 = 0.1;
pub const MIN_DISPLACEMENT_SHIFT_Y: f32 = 0.15;
pub const MAX_DISPLACEMENT_SHIFT_Y: f32 = 0.2;

pub const REACTION_SHIFT_X: f32 = 0.05;
pub const REACTION_HEADER_SHIFT_Y: f32 = 0.1;

pub const DRAWN_OBJECT_DEFAULT_RESULT_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0]; // blue

pub const EAR_SHIFT_X: f32 = 0.05;
pub const EAR_HEADER_SHIFT_Y: f32 = 0.1;
pub const EAR_COMPONENT_SHIFT_Y: f32 = 0.15;
pub const EAR_MIN_MAX_VALUE_SHIFT_X: f32 = 0.07;

pub const COLOR_BAR_SHIFT_X: f32 = 0.05;
pub const COLOR_BAR_Y_BOTTOM: f32 = 0.45;
pub const COLOR_BAR_Y_TOP: f32 = 0.2;
pub const COLOR_BAR_WIDTH: f32 = 0.015;


pub enum GLMode
{
    Selection,
    Visible,
}


pub struct SceneAdapter
{
    drawn_object: DrawnObject,
}


impl SceneAdapter
{
    pub fn create() -> Self
    {
        let points_coordinates = Vec::new();
        let points_colors_values = Vec::new();
        let lines_endpoints_coordinates = Vec::new();
        let lines_endpoints_colors_values = Vec::new();
        let triangles_vertices_coordinates= Vec::new();
        let triangles_vertices_colors_values = Vec::new();
        let triangles_vertices_indexes = Vec::new();

        let drawn_object = DrawnObject::create(
            Some(points_coordinates),
            Some(points_colors_values),
            Some(lines_endpoints_coordinates),
            Some(lines_endpoints_colors_values),
            Some(triangles_vertices_coordinates),
            Some(triangles_vertices_colors_values),
            Some(triangles_vertices_indexes));

        SceneAdapter { drawn_object }
    }


    pub fn add_point_objects(&mut self, point_objects: &HashMap<PointObjectKey, PointObject>,
        gl_mode: GLMode, under_selection_box_colors: &Vec<u8>, selected_colors: &HashSet<[u8; 4]>,
        is_geometry_visible: &bool, is_mesh_visible: &bool) -> Result<(), JsValue>
    {
        if !*is_geometry_visible && !*is_mesh_visible
        {
            return Ok(());
        }

        for (point_object_key, point_object) in point_objects.iter()
        {
            if !*is_geometry_visible && point_object_key.copy_object_type() == PointObjectType::Point
            {
                continue;
            }

            if !*is_mesh_visible && point_object_key.copy_object_type() == PointObjectType::Node
            {
                continue;
            }

            self.drawn_object.add_point_coordinates(&[
                point_object.copy_normalized_x()?,
                point_object.copy_normalized_y()?,
                point_object.copy_normalized_z()?
            ])?;

            let initial_color = match point_object_key.copy_object_type()
                {
                    PointObjectType::Point => DRAWN_POINTS_COLOR,
                    PointObjectType::Node => DRAWN_NODES_COLOR,
                };

            let point_object_color = define_drawn_object_color(
                &gl_mode, point_object.copy_uid()?,
                selected_colors, under_selection_box_colors, &initial_color);
            self.drawn_object.add_point_color_value(&point_object_color)?;
        }

        Ok(())
    }


    pub fn add_line_objects(&mut self,
        point_objects: &HashMap<PointObjectKey, PointObject>,
        line_objects: &HashMap<LineObjectKey, LineObject>,
        gl_mode: GLMode, under_selection_box_colors: &Vec<u8>,
        selected_colors: &HashSet<[u8; 4]>, base_points_number: u32,
        base_radius: f32, is_geometry_visible: &bool, is_mesh_visible: &bool) -> Result<(), JsValue>
    {
        if !*is_geometry_visible && !*is_mesh_visible
        {
            return Ok(());
        }

        for (line_object_key, line_object) in line_objects.iter()
        {
            if !*is_geometry_visible && line_object_key.get_object_type() == LineObjectType::Line
            {
                continue;
            }

            if !*is_mesh_visible && line_object_key.get_object_type() == LineObjectType::Element
            {
                continue;
            }

            let initial_color = match line_object_key.get_object_type()
                {
                    LineObjectType::Line =>
                        {
                            match line_object.copy_color_scheme()
                            {
                                LineObjectColorScheme::Default => DRAWN_LINES_DEFAULT_COLOR,
                                LineObjectColorScheme::TrussProps => DRAWN_LINES_TRUSS_PROPS_COLOR,
                                LineObjectColorScheme::BeamProps => DRAWN_LINES_BEAM_PROPS_COLOR,
                            }
                        },
                    LineObjectType::Element => DRAWN_ELEMENTS_COLOR,
                };

            let line_object_color = define_drawn_object_color(&gl_mode,
                line_object.copy_uid(), selected_colors, under_selection_box_colors,
                &initial_color);
            let start_point_object_coordinates =
                line_object.copy_start_point_object_coordinates(point_objects)?;
            let end_point_object_coordinates =
                line_object.copy_end_point_object_coordinates(point_objects)?;
            match gl_mode
            {
                GLMode::Selection =>
                    {
                        let start_index =
                            if self.ref_triangles_vertices_indexes()?.is_empty()
                            {
                                0
                            }
                            else
                            {
                                self.ref_triangles_vertices_indexes()?[
                                    self.ref_triangles_vertices_indexes()?.len() - 1] + 1
                            };
                        let (triangles_vertices_coordinates,
                            triangles_vertices_colors_values,
                            triangles_vertices_indexes) =
                            build_monochrome_cylinder_around_line(
                                &start_point_object_coordinates,
                                &end_point_object_coordinates,
                                Some(base_radius * 2.0),
                                base_radius, base_points_number, start_index,
                                &line_object_color, TOLERANCE)?;

                        self.drawn_object.add_triangle_vertex_coordinates(
                            &triangles_vertices_coordinates)?;
                        self.drawn_object.add_triangle_vertex_color_value(
                            &triangles_vertices_colors_values)?;
                        self.drawn_object.add_triangles_vertices_indexes(
                            &triangles_vertices_indexes)?;
                    },
                GLMode::Visible =>
                    {
                        self.drawn_object.add_line_endpoint_coordinates(
                            &start_point_object_coordinates)?;
                        self.drawn_object.add_line_endpoint_coordinates(
                            &end_point_object_coordinates)?;
                        self.drawn_object.add_line_endpoint_color_value(
                            &line_object_color)?;
                        self.drawn_object.add_line_endpoint_color_value(
                            &line_object_color)?;
                    }
            }
        }

        Ok(())
    }


    pub fn add_beam_section_orientation_for_preview(&mut self,
        point_objects: &HashMap<PointObjectKey, PointObject>,
        line_objects: &HashMap<LineObjectKey, LineObject>,
        beam_section_orientation: &BeamSectionOrientation,
        line_length: f32,
        base_points_number: u32,
        height: f32,
        base_radius: f32) -> Result<(), JsValue>
    {
        let local_axis_1_direction =
            beam_section_orientation.copy_local_axis_1_direction();
        for line_number in beam_section_orientation.ref_line_numbers()
        {
            let line_object_key = LineObjectKey::create(*line_number,
                LineObjectType::Line);
            if let Some(line_object) = line_objects.get(&line_object_key)
            {
                let a_x = - local_axis_1_direction[0];
                let a_y = - local_axis_1_direction[1];
                let a_z = - local_axis_1_direction[2];
                let line_start_point_coordinates =
                    line_object.copy_start_point_object_coordinates(point_objects)?;
                let line_end_point_coordinates =
                    line_object.copy_end_point_object_coordinates(point_objects)?;
                let b_x = line_end_point_coordinates[0] - line_start_point_coordinates[0];
                let b_y = line_end_point_coordinates[1] - line_start_point_coordinates[1];
                let b_z = line_end_point_coordinates[2] - line_start_point_coordinates[2];
                let a = ExtendedMatrix::create(3u32,
                    1u32, vec![a_x, a_y, a_z], TOLERANCE);
                let norm = 1f32 / (b_x.powi(2) + b_y.powi(2) + b_z.powi(2));
                let mut coeff_matrix = ExtendedMatrix::create(3u32,
                    3u32, vec![
                        - b_z * b_z - b_y * b_y, b_x * b_y, b_x * b_z,
                        b_y * b_x, - b_x * b_x - b_z * b_z,	b_y * b_z,
                        b_z * b_x,	b_z * b_y, - b_y * b_y - b_x * b_x,
                    ], TOLERANCE);
                coeff_matrix.multiply_by_number(norm);
                let local_axis_1_direction_projection_matrix =
                    coeff_matrix
                        .multiply_by_matrix(&a)
                        .map_err(|e| JsValue::from(e))?;
                let local_axis_1_direction_projection_all_values =
                    local_axis_1_direction_projection_matrix.copy_all_elements_values();
                let local_axis_1_direction_projection_x_coord_value = copy_element_value(
                    0, 0, &local_axis_1_direction_projection_all_values);
                let local_axis_1_direction_projection_y_coord_value = copy_element_value(
                    1, 0, &local_axis_1_direction_projection_all_values);
                let local_axis_1_direction_projection_z_coord_value = copy_element_value(
                    2, 0, &local_axis_1_direction_projection_all_values);

                let local_axis_1_direction_projection_length = calculate_line_length(
                    &[0.0, 0.0, 0.0],
                    &[local_axis_1_direction_projection_x_coord_value,
                        local_axis_1_direction_projection_y_coord_value,
                        local_axis_1_direction_projection_z_coord_value]);
                if local_axis_1_direction_projection_length == 0f32
                {
                    let error_message = format!("Renderer: Add beam section orientation for \
                        preview action: Projection of local axis 1 direction on line number {} \
                        equals to zero!", line_number);
                    return Err(JsValue::from(error_message));
                }

                let line_mid_point_coordinates =
                    [
                        (line_end_point_coordinates[0] + line_start_point_coordinates[0]) / 2.0,
                        (line_end_point_coordinates[1] + line_start_point_coordinates[1]) / 2.0,
                        (line_end_point_coordinates[2] + line_start_point_coordinates[2]) / 2.0,
                    ];
                let updated_local_axis_1_end_point_coordinates =
                    [
                        (line_mid_point_coordinates[0] +
                        local_axis_1_direction_projection_x_coord_value * line_length),
                        (line_mid_point_coordinates[1] +
                        local_axis_1_direction_projection_y_coord_value * line_length),
                        (line_mid_point_coordinates[2] +
                         local_axis_1_direction_projection_z_coord_value * line_length),
                    ];

                let updated_local_axis_1_color = DRAWN_BEAM_SECTION_ORIENTATION_COLOR;

                self.drawn_object.add_line_endpoint_coordinates(
                    &line_mid_point_coordinates)?;
                self.drawn_object.add_line_endpoint_coordinates(
                    &updated_local_axis_1_end_point_coordinates)?;
                self.drawn_object.add_line_endpoint_color_value(
                    &updated_local_axis_1_color)?;
                self.drawn_object.add_line_endpoint_color_value(
                    &updated_local_axis_1_color)?;

                let start_index =
                    if self.ref_triangles_vertices_indexes()?.is_empty()
                    {
                        0
                    }
                    else
                    {
                        self.ref_triangles_vertices_indexes()?[
                            self.ref_triangles_vertices_indexes()?.len() - 1] + 1
                    };

                let (triangles_vertices_coordinates,
                    triangles_vertices_colors_values,
                    triangles_vertices_indexes) = build_monochrome_cone(
                        &updated_local_axis_1_end_point_coordinates,
                        &line_mid_point_coordinates,
                        height, base_radius, base_points_number, start_index,
                        &DRAWN_BEAM_SECTION_ORIENTATION_COLOR, TOLERANCE)?;

                self.drawn_object.add_triangle_vertex_coordinates(
                    &triangles_vertices_coordinates)?;
                self.drawn_object.add_triangle_vertex_color_value(
                    &triangles_vertices_colors_values)?;
                self.drawn_object.add_triangles_vertices_indexes(&triangles_vertices_indexes)?;
            }
            else
            {
                let error_message = format!("Renderer: Add beam section orientation for \
                    preview action: Line number {} does not exist!", line_number);
                return Err(JsValue::from(error_message));
            }
        }
        Ok(())
    }


    fn add_load_line(&mut self, gl_mode: &GLMode, sign: &Sign,
        cs_axis: &CSAxis, start_coordinates: &[f32; 3], line_length: f32,
        base_points_number: u32, base_radius: f32, load_color: &[f32; 4])
        -> Result<(), JsValue>
    {
        let multiplier = match sign { Sign::Positive => 1f32, Sign::Negative => -1f32 };

        let end_coordinates =
            {
                match cs_axis
                {
                    CSAxis::X =>
                        {
                            [start_coordinates[0] + line_length * multiplier,
                            start_coordinates[1],
                            start_coordinates[2]]
                        },
                    CSAxis::Y =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1] + line_length * multiplier,
                            start_coordinates[2]]
                        },
                    CSAxis::Z =>
                        {
                             [start_coordinates[0],
                             start_coordinates[1],
                            start_coordinates[2] + line_length * multiplier]
                        }
                }

            };

        match gl_mode
        {
            GLMode::Selection =>
                {
                    let start_index =
                        if self.ref_triangles_vertices_indexes()?.is_empty()
                        {
                            0
                        }
                        else
                        {
                            self.ref_triangles_vertices_indexes()?[
                                self.ref_triangles_vertices_indexes()?.len() - 1] + 1
                        };
                    let (triangles_vertices_coordinates,
                        triangles_vertices_colors_values,
                        triangles_vertices_indexes) =
                        build_monochrome_cylinder_around_line(
                            start_coordinates,
                            &end_coordinates,
                            Some(base_radius * 4.0),
                            base_radius, base_points_number, start_index,
                            load_color, TOLERANCE)?;

                    self.drawn_object.add_triangle_vertex_coordinates(
                        &triangles_vertices_coordinates)?;
                    self.drawn_object.add_triangle_vertex_color_value(
                        &triangles_vertices_colors_values)?;
                    self.drawn_object.add_triangles_vertices_indexes(
                        &triangles_vertices_indexes)?;
                },
            GLMode::Visible =>
                {
                    self.drawn_object.add_line_endpoint_coordinates(
                        start_coordinates)?;
                    self.drawn_object.add_line_endpoint_coordinates(
                        &end_coordinates)?;
                    self.drawn_object.add_line_endpoint_color_value(
                        load_color)?;
                    self.drawn_object.add_line_endpoint_color_value(
                        load_color)?;
                },
        }
        Ok(())
    }


    fn add_load_cap(&mut self, sign: &Sign, cs_axis: &CSAxis,
        start_coordinates: &[f32; 3], line_length: f32, base_points_number: u32,
        base_radius: f32, height: f32, load_color: &[f32; 4]) -> Result<(), JsValue>
    {
        let multiplier = match sign { Sign::Positive => 1f32, Sign::Negative => -1f32 };

        let cap_vertex_coordinates =
            {
                match cs_axis
                {
                    CSAxis::X =>
                        {
                            [start_coordinates[0] + line_length * multiplier,
                            start_coordinates[1],
                            start_coordinates[2]]
                        },
                    CSAxis::Y =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1] + line_length * multiplier,
                            start_coordinates[2]]
                        },
                    CSAxis::Z =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1],
                            start_coordinates[2] + line_length * multiplier]
                        },
                }
            };

        let start_index =
            if self.ref_triangles_vertices_indexes()?.is_empty()
            {
                0
            }
            else
            {
                self.ref_triangles_vertices_indexes()?[
                    self.ref_triangles_vertices_indexes()?.len() - 1] + 1
            };

        let (triangles_vertices_coordinates,
            triangles_vertices_colors_values,
            triangles_vertices_indexes) = build_monochrome_cone(
            &cap_vertex_coordinates,
            &start_coordinates,
            height, base_radius, base_points_number, start_index,
            &load_color, TOLERANCE)?;

        self.drawn_object.add_triangle_vertex_coordinates(
            &triangles_vertices_coordinates)?;
        self.drawn_object.add_triangle_vertex_color_value(
            &triangles_vertices_colors_values)?;
        self.drawn_object.add_triangles_vertices_indexes(&triangles_vertices_indexes)?;

        Ok(())
    }


    pub fn add_concentrated_loads(&mut self, point_objects: &HashMap<PointObjectKey, PointObject>,
        concentrated_loads: &HashMap<u32, ConcentratedLoad>, gl_mode: GLMode,
        under_selection_box_colors: &Vec<u8>, selected_colors: &HashSet<[u8; 4]>,
        line_length: f32, base_points_number: u32, height: f32, base_radius: f32,
        is_load_visible: &bool) -> Result<(), JsValue>
    {
        if !*is_load_visible
        {
            return Ok(())
        }

        for (point_number, concentrated_load) in
            concentrated_loads.iter()
        {
            let initial_color = DRAWN_CONCENTRATED_LOADS_COLOR;
            let concentrated_load_color = define_drawn_object_color(&gl_mode,
                concentrated_load.copy_uid(), selected_colors, under_selection_box_colors,
                &initial_color);

            let point_object_key = PointObjectKey::create(*point_number,
                PointObjectType::Point);
            if let Some(point_object) = point_objects.get(&point_object_key)
            {
                let start_coordinates = [point_object.copy_normalized_x()?,
                    point_object.copy_normalized_y()?, point_object.copy_normalized_z()?];

                if let Some(sign) = concentrated_load.ref_optional_fx_sign()
                {
                    self.add_load_line(&gl_mode, sign, &CSAxis::X, &start_coordinates,
                        line_length, base_points_number, base_radius, &concentrated_load_color)?;
                    self.add_load_cap(sign, &CSAxis::X, &start_coordinates, line_length,
                        base_points_number, base_radius, height, &concentrated_load_color)?;
                }

                if let Some(sign) = concentrated_load.ref_optional_fy_sign()
                {
                    self.add_load_line(&gl_mode, sign,&CSAxis::Y, &start_coordinates,
                        line_length, base_points_number, base_radius, &concentrated_load_color)?;
                    self.add_load_cap(sign, &CSAxis::Y, &start_coordinates, line_length,
                        base_points_number, base_radius, height, &concentrated_load_color)?;
                }
                if let Some(sign) = concentrated_load.ref_optional_fz_sign()
                {
                    self.add_load_line(&gl_mode, sign, &CSAxis::Z, &start_coordinates,
                        line_length, base_points_number, base_radius, &concentrated_load_color)?;
                    self.add_load_cap(sign, &CSAxis::Z, &start_coordinates, line_length,
                        base_points_number, base_radius, height, &concentrated_load_color)?;
                }
                if let Some(sign) = concentrated_load.ref_optional_mx_sign()
                {
                    self.add_load_line(&gl_mode, sign, &CSAxis::X, &start_coordinates,
                        line_length * 0.67, base_points_number, base_radius,
                        &concentrated_load_color)?;
                    self.add_load_cap(sign, &CSAxis::X, &start_coordinates,
                        line_length * 0.67, base_points_number, base_radius,
                        height, &concentrated_load_color)?;
                    self.add_load_cap(sign, &CSAxis::X, &start_coordinates,
                        line_length * 0.67 - height, base_points_number, base_radius,
                        height, &concentrated_load_color)?;
                }
                if let Some(sign) = concentrated_load.ref_optional_my_sign()
                {
                    self.add_load_line(&gl_mode, sign, &CSAxis::Y, &start_coordinates,
                        line_length * 0.67, base_points_number, base_radius,
                        &concentrated_load_color)?;
                    self.add_load_cap(sign, &CSAxis::Y, &start_coordinates,
                        line_length * 0.67, base_points_number, base_radius,
                        height, &concentrated_load_color)?;
                    self.add_load_cap(sign, &CSAxis::Y, &start_coordinates,
                        line_length * 0.67 - height, base_points_number, base_radius,
                        height, &concentrated_load_color)?;
                }
                if let Some(sign) = concentrated_load.ref_optional_mz_sign()
                {
                    self.add_load_line(&gl_mode, sign, &CSAxis::Z, &start_coordinates,
                        line_length * 0.67, base_points_number, base_radius,
                        &concentrated_load_color)?;
                    self.add_load_cap(sign, &CSAxis::Z, &start_coordinates,
                        line_length * 0.67, base_points_number, base_radius,
                        height, &concentrated_load_color)?;
                    self.add_load_cap(sign, &CSAxis::Z, &start_coordinates,
                        line_length * 0.67 - height, base_points_number, base_radius,
                        height, &concentrated_load_color)?;
                }
            }
            else
            {
                let error_message = format!("Renderer: Point object extraction: \
                    Point with number {} does not exist!", point_number);
                return Err(JsValue::from(error_message));
            }
        }
        Ok(())
    }


    fn add_distributed_load_cap(&mut self, sign: &Sign, cs_axis: &CSAxis,
        start_coordinates: &[f32; 3], base_points_number: u32, base_radius: f32, height: f32,
        load_color: &[f32; 4]) -> Result<(), JsValue>
    {
        let multiplier = match sign { Sign::Positive => 1f32, Sign::Negative => -1f32 };

        let cap_base_center_point_coordinates =
            {
                match cs_axis
                {
                    CSAxis::X =>
                        {
                            [start_coordinates[0] - height * multiplier,
                            start_coordinates[1],
                            start_coordinates[2]]
                        },
                    CSAxis::Y =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1] - height * multiplier,
                            start_coordinates[2]]
                        },
                    CSAxis::Z =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1],
                            start_coordinates[2] - height * multiplier]
                        },
                }
            };

        let start_index =
            if self.ref_triangles_vertices_indexes()?.is_empty()
            {
                0
            }
            else
            {
                self.ref_triangles_vertices_indexes()?[
                    self.ref_triangles_vertices_indexes()?.len() - 1] + 1
            };

        let (triangles_vertices_coordinates,
            triangles_vertices_colors_values,
            triangles_vertices_indexes) = build_monochrome_cone(
            &start_coordinates,
            &cap_base_center_point_coordinates,
            height, base_radius, base_points_number, start_index,
            &load_color, TOLERANCE)?;

        self.drawn_object.add_triangle_vertex_coordinates(
            &triangles_vertices_coordinates)?;
        self.drawn_object.add_triangle_vertex_color_value(
            &triangles_vertices_colors_values)?;
        self.drawn_object.add_triangles_vertices_indexes(&triangles_vertices_indexes)?;

        Ok(())
    }


    pub fn add_distributed_line_loads(&mut self,
        point_objects: &HashMap<PointObjectKey, PointObject>,
        line_objects: &HashMap<LineObjectKey, LineObject>,
        distributed_line_loads: &HashMap<u32, DistributedLineLoad>, gl_mode: GLMode,
        under_selection_box_colors: &Vec<u8>, selected_colors: &HashSet<[u8; 4]>,
        line_length: f32, base_points_number: u32, height: f32, base_radius: f32,
        is_load_visible: &bool) -> Result<(), JsValue>
    {
        if !*is_load_visible
        {
            return Ok(())
        }

        for (line_number, distributed_line_load) in
            distributed_line_loads.iter()
        {
            let initial_color = DRAWN_DISTRIBUTED_LINE_LOADS_COLOR;
            let distributed_line_load_color = define_drawn_object_color(
                &gl_mode, distributed_line_load.copy_uid(), selected_colors,
                under_selection_box_colors, &initial_color);

            let line_object_key = LineObjectKey::create(*line_number,
                LineObjectType::Line);
            if let Some(line_object) = line_objects.get(&line_object_key)
            {
                let line_start_point_coordinates =
                    line_object.copy_start_point_object_coordinates(point_objects)?;
                let line_end_point_coordinates =
                    line_object.copy_end_point_object_coordinates(point_objects)?;

                for i in 0..=NUMBER_OF_DISTRIBUTED_LINE_LOAD_ARROWS
                {
                    let start_coordinates =
                        {
                            if i == 0
                            {
                                line_start_point_coordinates
                            }
                            else if i == NUMBER_OF_DISTRIBUTED_LINE_LOAD_ARROWS
                            {
                                line_end_point_coordinates
                            }
                            else
                            {
                                [line_start_point_coordinates[0] +
                                    (line_end_point_coordinates[0] -
                                        line_start_point_coordinates[0]) /
                                    (NUMBER_OF_DISTRIBUTED_LINE_LOAD_ARROWS - 1) as f32 * i as f32,
                                line_start_point_coordinates[1] +
                                    (line_end_point_coordinates[1] -
                                        line_start_point_coordinates[1]) /
                                    (NUMBER_OF_DISTRIBUTED_LINE_LOAD_ARROWS - 1) as f32 * i as f32,
                                line_start_point_coordinates[2] +
                                    (line_end_point_coordinates[2] -
                                        line_start_point_coordinates[2]) /
                                    (NUMBER_OF_DISTRIBUTED_LINE_LOAD_ARROWS - 1) as f32 * i as f32,
                                ]
                            }
                        };
                    if let Some(sign) = distributed_line_load.ref_optional_qx_sign()
                    {
                        self.add_load_line(&gl_mode, sign, &CSAxis::X, &start_coordinates,
                            -1f32 * line_length, base_points_number, base_radius,
                            &distributed_line_load_color)?;
                        self.add_distributed_load_cap(sign, &CSAxis::X, &start_coordinates,
                            base_points_number, base_radius, height, &distributed_line_load_color)?;
                    }
                    if let Some(sign) = distributed_line_load.ref_optional_qy_sign()
                    {
                        self.add_load_line(&gl_mode, sign, &CSAxis::Y, &start_coordinates,
                            -1f32 * line_length, base_points_number, base_radius,
                            &distributed_line_load_color)?;
                        self.add_distributed_load_cap(sign, &CSAxis::Y, &start_coordinates,
                            base_points_number, base_radius, height, &distributed_line_load_color)?;
                    }
                    if let Some(sign) = distributed_line_load.ref_optional_qz_sign()
                    {
                        self.add_load_line(&gl_mode, sign, &CSAxis::Z, &start_coordinates,
                            -1f32 * line_length, base_points_number, base_radius,
                            &distributed_line_load_color)?;
                        self.add_distributed_load_cap(sign, &CSAxis::Z, &start_coordinates,
                            base_points_number, base_radius, height, &distributed_line_load_color)?;
                    }
                }
            }
            else
            {
                let error_message = format!("Renderer: Line object extraction: \
                    Line with number {} does not exist!", line_number);
                return Err(JsValue::from(error_message));
            }
        }
        Ok(())
    }


    fn add_boundary_condition_cap(&mut self, cap_vertex_coordinates: &[f32; 3],
        base_points_number: u32, base_radius: f32, height: f32,
        boundary_condition_color: &[f32; 4]) -> Result<(), JsValue>
    {
        let start_index =
            if self.ref_triangles_vertices_indexes()?.is_empty()
            {
                0
            }
            else
            {
                self.ref_triangles_vertices_indexes()?[
                    self.ref_triangles_vertices_indexes()?.len() - 1] + 1
            };

        let (triangles_vertices_coordinates,
            triangles_vertices_colors_values,
            triangles_vertices_indexes) = build_monochrome_cone(
            &cap_vertex_coordinates,
            &[
                cap_vertex_coordinates[0],
                cap_vertex_coordinates[1] - height,
                cap_vertex_coordinates[2]
            ],
            height, base_radius, base_points_number, start_index,
            &boundary_condition_color, TOLERANCE)?;

        self.drawn_object.add_triangle_vertex_coordinates(
            &triangles_vertices_coordinates)?;
        self.drawn_object.add_triangle_vertex_color_value(
            &triangles_vertices_colors_values)?;
        self.drawn_object.add_triangles_vertices_indexes(&triangles_vertices_indexes)?;

       Ok(())
    }


    pub fn add_boundary_conditions(&mut self, point_objects: &HashMap<PointObjectKey, PointObject>,
        boundary_conditions: &HashMap<u32, BoundaryCondition>, gl_mode: GLMode,
        under_selection_box_colors: &Vec<u8>, selected_colors: &HashSet<[u8; 4]>,
        base_points_number: u32, height: f32, base_radius: f32,
        is_boundary_condition_visible: &bool) -> Result<(), JsValue>
    {
        if !*is_boundary_condition_visible
        {
            return Ok(())
        }

        for (point_number, boundary_condition) in boundary_conditions.iter()
        {
            let initial_color = DRAWN_BOUNDARY_CONDITION_COLOR;
            let boundary_condition_color = define_drawn_object_color(&gl_mode,
                boundary_condition.copy_uid(), selected_colors, under_selection_box_colors,
                &initial_color);

            let point_object_key = PointObjectKey::create(*point_number,
                PointObjectType::Point);
            if let Some(point_object) = point_objects.get(&point_object_key)
            {
                let cap_vertex_coordinates = [point_object.copy_normalized_x()?,
                    point_object.copy_normalized_y()?, point_object.copy_normalized_z()?];

                self.add_boundary_condition_cap(&cap_vertex_coordinates,
                    base_points_number, base_radius, height, &boundary_condition_color)?;
            }
            else
            {
                let error_message = format!("Renderer: Point object extraction: \
                    Point with number {} does not exist!", point_number);
                return Err(JsValue::from(error_message));
            }
        }
        Ok(())
    }


    pub fn ref_points_coordinates(&self) -> Result<&[f32], JsValue>
    {
        self.drawn_object.ref_points_coordinates()
    }


    pub fn ref_points_colors_values(&self) -> Result<&[f32], JsValue>
    {
        self.drawn_object.ref_points_colors_values()
    }


    pub fn ref_lines_endpoints_coordinates(&self) -> Result<&[f32], JsValue>
    {
        self.drawn_object.ref_lines_endpoints_coordinates()
    }


    pub fn ref_lines_endpoints_colors_values(&self) -> Result<&[f32], JsValue>
    {
        self.drawn_object.ref_lines_endpoints_colors_values()
    }


    pub fn ref_triangles_vertices_coordinates(&self) -> Result<&[f32], JsValue>
    {
        self.drawn_object.ref_triangles_vertices_coordinates()
    }


    pub fn ref_triangles_vertices_colors_values(&self) -> Result<&[f32], JsValue>
    {
        self.drawn_object.ref_triangles_vertices_colors_values()
    }


    pub fn ref_triangles_vertices_indexes(&self) -> Result<&[u32], JsValue>
    {
        self.drawn_object.ref_triangles_vertices_indexes()
    }


    pub fn draw_points(&self, gl: &GL) -> Result<(), JsValue>
    {
        self.drawn_object.draw_points(gl)
    }


    pub fn draw_lines(&self, gl: &GL) -> Result<(), JsValue>
    {
        self.drawn_object.draw_lines(gl)
    }


    pub fn draw_triangles(&self, gl: &GL) -> Result<(), JsValue>
    {
        self.drawn_object.draw_triangles(gl)
    }
}
