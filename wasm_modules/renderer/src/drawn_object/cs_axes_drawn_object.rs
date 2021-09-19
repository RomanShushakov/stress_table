use std::f32::consts::PI;
use web_sys::{WebGlRenderingContext as GL};

use crate::drawn_object::consts::
{
    CS_ORIGIN, CS_AXIS_X, CS_AXIS_X_COLOR, CS_AXIS_Y, CS_AXIS_Y_COLOR, CS_AXIS_Z, CS_AXIS_Z_COLOR
};

use crate::consts::TOLERANCE;


#[derive(Clone)]
pub struct CSAxesDrawnObject
{
    lines_vertices_coordinates: Vec<f32>,
    lines_vertices_colors_values: Vec<f32>,
    triangles_vertices_coordinates: Vec<f32>,
    triangles_vertices_colors_values: Vec<f32>,
    triangles_vertices_indexes: Vec<u32>,
}


impl CSAxesDrawnObject
{
    pub fn create() -> Self
    {
        let lines_vertices_coordinates = Vec::new();
        let lines_vertices_colors_values = Vec::new();
        let triangles_vertices_coordinates= Vec::new();
        let triangles_vertices_colors_values = Vec::new();
        let triangles_vertices_indexes = Vec::new();

        CSAxesDrawnObject
        {
            lines_vertices_coordinates,
            lines_vertices_colors_values,
            triangles_vertices_coordinates,
            triangles_vertices_colors_values,
            triangles_vertices_indexes,
        }
    }


    pub fn add_cs_axes_lines(&mut self)
    {
        self.lines_vertices_coordinates.extend(&CS_ORIGIN);
        self.lines_vertices_coordinates.extend(&CS_AXIS_X);
        self.lines_vertices_colors_values.extend(&CS_AXIS_X_COLOR);
        self.lines_vertices_colors_values.extend(&CS_AXIS_X_COLOR);
        self.lines_vertices_coordinates.extend(&CS_ORIGIN);
        self.lines_vertices_coordinates.extend(&CS_AXIS_Y);
        self.lines_vertices_colors_values.extend(&CS_AXIS_Y_COLOR);
        self.lines_vertices_colors_values.extend(&CS_AXIS_Y_COLOR);
        self.lines_vertices_coordinates.extend(&CS_ORIGIN);
        self.lines_vertices_coordinates.extend(&CS_AXIS_Z);
        self.lines_vertices_colors_values.extend(&CS_AXIS_Z_COLOR);
        self.lines_vertices_colors_values.extend(&CS_AXIS_Z_COLOR);
    }


    pub fn ref_lines_vertices_coordinates(&self) -> &[f32]
    {
        self.lines_vertices_coordinates.as_slice()
    }


    pub fn ref_lines_vertices_colors_values(&self) -> &[f32]
    {
        self.lines_vertices_colors_values.as_slice()
    }


    pub fn draw_axes_lines(&self, gl: &GL)
    {
        gl.draw_arrays(GL::LINES, 0, (self.lines_vertices_coordinates.len() / 3) as i32);
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
            if self.triangles_vertices_indexes.is_empty()
            {
                0
            }
            else
            {
                self.triangles_vertices_indexes[self.triangles_vertices_indexes.len() - 1] + 1
            };
        self.triangles_vertices_coordinates.extend(&CS_AXIS_X);
        for (local_x, local_y) in &local_coordinates
        {
            let coordinates = [1.0 - height, *local_y, *local_x];
            self.triangles_vertices_coordinates.extend(&coordinates);
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.triangles_vertices_colors_values.extend(&CS_AXIS_X_COLOR);
                self.triangles_vertices_colors_values.extend(&CS_AXIS_X_COLOR);
                self.triangles_vertices_colors_values.extend(&CS_AXIS_X_COLOR);
            }
            else
            {
                self.triangles_vertices_colors_values.extend(&CS_AXIS_X_COLOR);
            }
            self.triangles_vertices_indexes.extend(&[start_x_axis_cap_index,
                start_x_axis_cap_index + point_number, start_x_axis_cap_index + point_number + 1]);
        }
        self.triangles_vertices_indexes.extend(&[start_x_axis_cap_index,
            start_x_axis_cap_index + 1, start_x_axis_cap_index + base_points_number]);

        let start_y_axis_cap_index =
            if self.triangles_vertices_indexes.is_empty()
            {
                0
            }
            else
            {
                self.triangles_vertices_indexes[self.triangles_vertices_indexes.len() - 1] + 1
            };
        self.triangles_vertices_coordinates.extend(&CS_AXIS_Y);
        for (local_x, local_y) in &local_coordinates
        {
            let coordinates = [*local_y, 1.0 - height, *local_x];
            self.triangles_vertices_coordinates.extend(&coordinates);
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.triangles_vertices_colors_values.extend(&CS_AXIS_Y_COLOR);
                self.triangles_vertices_colors_values.extend(&CS_AXIS_Y_COLOR);
                self.triangles_vertices_colors_values.extend(&CS_AXIS_Y_COLOR);
            }
            else
            {
                self.triangles_vertices_colors_values.extend(&CS_AXIS_Y_COLOR);
            }
            self.triangles_vertices_indexes.extend(&[start_y_axis_cap_index,
                start_y_axis_cap_index + point_number, start_y_axis_cap_index + point_number + 1]);
        }
        self.triangles_vertices_indexes.extend(&[start_y_axis_cap_index,
            start_y_axis_cap_index + 1, start_y_axis_cap_index + base_points_number]);

        let start_z_axis_cap_index =
            if self.triangles_vertices_indexes.is_empty()
            {
                0
            }
            else
            {
                self.triangles_vertices_indexes[self.triangles_vertices_indexes.len() - 1] + 1
            };
        self.triangles_vertices_coordinates.extend(&CS_AXIS_Z);
        for (local_x, local_y) in &local_coordinates
        {
            let coordinates = [*local_x, *local_y, 1.0 - height];
            self.triangles_vertices_coordinates.extend(&coordinates);
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.triangles_vertices_colors_values.extend(&CS_AXIS_Z_COLOR);
                self.triangles_vertices_colors_values.extend(&CS_AXIS_Z_COLOR);
                self.triangles_vertices_colors_values.extend(&CS_AXIS_Z_COLOR);
            }
            else
            {
                self.triangles_vertices_colors_values.extend(&CS_AXIS_Z_COLOR);
            }
            self.triangles_vertices_indexes.extend(&[start_z_axis_cap_index,
                start_z_axis_cap_index + point_number, start_z_axis_cap_index + point_number + 1]);
        }
        self.triangles_vertices_indexes.extend(&[start_z_axis_cap_index,
            start_z_axis_cap_index + 1, start_z_axis_cap_index + base_points_number]);
    }


    pub fn ref_triangles_vertices_coordinates(&self) -> &[f32]
    {
        self.triangles_vertices_coordinates.as_slice()
    }


    pub fn ref_triangles_vertices_colors_values(&self) -> &[f32]
    {
        self.triangles_vertices_colors_values.as_slice()
    }


    pub fn ref_triangles_vertices_indexes(&self) -> &[u32]
    {
        self.triangles_vertices_indexes.as_slice()
    }


    pub fn draw_axes_caps(&self, gl: &GL)
    {
        gl.draw_elements_with_i32(GL::TRIANGLES, self.triangles_vertices_indexes.len() as i32,
            GL::UNSIGNED_INT, 0);
    }
}
