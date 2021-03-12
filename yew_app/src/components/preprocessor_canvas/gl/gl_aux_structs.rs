use crate::{GLElementsNumbers, GLElementsValues, TOLERANCE};
use crate::components::preprocessor_canvas::gl::gl_aux_functions::find_node_coordinates;

use crate::{ElementsValues, ElementsNumbers};
use crate::fem::{FENode, FEType};
use crate::auxiliary::{NormalizedNode, FEDrawnElementData, DrawnDisplacementData};

use web_sys::{WebGlBuffer, WebGlUniformLocation, WebGlProgram, WebGlRenderingContext as GL};
use std::f32::consts::PI;
use std::cell::RefCell;
use std::rc::Rc;


const CS_ORIGIN: [GLElementsValues; 3] = [0.0, 0.0, 0.0];
const CS_AXIS_X: [GLElementsValues; 3] = [1.0, 0.0, 0.0];
const CS_AXIS_Y: [GLElementsValues; 3] = [0.0, 1.0, 0.0];
const CS_AXIS_Z: [GLElementsValues; 3] = [0.0, 0.0, 1.0];

const CS_AXIS_X_COLOR: [GLElementsValues; 4] = [1.0, 0.0, 0.0, 1.0]; // red
const CS_AXIS_Y_COLOR: [GLElementsValues; 4] = [0.0, 1.0, 0.0, 1.0]; // green
const CS_AXIS_Z_COLOR: [GLElementsValues; 4] = [0.0, 0.0, 1.0, 1.0]; // blue

pub const CS_AXES_SCALE: GLElementsValues = 0.1;
pub const CS_AXES_CAPS_HEIGHT: GLElementsValues = 0.07; // arrow length
pub const CS_AXES_CAPS_WIDTH: GLElementsValues = 0.035; // half of arrow width
pub const CS_AXES_CAPS_BASE_POINTS_NUMBER: u16 = 12; // the number of points in cone circular base

pub const CS_AXES_X_SHIFT: GLElementsValues = 0.85; // shift of the cs in the x-direction
pub const CS_AXES_Y_SHIFT: GLElementsValues = 0.85; // shift of the cs in the y-direction
pub const CS_AXES_Z_SHIFT: GLElementsValues = -1.5; // shift of the cs in the z-direction

pub const AXIS_X_DENOTATION_SHIFT_X: GLElementsValues = 0.1;
pub const AXIS_X_DENOTATION_SHIFT_Y: GLElementsValues = -0.05;
pub const AXIS_Y_DENOTATION_SHIFT_X: GLElementsValues = -0.05;
pub const AXIS_Y_DENOTATION_SHIFT_Y: GLElementsValues = 0.1;
pub const AXIS_Z_DENOTATION_SHIFT_X: GLElementsValues = -0.05;
pub const AXIS_Z_DENOTATION_SHIFT_Y: GLElementsValues = -0.05;
pub const AXIS_Z_DENOTATION_SHIFT_Z: GLElementsValues = 0.1;

pub const CANVAS_AXES_DENOTATION_COLOR: &str = "white";

pub const DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE: GLElementsValues = 0.8;
pub const DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE: GLElementsValues = 0.9;

pub const DRAWN_NODES_COLOR: [GLElementsValues; 4] = [1.0, 1.0, 0.0, 1.0]; // yellow
pub const CANVAS_DRAWN_NODES_DENOTATION_COLOR: &str = "yellow";

pub const DRAWN_NODES_DENOTATION_SHIFT: GLElementsValues = 0.02;

pub const DRAWN_ELEMENTS_COLOR: [GLElementsValues; 4] = [0.0, 1.0, 1.0, 1.0]; // cyan
pub const CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR: &str = "cyan";

// pub const DRAWN_ELEMENTS_DENOTATION_SHIFT: GLElementsValues = 0.02;
//
// pub const CANVAS_BACKGROUND_COLOR: &str = "black";


pub const DRAWN_DISPLACEMENTS_COLOR: [GLElementsValues; 4] = [1.0, 0.5, 0.0, 1.0]; // orange
pub const CANVAS_DRAWN_DISPLACEMENTS_DENOTATION_COLOR: &str = "orange";

pub const DRAWN_DISPLACEMENTS_CAPS_HEIGHT: GLElementsValues = 0.015; // arrow length
pub const DRAWN_DISPLACEMENTS_CAPS_WIDTH: GLElementsValues = 0.007; // half of arrow width
pub const DRAWN_DISPLACEMENTS_CAPS_BASE_POINTS_NUMBER: u16 = 12; // the number of points in cone circular base


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
        let tolerance = TOLERANCE as GLElementsValues;
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


    pub fn add_nodes(&mut self, normalized_nodes: &Vec<NormalizedNode>)
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        for (i, node) in normalized_nodes.iter().enumerate()
        {
            self.vertices_coordinates.extend(&[node.x, node.y, node.z]);
            self.colors_values.extend(&DRAWN_NODES_COLOR);
            self.indexes_numbers.push(start_index + i as GLElementsNumbers);
        }
        self.modes.push(GLPrimitiveType::Points);
        self.elements_numbers.push(normalized_nodes.len() as i32);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    pub fn add_elements(&mut self, normalized_nodes: &Vec<NormalizedNode>,
                        drawn_elements: &Vec<FEDrawnElementData>) -> Result<(), String>
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        let mut count = 0;
        let mut point_elements = Vec::new();
        let mut line_elements = Vec::new();
        let mut complex_elements = Vec::new();
        for element in drawn_elements
        {
            if element.nodes_numbers.len() == 1
            {
                point_elements.push(element);
            }
            if element.nodes_numbers.len() == 2
            {
                line_elements.push(element);
            }
            if element.nodes_numbers.len() > 2
            {
                complex_elements.push(element);
            }
        }
        if !point_elements.is_empty()
        {
            for point_element in &point_elements
            {
                let node_number = point_element.nodes_numbers[0] as GLElementsNumbers;
                let node_coordinates =
                    match find_node_coordinates(node_number, normalized_nodes)
                    {
                        Ok(coordinates) => coordinates,
                        Err(e) =>
                            {
                                return Err(e);
                            }
                    };
                self.vertices_coordinates.extend(&node_coordinates);
                self.colors_values.extend(&DRAWN_ELEMENTS_COLOR);
                self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                count += 1;
            }
            self.modes.push(GLPrimitiveType::Points);
            self.elements_numbers.push(point_elements.len() as i32);
            let offset = self.define_offset();
            self.offsets.push(offset);
        }
        if !line_elements.is_empty()
        {
            for line_element in &line_elements
            {
                let node_1_number = line_element.nodes_numbers[0] as GLElementsNumbers;
                let node_1_coordinates =
                    match find_node_coordinates(node_1_number, normalized_nodes)
                    {
                        Ok(coordinates) => coordinates,
                        Err(e) =>
                            {
                                return Err(e);
                            }
                    };
                self.vertices_coordinates.extend(&node_1_coordinates);
                self.colors_values.extend(&DRAWN_ELEMENTS_COLOR);
                self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                count += 1;
                let node_2_number = line_element.nodes_numbers[1] as GLElementsNumbers;
                let node_2_coordinates =
                    match find_node_coordinates(node_2_number, normalized_nodes)
                    {
                        Ok(coordinates) => coordinates,
                        Err(e) =>
                            {
                                return Err(e);
                            }
                    };
                self.vertices_coordinates.extend(&node_2_coordinates);
                self.colors_values.extend(&DRAWN_ELEMENTS_COLOR);
                self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                count += 1;
            }
            self.modes.push(GLPrimitiveType::Lines);
            self.elements_numbers.push(line_elements.len() as i32 * 2);
            let offset = self.define_offset();
            self.offsets.push(offset);
        }
        if !complex_elements.is_empty()
        {
            for complex_element in &complex_elements
            {
                for i in 1..complex_element.nodes_numbers.len()
                {
                    let node_1_number = complex_element.nodes_numbers[i - 1] as GLElementsNumbers;
                    let node_1_coordinates =
                    match find_node_coordinates(node_1_number, normalized_nodes)
                    {
                        Ok(coordinates) => coordinates,
                        Err(e) =>
                            {
                                return Err(e);
                            }
                    };
                    self.vertices_coordinates.extend(&node_1_coordinates);
                    self.colors_values.extend(&DRAWN_ELEMENTS_COLOR);
                    self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                    count += 1;
                    let node_2_number = complex_element.nodes_numbers[i] as GLElementsNumbers;
                    let node_2_coordinates =
                        match find_node_coordinates(node_2_number, normalized_nodes)
                        {
                            Ok(coordinates) => coordinates,
                            Err(e) =>
                                {
                                    return Err(e);
                                }
                        };
                    self.vertices_coordinates.extend(&node_2_coordinates);
                    self.colors_values.extend(&DRAWN_ELEMENTS_COLOR);
                    self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                    count += 1;
                }
                let node_1_number = complex_element.nodes_numbers[0] as GLElementsNumbers;
                let node_1_coordinates =
                match find_node_coordinates(node_1_number, normalized_nodes)
                {
                    Ok(coordinates) => coordinates,
                    Err(e) =>
                        {
                            return Err(e);
                        }
                };
                self.vertices_coordinates.extend(&node_1_coordinates);
                self.colors_values.extend(&DRAWN_ELEMENTS_COLOR);
                self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                count += 1;
                let node_2_number =
                    complex_element.nodes_numbers[complex_element.nodes_numbers.len() - 1] as GLElementsNumbers;
                let node_2_coordinates =
                    match find_node_coordinates(node_2_number, normalized_nodes)
                    {
                        Ok(coordinates) => coordinates,
                        Err(e) =>
                            {
                                return Err(e);
                            }
                    };
                self.vertices_coordinates.extend(&node_2_coordinates);
                self.colors_values.extend(&DRAWN_ELEMENTS_COLOR);
                self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                count += 1;
            }
            self.modes.push(GLPrimitiveType::Lines);
            self.elements_numbers.push(complex_elements.len() as i32 * 2);
            let offset = self.define_offset();
            self.offsets.push(offset);
        }
        Ok(())
    }


    pub fn add_displacements(&mut self, normalized_nodes: &Vec<NormalizedNode>,
        drawn_displacements: &Vec<DrawnDisplacementData>, base_points_number: GLElementsNumbers,
        height: GLElementsValues, base_radius: GLElementsValues)
    {
        let mut start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        let tolerance = TOLERANCE as GLElementsValues;
        for (i, displacement) in drawn_displacements.iter().enumerate()
        {
            if let Some(position) = normalized_nodes
                .iter()
                .position(|node| node.number == displacement.node_number)
            {
                let x = normalized_nodes[position].x;
                let y = normalized_nodes[position].y;
                let z = normalized_nodes[position].z;
                self.vertices_coordinates.extend(&[x, y, z]);

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
                    let coordinates =
                        [x + local_y, y - height, z + local_x];
                    self.vertices_coordinates.extend(&coordinates);
                }

                for point_number in 1..base_points_number
                {
                    if point_number == 1
                    {
                        self.colors_values.extend(&DRAWN_DISPLACEMENTS_COLOR);
                        self.colors_values.extend(&DRAWN_DISPLACEMENTS_COLOR);
                        self.colors_values.extend(&DRAWN_DISPLACEMENTS_COLOR);
                    }
                    else
                    {
                        self.colors_values.extend(&DRAWN_DISPLACEMENTS_COLOR);
                    }
                    self.indexes_numbers.extend(&[start_index, start_index + point_number,
                        start_index + point_number + 1]);
                }
                self.indexes_numbers.extend(&[start_index, start_index + 1,
                    start_index + base_points_number]);
                let offset = self.define_offset();
                self.modes.push(GLPrimitiveType::Triangles);
                self.elements_numbers.push(base_points_number as i32 * 3);
                self.offsets.push(offset);
                start_index += base_points_number + 1;
            }
        }
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