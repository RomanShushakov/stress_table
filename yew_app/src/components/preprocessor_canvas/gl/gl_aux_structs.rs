use crate::components::preprocessor_canvas::preprocessor_canvas::
    {
        GLElementsNumbers, GLElementsValues
    };

use std::f32::consts::PI;
use web_sys::{WebGlBuffer, WebGlUniformLocation, WebGlProgram, WebGlRenderingContext as GL};


const CS_ORIGIN: [GLElementsValues; 3] = [0.0, 0.0, 0.0];
const CS_AXIS_X: [GLElementsValues; 3] = [1.0, 0.0, 0.0];
const CS_AXIS_Y: [GLElementsValues; 3] = [0.0, 1.0, 0.0];
const CS_AXIS_Z: [GLElementsValues; 3] = [0.0, 0.0, 1.0];

const CS_AXIS_X_COLOR: [GLElementsValues; 4] = [1.0, 0.0, 0.0, 1.0]; // red
const CS_AXIS_Y_COLOR: [GLElementsValues; 4] = [0.0, 1.0, 0.0, 1.0]; // green
const CS_AXIS_Z_COLOR: [GLElementsValues; 4] = [0.0, 0.0, 1.0, 1.0]; // blue

pub const CS_AXES_SCALE: GLElementsValues = 0.15;
pub const CS_AXES_CAPS_HEIGHT: GLElementsValues = 0.1; // arrow length
pub const CS_AXES_CAPS_WIDTH: GLElementsValues = 0.05; // half of arrow width
pub const CS_AXES_CAPS_BASE_POINTS_NUMBER: u16 = 36; // the number of points in cone circular base

pub const CS_AXES_X_SHIFT: GLElementsValues = 0.8; // shift of the cs in the x-direction
pub const CS_AXES_Y_SHIFT: GLElementsValues = 0.8; // shift of the cs in the y-direction
pub const CS_AXES_Z_SHIFT: GLElementsValues = -1.5; // shift of the cs in the z-direction


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


pub struct DrawnObject
{
    vertices_coordinates: Vec<GLElementsValues>,
    colors_values: Vec<GLElementsValues>,
    indexes_numbers: Vec<GLElementsNumbers>,
    pub modes: Vec<GLPrimitiveType>,
    pub elements_numbers: Vec<i32>,
    pub offsets: Vec<i32>,
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
            vertices_coordinates, colors_values, indexes_numbers, modes, elements_numbers, offsets
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
            previous_offset + previous_elements_number * 2
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


    pub fn add_cs_axis_cap(&mut self, cs_axis: CSAxis, base_points_number: GLElementsNumbers,
        height: GLElementsValues, base_radius: GLElementsValues)
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        let tolerance = 1e-6;
        match cs_axis
        {
            CSAxis::X => self.vertices_coordinates.extend(&CS_AXIS_X),
            CSAxis::Y => self.vertices_coordinates.extend(&CS_AXIS_Y),
            CSAxis::Z => self.vertices_coordinates.extend(&CS_AXIS_Z),
        }

        let angle = 2.0 * PI / base_points_number as GLElementsValues;
        for point_number in 0..base_points_number
        {
            let angle = angle * point_number as GLElementsValues;
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
            }
            else
            {
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
            gl.draw_elements_with_i32(mode, count, GL::UNSIGNED_SHORT, offset);
        }
    }
}


pub struct ShadersVariables
{
    vertex_position: u32,
    vertex_color: u32,
    pub projection_matrix: WebGlUniformLocation,
    pub model_view_matrix: WebGlUniformLocation,
}


impl ShadersVariables
{
    pub fn initialize(gl: &GL, shader_program: &WebGlProgram) -> Self
    {
        let vertex_position = gl.get_attrib_location(&shader_program, "aVertexPosition") as u32;
        let vertex_color = gl.get_attrib_location(&shader_program, "aVertexColor") as u32;
        let projection_matrix = gl
            .get_uniform_location(&shader_program, "uProjectionMatrix")
            .unwrap();
        let model_view_matrix = gl
            .get_uniform_location(&shader_program, "uModelViewMatrix")
            .unwrap();
        ShadersVariables { vertex_position, vertex_color, projection_matrix, model_view_matrix }
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
        let indexes = js_sys::Uint16Array::from(drawn_object.indexes_numbers.as_slice());
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