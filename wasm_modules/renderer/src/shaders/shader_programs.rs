use web_sys::{WebGlUniformLocation, WebGlProgram, WebGlRenderingContext as GL};


pub struct ShaderPrograms
{
    vertex_position: u32,
    vertex_color: u32,
    point_size: WebGlUniformLocation,
    projection_matrix: WebGlUniformLocation,
    model_view_matrix: WebGlUniformLocation,
}


impl ShaderPrograms
{
    pub fn initialize(gl: &GL) -> Self
    {
        let vertex_shader_code = include_str!("main_vert_shader.vert");
        let fragment_shader_code = include_str!("main_frag_shader.frag");

        let vertex_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vertex_shader, &vertex_shader_code);
        gl.compile_shader(&vertex_shader);
        let fragment_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&fragment_shader, &fragment_shader_code);
        gl.compile_shader(&fragment_shader);
        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vertex_shader);
        gl.attach_shader(&shader_program, &fragment_shader);
        gl.link_program(&shader_program);
        gl.use_program(Some(&shader_program));

        let vertex_position = gl.get_attrib_location(&shader_program, "aVertexPosition") as u32;
        let vertex_color = gl.get_attrib_location(&shader_program, "aVertexColor") as u32;
        let point_size = gl.get_uniform_location(&shader_program, "uPointSize").unwrap();
        let projection_matrix = gl
            .get_uniform_location(&shader_program, "uProjectionMatrix")
            .unwrap();
        let model_view_matrix = gl
            .get_uniform_location(&shader_program, "uModelViewMatrix")
            .unwrap();
        ShaderPrograms {
            vertex_position, vertex_color, point_size, projection_matrix, model_view_matrix }
    }


    pub fn copy_vertex_position(&self) -> u32
    {
        self.vertex_position
    }


    pub fn copy_vertex_color(&self) -> u32
    {
        self.vertex_color
    }


    pub fn ref_point_size(&self) -> &WebGlUniformLocation
    {
        &self.point_size
    }


    pub fn ref_projection_matrix(&self) -> &WebGlUniformLocation
    {
        &self.projection_matrix
    }


    pub fn ref_model_view_matrix(&self) -> &WebGlUniformLocation
    {
        &self.model_view_matrix
    }
}
