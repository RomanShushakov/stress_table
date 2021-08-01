use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext as GL};
use std::f32::consts::PI;
use std::collections::{HashMap, HashSet};

use extended_matrix::extended_matrix::ExtendedMatrix;
use extended_matrix::basic_matrix::basic_matrix::MatrixElementPosition;
use extended_matrix::functions::extract_element_value;

use crate::point_object::{PointObjectKey, PointObject};
use crate::point_object::{PointObjectType};

use crate::line_object::{LineObject, LineObjectKey, BeamSectionOrientation};
use crate::line_object::{LineObjectType, LineObjectColorScheme};

use crate::drawn_object::consts::
{
    DRAWN_POINTS_COLOR, DRAWN_NODES_COLOR, DRAWN_LINES_DEFAULT_COLOR, DRAWN_LINES_TRUSS_PROPS_COLOR,
    DRAWN_LINES_BEAM_PROPS_COLOR, DRAWN_ELEMENTS_COLOR, DRAWN_BEAM_SECTION_ORIENTATION_COLOR,
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

pub const DRAWN_FORCES_LINE_LENGTH: f32 = 0.07; // line length
pub const DRAWN_FORCES_CAPS_HEIGHT: f32 = 0.015; // arrow length
pub const DRAWN_FORCES_CAPS_WIDTH: f32 = 0.007; // half of arrow width
pub const DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER: u32 = 12; // the number of points in cone circular base
pub const DRAWN_FORCES_LINE_LENGTH_COEFFICIENT: f32 = 1.5; // line length coefficient for moments values
pub const DRAWN_FORCES_CAPS_LENGTH_COEFFICIENT: f32 = 1.5; // line length coefficient for moments values

pub const DRAWN_FORCES_DENOTATION_SHIFT_X: f32 = 0.01;
pub const DRAWN_FORCES_DENOTATION_SHIFT_Y: f32 = 0.01;

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


pub enum CSAxis
{
    X, Y, Z,
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
        gl_mode: GLMode, under_selection_box_colors: &Vec<u8>, selected_colors: &HashSet<[u8; 4]>)
        -> Result<(), JsValue>
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        for (i, (point_object_key, point_object))  in
            point_objects.iter().enumerate()
        {
            let initial_color = match point_object_key.get_object_type()
                {
                    PointObjectType::Point => DRAWN_POINTS_COLOR,
                    PointObjectType::Node => DRAWN_NODES_COLOR,
                };
            self.vertices_coordinates.extend(
                &[point_object.get_normalized_x()?,
                    point_object.get_normalized_y()?,
                    point_object.get_normalized_z()?]);
            let point_object_color = define_drawn_object_color(
                &gl_mode, point_object.get_uid()?,
                selected_colors, under_selection_box_colors, &initial_color);
            self.colors_values.extend(&point_object_color);
            self.indexes_numbers.push(start_index + i as u32);
        }
        self.modes.push(GLPrimitiveType::Points);
        self.elements_numbers.push(point_objects.len() as i32);
        let offset = self.define_offset();
        self.offsets.push(offset);
        Ok(())
    }


    pub fn add_line_objects(&mut self,
        point_objects: &HashMap<PointObjectKey, PointObject>,
        line_objects: &HashMap<LineObjectKey, LineObject>,
        gl_mode: GLMode, under_selection_box_colors: &Vec<u8>,
        selected_colors: &HashSet<[u8; 4]>, base_points_number: u32,
        base_radius: f32) -> Result<(), JsValue>
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        let mut count = 0;
        for (line_object_key, line_object) in line_objects.iter()
        {
            let initial_color = match line_object_key.get_object_type()
                {
                    LineObjectType::Line =>
                        {
                            match line_object.get_color_scheme()
                            {
                                LineObjectColorScheme::Default => DRAWN_LINES_DEFAULT_COLOR,
                                LineObjectColorScheme::TrussProps => DRAWN_LINES_TRUSS_PROPS_COLOR,
                                LineObjectColorScheme::BeamProps => DRAWN_LINES_BEAM_PROPS_COLOR,
                            }
                        },
                    LineObjectType::Element => DRAWN_ELEMENTS_COLOR,
                };
            let line_object_color = define_drawn_object_color(&gl_mode,
                line_object.get_uid(), selected_colors, under_selection_box_colors,
                &initial_color);
            let start_point_object_coordinates =
                line_object.get_start_point_object_coordinates(point_objects)?;
            let end_point_object_coordinates =
                line_object.get_end_point_object_coordinates(point_objects)?;
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
                                transformed_directional_vector.extract_all_elements_values();
                            let all_point_object_coordinates_shift_values =
                                transformed_point_object_coordinates_shift.extract_all_elements_values();
                            let directional_vector_x_coordinate =
                                extract_element_value(0, 0,
                                    &all_directional_vector_values);
                            let object_coordinates_shift_x_coordinate =
                                extract_element_value(0, 0,
                                    &all_point_object_coordinates_shift_values);
                            let directional_vector_y_coordinate = extract_element_value(1,
                                0,&all_directional_vector_values);
                            let object_coordinates_shift_y_coordinate =
                                extract_element_value(1, 0,
                                    &all_point_object_coordinates_shift_values);
                            let directional_vector_z_coordinate = extract_element_value(2,
                                0, &all_directional_vector_values);
                            let object_coordinates_shift_z_coordinate =
                                extract_element_value(2, 0,
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
            beam_section_orientation.extract_local_axis_1_direction();
        for line_number in beam_section_orientation.extract_line_numbers()
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
                    line_object.get_start_point_object_coordinates(point_objects)?;
                let line_end_point_coordinates =
                    line_object.get_end_point_object_coordinates(point_objects)?;
                let b_x = line_end_point_coordinates[0] - line_start_point_coordinates[0];
                let b_y = line_end_point_coordinates[1] - line_start_point_coordinates[1];
                let b_z = line_end_point_coordinates[2] - line_start_point_coordinates[2];
                let a = ExtendedMatrix::create(3u32,
                    1u32, vec![a_x, a_y, a_z], TOLERANCE);
                let coeff_matrix = ExtendedMatrix::create(3u32,
                    3u32, vec![
                        - b_z * b_z - b_y * b_y, b_x * b_y, b_x * b_z,
                        b_y * b_x, - b_x * b_x - b_z * b_z,	b_y * b_z,
                        b_z * b_x,	b_z * b_y, - b_y * b_y - b_x * b_x,
                    ], TOLERANCE);
                let local_axis_1_direction_projection_matrix =
                    coeff_matrix
                        .multiply_by_matrix(&a)
                        .map_err(|e| JsValue::from(e))?;
                let local_axis_1_direction_projection_all_values =
                    local_axis_1_direction_projection_matrix.extract_all_elements_values();
                let local_axis_1_direction_projection_x_coord_value = extract_element_value(
                    0, 0, &local_axis_1_direction_projection_all_values);
                let local_axis_1_direction_projection_y_coord_value = extract_element_value(
                    1, 0, &local_axis_1_direction_projection_all_values);
                let local_axis_1_direction_projection_z_coord_value = extract_element_value(
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
                    local_axis_1_direction_projection_x_coord_value /
                        local_axis_1_direction_projection_length * line_length),
                    (line_mid_point_coordinates[1] +
                    local_axis_1_direction_projection_y_coord_value /
                        local_axis_1_direction_projection_length * line_length),
                    (line_mid_point_coordinates[2] +
                     local_axis_1_direction_projection_z_coord_value /
                        local_axis_1_direction_projection_length * line_length),
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
                        transformed_local_coordinates.extract_all_elements_values();
                    let coordinates = [
                        extract_element_value(0, 0,
                            &transformed_local_coordinates_values) +
                            updated_local_axis_1_end_point_coordinates[0],
                        extract_element_value(1, 0,
                            &transformed_local_coordinates_values) +
                            updated_local_axis_1_end_point_coordinates[1],
                        extract_element_value(2, 0,
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
}
