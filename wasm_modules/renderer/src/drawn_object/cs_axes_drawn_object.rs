use std::f32::consts::PI;
use web_sys::{WebGlRenderingContext as GL};

use crate::drawn_object::drawn_object::DrawnObjectTrait;
use crate::drawn_object::aux_structs::GLPrimitiveType;
use crate::drawn_object::consts::
{
    CS_ORIGIN, CS_AXIS_X, CS_AXIS_X_COLOR, CS_AXIS_Y, CS_AXIS_Y_COLOR, CS_AXIS_Z, CS_AXIS_Z_COLOR
};

use crate::consts::TOLERANCE;


#[derive(Clone)]
pub struct CSAxesDrawnObject
{
    vertices_coordinates: Vec<f32>,
    colors_values: Vec<f32>,
    indexes_numbers: Vec<u32>,
    modes: Vec<GLPrimitiveType>,
    elements_numbers: Vec<i32>,
    offsets: Vec<i32>,
}


impl DrawnObjectTrait for CSAxesDrawnObject
{
    fn ref_vertices_coordinates(&self) -> &[f32]
    {
        self.vertices_coordinates.as_slice()
    }


    fn ref_colors_values(&self) -> &[f32]
    {
        self.colors_values.as_slice()
    }


    fn ref_indexes_numbers(&self) -> &[u32]
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


impl CSAxesDrawnObject
{
    pub fn create() -> Self
    {
        let vertices_coordinates = Vec::new();
        let colors_values = Vec::new();
        let indexes_numbers = Vec::new();
        let modes = Vec::new();
        let elements_numbers = Vec::new();
        let offsets = Vec::new();
        CSAxesDrawnObject {
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


    pub fn add_cs_axes_lines(&mut self)
    {
        let start_x_axis_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        self.vertices_coordinates.extend(&CS_ORIGIN);
        self.vertices_coordinates.extend(&CS_AXIS_X);
        self.colors_values.extend(&CS_AXIS_X_COLOR);
        self.colors_values.extend(&CS_AXIS_X_COLOR);
        self.indexes_numbers.extend(&[start_x_axis_index, start_x_axis_index + 1]);
        let start_y_axis_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        self.vertices_coordinates.extend(&CS_ORIGIN);
        self.vertices_coordinates.extend(&CS_AXIS_Y);
        self.colors_values.extend(&CS_AXIS_Y_COLOR);
        self.colors_values.extend(&CS_AXIS_Y_COLOR);
        self.indexes_numbers.extend(&[start_y_axis_index, start_y_axis_index + 1]);
        let start_z_axis_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        self.vertices_coordinates.extend(&CS_ORIGIN);
        self.vertices_coordinates.extend(&CS_AXIS_Z);
        self.colors_values.extend(&CS_AXIS_Z_COLOR);
        self.colors_values.extend(&CS_AXIS_Z_COLOR);
        self.indexes_numbers.extend(&[start_z_axis_index, start_z_axis_index + 1]);
        self.modes.push(GLPrimitiveType::Lines);
        self.elements_numbers.push(6);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }


    pub fn add_cs_axes_caps(&mut self, base_points_number: u32, height: f32, base_radius: f32)
    {
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
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        self.vertices_coordinates.extend(&CS_AXIS_X);
        for (local_x, local_y) in &local_coordinates
        {
            let coordinates = [1.0 - height, *local_y, *local_x];
            self.vertices_coordinates.extend(&coordinates);
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.colors_values.extend(&CS_AXIS_X_COLOR);
                self.colors_values.extend(&CS_AXIS_X_COLOR);
                self.colors_values.extend(&CS_AXIS_X_COLOR);
            }
            else
            {
                self.colors_values.extend(&CS_AXIS_X_COLOR);
            }
            self.indexes_numbers.extend(&[start_x_axis_cap_index,
                start_x_axis_cap_index + point_number, start_x_axis_cap_index + point_number + 1]);
        }
        self.indexes_numbers.extend(&[start_x_axis_cap_index,
            start_x_axis_cap_index + 1, start_x_axis_cap_index + base_points_number]);

        let start_y_axis_cap_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        self.vertices_coordinates.extend(&CS_AXIS_Y);
        for (local_x, local_y) in &local_coordinates
        {
            let coordinates = [*local_y, 1.0 - height, *local_x];
            self.vertices_coordinates.extend(&coordinates);
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.colors_values.extend(&CS_AXIS_Y_COLOR);
                self.colors_values.extend(&CS_AXIS_Y_COLOR);
                self.colors_values.extend(&CS_AXIS_Y_COLOR);
            }
            else
            {
                self.colors_values.extend(&CS_AXIS_Y_COLOR);
            }
            self.indexes_numbers.extend(&[start_y_axis_cap_index,
                start_y_axis_cap_index + point_number, start_y_axis_cap_index + point_number + 1]);
        }
        self.indexes_numbers.extend(&[start_y_axis_cap_index,
            start_y_axis_cap_index + 1, start_y_axis_cap_index + base_points_number]);

        let start_z_axis_cap_index =
            if let Some(index) = self.indexes_numbers.iter().max() { *index + 1 } else { 0 };
        self.vertices_coordinates.extend(&CS_AXIS_Z);
        for (local_x, local_y) in &local_coordinates
        {
            let coordinates = [*local_x, *local_y, 1.0 - height];
            self.vertices_coordinates.extend(&coordinates);
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.colors_values.extend(&CS_AXIS_Z_COLOR);
                self.colors_values.extend(&CS_AXIS_Z_COLOR);
                self.colors_values.extend(&CS_AXIS_Z_COLOR);
            }
            else
            {
                self.colors_values.extend(&CS_AXIS_Z_COLOR);
            }
            self.indexes_numbers.extend(&[start_z_axis_cap_index,
                start_z_axis_cap_index + point_number, start_z_axis_cap_index + point_number + 1]);
        }
        self.indexes_numbers.extend(&[start_z_axis_cap_index,
            start_z_axis_cap_index + 1, start_z_axis_cap_index + base_points_number]);

        self.modes.push(GLPrimitiveType::Triangles);
        self.elements_numbers.push(base_points_number as i32 * 9);
        let offset = self.define_offset();
        self.offsets.push(offset);
    }
}
