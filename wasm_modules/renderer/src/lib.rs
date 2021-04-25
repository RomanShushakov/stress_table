use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext as GL, WebGlShader};
use mat4;

mod aux_structs;
mod aux_functions;

use aux_structs::{ShadersVariables, Buffers, DrawnObject};
use aux_structs::{CSAxis};
use aux_structs::
    {
        CS_AXES_Y_SHIFT, CS_AXES_X_SHIFT, CS_AXES_Z_SHIFT, CS_AXES_SCALE,
        CS_AXES_CAPS_BASE_POINTS_NUMBER, CS_AXES_CAPS_WIDTH, CS_AXES_CAPS_HEIGHT,
        AXIS_X_DENOTATION_SHIFT_X, AXIS_X_DENOTATION_SHIFT_Y, AXIS_Y_DENOTATION_SHIFT_X,
        AXIS_Y_DENOTATION_SHIFT_Y, AXIS_Z_DENOTATION_SHIFT_X, AXIS_Z_DENOTATION_SHIFT_Y,
        AXIS_Z_DENOTATION_SHIFT_Z, CANVAS_AXES_DENOTATION_COLOR,
        CANVAS_DRAWN_NODES_DENOTATION_COLOR, DRAWN_NODES_DENOTATION_SHIFT,
        CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR, DRAWN_DISPLACEMENTS_CAPS_BASE_POINTS_NUMBER,
        DRAWN_DISPLACEMENTS_CAPS_HEIGHT, DRAWN_DISPLACEMENTS_CAPS_WIDTH,
        CANVAS_DRAWN_DISPLACEMENTS_DENOTATION_COLOR, DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_X,
        DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_Y, DRAWN_FORCES_LINE_LENGTH, DRAWN_FORCES_CAPS_HEIGHT,
        DRAWN_FORCES_CAPS_WIDTH, DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER,
        CANVAS_DRAWN_FORCES_DENOTATION_COLOR, DRAWN_FORCES_DENOTATION_SHIFT_X,
        DRAWN_FORCES_DENOTATION_SHIFT_Y, HINTS_COLOR, DRAWN_ELEMENTS_DENOTATION_SHIFT
    };
use aux_functions::initialize_shaders;


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
    under_cursor_color: [u8; 4],
    selected_color: [u8; 4],
    // nodes: Vec<Node>,
    // drawn_elements: Rc<Vec<FEDrawnElementData>>,
    // add_analysis_message: Callback<String>,
    // drawn_bcs: Rc<Vec<FEDrawnBCData>>,
    // add_object_info: Callback<String>,
    // reset_object_info: Callback<()>,
    timestamp: f32,
}


#[wasm_bindgen]
impl Renderer
{
    pub fn create(canvas_gl: web_sys::HtmlCanvasElement)
        -> Renderer
    {
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
            under_cursor_color: [0; 4],
            selected_color: [0; 4],
            // nodes: Vec::new(),
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


    pub fn select_object(&mut self) -> Option<String>
    {
        // self.selected_color = self.under_cursor_color;
        // Some(format!("{:?}", self.selected_color))
        None
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
            .dyn_into::<GL>()?;
        gl.get_extension("OES_element_index_uint")?;

        gl.clear_color(1.0, 1.0, 1.0, 1.0);
        gl.enable(GL::DEPTH_TEST);
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.clear(GL::DEPTH_BUFFER_BIT);

        let vertex_shader_code = include_str!("shaders/main_vert_shader.vert");
        let fragment_shader_code = include_str!("shaders/main_frag_shader.frag");

        let shader_program = initialize_shaders(&gl, vertex_shader_code, fragment_shader_code);
        let shaders_variables = ShadersVariables::initialize(&gl, &shader_program);

        gl.viewport(0, 0, width as i32, height as i32);

        let aspect: f32 = width as f32 / height as f32;
        let z_near = 1.0;
        let z_far = 101.0;

        let cs_buffers = Buffers::initialize(&gl);
        let mut cs_drawn_object = DrawnObject::create();
        gl.line_width(2.5);

        cs_drawn_object.add_cs_axis_line(CSAxis::X);
        cs_drawn_object.add_cs_axis_line(CSAxis::Y);
        cs_drawn_object.add_cs_axis_line(CSAxis::Z);
        cs_drawn_object.add_cs_axis_cap(
            CSAxis::X, CS_AXES_CAPS_BASE_POINTS_NUMBER,
            CS_AXES_CAPS_HEIGHT, CS_AXES_CAPS_WIDTH);
        cs_drawn_object.add_cs_axis_cap(
            CSAxis::Y, CS_AXES_CAPS_BASE_POINTS_NUMBER,
            CS_AXES_CAPS_HEIGHT, CS_AXES_CAPS_WIDTH);
        cs_drawn_object.add_cs_axis_cap(
            CSAxis::Z, CS_AXES_CAPS_BASE_POINTS_NUMBER,
            CS_AXES_CAPS_HEIGHT, CS_AXES_CAPS_WIDTH);

        cs_buffers.render(&gl, &cs_drawn_object, &shaders_variables);

        let point_size = 5.0;

        let mut projection_matrix = mat4::new_zero();
        mat4::orthographic(&mut projection_matrix,
            &1.0, &1.0, &-1.0, &-1.0, &z_near, &z_far);
        let mut model_view_matrix = mat4::new_identity();
        let mat_to_translate = model_view_matrix;
        let y_shift =
            if CS_AXES_Y_SHIFT > 0.0 { 1.0 - (1.0 - CS_AXES_Y_SHIFT) * aspect }
            else { - 1.0 + (1.0 + CS_AXES_Y_SHIFT) * aspect };
        mat4::translate(&mut model_view_matrix, &mat_to_translate,
            &[CS_AXES_X_SHIFT, y_shift, CS_AXES_Z_SHIFT]);
        let mat_to_scale = model_view_matrix;
        mat4::scale(&mut model_view_matrix, &mat_to_scale,
            &[CS_AXES_SCALE, CS_AXES_SCALE * aspect, CS_AXES_SCALE * aspect]);
        let mat_to_rotate = model_view_matrix;
        mat4::rotate_x(&mut model_view_matrix,&mat_to_rotate,&self.phi);
        let mat_to_rotate = model_view_matrix;
        mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.theta);
        gl.uniform1f(Some(&shaders_variables.point_size), point_size);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&shaders_variables.projection_matrix), false, &projection_matrix);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&shaders_variables.model_view_matrix), false, &model_view_matrix);

        cs_drawn_object.draw(&gl);

        // ctx.set_fill_style(&CANVAS_AXES_DENOTATION_COLOR.into());
        // add_denotation(&ctx,
        //     &[1.0 + AXIS_X_DENOTATION_SHIFT_X, 0.0 + AXIS_X_DENOTATION_SHIFT_Y, 0.0, 1.0],
        //     &model_view_matrix,
        //     self.props.canvas_width as f32,
        //     self.props.canvas_height as f32, "X");
        // add_denotation(&ctx,
        //     &[0.0 + AXIS_Y_DENOTATION_SHIFT_X, 1.0 + AXIS_Y_DENOTATION_SHIFT_Y, 0.0, 1.0],
        //     &model_view_matrix,
        //     self.props.canvas_width as f32,
        //     self.props.canvas_height as f32, "Y");
        // add_denotation(&ctx,
        //     &[0.0 + AXIS_Z_DENOTATION_SHIFT_X, 0.0 + AXIS_Z_DENOTATION_SHIFT_Y,
        //         1.0 + AXIS_Z_DENOTATION_SHIFT_Z, 1.0],
        //     &model_view_matrix,
        //     self.props.canvas_width as f32,
        //     self.props.canvas_height as f32, "Z");
        // ctx.stroke();
        //
        // ctx.set_fill_style(&HINTS_COLOR.into());
        // add_hints(&ctx, self.props.canvas_width as f32,
        //     self.props.canvas_height as f32);
        // ctx.stroke();

        Ok(())
    }
}
