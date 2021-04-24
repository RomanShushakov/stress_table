use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}


#[wasm_bindgen]
pub struct Renderer
{
    canvas_gl: web_sys::HtmlCanvasElement,
    cursor_coord_x: i32,
    cursor_coord_y: i32,
    theta: f32,
    phi: f32,
    dx: f32,
    dy: f32,
    d_scale: f32,
    timestamp: f32,
}


#[wasm_bindgen]
impl Renderer
{
    pub fn create(canvas_gl: web_sys::HtmlCanvasElement, canvas_width: f32, canvas_height: f32)
        -> Renderer
    {
        canvas_gl.set_width(canvas_width as u32);
        canvas_gl.set_height(canvas_height as u32);
        Renderer
        {
            canvas_gl,
            cursor_coord_x: -1,
            cursor_coord_y: -1,
            theta: 0.0,
            phi: 0.0,
            dx: 0.0,
            dy: 0.0,
            d_scale: 0.0,
            timestamp: 0.0
        }
    }


    pub fn update_canvas_size(&mut self, canvas_width: f32, canvas_height: f32)
    {
        self.canvas_gl.set_width(canvas_width as u32);
        self.canvas_gl.set_height(canvas_height as u32);
    }


    pub fn change_cursor_coordinates(&mut self, x: i32, y: i32)
    {
        self.cursor_coord_x = x;
        self.cursor_coord_y = y;
    }


    pub fn increment_angle_theta(&mut self, d_theta: f32)
    {
        self.theta += d_theta;
    }


    pub fn increment_angle_phi(&mut self, d_phi: f32)
    {
        self.phi += d_phi;
    }


    pub fn increment_dx(&mut self, dx: f32)
    {
        self.dx += dx;
    }


    pub fn increment_dy(&mut self, dy: f32)
    {
        self.dy += dy;
    }


    pub fn extract_d_scale(&self) -> f32
    {
        self.d_scale
    }


    pub fn change_d_scale(&mut self, d_scale: f32)
    {
        self.d_scale = d_scale;
        // log(&format!("{}", self.d_scale));
    }


    pub fn tick(&mut self) -> Result<(), JsValue>
    {
        self.timestamp += 1.0;
        self.render()?;
        Ok(())
    }


    fn render(&mut self) -> Result<(), JsValue>
    {
        let width = self.canvas_gl.width();
        let height = self.canvas_gl.height();

        let gl = self.canvas_gl
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        gl.clear_color(1.0, 1.0, 1.0, 1.0);
        gl.enable(WebGlRenderingContext::DEPTH_TEST);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        gl.clear(WebGlRenderingContext::DEPTH_BUFFER_BIT);

        let vert_shader = compile_shader(
            &gl,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            precision mediump float;
            attribute vec2 a_position;
            void main() {
                gl_Position = vec4(a_position, 0.0, 1.0);
            }
        "#,
        )?;
        let frag_shader = compile_shader(
            &gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            precision mediump float;
            uniform float u_time;
            void main() {
                float r = sin(u_time * 0.003);
                float g = sin(u_time * 0.005);
                float b = sin(u_time * 0.007);
                gl_FragColor = vec4(r, g, b, 1.0);
            }
        "#,
        )?;

        gl.viewport(0, 0, width as i32, height as i32);

        let program = link_program(&gl, &vert_shader, &frag_shader)?;
        gl.use_program(Some(&program));

        let vertices: Vec<f32> = vec![
                -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
            ];

        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        let vert_array = js_sys::Float32Array::from(vertices.as_slice());

        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        let position = gl.get_attrib_location(&program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        let time = gl.get_uniform_location(&program, "u_time");
        gl.uniform1f(time.as_ref(), self.timestamp);

        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (vertices.len() / 2) as i32);
        Ok(())
    }
}


fn compile_shader(gl: &WebGlRenderingContext, shader_type: u32, source: &str)
    -> Result<WebGlShader, String>
{
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    }
    else
    {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}


fn link_program(gl: &WebGlRenderingContext, vert_shader: &WebGlShader,
                    frag_shader: &WebGlShader) -> Result<WebGlProgram, String>
{
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    }
    else
    {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

