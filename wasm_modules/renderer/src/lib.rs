use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};


#[wasm_bindgen]
extern "C"
{
    // #[wasm_bindgen(js_namespace = console, js_name = log)]
    // fn log_f32(value: f32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}


#[wasm_bindgen]
pub struct Renderer
{
    canvas_gl: web_sys::HtmlCanvasElement,
    timestamp: f32,
}


#[wasm_bindgen]
impl Renderer
{
    pub fn create(canvas_gl: web_sys::HtmlCanvasElement) -> Renderer
    {
        Renderer { canvas_gl, timestamp: 0.0 }
    }


    pub fn tick(&mut self) -> Result<(), JsValue>
    {
        self.timestamp += 1.0;
        self.render()?;
        Ok(())
    }


    fn render(&self) -> Result<(), JsValue>
    {
        // let document = web_sys::window().unwrap().document().unwrap();
        // let canvas = document.get_element_by_id("canvas").unwrap();
        // let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

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

