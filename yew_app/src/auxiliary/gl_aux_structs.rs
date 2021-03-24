use web_sys::{WebGlBuffer, WebGlUniformLocation, WebGlProgram, WebGlRenderingContext as GL};
use std::f32::consts::PI;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{GLElementsNumbers, GLElementsValues, TOLERANCE};
use crate::auxiliary::gl_aux_functions::
    {
        find_node_coordinates, define_drawn_object_color, define_color_value,
        define_color_array_by_value, define_element_chunks,
    };

use crate::{ElementsValues, ElementsNumbers};
use crate::fem::{FENode, FEType, GlobalDOFParameter, StressStrainComponent, EARType, EARComponentTrait};
use crate::auxiliary::
    {
        NormalizedNode, FEDrawnElementData, FEDrawnBCData, FEDrawnAnalysisResultNodeData,
        DrawnAnalysisResultElementData
    };
use crate::auxiliary::aux_functions::transform_u32_to_array_of_u8;
use yew::Callback;
use crate::fem::global_analysis::fe_global_analysis_result::Reactions;


const CS_ORIGIN: [GLElementsValues; 3] = [0.0, 0.0, 0.0];
const CS_AXIS_X: [GLElementsValues; 3] = [1.0, 0.0, 0.0];
const CS_AXIS_Y: [GLElementsValues; 3] = [0.0, 1.0, 0.0];
const CS_AXIS_Z: [GLElementsValues; 3] = [0.0, 0.0, 1.0];

const CS_AXIS_X_COLOR: [GLElementsValues; 4] = [1.0, 0.0, 0.0, 1.0]; // red
const CS_AXIS_Y_COLOR: [GLElementsValues; 4] = [0.0, 1.0, 0.0, 1.0]; // green
const CS_AXIS_Z_COLOR: [GLElementsValues; 4] = [0.0, 0.0, 1.0, 1.0]; // blue

pub const CS_AXES_SCALE: GLElementsValues = 0.1;
pub const CS_AXES_CAPS_HEIGHT: GLElementsValues = 0.15; // arrow length
pub const CS_AXES_CAPS_WIDTH: GLElementsValues = 0.075; // half of arrow width
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
pub const DRAWN_ELEMENTS_DENOTATION_SHIFT: GLElementsValues = 0.01;

// pub const CANVAS_BACKGROUND_COLOR: &str = "black";

pub const DRAWN_DISPLACEMENTS_COLOR: [GLElementsValues; 4] = [1.0, 0.5, 0.0, 1.0]; // orange
pub const CANVAS_DRAWN_DISPLACEMENTS_DENOTATION_COLOR: &str = "orange";

pub const DRAWN_DISPLACEMENTS_CAPS_HEIGHT: GLElementsValues = 0.015; // arrow length
pub const DRAWN_DISPLACEMENTS_CAPS_WIDTH: GLElementsValues = 0.007; // half of arrow width
pub const DRAWN_DISPLACEMENTS_CAPS_BASE_POINTS_NUMBER: u16 = 12; // the number of points in cone circular base

pub const DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_X: GLElementsValues = 0.01;
pub const DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_Y: GLElementsValues = 0.015;


pub const DRAWN_FORCES_COLOR: [GLElementsValues; 4] = [1.0, 0.0, 1.0, 1.0]; // magenta
pub const CANVAS_DRAWN_FORCES_DENOTATION_COLOR: &str = "magenta";

pub const DRAWN_FORCES_LINE_LENGTH: GLElementsValues = 0.07; // line length
pub const DRAWN_FORCES_CAPS_HEIGHT: GLElementsValues = 0.015; // arrow length
pub const DRAWN_FORCES_CAPS_WIDTH: GLElementsValues = 0.007; // half of arrow width
pub const DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER: u16 = 12; // the number of points in cone circular base
pub const DRAWN_FORCES_LINE_LENGTH_COEFFICIENT: GLElementsValues = 1.5; // line length coefficient for moments values
pub const DRAWN_FORCES_CAPS_LENGTH_COEFFICIENT: GLElementsValues = 1.5; // line length coefficient for moments values

pub const DRAWN_FORCES_DENOTATION_SHIFT_X: GLElementsValues = 0.01;
pub const DRAWN_FORCES_DENOTATION_SHIFT_Y: GLElementsValues = 0.01;

pub const HINTS_COLOR: &str = "white";
pub const HINT_SHIFT_X: GLElementsValues = 0.05;
pub const ROTATION_HINT_SHIFT_Y: GLElementsValues = 0.85;
pub const ZOOM_HINT_SHIFT_Y: GLElementsValues = 0.9;
pub const PAN_HINT_SHIFT_Y: GLElementsValues = 0.95;

pub const DRAWN_DEFORMED_SHAPE_NODES_COLOR: [GLElementsValues; 4] = [1.0, 1.0, 1.0, 1.0]; // white
pub const DRAWN_DEFORMED_SHAPE_ELEMENTS_COLOR: [GLElementsValues; 4] = [1.0, 1.0, 1.0, 1.0]; // white
pub const CANVAS_DRAWN_DEFORMED_SHAPE_NODES_DENOTATION_COLOR: &str = "white";
pub const DRAWN_DEFORMED_SHAPE_NODES_DENOTATION_SHIFT: GLElementsValues = 0.02;

pub const DRAWN_OBJECT_SELECTED_COLOR: [GLElementsValues; 4] = [1.0, 0.0, 0.0, 1.0]; // red
pub const CANVAS_DRAWN_OBJECT_SELECTED_DENOTATION_COLOR: &str = "red";
pub const DRAWN_OBJECT_UNDER_CURSOR_COLOR: [GLElementsValues; 4] =
    [0.752941, 0.752941, 0.752941, 1.0]; // grey
pub const CANVAS_DRAWN_OBJECT_UNDER_CURSOR_DENOTATION_COLOR: &str = "grey";

pub const DISPLACEMENT_SHIFT_X: GLElementsValues = 0.05;
pub const DISPLACEMENT_HEADER_SHIFT_Y: GLElementsValues = 0.1;
pub const MIN_DISPLACEMENT_SHIFT_Y: GLElementsValues = 0.15;
pub const MAX_DISPLACEMENT_SHIFT_Y: GLElementsValues = 0.2;

pub const REACTION_SHIFT_X: GLElementsValues = 0.05;
pub const REACTION_HEADER_SHIFT_Y: GLElementsValues = 0.1;

pub const DRAWN_OBJECT_DEFAULT_RESULT_COLOR: [GLElementsValues; 4] = [0.0, 0.0, 1.0, 1.0]; // blue

pub const EAR_SHIFT_X: GLElementsValues = 0.05;
pub const EAR_HEADER_SHIFT_Y: GLElementsValues = 0.1;
pub const EAR_COMPONENT_SHIFT_Y: GLElementsValues = 0.15;
pub const EAR_MIN_MAX_VALUE_SHIFT_X: GLElementsValues = 0.07;

pub const COLOR_BAR_SHIFT_X: GLElementsValues = 0.05;
pub const COLOR_BAR_Y_BOTTOM: GLElementsValues = 0.45;
pub const COLOR_BAR_Y_TOP: GLElementsValues = 0.2;
pub const COLOR_BAR_WIDTH: GLElementsValues = 0.015;


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


    pub fn add_nodes(&mut self, normalized_nodes: &[NormalizedNode], gl_mode: GLMode,
        under_cursor_color: &[u8; 4], selected_color: &[u8; 4])
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        for (i, node) in normalized_nodes.iter().enumerate()
        {
            self.vertices_coordinates.extend(&[node.x, node.y, node.z]);
            let node_color = define_drawn_object_color(&gl_mode, node.uid, selected_color,
                under_cursor_color,&DRAWN_NODES_COLOR);
            self.colors_values.extend(&node_color);
            self.indexes_numbers.push(start_index + i as GLElementsNumbers);
        }
        self.modes.push(GLPrimitiveType::Points);
        self.elements_numbers.push(normalized_nodes.len() as i32);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    pub fn add_deformed_shape_nodes(&mut self, normalized_nodes: &[NormalizedNode], gl_mode: GLMode,
        under_cursor_color: &[u8; 4], selected_color: &[u8; 4])
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        for (i, node) in normalized_nodes.iter().enumerate()
        {
            self.vertices_coordinates.extend(&[node.x, node.y, node.z]);
            let node_color = define_drawn_object_color(&gl_mode, node.uid, selected_color,
                under_cursor_color,&DRAWN_DEFORMED_SHAPE_NODES_COLOR);
            self.colors_values.extend(&node_color);
            self.indexes_numbers.push(start_index + i as GLElementsNumbers);
        }
        self.modes.push(GLPrimitiveType::Points);
        self.elements_numbers.push(normalized_nodes.len() as i32);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    pub fn add_elements(&mut self, normalized_nodes: &[NormalizedNode],
        drawn_elements: &Vec<FEDrawnElementData>, gl_mode: GLMode,
        under_cursor_color: &[u8; 4], selected_color: &[u8; 4]) -> Result<(), String>
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
                let point_element_color = define_drawn_object_color(&gl_mode,
                    point_element.uid, selected_color, under_cursor_color,
                    &DRAWN_ELEMENTS_COLOR);
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
                self.colors_values.extend(&point_element_color);
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
                let line_element_color = define_drawn_object_color(&gl_mode,
                    line_element.uid, selected_color, under_cursor_color,
                    &DRAWN_ELEMENTS_COLOR);
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
                self.colors_values.extend(&line_element_color);
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
                self.colors_values.extend(&line_element_color);
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
                let complex_element_color = define_drawn_object_color(&gl_mode,
                    complex_element.uid, selected_color, under_cursor_color,
                    &DRAWN_ELEMENTS_COLOR);
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
                    self.colors_values.extend(&complex_element_color);
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
                    self.colors_values.extend(&complex_element_color);
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
                self.colors_values.extend(&complex_element_color);
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
                self.colors_values.extend(&complex_element_color);
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


    pub fn add_deformed_shape_elements(&mut self, normalized_nodes: &[NormalizedNode],
        drawn_elements: &Vec<FEDrawnElementData>, gl_mode: GLMode,
        under_cursor_color: &[u8; 4], selected_color: &[u8; 4]) -> Result<(), String>
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
                let point_element_color = define_drawn_object_color(&gl_mode,
                    point_element.uid, selected_color, under_cursor_color,
                    &DRAWN_DEFORMED_SHAPE_ELEMENTS_COLOR);
                let node_number = point_element.nodes_numbers[0] as GLElementsNumbers +
                    normalized_nodes.len() as GLElementsNumbers;
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
                self.colors_values.extend(&point_element_color);
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
                let line_element_color = define_drawn_object_color(&gl_mode,
                    line_element.uid, selected_color, under_cursor_color,
                    &DRAWN_DEFORMED_SHAPE_ELEMENTS_COLOR);
                let node_1_number = line_element.nodes_numbers[0] as GLElementsNumbers +
                    normalized_nodes.len() as GLElementsNumbers;
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
                self.colors_values.extend(&line_element_color);
                self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                count += 1;
                let node_2_number = line_element.nodes_numbers[1] as GLElementsNumbers +
                    normalized_nodes.len() as GLElementsNumbers;
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
                self.colors_values.extend(&line_element_color);
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
                let complex_element_color = define_drawn_object_color(&gl_mode,
                    complex_element.uid, selected_color, under_cursor_color,
                    &DRAWN_DEFORMED_SHAPE_ELEMENTS_COLOR);
                for i in 1..complex_element.nodes_numbers.len()
                {
                    let node_1_number =
                        complex_element.nodes_numbers[i - 1] as GLElementsNumbers +
                            normalized_nodes.len() as GLElementsNumbers;
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
                    self.colors_values.extend(&complex_element_color);
                    self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                    count += 1;
                    let node_2_number = complex_element.nodes_numbers[i] as GLElementsNumbers +
                        normalized_nodes.len() as GLElementsNumbers;
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
                    self.colors_values.extend(&complex_element_color);
                    self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                    count += 1;
                }
                let node_1_number = complex_element.nodes_numbers[0] as GLElementsNumbers +
                    normalized_nodes.len() as GLElementsNumbers;
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
                self.colors_values.extend(&complex_element_color);
                self.indexes_numbers.push(start_index + count as GLElementsNumbers);
                count += 1;
                let node_2_number =
                    complex_element.nodes_numbers[complex_element.nodes_numbers.len() - 1]
                        as GLElementsNumbers + normalized_nodes.len() as GLElementsNumbers;
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
                self.colors_values.extend(&complex_element_color);
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


    pub fn add_elements_with_results_for_selection(&mut self, normalized_nodes: &[NormalizedNode],
        drawn_elements: &Vec<FEDrawnElementData>,
        drawn_analysis_results_for_elements: &Vec<DrawnAnalysisResultElementData>)
        -> Result<(), String>
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
                let uid =
                    {
                        if let Some(position) = drawn_analysis_results_for_elements
                            .iter()
                            .position(|data|
                                data.element_analysis_data.number_same(point_element.number))
                        {
                            drawn_analysis_results_for_elements[position].uid
                        }
                        else
                        {
                            point_element.uid
                        }
                    };
                let default_color = [0; 4];
                let point_element_color = define_drawn_object_color(
                    &GLMode::Selection,
                    uid, &default_color, &default_color,
                    &DRAWN_ELEMENTS_COLOR);
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
                self.colors_values.extend(&point_element_color);
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
                let uid =
                    {
                        if let Some(position) = drawn_analysis_results_for_elements
                            .iter()
                            .position(|data|
                                data.element_analysis_data.number_same(line_element.number))
                        {
                            drawn_analysis_results_for_elements[position].uid
                        }
                        else
                        {
                            line_element.uid
                        }
                    };
                let default_color = [0; 4];
                let line_element_color = define_drawn_object_color(
                    &GLMode::Selection,
                    uid, &default_color, &default_color,
                    &DRAWN_ELEMENTS_COLOR);
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
                self.colors_values.extend(&line_element_color);
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
                self.colors_values.extend(&line_element_color);
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
                let uid =
                    {
                        if let Some(position) = drawn_analysis_results_for_elements
                            .iter()
                            .position(|data|
                                data.element_analysis_data.number_same(complex_element.number))
                        {
                            drawn_analysis_results_for_elements[position].uid
                        }
                        else
                        {
                            complex_element.uid
                        }
                    };
                let default_color = [0; 4];
                let complex_element_color = define_drawn_object_color(
                    &GLMode::Selection,
                    uid, &default_color, &default_color,
                    &DRAWN_ELEMENTS_COLOR);
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
                    self.colors_values.extend(&complex_element_color);
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
                    self.colors_values.extend(&complex_element_color);
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
                self.colors_values.extend(&complex_element_color);
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
                self.colors_values.extend(&complex_element_color);
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


    pub fn add_elements_with_results_for_visualization(&mut self,
        normalized_nodes: &[NormalizedNode],
        drawn_elements: &Vec<FEDrawnElementData>,
        drawn_analysis_results_for_elements: &Vec<DrawnAnalysisResultElementData>,
        ear_type: &EARType,
        component: &Box<dyn EARComponentTrait>,
        min_selected_value: &Option<ElementsValues>,
        max_selected_value: &Option<ElementsValues>,
        gl_mode: GLMode,
        under_cursor_color: &[u8; 4], selected_color: &[u8; 4]) -> Result<(), String>
    {
        let start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        let mut lines_count = 0;
        for element in drawn_elements.iter()
            .filter(|elem| elem.fe_type == FEType::Truss2n2ip)
        {
            let uid =
            {
                if let Some(position) = drawn_analysis_results_for_elements
                    .iter()
                    .position(|data|
                        data.element_analysis_data.number_same(element.number))
                {
                    drawn_analysis_results_for_elements[position].uid
                }
                else
                {
                    element.uid
                }
            };
            if min_selected_value.is_none() || max_selected_value.is_none()
            {
                let element_color = define_drawn_object_color(&gl_mode,
                    uid, selected_color, under_cursor_color,
                    &DRAWN_OBJECT_DEFAULT_RESULT_COLOR);
                let node_1_number = element.nodes_numbers[0] as GLElementsNumbers;
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
                self.colors_values.extend(&element_color);
                self.indexes_numbers.push(start_index + lines_count as GLElementsNumbers);
                lines_count += 1;
                let node_2_number = element.nodes_numbers[1] as GLElementsNumbers;
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
                self.colors_values.extend(&element_color);
                self.indexes_numbers.push(start_index + lines_count as GLElementsNumbers);
                lines_count += 1;
            }
            else
            {
                let min_value = min_selected_value.unwrap();
                let max_value = max_selected_value.unwrap();
                if let Some(position) = drawn_analysis_results_for_elements.iter()
                    .position(|data|
                        data.element_analysis_data.number_same(element.number))
                {
                    let element_analysis_data =
                        &drawn_analysis_results_for_elements[position].element_analysis_data;

                    let (values, components) =
                        {
                            match ear_type
                            {
                                EARType::Stress =>
                                    {
                                        let mut boxed_components: Vec<Box<dyn EARComponentTrait>> =
                                            Vec::new();
                                        let data =
                                            element_analysis_data.extract_stresses();
                                        for component in
                                            data.stresses_components.iter()
                                        {
                                            boxed_components.push(Box::new(*component))
                                        }
                                        (data.stresses_values, boxed_components)
                                    },
                                EARType::Strain =>
                                    {
                                        let mut boxed_components: Vec<Box<dyn EARComponentTrait>> =
                                            Vec::new();
                                        let data =
                                            element_analysis_data.extract_strains();
                                        for component in
                                            data.strains_components.iter()
                                        {
                                            boxed_components.push(Box::new(*component))
                                        }
                                        (data.strains_values, boxed_components)
                                    },
                                EARType::Force =>
                                    {
                                        let mut boxed_components: Vec<Box<dyn EARComponentTrait>> =
                                            Vec::new();
                                        let data =
                                            element_analysis_data.extract_forces();
                                        for component in
                                            data.forces_components.iter()
                                        {
                                            boxed_components.push(Box::new(*component))
                                        }
                                        (data.forces_values, boxed_components)
                                    },
                            }
                        };
                    if components.len() == 1 && components[0].same(component)
                    {
                        let value = values[0];
                        let color_value = define_color_value(value, min_value,
                            max_value);
                        let color_array = define_color_array_by_value(color_value);
                        let element_color = define_drawn_object_color(&gl_mode,
                            uid, selected_color, under_cursor_color,
                            &color_array);
                        let node_1_number = element.nodes_numbers[0] as GLElementsNumbers;
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
                        self.colors_values.extend(&element_color);
                        self.indexes_numbers.push(start_index +
                            lines_count as GLElementsNumbers);
                        lines_count += 1;
                        let node_2_number = element.nodes_numbers[1] as GLElementsNumbers;
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
                        self.colors_values.extend(&element_color);
                        self.indexes_numbers.push(start_index +
                            lines_count as GLElementsNumbers);
                        lines_count += 1;
                    }
                    else if components
                        .iter()
                        .all(|comp| comp.same(component))
                    {
                        let node_1_number = element.nodes_numbers[0] as GLElementsNumbers;
                        let node_1_coordinates =
                            match find_node_coordinates(node_1_number, normalized_nodes)
                            {
                                Ok(coordinates) => coordinates,
                                Err(e) =>
                                    {
                                        return Err(e);
                                    }
                            };
                        let node_2_number = element.nodes_numbers[1] as GLElementsNumbers;
                        let node_2_coordinates =
                            match find_node_coordinates(node_2_number, normalized_nodes)
                            {
                                Ok(coordinates) => coordinates,
                                Err(e) =>
                                    {
                                        return Err(e);
                                    }
                            };
                        let values_number = values.len();
                        let value_at_node_1 = values[0];
                        let value_at_node_2 = values[values_number - 1];
                        let color_value_at_node_1 = define_color_value(value_at_node_1,
                            min_value, max_value);
                        let color_value_at_node_2 = define_color_value(value_at_node_2,
                            min_value, max_value);
                        let element_chunks =
                            if value_at_node_1 < value_at_node_2
                            {
                                define_element_chunks(&[color_value_at_node_1,
                                    color_value_at_node_2], &[&node_1_coordinates,
                                    &node_2_coordinates])
                            }
                            else
                            {
                                define_element_chunks(&[color_value_at_node_2,
                                    color_value_at_node_1], &[&node_2_coordinates,
                                    &node_1_coordinates])
                            };
                        for i in 1..element_chunks.0.len()
                        {
                            let color_a = define_drawn_object_color(&gl_mode,
                                uid, selected_color, under_cursor_color,
                                &define_color_array_by_value(
                                    element_chunks.0[i - 1]));
                            let coordinates_a = element_chunks.1[i - 1];
                            self.vertices_coordinates.extend(&coordinates_a);
                            self.colors_values.extend(&color_a);
                            self.indexes_numbers.push(start_index +
                                lines_count as GLElementsNumbers);
                            lines_count += 1;
                            let color_b = define_drawn_object_color(&gl_mode,
                                uid, selected_color, under_cursor_color,
                                &define_color_array_by_value(
                                    element_chunks.0[i]));
                            let coordinates_b = element_chunks.1[i];
                            self.vertices_coordinates.extend(&coordinates_b);
                            self.colors_values.extend(&color_b);
                            self.indexes_numbers.push(start_index +
                                lines_count as GLElementsNumbers);
                            lines_count += 1;
                        }
                    }
                }
            }
        }
        self.modes.push(GLPrimitiveType::Lines);
        self.elements_numbers.push(lines_count as i32);
        let offset = self.define_offset();
        self.offsets.push(offset);
        Ok(())
    }


    pub fn add_displacements(&mut self, normalized_nodes: &Vec<NormalizedNode>,
        drawn_displacements: &Vec<&FEDrawnBCData>, base_points_number: GLElementsNumbers,
        height: GLElementsValues, base_radius: GLElementsValues, gl_mode: GLMode,
        under_cursor_color: &[u8; 4], selected_color: &[u8; 4])
    {
        let mut start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        let tolerance = TOLERANCE as GLElementsValues;
        for displacement in drawn_displacements.iter()
        {
            let displacement_color = define_drawn_object_color(&gl_mode,
                displacement.uid, selected_color, under_cursor_color,
                &DRAWN_DISPLACEMENTS_COLOR);
            if let Some(position) = normalized_nodes
                .iter()
                .position(|node| node.number ==
                    displacement.node_number as GLElementsNumbers)
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
                        self.colors_values.extend(&displacement_color);
                        self.colors_values.extend(&displacement_color);
                        self.colors_values.extend(&displacement_color);
                    }
                    else
                    {
                        self.colors_values.extend(&displacement_color);
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


    fn add_force_line(&mut self, dof_parameter: GlobalDOFParameter, value: ElementsValues,
        x_start: GLElementsValues, y_start: GLElementsValues, z_start: GLElementsValues,
        line_length: GLElementsValues, start_index: GLElementsNumbers,
        force_color: &[GLElementsValues; 4])
        -> (GLElementsValues, GLElementsValues, GLElementsValues)
    {
        let (x_end, y_end, z_end) =
            {
                match dof_parameter
                {
                    GlobalDOFParameter::X =>
                        {
                            let x_end =
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    x_start + line_length
                                }
                                else
                                {
                                    x_start - line_length
                                }
                            };
                            let y_end = y_start;
                            let z_end = z_start;
                            (x_end, y_end, z_end)
                        },
                    GlobalDOFParameter::Y =>
                        {
                            let x_end = x_start;
                            let y_end =
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    y_start + line_length
                                }
                                else
                                {
                                    y_start - line_length
                                }
                            };
                            let z_end = z_start;
                            (x_end, y_end, z_end)
                        },
                    GlobalDOFParameter::Z =>
                        {
                            let x_end = x_start;
                            let y_end = y_start;
                            let z_end =
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    z_start + line_length
                                }
                                else
                                {
                                    z_start - line_length
                                }
                            };
                            (x_end, y_end, z_end)
                        },
                    GlobalDOFParameter::ThX =>
                        {
                            let x_end =
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    x_start + line_length * DRAWN_FORCES_LINE_LENGTH_COEFFICIENT
                                }
                                else
                                {
                                    x_start - line_length * DRAWN_FORCES_LINE_LENGTH_COEFFICIENT
                                }
                            };
                            let y_end = y_start;
                            let z_end = z_start;
                            (x_end, y_end, z_end)
                        },
                    GlobalDOFParameter::ThY =>
                        {
                            let x_end = x_start;
                            let y_end =
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    y_start + line_length * DRAWN_FORCES_LINE_LENGTH_COEFFICIENT
                                }
                                else
                                {
                                    y_start - line_length * DRAWN_FORCES_LINE_LENGTH_COEFFICIENT
                                }
                            };
                            let z_end = z_start;
                            (x_end, y_end, z_end)
                        },
                    GlobalDOFParameter::ThZ =>
                        {
                            let x_end = x_start;
                            let y_end = y_start;
                            let z_end =
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    z_start + line_length * DRAWN_FORCES_LINE_LENGTH_COEFFICIENT
                                }
                                else
                                {
                                    z_start - line_length * DRAWN_FORCES_LINE_LENGTH_COEFFICIENT
                                }
                            };
                            (x_end, y_end, z_end)
                        },
                }
            };
        self.vertices_coordinates.extend(&[x_start, y_start, z_start]);
        self.vertices_coordinates.extend(&[x_end, y_end, z_end]);
        self.colors_values.extend(force_color);
        self.colors_values.extend(force_color);
        self.indexes_numbers.extend(&[start_index, start_index + 1]);
        self.modes.push(GLPrimitiveType::Lines);
        self.elements_numbers.push(2);
        let offset = self.define_offset();
        self.offsets.push(offset);
        (x_end, y_end, z_end)
    }


    fn add_force_cap(&mut self, dof_parameter: GlobalDOFParameter, value: ElementsValues,
        base_points_number: GLElementsNumbers, height: GLElementsValues,
        base_radius: GLElementsValues, x_end: GLElementsValues, y_end: GLElementsValues,
        z_end: GLElementsValues, start_index: GLElementsNumbers,
        force_color: &[GLElementsValues; 4])
    {
        self.vertices_coordinates.extend(&[x_end, y_end, z_end]);
        let tolerance = TOLERANCE as GLElementsValues;
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
                {
                    match dof_parameter
                    {
                        GlobalDOFParameter::X =>
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    [x_end - height, y_end + local_y, z_end + local_x]
                                }
                                else
                                {
                                    [x_end + height, y_end + local_y, z_end + local_x]
                                }
                            },
                        GlobalDOFParameter::Y =>
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    [x_end + local_y, y_end - height, z_end + local_x]
                                }
                                else
                                {
                                    [x_end + local_y, y_end + height, z_end + local_x]
                                }
                            },
                        GlobalDOFParameter::Z =>
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    [x_end + local_x, y_end + local_y, z_end - height]
                                }
                                else
                                {
                                    [x_end + local_x, y_end + local_y, z_end + height]
                                }
                            },
                        GlobalDOFParameter::ThX =>
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    [x_end - height * DRAWN_FORCES_CAPS_LENGTH_COEFFICIENT,
                                    y_end + local_y, z_end + local_x]
                                }
                                else
                                {
                                    [x_end + height * DRAWN_FORCES_CAPS_LENGTH_COEFFICIENT,
                                    y_end + local_y, z_end + local_x]
                                }
                            },
                        GlobalDOFParameter::ThY =>
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    [x_end + local_y,
                                    y_end - height * DRAWN_FORCES_CAPS_LENGTH_COEFFICIENT,
                                    z_end + local_x]
                                }
                                else
                                {
                                    [x_end + local_y,
                                    y_end + height * DRAWN_FORCES_CAPS_LENGTH_COEFFICIENT,
                                    z_end + local_x]
                                }
                            },
                        GlobalDOFParameter::ThZ =>
                            {
                                if value >= 0.0 as ElementsValues
                                {
                                    [x_end + local_x, y_end + local_y,
                                    z_end - height * DRAWN_FORCES_CAPS_LENGTH_COEFFICIENT]
                                }
                                else
                                {
                                    [x_end + local_x, y_end + local_y,
                                    z_end + height * DRAWN_FORCES_CAPS_LENGTH_COEFFICIENT]
                                }
                            },
                    }
                };
            self.vertices_coordinates.extend(&coordinates);
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.colors_values.extend(force_color);
                self.colors_values.extend(force_color);
                self.colors_values.extend(force_color);
            }
            else
            {
                self.colors_values.extend(force_color);
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
    }


    pub fn add_forces(&mut self, normalized_nodes: &Vec<NormalizedNode>,
        drawn_forces: &Vec<&FEDrawnBCData>, line_length: GLElementsValues,
        base_points_number: GLElementsNumbers, height: GLElementsValues,
        base_radius: GLElementsValues, gl_mode: GLMode, under_cursor_color: &[u8; 4],
        selected_color: &[u8; 4])
    {
        let mut start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        for force in drawn_forces.iter()
        {
            let force_color = define_drawn_object_color(&gl_mode,
                force.uid, selected_color, under_cursor_color,
                &DRAWN_FORCES_COLOR);
            if let Some(position) = normalized_nodes
                .iter()
                .position(|node|
                    node.number == force.node_number as GLElementsNumbers)
            {
                let x_start = normalized_nodes[position].x;
                let y_start = normalized_nodes[position].y;
                let z_start = normalized_nodes[position].z;
                if let Some(x_value) = force.x_direction_value
                {
                    let (x_end, y_end, z_end) =
                        self.add_force_line(GlobalDOFParameter::X, x_value, x_start,
                            y_start, z_start, line_length, start_index, &force_color);
                    start_index += 2;
                    self.add_force_cap(GlobalDOFParameter::X, x_value,
                        base_points_number, height, base_radius, x_end, y_end, z_end, start_index,
                        &force_color);
                    start_index += base_points_number + 1;
                }
                if let Some(y_value) = force.y_direction_value
                {
                    let (x_end, y_end, z_end) =
                        self.add_force_line(GlobalDOFParameter::Y, y_value, x_start,
                            y_start, z_start, line_length, start_index, &force_color);
                    start_index += 2;
                    self.add_force_cap(GlobalDOFParameter::Y, y_value,
                        base_points_number, height, base_radius, x_end, y_end, z_end, start_index,
                        &force_color);
                    start_index += base_points_number + 1;
                }
                if let Some(z_value) = force.z_direction_value
                {
                    let (x_end, y_end, z_end) =
                        self.add_force_line(GlobalDOFParameter::Z, z_value, x_start,
                            y_start, z_start, line_length, start_index, &force_color);
                    start_index += 2;
                    self.add_force_cap(GlobalDOFParameter::Z, z_value,
                        base_points_number, height, base_radius, x_end, y_end, z_end, start_index,
                        &force_color);
                    start_index += base_points_number + 1;
                }
            }
        }
    }


    pub fn add_reactions(&mut self, normalized_nodes: &[NormalizedNode],
        reactions: &Vec<&FEDrawnAnalysisResultNodeData>,
        line_length: GLElementsValues, base_points_number: GLElementsNumbers,
        height: GLElementsValues, base_radius: GLElementsValues, gl_mode: GLMode,
        under_cursor_color: &[u8; 4], selected_color: &[u8; 4])
    {
        let mut start_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        for reaction in reactions
        {
            let reaction_color = define_drawn_object_color(&gl_mode,
                reaction.uid, selected_color, under_cursor_color,
                &DRAWN_FORCES_COLOR);
            if let Some(position) = normalized_nodes
                .iter()
                .position(|node|
                    node.number == reaction.node_number as GLElementsNumbers)
            {
                let x_start = normalized_nodes[position].x;
                let y_start = normalized_nodes[position].y;
                let z_start = normalized_nodes[position].z;
                if let Some(x_value) = reaction.x_direction_value
                {
                    let (x_end, y_end, z_end) =
                        self.add_force_line(GlobalDOFParameter::X, x_value, x_start,
                            y_start, z_start, line_length, start_index, &reaction_color);
                    start_index += 2;
                    self.add_force_cap(GlobalDOFParameter::X, x_value,
                        base_points_number, height, base_radius, x_end, y_end, z_end, start_index,
                        &reaction_color);
                    start_index += base_points_number + 1;
                }
                if let Some(y_value) = reaction.y_direction_value
                {
                    let (x_end, y_end, z_end) =
                        self.add_force_line(GlobalDOFParameter::Y, y_value, x_start,
                            y_start, z_start, line_length, start_index, &reaction_color);
                    start_index += 2;
                    self.add_force_cap(GlobalDOFParameter::Y, y_value,
                        base_points_number, height, base_radius, x_end, y_end, z_end, start_index,
                        &reaction_color);
                    start_index += base_points_number + 1;
                }
                if let Some(z_value) = reaction.z_direction_value
                {
                    let (x_end, y_end, z_end) =
                        self.add_force_line(GlobalDOFParameter::Z, z_value, x_start,
                            y_start, z_start, line_length, start_index, &reaction_color);
                    start_index += 2;
                    self.add_force_cap(GlobalDOFParameter::Z, z_value,
                        base_points_number, height, base_radius, x_end, y_end, z_end, start_index,
                        &reaction_color);
                    start_index += base_points_number + 1;
                }
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