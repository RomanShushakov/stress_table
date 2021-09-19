use std::f32::consts::PI;
use web_sys::{WebGlRenderingContext as GL};


use crate::drawn_object::drawn_object::DrawnObject;
use crate::drawn_object::consts::
{
    CS_ORIGIN, CS_AXIS_X, CS_AXIS_X_COLOR, CS_AXIS_Y, CS_AXIS_Y_COLOR, CS_AXIS_Z, CS_AXIS_Z_COLOR
};

use crate::consts::TOLERANCE;
use wasm_bindgen::JsValue;



pub struct CSAxesDrawnObject
{
    drawn_object: DrawnObject,
}


impl CSAxesDrawnObject
{
    pub fn create() -> Self
    {
        let lines_endpoints_coordinates = Vec::new();
        let lines_endpoints_colors_values = Vec::new();
        let triangles_vertices_coordinates= Vec::new();
        let triangles_vertices_colors_values = Vec::new();
        let triangles_vertices_indexes = Vec::new();

        let drawn_object = DrawnObject::create(None,
            None,
            Some(lines_endpoints_coordinates),
            Some(lines_endpoints_colors_values),
            Some(triangles_vertices_coordinates),
            Some(triangles_vertices_colors_values),
            Some(triangles_vertices_indexes));

        CSAxesDrawnObject { drawn_object }
    }


    pub fn add_cs_axes_lines(&mut self) -> Result<(), JsValue>
    {
        self.drawn_object.add_line_endpoint_coordinates(&CS_ORIGIN)?;
        self.drawn_object.add_line_endpoint_coordinates(&CS_AXIS_X)?;
        self.drawn_object.add_line_endpoint_color_value(&CS_AXIS_X_COLOR)?;
        self.drawn_object.add_line_endpoint_color_value(&CS_AXIS_X_COLOR)?;
        self.drawn_object.add_line_endpoint_coordinates(&CS_ORIGIN)?;
        self.drawn_object.add_line_endpoint_coordinates(&CS_AXIS_Y)?;
        self.drawn_object.add_line_endpoint_color_value(&CS_AXIS_Y_COLOR)?;
        self.drawn_object.add_line_endpoint_color_value(&CS_AXIS_Y_COLOR)?;
        self.drawn_object.add_line_endpoint_coordinates(&CS_ORIGIN)?;
        self.drawn_object.add_line_endpoint_coordinates(&CS_AXIS_Z)?;
        self.drawn_object.add_line_endpoint_color_value(&CS_AXIS_Z_COLOR)?;
        self.drawn_object.add_line_endpoint_color_value(&CS_AXIS_Z_COLOR)?;
        Ok(())
    }


    pub fn ref_lines_endpoints_coordinates(&self) -> Result<&[f32], JsValue>
    {
        self.drawn_object.ref_lines_endpoints_coordinates()
    }


    pub fn ref_lines_endpoints_colors_values(&self) -> Result<&[f32], JsValue>
    {
        self.drawn_object.ref_lines_endpoints_colors_values()
    }


    pub fn draw_lines(&self, gl: &GL) -> Result<(), JsValue>
    {
        self.drawn_object.draw_lines(gl)
    }


    pub fn add_cs_axes_caps(&mut self, base_points_number: u32, height: f32, base_radius: f32)
        -> Result<(), JsValue>
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
            if self.drawn_object.ref_triangles_vertices_indexes()?.is_empty()
            {
                0
            }
            else
            {
                self.drawn_object.ref_triangles_vertices_indexes()?[
                    self.drawn_object.ref_triangles_vertices_indexes()?.len() - 1] + 1
            };
        self.drawn_object.add_triangle_vertex_coordinates(&CS_AXIS_X)?;
        for (local_x, local_y) in &local_coordinates
        {
            let coordinates = [1.0 - height, *local_y, *local_x];
            self.drawn_object.add_triangle_vertex_coordinates(&coordinates)?;
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_X_COLOR)?;
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_X_COLOR)?;
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_X_COLOR)?;
            }
            else
            {
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_X_COLOR)?;
            }
            self.drawn_object.add_triangle_vertex_index(start_x_axis_cap_index)?;
            self.drawn_object.add_triangle_vertex_index(
                start_x_axis_cap_index + point_number)?;
            self.drawn_object.add_triangle_vertex_index(
                start_x_axis_cap_index + point_number + 1)?;
        }
        self.drawn_object.add_triangle_vertex_index(start_x_axis_cap_index)?;
            self.drawn_object.add_triangle_vertex_index(
                start_x_axis_cap_index + 1)?;
            self.drawn_object.add_triangle_vertex_index(
                start_x_axis_cap_index + base_points_number)?;

        let start_y_axis_cap_index =
            if self.ref_triangles_vertices_indexes()?.is_empty()
            {
                0
            }
            else
            {
                self.ref_triangles_vertices_indexes()?[
                    self.ref_triangles_vertices_indexes()?.len() - 1] + 1
            };
        self.drawn_object.add_triangle_vertex_coordinates(&CS_AXIS_Y)?;
        for (local_x, local_y) in &local_coordinates
        {
            let coordinates = [*local_y, 1.0 - height, *local_x];
            self.drawn_object.add_triangle_vertex_coordinates(&coordinates)?;
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_Y_COLOR)?;
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_Y_COLOR)?;
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_Y_COLOR)?;
            }
            else
            {
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_Y_COLOR)?;
            }
            self.drawn_object.add_triangle_vertex_index(start_y_axis_cap_index)?;
            self.drawn_object.add_triangle_vertex_index(
                start_y_axis_cap_index + point_number)?;
            self.drawn_object.add_triangle_vertex_index(
                start_y_axis_cap_index + point_number + 1)?;
        }
        self.drawn_object.add_triangle_vertex_index(start_y_axis_cap_index)?;
        self.drawn_object.add_triangle_vertex_index(start_y_axis_cap_index + 1)?;
        self.drawn_object.add_triangle_vertex_index(
            start_y_axis_cap_index + base_points_number)?;

        let start_z_axis_cap_index =
            if self.ref_triangles_vertices_indexes()?.is_empty()
            {
                0
            }
            else
            {
                self.ref_triangles_vertices_indexes()?[
                    self.ref_triangles_vertices_indexes()?.len() - 1] + 1
            };
        self.drawn_object.add_triangle_vertex_coordinates(&CS_AXIS_Z)?;
        for (local_x, local_y) in &local_coordinates
        {
            let coordinates = [*local_x, *local_y, 1.0 - height];
            self.drawn_object.add_triangle_vertex_coordinates(&coordinates)?;
        }
        for point_number in 1..base_points_number
        {
            if point_number == 1
            {
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_Z_COLOR)?;
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_Z_COLOR)?;
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_Z_COLOR)?;
            }
            else
            {
                self.drawn_object.add_triangle_vertex_color_value(&CS_AXIS_Z_COLOR)?;
            }
            self.drawn_object.add_triangle_vertex_index(start_z_axis_cap_index)?;
            self.drawn_object.add_triangle_vertex_index(
                start_z_axis_cap_index + point_number)?;
            self.drawn_object.add_triangle_vertex_index(
                start_z_axis_cap_index + point_number + 1)?;
        }
        self.drawn_object.add_triangle_vertex_index(start_z_axis_cap_index)?;
        self.drawn_object.add_triangle_vertex_index(start_z_axis_cap_index + 1)?;
        self.drawn_object.add_triangle_vertex_index(
            start_z_axis_cap_index + base_points_number)?;
        Ok(())
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


    pub fn draw_triangles(&self, gl: &GL) -> Result<(), JsValue>
    {
        self.drawn_object.draw_triangles(gl)
    }
}
