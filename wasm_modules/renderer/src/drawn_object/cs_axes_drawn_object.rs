use std::f32::consts::PI;
use web_sys::{WebGlRenderingContext as GL};


use crate::drawn_object::drawn_object::DrawnObject;
use crate::drawn_object::consts::
{
    CS_ORIGIN, CS_AXIS_X, CS_AXIS_X_COLOR, CS_AXIS_Y, CS_AXIS_Y_COLOR, CS_AXIS_Z, CS_AXIS_Z_COLOR
};
use crate::drawn_object::functions::create_monochrome_cone;

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
        let (axes_x_cap_vertices_coordinates, axis_x_cap_vertices_colors_values,
            axes_x_cap_vertices_indexes) = create_monochrome_cone(
                &CS_AXIS_X, &[1.0 - height, 0.0, 0.0],
                height, base_radius, base_points_number, start_x_axis_cap_index,
                &CS_AXIS_X_COLOR, TOLERANCE)?;
        self.drawn_object.add_triangle_vertex_coordinates(
            &axes_x_cap_vertices_coordinates)?;
        self.drawn_object.add_triangle_vertex_color_value(
            &axis_x_cap_vertices_colors_values)?;
        self.drawn_object.add_triangles_vertices_indexes(&axes_x_cap_vertices_indexes)?;

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
        let (axes_y_cap_vertices_coordinates, axis_y_cap_vertices_colors_values,
            axes_y_cap_vertices_indexes) = create_monochrome_cone(
                &CS_AXIS_Y, &[0.0, 1.0 - height, 0.0],
                height, base_radius, base_points_number, start_y_axis_cap_index,
                &CS_AXIS_Y_COLOR, TOLERANCE)?;
        self.drawn_object.add_triangle_vertex_coordinates(
            &axes_y_cap_vertices_coordinates)?;
        self.drawn_object.add_triangle_vertex_color_value(
            &axis_y_cap_vertices_colors_values)?;
        self.drawn_object.add_triangles_vertices_indexes(&axes_y_cap_vertices_indexes)?;

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
        let (axes_z_cap_vertices_coordinates, axis_z_cap_vertices_colors_values,
            axes_z_cap_vertices_indexes) = create_monochrome_cone(
                &CS_AXIS_Z, &[0.0, 0.0, 1.0 - height],
                height, base_radius, base_points_number, start_z_axis_cap_index,
                &CS_AXIS_Z_COLOR, TOLERANCE)?;
        self.drawn_object.add_triangle_vertex_coordinates(
            &axes_z_cap_vertices_coordinates)?;
        self.drawn_object.add_triangle_vertex_color_value(
            &axis_z_cap_vertices_colors_values)?;
        self.drawn_object.add_triangles_vertices_indexes(&axes_z_cap_vertices_indexes)?;

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
