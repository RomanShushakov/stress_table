use web_sys::{WebGlBuffer, WebGlRenderingContext as GL};

use crate::drawn_object::drawn_object::DrawnObjectTrait;
use crate::drawn_object::drawn_object::{DrawnObject};
use crate::shader_programs::ShaderPrograms;


pub struct BufferObjects
{
    vertex: WebGlBuffer,
    color: WebGlBuffer,
    index: WebGlBuffer,
}


impl BufferObjects
{
    pub fn initialize(gl: &GL) -> Self
    {
        let vertex = gl.create_buffer().unwrap();
        let color = gl.create_buffer().unwrap();
        let index = gl.create_buffer().unwrap();
        BufferObjects { vertex, color, index }
    }


    pub fn store_drawn_object(&self, gl: &GL, drawn_object: &impl DrawnObjectTrait)
    {
        let vertices = js_sys::Float32Array::from(drawn_object.get_vertices_coordinates());
        let colors = js_sys::Float32Array::from(drawn_object.get_colors_values());
        let indexes = js_sys::Uint32Array::from(drawn_object.get_indexes_numbers());
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, GL::STATIC_DRAW);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &colors, GL::STATIC_DRAW);
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indexes, GL::STATIC_DRAW);
    }


    pub fn associate_with_shader_programs(&self, gl: &GL, shader_programs: &ShaderPrograms)
    {
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex));
        gl.vertex_attrib_pointer_with_i32(shader_programs.get_vertex_position(), 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(shader_programs.get_vertex_position());
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color));
        gl.vertex_attrib_pointer_with_i32(shader_programs.get_vertex_color(), 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(shader_programs.get_vertex_color());
    }
}
