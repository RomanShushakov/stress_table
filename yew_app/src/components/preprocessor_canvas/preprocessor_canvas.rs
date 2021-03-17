use std::f32::consts::PI;
use std::rc::Rc;
use std::cell::RefCell;
use mat4;
use vec4;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::
    {
        Document, HtmlCanvasElement, WebGlRenderingContext as GL,
        WebGlUniformLocation, Window, CanvasRenderingContext2d as CTX
    };
use yew::{Component, ComponentLink, html, Html, NodeRef, ShouldRender};
use yew::prelude::*;
use yew::services::keyboard::{KeyboardService, KeyListenerHandle};
use yew::services::render::RenderTask;
use yew::services::RenderService;

use crate::auxiliary::gl_aux_functions::
    {
        add_denotation, initialize_shaders, normalize_nodes, add_hints,
    };
use crate::auxiliary::gl_aux_structs::
    {
        Buffers, ShadersVariables, DrawnObject, CSAxis, CS_AXES_Y_SHIFT, CS_AXES_X_SHIFT,
        CS_AXES_Z_SHIFT, CS_AXES_SCALE, CS_AXES_CAPS_BASE_POINTS_NUMBER, CS_AXES_CAPS_WIDTH,
        CS_AXES_CAPS_HEIGHT, AXIS_X_DENOTATION_SHIFT_X, AXIS_X_DENOTATION_SHIFT_Y,
        AXIS_Y_DENOTATION_SHIFT_X, AXIS_Y_DENOTATION_SHIFT_Y, AXIS_Z_DENOTATION_SHIFT_X,
        AXIS_Z_DENOTATION_SHIFT_Y, AXIS_Z_DENOTATION_SHIFT_Z, CANVAS_AXES_DENOTATION_COLOR,
        CANVAS_DRAWN_NODES_DENOTATION_COLOR, DRAWN_NODES_DENOTATION_SHIFT,
        CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR, DRAWN_DISPLACEMENTS_CAPS_BASE_POINTS_NUMBER,
        DRAWN_DISPLACEMENTS_CAPS_HEIGHT, DRAWN_DISPLACEMENTS_CAPS_WIDTH,
        CANVAS_DRAWN_DISPLACEMENTS_DENOTATION_COLOR, DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_X,
        DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_Y, DRAWN_FORCES_LINE_LENGTH, DRAWN_FORCES_CAPS_HEIGHT,
        DRAWN_FORCES_CAPS_WIDTH, DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER,
        CANVAS_DRAWN_FORCES_DENOTATION_COLOR, DRAWN_FORCES_DENOTATION_SHIFT_X,
        DRAWN_FORCES_DENOTATION_SHIFT_Y, HINTS_COLOR,
    };

use crate::fem::{FENode, FEType, BCType};
use crate::{ElementsNumbers, ElementsValues, GLElementsNumbers, GLElementsValues};
use crate::auxiliary::{View, FEDrawnElementData, DrawnBCData};


const PREPROCESSOR_CANVAS_CONTAINER_CLASS: &str = "preprocessor_canvas_container";
const PREPROCESSOR_CANVAS_TEXT_CLASS: &str = "preprocessor_canvas_text";
const PREPROCESSOR_CANVAS_GL_CLASS: &str = "preprocessor_canvas_gl";
const PREPROCESSOR_CANVAS_GL_ID: &str = "preprocessor_canvas_gl_id";


fn window() -> Window
{
    web_sys::window().expect("no global `window` exists")
}


fn document() -> Document
{
    window().document().expect("should have a document on window")
}


#[derive(Properties, Clone)]
pub struct Props
{
    pub view: Option<View>,
    pub discard_view: Callback<()>,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub nodes: Rc<Vec<Rc<RefCell<FENode<ElementsNumbers, ElementsValues>>>>>,
    pub drawn_elements: Rc<Vec<FEDrawnElementData>>,
    pub add_analysis_message: Callback<String>,
    pub drawn_bcs: Rc<Vec<DrawnBCData>>,
}


pub enum Msg
{
    Render(f64),
    MouseMove(web_sys::MouseEvent),
    MouseLeave,
    MouseDown,
    MouseUp,
    KeyDown(web_sys::KeyboardEvent),
    KeyUp(web_sys::KeyboardEvent),
    MouseWheel(web_sys::WheelEvent),
}


struct State
{
    // x: i32,
    // y: i32,
    dx: GLElementsValues,
    dy: GLElementsValues,
    d_scale: GLElementsValues,
    theta: GLElementsValues,
    phi: GLElementsValues,
    pan: bool,
    rotate: bool,
    shift_key_pressed: bool,
}


pub struct PreprocessorCanvas
{
    props: Props,
    canvas: Option<HtmlCanvasElement>,
    gl: Option<GL>,
    link: ComponentLink<Self>,
    canvas_node_ref: NodeRef,
    canvas_text: Option<HtmlCanvasElement>,
    ctx: Option<CTX>,
    canvas_text_node_ref: NodeRef,
    render_loop: Option<RenderTask>,
    key_down_task: Option<KeyListenerHandle>,
    key_up_task: Option<KeyListenerHandle>,
    state: State,
}


impl PreprocessorCanvas
{
    fn key_press(&mut self, element: &Window)
    {
        let key_down_callback: Callback<KeyboardEvent> = self.link
            .callback(|event: KeyboardEvent| Msg::KeyDown(event));
        let key_down_task =
            KeyboardService::register_key_down(element, key_down_callback);
        self.key_down_task = Some(key_down_task);
        let key_up_callback: Callback<KeyboardEvent> = self.link
            .callback(|event: KeyboardEvent| Msg::KeyUp(event));
        let key_up_task =
            KeyboardService::register_key_up(element, key_up_callback);
        self.key_up_task = Some(key_up_task);
    }
}


impl Component for PreprocessorCanvas
{
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let (dx, dy, d_scale, theta, phi) =
            (GLElementsValues::default(), GLElementsValues::default(),
             GLElementsValues::default(), GLElementsValues::default(),
             GLElementsValues::default());
        let pan = false;
        let rotate = false;
        let shift_key_pressed = false;
        let state = State { // x: -1, y: -1,
            dx, dy, d_scale, theta, phi, pan, rotate, shift_key_pressed };
        Self
        {
            props,
            canvas: None,
            gl: None,
            link,
            canvas_node_ref: NodeRef::default(),
            canvas_text: None,
            ctx: None,
            canvas_text_node_ref: NodeRef::default(),
            render_loop: None,
            key_down_task: None,
            key_up_task: None,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::Render(timestamp) =>
                {
                    self.render_gl_and_ctx(timestamp);
                    false
                },
            Msg::MouseMove(mouse_event) =>
                {
                    // let mut mouse_x = mouse_event.client_x();
                    // let mut mouse_y = mouse_event.client_y();
                    // let rect = document().get_element_by_id(PREPROCESSOR_CANVAS_GL_ID).unwrap()
                    //     .get_bounding_client_rect();
                    // let x = mouse_x - rect.left() as i32;
                    // let y = rect.bottom() as i32 - mouse_y;
                    // self.state.x = x;
                    // self.state.y = y;

                    if self.state.rotate
                    {
                        self.state.theta +=
                            mouse_event.movement_x() as GLElementsValues * 2.0 * PI  /
                            self.props.canvas_width as GLElementsValues;
                        self.state.phi +=
                            mouse_event.movement_y() as GLElementsValues * 2.0 * PI  /
                            self.props.canvas_height as GLElementsValues;
                    }
                    if self.state.pan
                    {
                        self.state.dx += mouse_event.movement_x() as GLElementsValues /
                            self.props.canvas_width as GLElementsValues;
                        self.state.dy += - mouse_event.movement_y() as GLElementsValues /
                        self.props.canvas_height as GLElementsValues;
                    }
                    false
                },
            Msg::MouseLeave =>
                {
                    self.state.rotate = false;
                    self.state.pan = false;
                    false
                },
            Msg::MouseDown =>
                {
                    if self.state.shift_key_pressed
                    {
                        self.state.pan = true;
                    }
                    else
                    {
                        self.state.rotate = true;
                    }
                    false
                },
            Msg::MouseUp =>
                {
                    self.state.rotate = false;
                    self.state.pan = false;
                    false
                },
            Msg::KeyDown(keyboard_event) =>
                {
                    if keyboard_event.key() == "Shift"
                    {
                        self.state.shift_key_pressed = true;
                    }
                    false
                },
            Msg::KeyUp(_keyboard_event) =>
                {
                    self.state.shift_key_pressed = false;
                    self.state.pan = false;
                    false
                },
            Msg::MouseWheel(wheel_event) =>
                {
                    let current_d_scale =
                        self.state.d_scale + wheel_event.delta_y() as GLElementsValues /
                        self.props.canvas_height as GLElementsValues;
                    if 1.0 + current_d_scale > 50.0
                    {
                        self.state.d_scale = 48.95;
                    }
                    else if 1.0 + current_d_scale < 0.0
                    {
                        self.state.d_scale = -0.95;
                    }
                    else
                    {
                        self.state.d_scale = current_d_scale;
                    }
                    false
                },
        }
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if (&self.props.view, &self.props.canvas_height, &self.props.canvas_width) !=
            (&props.view, &props.canvas_height, &props.canvas_width) ||
            !Rc::ptr_eq(&self.props.nodes, &props.nodes) ||
            !Rc::ptr_eq(&self.props.drawn_elements, &props.drawn_elements) ||
            !Rc::ptr_eq(&self.props.drawn_bcs, &props.drawn_bcs)
        {
            self.props = props;
            if let Some(view) = &self.props.view
            {
                match view
                {
                    View::PlaneXY =>
                        {
                            self.state.theta = 0.0;
                            self.state.phi = 0.0;
                            self.props.discard_view.emit(());
                        },
                    View::PlaneXZ =>
                        {
                            self.state.theta = 0.0;
                            self.state.phi = -90.0 * PI / 180.0;
                            self.props.discard_view.emit(());
                        },
                    View::PlaneZY =>
                        {
                            self.state.theta = 90.0 * PI / 180.0;
                            self.state.phi = 0.0;
                            self.props.discard_view.emit(());
                        },
                    View::Isometric =>
                        {
                            self.state.theta = -45.0 * PI / 180.0;
                            self.state.phi = 35.264 * PI / 180.0;
                            self.props.discard_view.emit(());
                        },
                }
            }
            true
        }
        else
        {
            false
        }
    }


    fn view(&self) -> Html
    {
        html!
        {
            <div class={ PREPROCESSOR_CANVAS_CONTAINER_CLASS }>
                <canvas ref=self.canvas_text_node_ref.clone(),
                    class={ PREPROCESSOR_CANVAS_TEXT_CLASS },
                    onmousemove=self.link.callback(move |event: MouseEvent| Msg::MouseMove(event)),
                    onmouseleave=self.link.callback(|_| Msg::MouseLeave),
                    onmousedown=self.link.callback(|_| Msg::MouseDown),
                    onmouseup=self.link.callback(|_| Msg::MouseUp),
                    onwheel=self.link.callback(move |event: WheelEvent| Msg::MouseWheel(event)),
                />
                <canvas ref=self.canvas_node_ref.clone()
                    id= { PREPROCESSOR_CANVAS_GL_ID },
                    class={ PREPROCESSOR_CANVAS_GL_CLASS },
                />
            </div>
        }
    }


    fn rendered(&mut self, first_render: bool)
    {
        let canvas_text = self.canvas_text_node_ref.cast::<HtmlCanvasElement>().unwrap();
        canvas_text.set_width(self.props.canvas_width);
        canvas_text.set_height(self.props.canvas_height);
        let ctx: CTX = canvas_text.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        self.canvas_text = Some(canvas_text);
        self.ctx = Some(ctx);

        let canvas = self.canvas_node_ref.cast::<HtmlCanvasElement>().unwrap();
        canvas.set_width(self.props.canvas_width);
        canvas.set_height(self.props.canvas_height);

        // let mut gl_context_attributes = web_sys::WebGlContextAttributes::new();
        // gl_context_attributes.preserve_drawing_buffer(true);

        let gl: GL = canvas
            .get_context("webgl")
            // .get_context_with_context_options("webgl", &gl_context_attributes)
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.canvas = Some(canvas);
        self.gl = Some(gl);

        if first_render
        {
            let window = window();
            self.key_press(&window);
            let render_frame = self.link.callback(Msg::Render);
            let handle = RenderService::request_animation_frame(render_frame);

            self.render_loop = Some(handle);
        }
    }
}

impl PreprocessorCanvas
{
    fn render_gl_and_ctx(&mut self, _timestamp: f64)
    {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");
        let ctx = self.ctx.as_ref().expect("CTX Context not initialized!");
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear_rect(0.0, 0.0, self.props.canvas_width as f64, self.props.canvas_height as f64);
        gl.enable(GL::DEPTH_TEST);
        gl.clear(GL::COLOR_BUFFER_BIT);
        let vertex_shader_code = include_str!("shaders/prep_shader.vert");
        let fragment_shader_code = include_str!("shaders/prep_shader.frag");

        let shader_program = initialize_shaders(&gl, vertex_shader_code, fragment_shader_code);
        let shaders_variables = ShadersVariables::initialize(&gl, &shader_program);

        gl.viewport(0, 0, self.props.canvas_width as i32, self.props.canvas_height as i32);
        let aspect: GLElementsValues = self.props.canvas_width as GLElementsValues /
            self.props.canvas_height as GLElementsValues;
        let z_near = 1.0 as GLElementsValues;
        let z_far = 101.0 as GLElementsValues;

        let cs_buffers = Buffers::initialize(&gl);
        let mut cs_drawn_object = DrawnObject::create();

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

        let mut projection_matrix = mat4::new_zero();
        mat4::orthographic(&mut projection_matrix,
            &(1.0 as GLElementsValues), &(1.0 as GLElementsValues),
            &(-1.0 as GLElementsValues), &(-1.0 as GLElementsValues), &z_near, &z_far);
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
        mat4::rotate_x(&mut model_view_matrix,&mat_to_rotate,&self.state.phi);
        let mat_to_rotate = model_view_matrix;
        mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.state.theta);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&shaders_variables.projection_matrix), false, &projection_matrix);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&shaders_variables.model_view_matrix), false, &model_view_matrix);

        cs_drawn_object.draw(&gl);

        ctx.set_fill_style(&CANVAS_AXES_DENOTATION_COLOR.into());
        add_denotation(&ctx,
            &[1.0 + AXIS_X_DENOTATION_SHIFT_X, 0.0 + AXIS_X_DENOTATION_SHIFT_Y, 0.0, 1.0],
            &model_view_matrix,
            self.props.canvas_width as f32,
            self.props.canvas_height as f32, "X");
        add_denotation(&ctx,
            &[0.0 + AXIS_Y_DENOTATION_SHIFT_X, 1.0 + AXIS_Y_DENOTATION_SHIFT_Y, 0.0, 1.0],
            &model_view_matrix,
            self.props.canvas_width as f32,
            self.props.canvas_height as f32, "Y");
        add_denotation(&ctx,
            &[0.0 + AXIS_Z_DENOTATION_SHIFT_X, 0.0 + AXIS_Z_DENOTATION_SHIFT_Y,
                1.0 + AXIS_Z_DENOTATION_SHIFT_Z, 1.0],
            &model_view_matrix,
            self.props.canvas_width as f32,
            self.props.canvas_height as f32, "Z");
        ctx.stroke();

        if !self.props.nodes.is_empty()
        {
            let normalized_nodes = normalize_nodes(
                Rc::clone(&self.props.nodes),
                self.props.canvas_width as GLElementsValues,
                self.props.canvas_height as GLElementsValues,
                aspect as GLElementsValues);

            let drawn_objects_buffers = Buffers::initialize(&gl);
            let mut drawn_object = DrawnObject::create();
            drawn_object.add_nodes(&normalized_nodes);

            if !self.props.drawn_elements.is_empty()
            {
                match drawn_object.add_elements(&normalized_nodes, &self.props.drawn_elements)
                {
                    Err(e) => self.props.add_analysis_message.emit(e),
                    Ok(()) => (),
                }
            }

            let drawn_displacements: Vec<&DrawnBCData> = self.props.drawn_bcs
                    .iter()
                    .filter(|bc|
                        bc.bc_type == BCType::Displacement)
                    .collect();
            if !drawn_displacements.is_empty()
            {
                drawn_object.add_displacements(
                    &normalized_nodes, &drawn_displacements,
                    DRAWN_DISPLACEMENTS_CAPS_BASE_POINTS_NUMBER,
                    DRAWN_DISPLACEMENTS_CAPS_HEIGHT / (1.0 + self.state.d_scale),
                    DRAWN_DISPLACEMENTS_CAPS_WIDTH / (1.0 + self.state.d_scale));
            }

            let drawn_forces: Vec<&DrawnBCData> = self.props.drawn_bcs
                    .iter()
                    .filter(|bc|
                        bc.bc_type == BCType::Force)
                    .collect();
            if !drawn_forces.is_empty()
            {
                drawn_object.add_forces(
                    &normalized_nodes, &drawn_forces,
                    DRAWN_FORCES_LINE_LENGTH / (1.0 + self.state.d_scale),
                    DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER,
                    DRAWN_FORCES_CAPS_HEIGHT / (1.0 + self.state.d_scale),
                    DRAWN_FORCES_CAPS_WIDTH / (1.0 + self.state.d_scale));
            }

            drawn_objects_buffers.render(&gl, &drawn_object, &shaders_variables);

            // let field_of_view = 45.0 * PI / 180.0;
            let mut projection_matrix = mat4::new_zero();

            // mat4::perspective(&mut projection_matrix, &field_of_view, &aspect, &z_near, &z_far);

            mat4::orthographic(&mut projection_matrix,
                &(1.0 as GLElementsValues / aspect), &(1.0 as GLElementsValues),
                &(-1.0 as GLElementsValues / aspect), &(-1.0 as GLElementsValues),
                &z_near, &z_far);
            let mut model_view_matrix = mat4::new_identity();
            let mat_to_translate = model_view_matrix;
            mat4::translate(&mut model_view_matrix, &mat_to_translate,
                &[self.state.dx, self.state.dy, -2.0]);
            let mat_to_scale = model_view_matrix;
            mat4::scale(&mut model_view_matrix, &mat_to_scale,
                &[1.0 + self.state.d_scale, 1.0 + self.state.d_scale, 1.0 + self.state.d_scale]);
            let mat_to_rotate = model_view_matrix;
            mat4::rotate_x(&mut model_view_matrix,&mat_to_rotate,&self.state.phi);
            let mat_to_rotate = model_view_matrix;
            mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.state.theta);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.projection_matrix), false, &projection_matrix);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.model_view_matrix), false, &model_view_matrix);

            drawn_object.draw(&gl);

            // let mut pixels = [0u8; 4];
            // match gl.read_pixels_with_opt_u8_array(
            //     self.state.x, self.state.y, 1, 1, GL::RGBA, GL::UNSIGNED_BYTE, Some(&mut pixels))
            // {
            //     Ok(_) => yew::services::ConsoleService::log(&format!("{:?}", pixels)),
            //     Err(e) => yew::services::ConsoleService::log(&format!("{:?}", e)),
            // }

            let mut matrix = mat4::new_identity();
            mat4::mul(&mut matrix, &projection_matrix, &model_view_matrix);

            ctx.set_fill_style(&CANVAS_DRAWN_NODES_DENOTATION_COLOR.into());
            for node in normalized_nodes.iter()
            {
                add_denotation(&ctx,
                &[node.x - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                    node.y - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                    node.z,
                    1.0],
                &matrix,
                self.props.canvas_width as f32,
                self.props.canvas_height as f32, &node.number.to_string());
            }
            ctx.stroke();

            if !self.props.drawn_elements.is_empty()
            {
                for element in self.props.drawn_elements.as_ref()
                {
                    match element.find_denotation_coordinates(&normalized_nodes)
                    {
                        Ok(coordinates) =>
                            {
                                ctx.set_fill_style(&CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR.into());
                                add_denotation(&ctx,
                                &coordinates,
                                &matrix,
                                self.props.canvas_width as f32,
                                self.props.canvas_height as f32,
                                &element.number.to_string());
                                ctx.stroke();
                            },
                        Err(e) => self.props.add_analysis_message.emit(e),
                    }
                }
            }

            if !drawn_displacements.is_empty()
            {
                for displacement in drawn_displacements
                {
                    match displacement.find_denotation_coordinates(&normalized_nodes)
                    {
                        Ok(coordinates) =>
                            {
                                ctx.set_fill_style(&CANVAS_DRAWN_DISPLACEMENTS_DENOTATION_COLOR.into());
                                add_denotation(&ctx,
                                &[coordinates[0] + DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_X /
                                    (1.0 + self.state.d_scale),
                                    coordinates[1] - DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_Y /
                                    (1.0 + self.state.d_scale),
                                    coordinates[2], coordinates[3]],
                                &matrix,
                                self.props.canvas_width as f32,
                                self.props.canvas_height as f32,
                                &displacement.number.to_string());
                                ctx.stroke();
                            },
                        Err(e) => self.props.add_analysis_message.emit(e)
                    }
                }
            }


            if !drawn_forces.is_empty()
            {
                for force in drawn_forces
                {
                    match force.find_denotation_coordinates(&normalized_nodes)
                    {
                        Ok(coordinates) =>
                            {
                                ctx.set_fill_style(&CANVAS_DRAWN_FORCES_DENOTATION_COLOR.into());
                                add_denotation(&ctx,
                                &[coordinates[0] + DRAWN_FORCES_DENOTATION_SHIFT_X /
                                    (1.0 + self.state.d_scale),
                                    coordinates[1] + DRAWN_FORCES_DENOTATION_SHIFT_Y /
                                    (1.0 + self.state.d_scale),
                                    coordinates[2], coordinates[3]],
                                &matrix,
                                self.props.canvas_width as f32,
                                self.props.canvas_height as f32,
                                &format!("#{}", force.number));
                                ctx.stroke();
                            },
                        Err(e) => self.props.add_analysis_message.emit(e)
                    }
                }
            }
        }

        ctx.set_fill_style(&HINTS_COLOR.into());
        add_hints(&ctx, self.props.canvas_width as f32,
            self.props.canvas_height as f32);
        ctx.stroke();

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);

        self.render_loop = Some(handle);
    }
}
