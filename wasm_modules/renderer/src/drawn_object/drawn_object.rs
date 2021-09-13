use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext as GL};
use std::f32::consts::PI;
use std::collections::{HashMap, HashSet};

use extended_matrix::extended_matrix::ExtendedMatrix;
use extended_matrix::basic_matrix::basic_matrix::MatrixElementPosition;
use extended_matrix::functions::copy_element_value;

use crate::point_object::{PointObjectKey, PointObject};
use crate::point_object::{PointObjectType};

use crate::line_object::{LineObject, LineObjectKey, BeamSectionOrientation};
use crate::line_object::{LineObjectType, LineObjectColorScheme};

use crate::concentrated_load::{ConcentratedLoad, Sign, CSAxis};

use crate::distributed_line_load::DistributedLineLoad;

use crate::boundary_condition::BoundaryCondition;

use crate::drawn_object::consts::{
    DRAWN_POINTS_COLOR, DRAWN_NODES_COLOR, DRAWN_LINES_DEFAULT_COLOR, DRAWN_LINES_TRUSS_PROPS_COLOR,
    DRAWN_LINES_BEAM_PROPS_COLOR, DRAWN_ELEMENTS_COLOR, DRAWN_BEAM_SECTION_ORIENTATION_COLOR,
    DRAWN_CONCENTRATED_LOADS_COLOR, DRAWN_DISTRIBUTED_LINE_LOADS_COLOR,
    NUMBER_OF_DISTRIBUTED_LINE_LOAD_ARROWS, DRAWN_BOUNDARY_CONDITION_COLOR,
};

use crate::consts::TOLERANCE;

use crate::functions::{define_drawn_object_color, compose_rotation_matrix_for_vector};

use crate::log;

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


pub trait DrawnObjectTrait
{
    fn get_vertices_coordinates(&self) -> &[f32];
    fn get_colors_values(&self) -> &[f32];
    fn get_indexes_numbers(&self) -> &[u32];
    fn draw(&self, gl: &GL);
}


#[derive(Clone)]
pub enum GLPrimitiveType
{
    Points,
    Lines,
    Triangles,
}


pub enum GLMode
{
    Selection,
    Visible,
}


#[derive(Clone)]
pub struct DrawnObject
{
    vertices_coordinates: Vec<f32>,
    colors_values: Vec<f32>,
    indexes_numbers: Vec<u32>,
    modes: Vec<GLPrimitiveType>,
    elements_numbers: Vec<i32>,
    offsets: Vec<i32>,
}


impl DrawnObjectTrait for DrawnObject
{
    fn get_vertices_coordinates(&self) -> &[f32]
    {
        self.vertices_coordinates.as_slice()
    }


    fn get_colors_values(&self) -> &[f32]
    {
        self.colors_values.as_slice()
    }


    fn get_indexes_numbers(&self) -> &[u32]
    {
        self.indexes_numbers.as_slice()
    }


    fn draw(&self, gl: &GL)
    {
        for index in 0..self.modes.len()
        {
            let count = self.elements_numbers[index];
            let offset = self.offsets[index];
            let mode = match self.modes[index]
            {
                GLPrimitiveType::Lines => GL::LINES,
                GLPrimitiveType::Triangles => GL::TRIANGLES,
                GLPrimitiveType::Points => GL::POINTS,
            };
            gl.draw_elements_with_i32(mode, count, GL::UNSIGNED_INT, offset);
        }
    }
}


impl DrawnObject
{
    pub fn create() -> Self
    {
        let vertices_coordinates = Vec::new();
        let colors_values = Vec::new();
        let indexes_numbers = Vec::new();
        let modes = Vec::new();
        let elements_numbers = Vec::new();
        let offsets = Vec::new();
        DrawnObject {
            vertices_coordinates,
            colors_values,
            indexes_numbers,
            modes,
            elements_numbers,
            offsets
        }
    }


    fn define_offset(&self) -> i32
    {
        if self.offsets.is_empty()
        {
            0
        }
        else
        {
            let previous_index = &self.offsets.len() - 1;
            let previous_elements_number = self.elements_numbers[previous_index];
            let previous_offset = self.offsets[previous_index];
            previous_offset + previous_elements_number * 4
        }
    }


    pub fn add_point_object(&mut self, point_objects: &HashMap<PointObjectKey, PointObject>,
        gl_mode: GLMode, under_selection_box_colors: &Vec<u8>, selected_colors: &HashSet<[u8; 4]>,
        is_geometry_visible: &bool, is_mesh_visible: &bool) -> Result<(), JsValue>
    {
        if !*is_geometry_visible && !*is_mesh_visible
        {
            return Ok(());
        }

        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };

        let mut excluded_point_objects = 0i32;
        for (i, (point_object_key, point_object))  in
            point_objects.iter().enumerate()
        {
            if !*is_geometry_visible && point_object_key.copy_object_type() == PointObjectType::Point
            {
                excluded_point_objects += 1i32;
                continue;
            }

            if !*is_mesh_visible && point_object_key.copy_object_type() == PointObjectType::Node
            {
                excluded_point_objects += 1i32;
                continue;
            }

            let initial_color = match point_object_key.copy_object_type()
                {
                    PointObjectType::Point => DRAWN_POINTS_COLOR,
                    PointObjectType::Node => DRAWN_NODES_COLOR,
                };
            self.vertices_coordinates.extend(
                &[point_object.copy_normalized_x()?,
                    point_object.copy_normalized_y()?,
                    point_object.copy_normalized_z()?]);
            let point_object_color = define_drawn_object_color(
                &gl_mode, point_object.copy_uid()?,
                selected_colors, under_selection_box_colors, &initial_color);
            self.colors_values.extend(&point_object_color);
            self.indexes_numbers.push(start_index + i as u32);
        }
        self.modes.push(GLPrimitiveType::Points);
        self.elements_numbers.push(point_objects.len() as i32 - excluded_point_objects);
        let offset = self.define_offset();
        self.offsets.push(offset);
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

        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        let mut count = 0;

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
                        let mut rotation_matrix =
                            line_object.extract_rotation_matrix(point_objects)?;
                        rotation_matrix.transpose();
                        let point_object_coordinates_shift =
                            ExtendedMatrix::create(3u32, 1u32,
                            vec![base_radius * 2.0, 0.0, 0.0], TOLERANCE);
                        let mut directional_vectors = Vec::new();
                        let angle = 2.0 * PI / base_points_number as f32;
                        for point_number in 0..base_points_number
                        {
                            let angle = angle * point_number as f32;
                            let local_x = {
                                let value = base_radius * angle.cos();
                                if value.abs() < TOLERANCE { 0.0 } else { value }
                            };
                            let local_y =
                                {
                                    let value = base_radius * angle.sin();
                                    if value.abs() < TOLERANCE { 0.0 } else { value }
                                };
                            let directional_vector =
                                ExtendedMatrix::create(3u32,
                                1u32, vec![0.0, local_y, local_x],
                                TOLERANCE);
                            directional_vectors.push(directional_vector);
                        }
                        for directional_vector in &directional_vectors
                        {
                            let mut directional_vector_start_point_object_coordinates =
                                start_point_object_coordinates;
                            let mut directional_vector_end_point_object_coordinates =
                                end_point_object_coordinates;
                            let transformed_directional_vector =
                                rotation_matrix.multiply_by_matrix(directional_vector)
                                    .map_err(|e| JsValue::from(e))?;
                            let transformed_point_object_coordinates_shift =
                                rotation_matrix.multiply_by_matrix(&point_object_coordinates_shift)
                                    .map_err(|e| JsValue::from(e))?;
                            let all_directional_vector_values =
                                transformed_directional_vector.copy_all_elements_values();
                            let all_point_object_coordinates_shift_values =
                                transformed_point_object_coordinates_shift.copy_all_elements_values();
                            let directional_vector_x_coordinate =
                                copy_element_value(0, 0,
                                    &all_directional_vector_values);
                            let object_coordinates_shift_x_coordinate =
                                copy_element_value(0, 0,
                                    &all_point_object_coordinates_shift_values);
                            let directional_vector_y_coordinate = copy_element_value(1,
                                0,&all_directional_vector_values);
                            let object_coordinates_shift_y_coordinate =
                                copy_element_value(1, 0,
                                    &all_point_object_coordinates_shift_values);
                            let directional_vector_z_coordinate = copy_element_value(2,
                                0, &all_directional_vector_values);
                            let object_coordinates_shift_z_coordinate =
                                copy_element_value(2, 0,
                                    &all_point_object_coordinates_shift_values);
                            directional_vector_start_point_object_coordinates[0] +=
                                directional_vector_x_coordinate +
                                    object_coordinates_shift_x_coordinate;
                            directional_vector_start_point_object_coordinates[1] +=
                                directional_vector_y_coordinate +
                                    object_coordinates_shift_y_coordinate;
                            directional_vector_start_point_object_coordinates[2] +=
                                directional_vector_z_coordinate +
                                    object_coordinates_shift_z_coordinate;
                            self.vertices_coordinates.extend(
                                &directional_vector_start_point_object_coordinates);
                            self.colors_values.extend(&line_object_color);
                            self.indexes_numbers.push(start_index + count);
                            count += 1;
                            directional_vector_end_point_object_coordinates[0] +=
                                directional_vector_x_coordinate - object_coordinates_shift_x_coordinate;
                            directional_vector_end_point_object_coordinates[1] +=
                                directional_vector_y_coordinate - object_coordinates_shift_y_coordinate;
                            directional_vector_end_point_object_coordinates[2] +=
                                directional_vector_z_coordinate - object_coordinates_shift_z_coordinate;
                            self.vertices_coordinates.extend(
                                &directional_vector_end_point_object_coordinates);
                            self.colors_values.extend(&line_object_color);
                            self.indexes_numbers.push(start_index + count);
                            count += 1;
                        }
                    },
                _ => ()
            }
            self.vertices_coordinates.extend(&start_point_object_coordinates);
            self.colors_values.extend(&line_object_color);
            self.indexes_numbers.push(start_index + count);
            count += 1;
            self.vertices_coordinates.extend(&end_point_object_coordinates);
            self.colors_values.extend(&line_object_color);
            self.indexes_numbers.push(start_index + count);
            count += 1;
        }
        self.modes.push(GLPrimitiveType::Lines);
        self.elements_numbers.push(count as i32);
        let offset = self.define_offset();
        self.offsets.push(offset);
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
                let start_index = if let Some(index) =
                    self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
                let mut count = 0;
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
                let local_axis_1_direction_projection_length = f32::sqrt(
                    (local_axis_1_direction_projection_x_coord_value.powi(2) +
                        local_axis_1_direction_projection_y_coord_value.powi(2) +
                        local_axis_1_direction_projection_z_coord_value.powi(2)));
                if local_axis_1_direction_projection_length == 0f32
                {
                    let error_message = format!("Renderer: Add beam section orientation for \
                        preview action: Projection of local axis 1 direction on line number {} \
                        equals to zero!", line_number);
                    return Err(JsValue::from(error_message));
                }
                let line_mid_point_coordinates = [
                    (line_end_point_coordinates[0] + line_start_point_coordinates[0]) / 2.0,
                    (line_end_point_coordinates[1] + line_start_point_coordinates[1]) / 2.0,
                    (line_end_point_coordinates[2] + line_start_point_coordinates[2]) / 2.0,
                ];
                let updated_local_axis_1_end_point_coordinates = [
                    (line_mid_point_coordinates[0] +
                    local_axis_1_direction_projection_x_coord_value * line_length),
                    (line_mid_point_coordinates[1] +
                    local_axis_1_direction_projection_y_coord_value * line_length),
                    (line_mid_point_coordinates[2] +
                     local_axis_1_direction_projection_z_coord_value * line_length),
                ];
                let updated_local_axis_1_color = DRAWN_BEAM_SECTION_ORIENTATION_COLOR;
                self.vertices_coordinates.extend(&line_mid_point_coordinates);
                self.colors_values.extend(&updated_local_axis_1_color);
                self.indexes_numbers.push(start_index + count);
                count += 1;
                self.vertices_coordinates.extend(&updated_local_axis_1_end_point_coordinates);
                self.colors_values.extend(&updated_local_axis_1_color);
                self.indexes_numbers.push(start_index + count);
                count += 1;
                self.modes.push(GLPrimitiveType::Lines);
                self.elements_numbers.push(count as i32);
                let offset = self.define_offset();
                self.offsets.push(offset);
                let mut rotation_matrix_for_cap =
                    compose_rotation_matrix_for_vector(
                        line_mid_point_coordinates,
                        updated_local_axis_1_end_point_coordinates
                    );
                rotation_matrix_for_cap.transpose();
                let d_angle = 2.0 * PI / base_points_number as f32;
                let local_coordinates = (0..base_points_number)
                    .map(|point_number|
                        {
                            let angle = d_angle * point_number as f32;
                            let local_x =
                                {
                                    let value = base_radius * angle.cos();
                                    if value.abs() < TOLERANCE { 0.0 } else { value }
                                };
                            let local_y =
                                {
                                    let value = base_radius * angle.sin();
                                    if value.abs() < TOLERANCE { 0.0 } else { value }
                                };
                            (local_x, local_y)
                        })
                    .collect::<Vec<(f32, f32)>>();

                let start_x_axis_cap_index =
                    if let Some(index) =
                        self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
                self.vertices_coordinates.extend(&updated_local_axis_1_end_point_coordinates);
                for (local_x, local_y) in &local_coordinates
                {
                    let local_coordinates = ExtendedMatrix::create(
                        3u32, 1u32, vec![
                            - height,
                            *local_y,
                            *local_x,
                        ], TOLERANCE);
                    let transformed_local_coordinates = rotation_matrix_for_cap
                        .multiply_by_matrix(&local_coordinates)
                        .map_err(|e| JsValue::from(e))?;
                    let transformed_local_coordinates_values =
                        transformed_local_coordinates.copy_all_elements_values();
                    let coordinates = [
                        copy_element_value(0, 0,
                            &transformed_local_coordinates_values) +
                            updated_local_axis_1_end_point_coordinates[0],
                        copy_element_value(1, 0,
                            &transformed_local_coordinates_values) +
                            updated_local_axis_1_end_point_coordinates[1],
                        copy_element_value(2, 0,
                            &transformed_local_coordinates_values) +
                            updated_local_axis_1_end_point_coordinates[2],
                    ];
                    self.vertices_coordinates.extend(&coordinates);
                }
                for point_number in 1..base_points_number
                {
                    if point_number == 1
                    {
                        self.colors_values.extend(&DRAWN_BEAM_SECTION_ORIENTATION_COLOR);
                        self.colors_values.extend(&DRAWN_BEAM_SECTION_ORIENTATION_COLOR);
                        self.colors_values.extend(&DRAWN_BEAM_SECTION_ORIENTATION_COLOR);
                    }
                    else
                    {
                        self.colors_values.extend(&DRAWN_BEAM_SECTION_ORIENTATION_COLOR);
                    }
                    self.indexes_numbers.extend(&[start_x_axis_cap_index,
                        start_x_axis_cap_index + point_number,
                        start_x_axis_cap_index + point_number + 1]);
                }
                self.indexes_numbers.extend(&[start_x_axis_cap_index,
                    start_x_axis_cap_index + 1, start_x_axis_cap_index + base_points_number]);
                self.modes.push(GLPrimitiveType::Triangles);
                self.elements_numbers.push(base_points_number as i32 * 3);
                let offset = self.define_offset();
                self.offsets.push(offset);
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


    fn add_concentrated_load_line_for_force(&mut self, gl_mode: &GLMode, sign: &Sign,
        cs_axis: &CSAxis, start_coordinates: &[f32; 3], line_length: f32,
        base_points_number_for_lines: u32, base_radius: f32,
        concentrated_load_color: &[f32; 4])
    {
        let start_index = if let Some(index) =
            self.indexes_numbers.iter().max() { *index + 1 } else { 0 };

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

        let mut count = 0;
        match gl_mode
            {
                GLMode::Selection =>
                    {
                        let coordinate_shift = base_radius * 4.0;

                        let d_angle = 2.0 * PI / base_points_number_for_lines as f32;
                        for point_number in 0..base_points_number_for_lines
                        {
                            let angle = d_angle * point_number as f32;
                            let local_x = {
                                let value = base_radius * 2.0 * angle.cos();
                                if value.abs() < TOLERANCE { 0.0 } else { value }
                            };
                            let local_y =
                                {
                                    let value = base_radius * 2.0 * angle.sin();
                                    if value.abs() < TOLERANCE { 0.0 } else { value }
                                };
                            let updated_start_point_coordinates =
                                {
                                    match cs_axis
                                    {
                                        CSAxis::X =>
                                            {
                                                [start_coordinates[0] + coordinate_shift *
                                                    multiplier,
                                                start_coordinates[1] + local_y,
                                                start_coordinates[2] + local_x]
                                            },
                                        CSAxis::Y =>
                                            {
                                                [start_coordinates[0] + local_y,
                                                start_coordinates[1] + coordinate_shift *
                                                    multiplier,
                                                start_coordinates[2] + local_x]
                                            },
                                        CSAxis::Z =>
                                            {
                                                [start_coordinates[0] + local_x,
                                                start_coordinates[1] + local_x,
                                                start_coordinates[2] + coordinate_shift *
                                                    multiplier]
                                            }
                                    }
                                };

                            let updated_end_point_coordinates =
                                {
                                    match cs_axis
                                    {
                                        CSAxis::X =>
                                            {
                                                [end_coordinates[0] - coordinate_shift * multiplier,
                                                end_coordinates[1] + local_y,
                                                end_coordinates[2] + local_x]
                                            },
                                        CSAxis::Y =>
                                            {
                                                [end_coordinates[0] + local_y,
                                                end_coordinates[1] - coordinate_shift * multiplier,
                                                end_coordinates[2] + local_x]
                                            },
                                        CSAxis::Z =>
                                            {
                                                [end_coordinates[0] + local_x,
                                                end_coordinates[1] + local_y,
                                                end_coordinates[2] - coordinate_shift * multiplier]
                                            },
                                    }
                                };

                            self.vertices_coordinates.extend(&updated_start_point_coordinates);
                            self.colors_values.extend(concentrated_load_color);
                            self.indexes_numbers.push(start_index + count);
                            count += 1;
                            self.vertices_coordinates.extend(&updated_end_point_coordinates);
                            self.colors_values.extend(concentrated_load_color);
                            self.indexes_numbers.push(start_index + count);
                            count += 1;
                        }
                    },
                _ => ()
            }
        self.vertices_coordinates.extend(start_coordinates);
        self.colors_values.extend(concentrated_load_color);
        self.indexes_numbers.push(start_index + count);
        count += 1;
        self.vertices_coordinates.extend(&end_coordinates);
        self.colors_values.extend(concentrated_load_color);
        self.indexes_numbers.push(start_index + count);
        count += 1;
        self.modes.push(GLPrimitiveType::Lines);
        self.elements_numbers.push(count as i32);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    fn add_concentrated_load_cap_for_force(&mut self, sign: &Sign, cs_axis: &CSAxis,
        start_coordinates: &[f32; 3], line_length: f32, base_points_number_for_caps: u32,
        base_radius: f32, height: f32, concentrated_load_color: &[f32; 4])
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

        let d_angle = 2.0 * PI / base_points_number_for_caps as f32;
        let local_coordinates = (0..base_points_number_for_caps)
            .map(|point_number|
                {
                    let angle = d_angle * point_number as f32;
                    let local_x =
                        {
                            let value = base_radius * angle.cos();
                            if value.abs() < TOLERANCE { 0.0 } else { value }
                        };
                    let local_y =
                        {
                            let value = base_radius * angle.sin();
                            if value.abs() < TOLERANCE { 0.0 } else { value }
                        };
                    (local_x, local_y)
                })
            .collect::<Vec<(f32, f32)>>();

        let start_index = if let Some(index) =
            self.indexes_numbers.iter().max() { *index + 1 } else { 0 };

        self.vertices_coordinates.extend(cap_vertex_coordinates);

        for (local_x, local_y) in &local_coordinates
        {
            let coordinates =
                {
                    match cs_axis
                    {
                        CSAxis::X =>
                            {
                                [cap_vertex_coordinates[0] - height * multiplier,
                                cap_vertex_coordinates[1] + local_y,
                                cap_vertex_coordinates[2] + local_x]
                            },
                        CSAxis::Y =>
                            {
                                [cap_vertex_coordinates[0] + local_y,
                                cap_vertex_coordinates[1] - height * multiplier,
                                cap_vertex_coordinates[2] + local_x]
                            },
                        CSAxis::Z =>
                            {
                                [cap_vertex_coordinates[0] + local_x ,
                                cap_vertex_coordinates[1] + local_y,
                                cap_vertex_coordinates[2] - height * multiplier]
                            },
                    }
                };
            self.vertices_coordinates.extend(&coordinates);
        }
        for point_number in 1..base_points_number_for_caps
        {
            if point_number == 1
            {
                self.colors_values.extend(concentrated_load_color);
                self.colors_values.extend(concentrated_load_color);
                self.colors_values.extend(concentrated_load_color);
            }
            else
            {
                self.colors_values.extend(concentrated_load_color);
            }
            self.indexes_numbers.extend(&[start_index,
                start_index + point_number, start_index + point_number + 1]);
        }
        self.indexes_numbers.extend(&[start_index,
            start_index + 1, start_index + base_points_number_for_caps]);

        self.modes.push(GLPrimitiveType::Triangles);
        self.elements_numbers.push(base_points_number_for_caps as i32 * 3);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    fn add_concentrated_load_line_for_moment(&mut self, gl_mode: &GLMode, sign: &Sign,
        cs_axis: &CSAxis, start_coordinates: &[f32; 3], line_length: f32,
        base_points_number_for_lines: u32, base_radius: f32,
        concentrated_load_color: &[f32; 4])
    {
        let start_index = if let Some(index) =
            self.indexes_numbers.iter().max() { *index + 1 } else { 0 };

        let multiplier = match sign { Sign::Positive => 1f32, Sign::Negative => -1f32 };

        let end_coordinates =
            {
                match cs_axis
                {
                    CSAxis::X =>
                        {
                            [start_coordinates[0] + line_length * multiplier * 0.67,
                            start_coordinates[1],
                            start_coordinates[2]]
                        },
                    CSAxis::Y =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1] + line_length * multiplier * 0.67,
                            start_coordinates[2]]
                        },
                    CSAxis::Z =>
                        {
                             [start_coordinates[0],
                             start_coordinates[1],
                            start_coordinates[2] + line_length * multiplier * 0.67]
                        }
                }

            };

        let mut count = 0;
        match gl_mode
            {
                GLMode::Selection =>
                    {
                        let coordinate_shift = base_radius * 4.0;

                        let d_angle = 2.0 * PI / base_points_number_for_lines as f32;
                        for point_number in 0..base_points_number_for_lines
                        {
                            let angle = d_angle * point_number as f32;
                            let local_x = {
                                let value = base_radius * 2.0 * angle.cos();
                                if value.abs() < TOLERANCE { 0.0 } else { value }
                            };
                            let local_y =
                                {
                                    let value = base_radius * 2.0 * angle.sin();
                                    if value.abs() < TOLERANCE { 0.0 } else { value }
                                };
                            let updated_start_point_coordinates =
                                {
                                    match cs_axis
                                    {
                                        CSAxis::X =>
                                            {
                                                [start_coordinates[0] + coordinate_shift *
                                                    multiplier,
                                                start_coordinates[1] + local_y,
                                                start_coordinates[2] + local_x]
                                            },
                                        CSAxis::Y =>
                                            {
                                                [start_coordinates[0] + local_y,
                                                start_coordinates[1] + coordinate_shift *
                                                    multiplier,
                                                start_coordinates[2] + local_x]
                                            },
                                        CSAxis::Z =>
                                            {
                                                [start_coordinates[0] + local_x,
                                                start_coordinates[1] + local_x,
                                                start_coordinates[2] + coordinate_shift *
                                                    multiplier]
                                            }
                                    }
                                };

                            let updated_end_point_coordinates =
                                {
                                    match cs_axis
                                    {
                                        CSAxis::X =>
                                            {
                                                [end_coordinates[0] - coordinate_shift *
                                                    multiplier,
                                                end_coordinates[1] + local_y,
                                                end_coordinates[2] + local_x]
                                            },
                                        CSAxis::Y =>
                                            {
                                                [end_coordinates[0] + local_y,
                                                end_coordinates[1] - coordinate_shift *
                                                    multiplier,
                                                end_coordinates[2] + local_x]
                                            },
                                        CSAxis::Z =>
                                            {
                                                [end_coordinates[0] + local_x,
                                                end_coordinates[1] + local_y,
                                                end_coordinates[2] - coordinate_shift *
                                                    multiplier]
                                            },
                                    }
                                };

                            self.vertices_coordinates.extend(&updated_start_point_coordinates);
                            self.colors_values.extend(concentrated_load_color);
                            self.indexes_numbers.push(start_index + count);
                            count += 1;
                            self.vertices_coordinates.extend(&updated_end_point_coordinates);
                            self.colors_values.extend(concentrated_load_color);
                            self.indexes_numbers.push(start_index + count);
                            count += 1;
                        }
                    },
                _ => ()
            }
        self.vertices_coordinates.extend(start_coordinates);
        self.colors_values.extend(concentrated_load_color);
        self.indexes_numbers.push(start_index + count);
        count += 1;
        self.vertices_coordinates.extend(&end_coordinates);
        self.colors_values.extend(concentrated_load_color);
        self.indexes_numbers.push(start_index + count);
        count += 1;
        self.modes.push(GLPrimitiveType::Lines);
        self.elements_numbers.push(count as i32);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    fn add_concentrated_load_cap_for_moment(&mut self, sign: &Sign, cs_axis: &CSAxis,
        start_coordinates: &[f32; 3], line_length: f32, base_points_number_for_caps: u32,
        base_radius: f32, height: f32, concentrated_load_color: &[f32; 4])
    {
        let multiplier = match sign { Sign::Positive => 1f32, Sign::Negative => -1f32 };

        let first_cap_vertex_coordinates =
            {
                match cs_axis
                {
                    CSAxis::X =>
                        {
                            [start_coordinates[0] + line_length * multiplier * 0.67,
                            start_coordinates[1],
                            start_coordinates[2]]
                        },
                    CSAxis::Y =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1] + line_length * multiplier * 0.67,
                            start_coordinates[2]]
                        },
                    CSAxis::Z =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1],
                            start_coordinates[2] + line_length * multiplier * 0.67]
                        },
                }
            };

        let d_angle = 2.0 * PI / base_points_number_for_caps as f32;
        let local_coordinates = (0..base_points_number_for_caps)
            .map(|point_number|
                {
                    let angle = d_angle * point_number as f32;
                    let local_x =
                        {
                            let value = base_radius * angle.cos();
                            if value.abs() < TOLERANCE { 0.0 } else { value }
                        };
                    let local_y =
                        {
                            let value = base_radius * angle.sin();
                            if value.abs() < TOLERANCE { 0.0 } else { value }
                        };
                    (local_x, local_y)
                })
            .collect::<Vec<(f32, f32)>>();

        let start_index = if let Some(index) =
            self.indexes_numbers.iter().max() { *index + 1 } else { 0 };

        self.vertices_coordinates.extend(first_cap_vertex_coordinates);

        for (local_x, local_y) in &local_coordinates
        {
            let coordinates =
                {
                    match cs_axis
                    {
                        CSAxis::X =>
                            {
                                [first_cap_vertex_coordinates[0] - height * multiplier,
                                first_cap_vertex_coordinates[1] + local_y,
                                first_cap_vertex_coordinates[2] + local_x]
                            },
                        CSAxis::Y =>
                            {
                                [first_cap_vertex_coordinates[0] + local_y,
                                first_cap_vertex_coordinates[1] - height * multiplier,
                                first_cap_vertex_coordinates[2] + local_x]
                            },
                        CSAxis::Z =>
                            {
                                [first_cap_vertex_coordinates[0] + local_x ,
                                first_cap_vertex_coordinates[1] + local_y,
                                first_cap_vertex_coordinates[2] - height * multiplier]
                            },
                    }
                };
            self.vertices_coordinates.extend(&coordinates);
        }

        for point_number in 1..base_points_number_for_caps
        {
            if point_number == 1
            {
                self.colors_values.extend(concentrated_load_color);
                self.colors_values.extend(concentrated_load_color);
                self.colors_values.extend(concentrated_load_color);
            }
            else
            {
                self.colors_values.extend(concentrated_load_color);
            }
            self.indexes_numbers.extend(&[start_index,
                start_index + point_number, start_index + point_number + 1]);
        }
        self.indexes_numbers.extend(&[start_index,
            start_index + 1, start_index + base_points_number_for_caps]);

        let second_cap_vertex_coordinates =
            {
                match cs_axis
                {
                    CSAxis::X =>
                        {
                            [start_coordinates[0] + (line_length * 0.67 - height) * multiplier,
                            start_coordinates[1],
                            start_coordinates[2]]
                        },
                    CSAxis::Y =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1] + (line_length * 0.67 - height) * multiplier,
                            start_coordinates[2]]
                        },
                    CSAxis::Z =>
                        {
                            [start_coordinates[0],
                            start_coordinates[1],
                            start_coordinates[2] + (line_length * 0.67 - height) * multiplier]
                        },
                }
            };

        let d_angle = 2.0 * PI / base_points_number_for_caps as f32;
        let local_coordinates = (0..base_points_number_for_caps)
            .map(|point_number|
                {
                    let angle = d_angle * point_number as f32;
                    let local_x =
                        {
                            let value = base_radius * angle.cos();
                            if value.abs() < TOLERANCE { 0.0 } else { value }
                        };
                    let local_y =
                        {
                            let value = base_radius * angle.sin();
                            if value.abs() < TOLERANCE { 0.0 } else { value }
                        };
                    (local_x, local_y)
                })
            .collect::<Vec<(f32, f32)>>();

        let start_index = if let Some(index) =
            self.indexes_numbers.iter().max() { *index + 1 } else { 0 };

        self.vertices_coordinates.extend(second_cap_vertex_coordinates);

        for (local_x, local_y) in &local_coordinates
        {
            let coordinates =
                {
                    match cs_axis
                    {
                        CSAxis::X =>
                            {
                                [second_cap_vertex_coordinates[0] - height * multiplier,
                                second_cap_vertex_coordinates[1] + local_y,
                                second_cap_vertex_coordinates[2] + local_x]
                            },
                        CSAxis::Y =>
                            {
                                [second_cap_vertex_coordinates[0] + local_y,
                                second_cap_vertex_coordinates[1] - height * multiplier,
                                second_cap_vertex_coordinates[2] + local_x]
                            },
                        CSAxis::Z =>
                            {
                                [second_cap_vertex_coordinates[0] + local_x ,
                                second_cap_vertex_coordinates[1] + local_y,
                                second_cap_vertex_coordinates[2] - height * multiplier]
                            },
                    }
                };
            self.vertices_coordinates.extend(&coordinates);
        }

        for point_number in 1..base_points_number_for_caps
        {
            if point_number == 1
            {
                self.colors_values.extend(concentrated_load_color);
                self.colors_values.extend(concentrated_load_color);
                self.colors_values.extend(concentrated_load_color);
            }
            else
            {
                self.colors_values.extend(concentrated_load_color);
            }
            self.indexes_numbers.extend(&[start_index,
                start_index + point_number, start_index + point_number + 1]);
        }
        self.indexes_numbers.extend(&[start_index,
            start_index + 1, start_index + base_points_number_for_caps]);

        self.modes.push(GLPrimitiveType::Triangles);
        self.elements_numbers.push(base_points_number_for_caps as i32 * 6);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    pub fn add_concentrated_loads(&mut self, point_objects: &HashMap<PointObjectKey, PointObject>,
        concentrated_loads: &HashMap<u32, ConcentratedLoad>, gl_mode: GLMode,
        under_selection_box_colors: &Vec<u8>, selected_colors: &HashSet<[u8; 4]>,
        line_length: f32, base_points_number_for_lines: u32, base_points_number_for_caps: u32,
        height: f32, base_radius: f32) -> Result<(), JsValue>
    {
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
                    self.add_concentrated_load_line_for_force(&gl_mode, sign, &CSAxis::X,
                        &start_coordinates, line_length, base_points_number_for_lines, base_radius,
                        &concentrated_load_color);
                    self.add_concentrated_load_cap_for_force(sign, &CSAxis::X,
                        &start_coordinates, line_length, base_points_number_for_caps, base_radius,
                        height, &concentrated_load_color);
                }
                if let Some(sign) = concentrated_load.ref_optional_fy_sign()
                {
                    self.add_concentrated_load_line_for_force(&gl_mode, sign,
                        &CSAxis::Y, &start_coordinates, line_length,
                        base_points_number_for_lines, base_radius, &concentrated_load_color);
                    self.add_concentrated_load_cap_for_force(sign, &CSAxis::Y,
                        &start_coordinates, line_length, base_points_number_for_caps, base_radius,
                        height, &concentrated_load_color);
                }
                if let Some(sign) = concentrated_load.ref_optional_fz_sign()
                {
                    self.add_concentrated_load_line_for_force(&gl_mode, sign,
                        &CSAxis::Z, &start_coordinates, line_length,
                        base_points_number_for_lines, base_radius, &concentrated_load_color);
                    self.add_concentrated_load_cap_for_force(sign, &CSAxis::Z,
                        &start_coordinates, line_length, base_points_number_for_caps, base_radius,
                        height, &concentrated_load_color);
                }
                if let Some(sign) = concentrated_load.ref_optional_mx_sign()
                {
                    self.add_concentrated_load_line_for_moment(&gl_mode, sign,
                        &CSAxis::X, &start_coordinates, line_length,
                        base_points_number_for_lines, base_radius, &concentrated_load_color);
                    self.add_concentrated_load_cap_for_moment(sign, &CSAxis::X,
                        &start_coordinates, line_length, base_points_number_for_caps, base_radius,
                        height, &concentrated_load_color);
                }
                if let Some(sign) = concentrated_load.ref_optional_my_sign()
                {
                    self.add_concentrated_load_line_for_moment(&gl_mode, sign,
                        &CSAxis::Y, &start_coordinates, line_length,
                        base_points_number_for_lines, base_radius, &concentrated_load_color);
                    self.add_concentrated_load_cap_for_moment(sign, &CSAxis::Y,
                        &start_coordinates, line_length, base_points_number_for_caps, base_radius,
                        height, &concentrated_load_color);
                }
                if let Some(sign) = concentrated_load.ref_optional_mz_sign()
                {
                    self.add_concentrated_load_line_for_moment(&gl_mode, sign,
                        &CSAxis::Z, &start_coordinates, line_length,
                        base_points_number_for_lines, base_radius, &concentrated_load_color);
                    self.add_concentrated_load_cap_for_moment(sign, &CSAxis::Z,
                        &start_coordinates, line_length, base_points_number_for_caps, base_radius,
                        height, &concentrated_load_color);
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


    pub fn add_distributed_line_loads(&mut self,
        point_objects: &HashMap<PointObjectKey, PointObject>,
        line_objects: &HashMap<LineObjectKey, LineObject>,
        distributed_line_loads: &HashMap<u32, DistributedLineLoad>, gl_mode: GLMode,
        under_selection_box_colors: &Vec<u8>, selected_colors: &HashSet<[u8; 4]>,
        line_length: f32, base_points_number_for_lines: u32, base_points_number_for_caps: u32,
        height: f32, base_radius: f32) -> Result<(), JsValue>
    {
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
                        self.add_concentrated_load_line_for_force(&gl_mode, sign,
                            &CSAxis::X, &start_coordinates, -1f32 * line_length,
                            base_points_number_for_lines, base_radius,
                            &distributed_line_load_color);
                        self.add_concentrated_load_cap_for_force(sign, &CSAxis::X,
                            &start_coordinates, 0f32, base_points_number_for_caps,
                            base_radius, height, &distributed_line_load_color);
                    }
                    if let Some(sign) = distributed_line_load.ref_optional_qy_sign()
                    {
                        self.add_concentrated_load_line_for_force(&gl_mode, sign,
                            &CSAxis::Y, &start_coordinates, -1f32 * line_length,
                            base_points_number_for_lines, base_radius,
                            &distributed_line_load_color);
                        self.add_concentrated_load_cap_for_force(sign, &CSAxis::Y,
                            &start_coordinates, 0f32, base_points_number_for_caps,
                            base_radius, height, &distributed_line_load_color);
                    }
                    if let Some(sign) = distributed_line_load.ref_optional_qz_sign()
                    {
                        self.add_concentrated_load_line_for_force(&gl_mode, sign,
                            &CSAxis::Z, &start_coordinates, -1f32 * line_length,
                            base_points_number_for_lines, base_radius,
                            &distributed_line_load_color);
                        self.add_concentrated_load_cap_for_force(sign, &CSAxis::Z,
                            &start_coordinates, 0f32, base_points_number_for_caps,
                            base_radius, height, &distributed_line_load_color);
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
        base_points_number_for_caps: u32, base_radius: f32, height: f32,
        boundary_condition_color: &[f32; 4])
    {
        let d_angle = 2.0 * PI / base_points_number_for_caps as f32;
        let local_coordinates = (0..base_points_number_for_caps)
            .map(|point_number|
                {
                    let angle = d_angle * point_number as f32;
                    let local_x =
                        {
                            let value = base_radius * angle.cos();
                            if value.abs() < TOLERANCE { 0.0 } else { value }
                        };
                    let local_y =
                        {
                            let value = base_radius * angle.sin();
                            if value.abs() < TOLERANCE { 0.0 } else { value }
                        };
                    (local_x, local_y)
                })
            .collect::<Vec<(f32, f32)>>();

        let start_index = if let Some(index) =
            self.indexes_numbers.iter().max() { *index + 1 } else { 0 };

        self.vertices_coordinates.extend(cap_vertex_coordinates);

        for (local_x, local_y) in &local_coordinates
        {
            let coordinates =
                [
                    cap_vertex_coordinates[0] + local_y,
                    cap_vertex_coordinates[1] - height,
                    cap_vertex_coordinates[2] + local_x
                ];
            self.vertices_coordinates.extend(&coordinates);
        }
        for point_number in 1..base_points_number_for_caps
        {
            if point_number == 1
            {
                self.colors_values.extend(boundary_condition_color);
                self.colors_values.extend(boundary_condition_color);
                self.colors_values.extend(boundary_condition_color);
            }
            else
            {
                self.colors_values.extend(boundary_condition_color);
            }
            self.indexes_numbers.extend(&[start_index,
                start_index + point_number, start_index + point_number + 1]);
        }
        self.indexes_numbers.extend(&[start_index,
            start_index + 1, start_index + base_points_number_for_caps]);

        self.modes.push(GLPrimitiveType::Triangles);
        self.elements_numbers.push(base_points_number_for_caps as i32 * 3);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    pub fn add_boundary_conditions(&mut self, point_objects: &HashMap<PointObjectKey, PointObject>,
        boundary_conditions: &HashMap<u32, BoundaryCondition>, gl_mode: GLMode,
        under_selection_box_colors: &Vec<u8>, selected_colors: &HashSet<[u8; 4]>,
        base_points_number_for_caps: u32, height: f32, base_radius: f32) -> Result<(), JsValue>
    {
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
                    base_points_number_for_caps, base_radius, height, &boundary_condition_color);
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
}
