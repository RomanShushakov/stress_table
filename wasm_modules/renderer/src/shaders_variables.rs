use web_sys::{WebGlUniformLocation, WebGlProgram, WebGlRenderingContext as GL};


pub struct ShadersVariables
{
    vertex_position: u32,
    vertex_color: u32,
    point_size: WebGlUniformLocation,
    projection_matrix: WebGlUniformLocation,
    model_view_matrix: WebGlUniformLocation,
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


    pub fn get_vertex_position(&self) -> u32
    {
        self.vertex_position
    }


    pub fn get_vertex_color(&self) -> u32
    {
        self.vertex_color
    }


    pub fn get_point_size(&self) -> &WebGlUniformLocation
    {
        &self.point_size
    }


    pub fn get_projection_matrix(&self) -> &WebGlUniformLocation
    {
        &self.projection_matrix
    }


    pub fn get_model_view_matrix(&self) -> &WebGlUniformLocation
    {
        &self.model_view_matrix
    }
}
