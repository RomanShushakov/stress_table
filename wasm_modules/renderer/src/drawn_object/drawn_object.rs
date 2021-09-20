use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext as GL};


pub struct DrawnObject
{
    optional_points_coordinates: Option<Vec<f32>>,
    optional_points_colors_values: Option<Vec<f32>>,
    optional_lines_endpoints_coordinates: Option<Vec<f32>>,
    optional_lines_endpoints_colors_values: Option<Vec<f32>>,
    optional_triangles_vertices_coordinates: Option<Vec<f32>>,
    optional_triangles_vertices_colors_values: Option<Vec<f32>>,
    optional_triangles_vertices_indexes: Option<Vec<u32>>,
}


impl DrawnObject
{
    pub fn create(optional_points_coordinates: Option<Vec<f32>>,
        optional_points_colors_values: Option<Vec<f32>>,
        optional_lines_endpoints_coordinates: Option<Vec<f32>>,
        optional_lines_endpoints_colors_values: Option<Vec<f32>>,
        optional_triangles_vertices_coordinates: Option<Vec<f32>>,
        optional_triangles_vertices_colors_values: Option<Vec<f32>>,
        optional_triangles_vertices_indexes: Option<Vec<u32>>) -> Self
    {
        DrawnObject { optional_points_coordinates, optional_points_colors_values,
            optional_lines_endpoints_coordinates, optional_lines_endpoints_colors_values,
            optional_triangles_vertices_coordinates, optional_triangles_vertices_colors_values,
            optional_triangles_vertices_indexes }
    }


    pub fn add_point_coordinates(&mut self, point_coordinates: &[f32]) -> Result<(), JsValue>
    {
        if let Some(points_coordinates) = self.optional_points_coordinates.as_mut()
        {
            points_coordinates.extend(point_coordinates);
            Ok(())
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Add point coordinates: \
                'optional_points_coordinates' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_point_color_value(&mut self, point_color_value: &[f32]) -> Result<(), JsValue>
    {
        if let Some(points_colors_values) = self.optional_points_colors_values.as_mut()
        {
            points_colors_values.extend(point_color_value);
            Ok(())
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Add point color value: \
                'optional_points_colors_values' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_line_endpoint_coordinates(&mut self, endpoint_coordinates: &[f32])
        -> Result<(), JsValue>
    {
        if let Some(lines_endpoints_coordinates) =
            self.optional_lines_endpoints_coordinates.as_mut()
        {
            lines_endpoints_coordinates.extend(endpoint_coordinates);
            Ok(())
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Add line endpoint coordinates: \
                'optional_lines_endpoints_coordinates' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_line_endpoint_color_value(&mut self, endpoint_color_value: &[f32])
        -> Result<(), JsValue>
    {
        if let Some(lines_endpoints_colors_values) =
            self.optional_lines_endpoints_colors_values.as_mut()
        {
            lines_endpoints_colors_values.extend(endpoint_color_value);
            Ok(())
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Add line endpoint color value: \
                'optional_lines_endpoints_colors_values' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_triangle_vertex_coordinates(&mut self, vertex_coordinates: &[f32])
        -> Result<(), JsValue>
    {
        if let Some(triangles_vertices_coordinates) =
            self.optional_triangles_vertices_coordinates.as_mut()
        {
            triangles_vertices_coordinates.extend(vertex_coordinates);
            Ok(())
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Add triangle vertex coordinates: \
                'optional_triangles_vertices_coordinates' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_triangle_vertex_color_value(&mut self, vertex_color_value: &[f32])
        -> Result<(), JsValue>
    {
        if let Some(triangles_vertices_colors_values) =
            self.optional_triangles_vertices_colors_values.as_mut()
        {
            triangles_vertices_colors_values.extend(vertex_color_value);
            Ok(())
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Add triangle vertex color value: \
                'optional_triangles_vertices_colors_values' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_triangle_vertex_index(&mut self, vertex_index: u32)
        -> Result<(), JsValue>
    {
        if let Some(triangles_vertices_indexes) =
            self.optional_triangles_vertices_indexes.as_mut()
        {
            triangles_vertices_indexes.push(vertex_index);
            Ok(())
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Add triangle vertex index: \
                'optional_triangles_vertices_indexes' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn add_triangles_vertices_indexes(&mut self, vertices_indexes: &[u32])
        -> Result<(), JsValue>
    {
        if let Some(triangles_vertices_indexes) =
            self.optional_triangles_vertices_indexes.as_mut()
        {
            triangles_vertices_indexes.extend(vertices_indexes);
            Ok(())
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Add triangle vertex index: \
                'optional_triangles_vertices_indexes' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn ref_points_coordinates(&self) -> Result<&[f32], JsValue>
    {
        if let Some(points_coordinates) = self.optional_points_coordinates.as_ref()
        {
            Ok(points_coordinates)
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Ref points coordinates: \
                'optional_points_coordinates' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn ref_points_colors_values(&self) -> Result<&[f32], JsValue>
    {
        if let Some(points_colors_values) = self.optional_points_colors_values.as_ref()
        {
            Ok(points_colors_values)
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Ref points colors values: \
                'optional_points_colors_values' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn ref_lines_endpoints_coordinates(&self) -> Result<&[f32], JsValue>
    {
        if let Some(lines_endpoints_coordinates) =
            self.optional_lines_endpoints_coordinates.as_ref()
        {
            Ok(lines_endpoints_coordinates)
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Ref lines endpoints coordinates: \
                'optional_lines_endpoints_coordinates' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn ref_lines_endpoints_colors_values(&self) -> Result<&[f32], JsValue>
    {
        if let Some(lines_endpoints_colors_values) =
            self.optional_lines_endpoints_colors_values.as_ref()
        {
            Ok(lines_endpoints_colors_values)
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Ref lines endpoints colors values: \
                'optional_lines_endpoints_colors_values' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn ref_triangles_vertices_coordinates(&self) -> Result<&[f32], JsValue>
    {
        if let Some(triangles_vertices_coordinates) =
            self.optional_triangles_vertices_coordinates.as_ref()
        {
            Ok(triangles_vertices_coordinates)
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Ref triangles vertices coordinates: \
                'optional_triangles_vertices_coordinates' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn ref_triangles_vertices_colors_values(&self) -> Result<&[f32], JsValue>
    {
        if let Some(triangles_vertices_colors_values) =
            self.optional_triangles_vertices_colors_values.as_ref()
        {
            Ok(triangles_vertices_colors_values)
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Ref triangles vertices colors values: \
                'optional_triangles_vertices_colors_values' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn ref_triangles_vertices_indexes(&self) -> Result<&[u32], JsValue>
    {
        if let Some(triangles_vertices_indexes) =
            self.optional_triangles_vertices_indexes.as_ref()
        {
            Ok(triangles_vertices_indexes)
        }
        else
        {
            let error_message = "Renderer: DrawnObject: Ref triangles vertices indexes: \
                'optional_triangles_vertices_indexes' attribute was defined as None for \
                current DrawnObject!";
            Err(JsValue::from(error_message))
        }
    }


    pub fn draw_points(&self, gl: &GL) -> Result<(), JsValue>
    {
        if self.optional_points_coordinates.is_none() ||
            self.optional_points_colors_values.is_none()
        {
            let error_message = "Renderer: DrawnObject: Draw points: \
                'optional_points_coordinates' and/or 'optional_points_colors_values' attributes \
                 were defined as None for current DrawnObject!";
            Err(JsValue::from(error_message))
        }
        else
        {
            let first = 0;
            let count = (self.optional_points_coordinates.as_ref().unwrap().len() / 3) as i32;
            gl.draw_arrays(GL::POINTS, first, count);
            Ok(())
        }
    }


    pub fn draw_lines(&self, gl: &GL) -> Result<(), JsValue>
    {
        if self.optional_lines_endpoints_coordinates.is_none() ||
            self.optional_lines_endpoints_colors_values.is_none()
        {
            let error_message = "Renderer: DrawnObject: Draw lines: \
                'optional_lines_endpoints_coordinates' and/or \
                'optional_lines_endpoints_colors_values' attributes \
                 were defined as None for current DrawnObject!";
            Err(JsValue::from(error_message))
        }
        else
        {
            let first = 0;
            let count = (self.optional_lines_endpoints_coordinates.as_ref().unwrap()
                .len() / 3) as i32;
            gl.draw_arrays(GL::LINES, first, count);
            Ok(())
        }
    }


    pub fn draw_triangles(&self, gl: &GL) -> Result<(), JsValue>
    {
        if self.optional_triangles_vertices_coordinates.is_none() ||
            self.optional_triangles_vertices_colors_values.is_none() ||
            self.optional_triangles_vertices_indexes.is_none()
        {
            let error_message = "Renderer: DrawnObject: Draw triangles: \
                'optional_triangles_vertices_coordinates' and/or \
                'optional_triangles_vertices_coordinates' and/or \
                'optional_triangles_vertices_indexes' attributes \
                 were defined as None for current DrawnObject!";
            Err(JsValue::from(error_message))
        }
        else
        {
            let offset = 0;
            let count = self.optional_triangles_vertices_indexes.as_ref().unwrap().len() as i32;
            gl.draw_elements_with_i32(GL::TRIANGLES, count, GL::UNSIGNED_INT, offset);
            Ok(())
        }
    }
}
