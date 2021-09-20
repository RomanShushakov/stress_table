use web_sys::{WebGlBuffer, WebGlRenderingContext as GL};

use crate::shaders::shader_programs::ShaderPrograms;


pub struct VertexBuffer(WebGlBuffer);


impl VertexBuffer
{
    pub fn initialize(gl: &GL) -> Self
    {
        VertexBuffer(gl.create_buffer().unwrap())
    }


    pub fn store_vertices_coordinates(&self, gl: &GL, vertices_coordinates: &[f32])
    {
        let vertices = js_sys::Float32Array::from(vertices_coordinates);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.0));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, GL::STATIC_DRAW);
    }


    pub fn associate_with_shader_programs(&self, gl: &GL, shader_programs: &ShaderPrograms)
    {
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.0));
        gl.vertex_attrib_pointer_with_i32(shader_programs.copy_vertex_position(), 3, GL::FLOAT,
            false, 0, 0);
        gl.enable_vertex_attrib_array(shader_programs.copy_vertex_position());
    }
}


pub struct ColorBuffer(WebGlBuffer);


impl ColorBuffer
{
    pub fn initialize(gl: &GL) -> Self
    {
        ColorBuffer(gl.create_buffer().unwrap())
    }


    pub fn store_colors_values(&self, gl: &GL, colors_values: &[f32])
    {
        let colors = js_sys::Float32Array::from(colors_values);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.0));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &colors, GL::STATIC_DRAW);
    }


    pub fn associate_with_shader_programs(&self, gl: &GL, shader_programs: &ShaderPrograms)
    {
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.0));
        gl.vertex_attrib_pointer_with_i32(shader_programs.copy_vertex_color(), 4, GL::FLOAT,
            false, 0, 0);
        gl.enable_vertex_attrib_array(shader_programs.copy_vertex_color());
    }
}


pub struct IndexBuffer(WebGlBuffer);


impl IndexBuffer
{
    pub fn initialize(gl: &GL) -> Self
    {
        IndexBuffer(gl.create_buffer().unwrap())
    }


    pub fn store_indexes_numbers(&self, gl: &GL, indexes_numbers: &[u32])
    {
        let indexes = js_sys::Uint32Array::from(indexes_numbers);
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.0));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indexes, GL::STATIC_DRAW);
    }
}
