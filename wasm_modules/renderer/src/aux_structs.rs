use web_sys::{WebGlBuffer, WebGlUniformLocation, WebGlProgram, WebGlRenderingContext as GL};
use std::f32::consts::PI;

use crate::aux_functions::define_drawn_object_color;


const TOLERANCE: f32 = 1e-6;

const CS_ORIGIN: [f32; 3] = [0.0, 0.0, 0.0];
const CS_AXIS_X: [f32; 3] = [1.0, 0.0, 0.0];
const CS_AXIS_Y: [f32; 3] = [0.0, 1.0, 0.0];
const CS_AXIS_Z: [f32; 3] = [0.0, 0.0, 1.0];

const CS_AXIS_X_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // red
const CS_AXIS_Y_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0]; // green
const CS_AXIS_Z_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0]; // blue

pub const CS_AXES_SCALE: f32 = 0.1;
pub const CS_AXES_CAPS_HEIGHT: f32 = 0.15; // arrow length
pub const CS_AXES_CAPS_WIDTH: f32 = 0.075; // half of arrow width
pub const CS_AXES_CAPS_BASE_POINTS_NUMBER: u32 = 12; // the number of points in cone circular base

pub const CS_AXES_X_SHIFT: f32 = 0.85; // shift of the cs in the x-direction
pub const CS_AXES_Y_SHIFT: f32 = 0.85; // shift of the cs in the y-direction
pub const CS_AXES_Z_SHIFT: f32 = -1.5; // shift of the cs in the z-direction

pub const AXIS_X_DENOTATION_SHIFT_X: f32 = 0.1;
pub const AXIS_X_DENOTATION_SHIFT_Y: f32 = -0.05;
pub const AXIS_Y_DENOTATION_SHIFT_X: f32 = -0.05;
pub const AXIS_Y_DENOTATION_SHIFT_Y: f32 = 0.1;
pub const AXIS_Z_DENOTATION_SHIFT_X: f32 = -0.05;
pub const AXIS_Z_DENOTATION_SHIFT_Y: f32 = -0.05;
pub const AXIS_Z_DENOTATION_SHIFT_Z: f32 = 0.1;

pub const CANVAS_AXES_DENOTATION_COLOR: &str = "white";

pub const DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE: f32 = 0.8;
pub const DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE: f32 = 0.9;

pub const DRAWN_NODES_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0]; // yellow
pub const CANVAS_DRAWN_NODES_DENOTATION_COLOR: &str = "yellow";

pub const DRAWN_POINTS_COLOR: [f32; 4] = [0.26, 0.81, 0.20, 1.0]; // apple
pub const CANVAS_DRAWN_POINTS_DENOTATION_COLOR: &str = "rgb(67, 208, 52)";

pub const DRAWN_POINT_OBJECT_DENOTATION_SHIFT: f32 = 0.02;

pub const DRAWN_ELEMENTS_COLOR: [f32; 4] = [0.0, 1.0, 1.0, 1.0]; // cyan
pub const CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR: &str = "cyan";
pub const DRAWN_ELEMENTS_DENOTATION_SHIFT: f32 = 0.01;

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
pub const DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER: u16 = 12; // the number of points in cone circular base
pub const DRAWN_FORCES_LINE_LENGTH_COEFFICIENT: f32 = 1.5; // line length coefficient for moments values
pub const DRAWN_FORCES_CAPS_LENGTH_COEFFICIENT: f32 = 1.5; // line length coefficient for moments values

pub const DRAWN_FORCES_DENOTATION_SHIFT_X: f32 = 0.01;
pub const DRAWN_FORCES_DENOTATION_SHIFT_Y: f32 = 0.01;

pub const HINTS_COLOR: &str = "white";
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


#[derive(Debug, Copy, Clone)]
pub enum PointObjectType
{
    Point,
    Node,
}

#[derive(Debug)]
pub struct PointObject
{
    pub number: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub object_type: PointObjectType,
}


#[derive(Debug)]
pub struct NormalizedPointObject
{
    pub number: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub object_type: PointObjectType,
    pub uid: u32,
}


pub struct ShadersVariables
{
    vertex_position: u32,
    vertex_color: u32,
    pub point_size: WebGlUniformLocation,
    pub projection_matrix: WebGlUniformLocation,
    pub model_view_matrix: WebGlUniformLocation,
}


impl ShadersVariables
{
    pub fn initialize(gl: &GL, shader_program: &WebGlProgram) -> Self
    {
        let vertex_position = gl.get_attrib_location(&shader_program, "aVertexPosition") as u32;
        let vertex_color = gl.get_attrib_location(&shader_program, "aVertexColor") as u32;
        let point_size = gl.get_uniform_location(&shader_program, "uPointSize").unwrap();
        let projection_matrix = gl
            .get_uniform_location(&shader_program, "uProjectionMatrix")
            .unwrap();
        let model_view_matrix = gl
            .get_uniform_location(&shader_program, "uModelViewMatrix")
            .unwrap();
        ShadersVariables {
            vertex_position, vertex_color, point_size, projection_matrix, model_view_matrix }
    }
}


pub struct Buffers
{
    vertex: WebGlBuffer,
    color: WebGlBuffer,
    index: WebGlBuffer,
}


impl Buffers
{
    pub fn initialize(gl: &GL) -> Self
    {
        let vertex = gl.create_buffer().unwrap();
        let color = gl.create_buffer().unwrap();
        let index = gl.create_buffer().unwrap();
        Buffers { vertex, color, index }
    }


    pub fn render(&self, gl: &GL, drawn_object: &DrawnObject, shaders_variables: &ShadersVariables)
    {
        let vertices = js_sys::Float32Array::from(drawn_object.vertices_coordinates.as_slice());
        let colors = js_sys::Float32Array::from(drawn_object.colors_values.as_slice());
        let indexes = js_sys::Uint32Array::from(drawn_object.indexes_numbers.as_slice());
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(shaders_variables.vertex_position, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(shaders_variables.vertex_position);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &colors, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(shaders_variables.vertex_color, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(shaders_variables.vertex_color);
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indexes, GL::STATIC_DRAW);
    }
}


pub enum CSAxis
{
    X, Y, Z,
}


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


pub struct DrawnObject
{
    vertices_coordinates: Vec<f32>,
    colors_values: Vec<f32>,
    indexes_numbers: Vec<u32>,
    modes: Vec<GLPrimitiveType>,
    elements_numbers: Vec<i32>,
    offsets: Vec<i32>,
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


    pub fn add_cs_axis_line(&mut self, cs_axis: CSAxis)
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        self.vertices_coordinates.extend(&CS_ORIGIN);
        match cs_axis
        {
            CSAxis::X =>
                {
                    self.vertices_coordinates.extend(&CS_AXIS_X);
                    self.colors_values.extend(&CS_AXIS_X_COLOR);
                    self.colors_values.extend(&CS_AXIS_X_COLOR);
                },
            CSAxis::Y =>
                {
                    self.vertices_coordinates.extend(&CS_AXIS_Y);
                    self.colors_values.extend(&CS_AXIS_Y_COLOR);
                    self.colors_values.extend(&CS_AXIS_Y_COLOR);
                },
            CSAxis::Z =>
                {
                    self.vertices_coordinates.extend(&CS_AXIS_Z);
                    self.colors_values.extend(&CS_AXIS_Z_COLOR);
                    self.colors_values.extend(&CS_AXIS_Z_COLOR);
                },
        }
        self.indexes_numbers.extend(&[start_index, start_index + 1]);
        self.modes.push(GLPrimitiveType::Lines);
        self.elements_numbers.push(2);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    pub fn add_cs_axis_cap(&mut self, cs_axis: CSAxis, base_points_number: u32,
                           height: f32, base_radius: f32)
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        let tolerance = TOLERANCE;
        match cs_axis
        {
            CSAxis::X => self.vertices_coordinates.extend(&CS_AXIS_X),
            CSAxis::Y => self.vertices_coordinates.extend(&CS_AXIS_Y),
            CSAxis::Z => self.vertices_coordinates.extend(&CS_AXIS_Z),
        }

        let angle = 2.0 * PI / base_points_number as f32;
        for point_number in 0..base_points_number
        {
            let angle = angle * point_number as f32;
            let local_x = {
                let value = base_radius * angle.cos();
                if value.abs() < tolerance { 0.0 } else { value }
            };
            let local_y =
                {
                    let value = base_radius * angle.sin();
                    if value.abs() < tolerance { 0.0 } else { value }
                };
            let coordinates = match cs_axis
            {
                CSAxis::X => [1.0 - height, local_y, local_x],
                CSAxis::Y => [local_y, 1.0 - height, local_x],
                CSAxis::Z => [local_x, local_y, 1.0 - height],
            };
            self.vertices_coordinates.extend(&coordinates);
        }

        let local_color_value = match cs_axis
        {
            CSAxis::X => CS_AXIS_X_COLOR,
            CSAxis::Y => CS_AXIS_Y_COLOR,
            CSAxis::Z => CS_AXIS_Z_COLOR,
        };

        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.colors_values.extend(&local_color_value);
                self.colors_values.extend(&local_color_value);
                self.colors_values.extend(&local_color_value);
            } else {
                self.colors_values.extend(&local_color_value);
            }
            self.indexes_numbers.extend(
                &[start_index, start_index + point_number, start_index + point_number + 1]);
        }
        self.indexes_numbers.extend(
            &[start_index, start_index + 1, start_index + base_points_number]);
        let offset = self.define_offset();
        self.modes.push(GLPrimitiveType::Triangles);
        self.elements_numbers.push(base_points_number as i32 * 3);
        self.offsets.push(offset);
    }


    pub fn add_point_object(&mut self, normalized_point_objects: &[NormalizedPointObject],
        gl_mode: GLMode, under_cursor_color: &[u8; 4], selected_color: &[u8; 4])
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        for (i, point_object) in
            normalized_point_objects.iter().enumerate()
        {
            let initial_color = match point_object.object_type
                {
                    PointObjectType::Point => DRAWN_POINTS_COLOR,
                    PointObjectType::Node => DRAWN_NODES_COLOR,
                };
            self.vertices_coordinates.extend(
                &[point_object.x, point_object.y, point_object.z]);
            let node_color = define_drawn_object_color(
                &gl_mode, point_object.uid,
                selected_color, under_cursor_color, &initial_color);
            self.colors_values.extend(&node_color);
            self.indexes_numbers.push(start_index + i as u32);
        }
        self.modes.push(GLPrimitiveType::Points);
        self.elements_numbers.push(normalized_point_objects.len() as i32);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    pub fn draw(&self, gl: &GL)
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