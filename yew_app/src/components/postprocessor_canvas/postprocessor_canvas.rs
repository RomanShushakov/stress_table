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


use crate::auxiliary::
    {
        FEDrawnElementData, FEDrawnBCData, FEDrawnNodeData, FEDrawnAnalysisResultNodeData,
        DrawnAnalysisResultElementData, NormalizedNode
    };
use crate::auxiliary::{View};
use crate::auxiliary::aux_functions::{transform_u32_to_array_of_u8, value_to_string};
use crate::auxiliary::gl_aux_functions::
    {
        add_denotation, initialize_shaders, normalize_nodes, add_hints,
        define_drawn_object_denotation_color, add_displacements_hints,
        add_reactions_hints, extend_by_elements_analysis_result, add_stresses_hints,
        add_color_bar,
    };
use crate::auxiliary::gl_aux_structs::{Buffers, ShadersVariables, DrawnObject};
use crate::auxiliary::gl_aux_structs::{CSAxis, GLMode};
use crate::auxiliary::gl_aux_structs::
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
        DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_Y, DRAWN_FORCES_LINE_LENGTH,
        DRAWN_FORCES_CAPS_HEIGHT, DRAWN_FORCES_CAPS_WIDTH, DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER,
        CANVAS_DRAWN_FORCES_DENOTATION_COLOR, DRAWN_FORCES_DENOTATION_SHIFT_X,
        DRAWN_FORCES_DENOTATION_SHIFT_Y, HINTS_COLOR, DRAWN_DEFORMED_SHAPE_NODES_COLOR,
        CANVAS_DRAWN_DEFORMED_SHAPE_NODES_DENOTATION_COLOR,
        DRAWN_DEFORMED_SHAPE_NODES_DENOTATION_SHIFT, DRAWN_ELEMENTS_DENOTATION_SHIFT,
    };

use crate::fem::{FENode, GlobalAnalysisResult, Displacements, Reactions, ElementsAnalysisResult, EARComponentTrait};
use crate::fem::{FEType, BCType, GlobalDOFParameter, StressStrainComponent, EARType};


use crate::{ElementsNumbers, ElementsValues, GLElementsNumbers, GLElementsValues, UIDNumbers};


const POSTPROCESSOR_CANVAS_CONTAINER_CLASS: &str = "postprocessor_canvas_container";
const POSTPROCESSOR_CANVAS_TEXT_CLASS: &str = "postprocessor_canvas_text";
const POSTPROCESSOR_CANVAS_GL_CLASS: &str = "postprocessor_canvas_gl";
const POSTPROCESSOR_CANVAS_GL_ID: &str = "postprocessor_canvas_gl_id";


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
    pub drawn_elements: Rc<Vec<FEDrawnElementData>>,
    pub drawn_nodes_extended: Vec<FEDrawnNodeData>,
    pub drawn_analysis_results_for_nodes: Vec<FEDrawnAnalysisResultNodeData>,
    pub drawn_analysis_results_for_elements: Vec<DrawnAnalysisResultElementData>,
    pub is_plot_displacements_selected: bool,
    pub is_plot_reactions_selected: bool,
    pub stress_component_selected: Option<StressStrainComponent>,
    pub min_selected_value: Option<ElementsValues>,
    pub max_selected_value: Option<ElementsValues>,
    pub add_object_info: Callback<String>,
    pub reset_object_info: Callback<()>,
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
    MouseClick,
}


struct State
{
    dx: GLElementsValues,
    dy: GLElementsValues,
    d_scale: GLElementsValues,
    theta: GLElementsValues,
    phi: GLElementsValues,
    pan: bool,
    rotate: bool,
    shift_key_pressed: bool,
    selected_color: [u8; 4],
    under_cursor_color: [u8; 4],
    cursor_coord_x: i32,
    cursor_coord_y: i32,
    normalized_nodes: Vec<NormalizedNode>,
}


pub struct PostprocessorCanvas
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


impl PostprocessorCanvas
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


    fn extract_object_info(&self) -> Option<String>
    {
        let object_info =
            {
                if let Some(position) = self.props.drawn_analysis_results_for_nodes
                    .iter()
                    .position(|result|
                        transform_u32_to_array_of_u8(result.uid) == self.state.selected_color)
                {
                    let result_type =
                        &self.props.drawn_analysis_results_for_nodes[position].bc_type;
                    let node_number =
                        &self.props.drawn_analysis_results_for_nodes[position].node_number;
                    let x_value = value_to_string(
                        &self.props.drawn_analysis_results_for_nodes[position].x_direction_value);
                    let y_value = value_to_string(
                        &self.props.drawn_analysis_results_for_nodes[position].y_direction_value);
                    let z_value = value_to_string(
                        &self.props.drawn_analysis_results_for_nodes[position].z_direction_value);
                    let xy_value = value_to_string(
                        &self.props.drawn_analysis_results_for_nodes[position].xy_plane_value);
                    let yz_value = value_to_string(
                        &self.props.drawn_analysis_results_for_nodes[position].yz_plane_value);
                    let zx_value = value_to_string(
                        &self.props.drawn_analysis_results_for_nodes[position].zx_plane_value);
                    match result_type
                    {
                        BCType::Displacement =>
                            {
                                let object_info = format!("Displacement at node: #{}, \
                                Ux: {}, Uy: {}, Uz: {}, URx: {}, URy: {}, URz: {}", node_number,
                                x_value, y_value, z_value,
                                yz_value, zx_value, xy_value);
                                Some(object_info)
                            },
                        BCType::Force =>
                            {
                                let object_info = format!("Reaction at node: #{}, Rx: {}, \
                                Ry: {}, Rz: {}, Mx: {}, My: {}, Mz: {}", node_number, x_value,
                                y_value, z_value, yz_value, zx_value, xy_value);
                                Some(object_info)
                            },
                    }
                }
                else if let Some(position) = self.props.drawn_analysis_results_for_elements
                    .iter()
                    .position(|result|
                        transform_u32_to_array_of_u8(result.uid) == self.state.selected_color)
                {
                    if self.props.stress_component_selected.is_some()
                    {
                        let element_number = self.props
                            .drawn_analysis_results_for_elements[position]
                            .element_analysis_data.extract_element_number();
                        let stresses = self.props
                            .drawn_analysis_results_for_elements[position]
                            .element_analysis_data
                            .extract_stresses();
                        let object_info = format!("Stresses at element: #{}, \
                                {:?}", element_number, stresses);
                        Some(object_info)
                    }
                    else
                    {
                        None
                    }
                }
                else
                {
                    None
                }
            };
        object_info
    }

}


impl Component for PostprocessorCanvas
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
        let normalized_nodes = if !props.drawn_nodes_extended.is_empty()
            {
                normalize_nodes(Rc::new(props.drawn_nodes_extended.to_owned()),
                    props.canvas_width as GLElementsValues,
                    props.canvas_height as GLElementsValues)
            }
            else
            {
                Vec::new()
            };
        let state = State {
            dx, dy, d_scale, theta, phi, pan, rotate, shift_key_pressed, selected_color: [0; 4],
            under_cursor_color: [0; 4], cursor_coord_x: -1, cursor_coord_y: -1,
            normalized_nodes };
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
                    let mouse_x = mouse_event.client_x();
                    let mouse_y = mouse_event.client_y();
                    let rect = document().get_element_by_id(POSTPROCESSOR_CANVAS_GL_ID).unwrap()
                        .get_bounding_client_rect();
                    let x = mouse_x - rect.left() as i32;
                    let y = rect.bottom() as i32 - mouse_y;
                    self.state.cursor_coord_x = x;
                    self.state.cursor_coord_y = y;

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
            Msg::MouseClick =>
                {
                    self.state.selected_color = self.state.under_cursor_color;
                    let object_info = self.extract_object_info();
                    if object_info.is_some()
                    {
                        self.props.add_object_info.emit(object_info.unwrap());
                    }
                    else
                    {
                        self.props.reset_object_info.emit(());
                    }
                    false
                },
        }
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if (&self.props.view, &self.props.canvas_height, &self.props.canvas_width,
            &self.props.is_plot_displacements_selected, &self.props.is_plot_reactions_selected,
            &self.props.drawn_nodes_extended, &self.props.drawn_analysis_results_for_nodes,
            &self.props.drawn_analysis_results_for_elements, &self.props.min_selected_value,
            &self.props.max_selected_value) !=
            (&props.view, &props.canvas_height, &props.canvas_width,
            &props.is_plot_displacements_selected, &props.is_plot_reactions_selected,
            &props.drawn_nodes_extended, &props.drawn_analysis_results_for_nodes,
            &props.drawn_analysis_results_for_elements, &props.min_selected_value,
            &props.max_selected_value) ||
            !Rc::ptr_eq(&self.props.drawn_elements, &props.drawn_elements)
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
            self.state.normalized_nodes = if !self.props.drawn_nodes_extended.is_empty()
                {
                    normalize_nodes(
                    Rc::new(self.props.drawn_nodes_extended.to_owned()),
                    self.props.canvas_width as GLElementsValues,
                    self.props.canvas_height as GLElementsValues)
                }
                else
                {
                    Vec::new()
                };
            self.state.under_cursor_color = [0; 4];
            self.state.selected_color = [0; 4];
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
            <div class={ POSTPROCESSOR_CANVAS_CONTAINER_CLASS }>
                <canvas ref=self.canvas_text_node_ref.clone(),
                    class={ POSTPROCESSOR_CANVAS_TEXT_CLASS },
                    onmousemove=self.link.callback(move |event: MouseEvent| Msg::MouseMove(event)),
                    onmouseleave=self.link.callback(|_| Msg::MouseLeave),
                    onmousedown=self.link.callback(|_| Msg::MouseDown),
                    onmouseup=self.link.callback(|_| Msg::MouseUp),
                    onwheel=self.link.callback(move |event: WheelEvent| Msg::MouseWheel(event)),
                    onclick=self.link.callback(|_| Msg::MouseClick)
                />
                <canvas ref=self.canvas_node_ref.clone()
                    id= { POSTPROCESSOR_CANVAS_GL_ID },
                    class={ POSTPROCESSOR_CANVAS_GL_CLASS },
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

        let gl: GL = canvas
            .get_context("webgl")
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


impl PostprocessorCanvas
{
    fn render_gl_and_ctx(&mut self, _timestamp: f64)
    {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");
        let ctx = self.ctx.as_ref().expect("CTX Context not initialized!");
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear_rect(0.0, 0.0, self.props.canvas_width as f64, self.props.canvas_height as f64);
        gl.enable(GL::DEPTH_TEST);
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.clear(GL::DEPTH_BUFFER_BIT);
        gl.line_width(10.0);
        let vertex_shader_code = include_str!("shaders/post_shader.vert");
        let fragment_shader_code = include_str!("shaders/post_shader.frag");

        let shader_program = initialize_shaders(&gl, vertex_shader_code, fragment_shader_code);
        let shaders_variables = ShadersVariables::initialize(&gl, &shader_program);

        gl.viewport(0, 0, self.props.canvas_width as i32, self.props.canvas_height as i32);
        let aspect: GLElementsValues = self.props.canvas_width as GLElementsValues /
            self.props.canvas_height as GLElementsValues;
        let z_near = 1.0 as GLElementsValues;
        let z_far = 101.0 as GLElementsValues;

        if !self.state.normalized_nodes.is_empty()
        {
            let nodes_number = self.state.normalized_nodes.len();
            let mut drawn_objects_buffers = Buffers::initialize(&gl);
            let mut drawn_object = DrawnObject::create();
            if self.props.is_plot_displacements_selected
            {
                drawn_object.add_deformed_shape_nodes(
                    &self.state.normalized_nodes[nodes_number / 2..],
                    GLMode::Selection, &self.state.under_cursor_color,
                    &self.state.selected_color);
            }

            if self.props.is_plot_reactions_selected
            {
                let reactions =
                    self.props.drawn_analysis_results_for_nodes
                        .iter()
                        .filter(|result|
                            result.bc_type == BCType::Force)
                        .collect::<Vec<_>>();
                if !reactions.is_empty()
                {
                    drawn_object.add_reactions(
                    &self.state.normalized_nodes.as_slice(),
                    &reactions,
                    DRAWN_FORCES_LINE_LENGTH / (1.0 + self.state.d_scale),
                    DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER,
                    DRAWN_FORCES_CAPS_HEIGHT / (1.0 + self.state.d_scale),
                    DRAWN_FORCES_CAPS_WIDTH / (1.0 + self.state.d_scale),
                    GLMode::Selection, &self.state.under_cursor_color,
                    &self.state.selected_color);
                }
            }

            if self.props.stress_component_selected.is_some()
            {
                if !self.props.drawn_elements.is_empty()
                {
                    match drawn_object.add_elements_with_results_for_selection(
                        &self.state.normalized_nodes.as_slice(),
                        &self.props.drawn_elements,
                        &self.props.drawn_analysis_results_for_elements)
                    {
                        Err(msg) =>
                            yew::services::DialogService::alert(&format!("{:?}", msg)),
                        Ok(()) => (),
                    }
                }
            }

            drawn_objects_buffers.render(&gl, &drawn_object, &shaders_variables);
            let point_size = 10.0 as GLElementsValues;

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

            gl.uniform1f(Some(&shaders_variables.point_size), point_size);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.projection_matrix), false, &projection_matrix);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.model_view_matrix), false, &model_view_matrix);

            drawn_object.draw(&gl);

            let mut pixels = [0u8; 4];
            match gl.read_pixels_with_opt_u8_array(
                self.state.cursor_coord_x, self.state.cursor_coord_y, 1, 1, GL::RGBA,
                GL::UNSIGNED_BYTE, Some(&mut pixels))
            {
                Ok(_) => self.state.under_cursor_color = pixels,
                Err(msg) => yew::services::DialogService::alert(&format!("{:?}", msg)),
            }

            gl.clear(GL::COLOR_BUFFER_BIT);
            gl.clear(GL::DEPTH_BUFFER_BIT);
            gl.line_width(1.0);

            drawn_object = DrawnObject::create();
            drawn_objects_buffers = Buffers::initialize(&gl);

            if self.props.is_plot_displacements_selected
            {
                drawn_object.add_nodes(
                    &self.state.normalized_nodes[..nodes_number / 2],
                    GLMode::Visible, &self.state.under_cursor_color,
                    &self.state.selected_color);

                drawn_object.add_deformed_shape_nodes(
                    &self.state.normalized_nodes[nodes_number / 2..],
                    GLMode::Visible, &self.state.under_cursor_color,
                    &self.state.selected_color);

                if !self.props.drawn_elements.is_empty()
                {
                    match drawn_object.add_elements(
                        &self.state.normalized_nodes[..nodes_number / 2],
                        &self.props.drawn_elements,
                        GLMode::Visible, &self.state.under_cursor_color,
                        &self.state.selected_color)
                    {
                        Err(msg) =>
                            yew::services::DialogService::alert(&format!("{:?}", msg)),
                        Ok(()) => (),
                    }

                    match drawn_object.add_deformed_shape_elements(
                        &self.state.normalized_nodes[nodes_number / 2..],
                        &self.props.drawn_elements,
                        GLMode::Visible, &self.state.under_cursor_color,
                        &self.state.selected_color)
                    {
                        Err(msg) =>
                            yew::services::DialogService::alert(&format!("{:?}", msg)),
                        Ok(()) => (),
                    }
                }
            }
            else if self.props.is_plot_reactions_selected
            {
                drawn_object.add_nodes(&self.state.normalized_nodes.as_slice(),
                    GLMode::Visible, &self.state.under_cursor_color,
                    &self.state.selected_color);

                if !self.props.drawn_elements.is_empty()
                {
                    match drawn_object.add_elements(
                        &self.state.normalized_nodes.as_slice(),
                        &self.props.drawn_elements,
                        GLMode::Visible, &self.state.under_cursor_color,
                        &self.state.selected_color)
                    {
                        Err(msg) =>
                            yew::services::DialogService::alert(&format!("{:?}", msg)),
                        Ok(()) => (),
                    }
                }

                let reactions =
                    self.props.drawn_analysis_results_for_nodes
                        .iter()
                        .filter(|result|
                            result.bc_type == BCType::Force)
                        .collect::<Vec<_>>();
                if !reactions.is_empty()
                {
                    drawn_object.add_reactions(
                    &self.state.normalized_nodes.as_slice(),
                    &reactions,
                    DRAWN_FORCES_LINE_LENGTH / (1.0 + self.state.d_scale),
                    DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER,
                    DRAWN_FORCES_CAPS_HEIGHT / (1.0 + self.state.d_scale),
                    DRAWN_FORCES_CAPS_WIDTH / (1.0 + self.state.d_scale),
                    GLMode::Visible, &self.state.under_cursor_color,
                    &self.state.selected_color);
                }
            }
            else if let Some(component) = self.props.stress_component_selected
            {
                drawn_object.add_nodes(&self.state.normalized_nodes.as_slice(),
                    GLMode::Visible, &self.state.under_cursor_color,
                    &self.state.selected_color);

                if !self.props.drawn_elements.is_empty()
                {
                    let boxed_component: Box<dyn EARComponentTrait> = Box::new(component);
                    match drawn_object.add_elements_with_results_for_visualization(
                        &self.state.normalized_nodes.as_slice(),
                        &self.props.drawn_elements,
                        &self.props.drawn_analysis_results_for_elements,
                        &EARType::Stress,
                        &boxed_component,
                        &self.props.min_selected_value,
                        &self.props.max_selected_value,
                        GLMode::Visible, &self.state.under_cursor_color,
                        &self.state.selected_color)
                    {
                        Err(msg) =>
                            yew::services::DialogService::alert(&format!("{:?}", msg)),
                        Ok(()) => (),
                    }
                }
            }
            else
            {
                drawn_object.add_nodes(&self.state.normalized_nodes.as_slice(),
                    GLMode::Visible, &self.state.under_cursor_color,
                    &self.state.selected_color);

                if !self.props.drawn_elements.is_empty()
                {
                    match drawn_object.add_elements(
                        &self.state.normalized_nodes.as_slice(),
                        &self.props.drawn_elements,
                        GLMode::Visible, &self.state.under_cursor_color,
                        &self.state.selected_color)
                    {
                        Err(msg) =>
                            yew::services::DialogService::alert(&format!("{:?}", msg)),
                        Ok(()) => (),
                    }
                }
            }

            drawn_objects_buffers.render(&gl, &drawn_object, &shaders_variables);
            let point_size = 5.0 as GLElementsValues;

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
            gl.uniform1f(Some(&shaders_variables.point_size), point_size);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.projection_matrix), false, &projection_matrix);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.model_view_matrix), false, &model_view_matrix);

            drawn_object.draw(&gl);

            let mut matrix = mat4::new_identity();
            mat4::mul(&mut matrix, &projection_matrix, &model_view_matrix);

            if self.props.is_plot_displacements_selected
            {
                for node in self.state.normalized_nodes[..nodes_number / 2].iter()
                {
                    let denotation_color = define_drawn_object_denotation_color(node.uid,
                        &self.state.selected_color, &self.state.under_cursor_color,
                        CANVAS_DRAWN_NODES_DENOTATION_COLOR);
                    ctx.set_fill_style(&denotation_color.into());
                    add_denotation(&ctx,
                    &[node.x - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                        node.y - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                        node.z,
                        1.0],
                    &matrix,
                    self.props.canvas_width as f32,
                    self.props.canvas_height as f32,
                    &node.number.to_string());
                    ctx.stroke();
                }

                let mut min_displacement = 0 as ElementsValues;
                let mut max_displacement = 0 as ElementsValues;
                for node in self.state.normalized_nodes[nodes_number / 2..].iter()
                {
                    let denotation_color = define_drawn_object_denotation_color(node.uid,
                        &self.state.selected_color, &self.state.under_cursor_color,
                        CANVAS_DRAWN_DEFORMED_SHAPE_NODES_DENOTATION_COLOR);
                    ctx.set_fill_style(&denotation_color.into());
                    let denotation =
                        {
                            let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);

                            let node_number = node.number as ElementsNumbers -
                                nodes_number as ElementsNumbers / 2;
                            let analysis_data =
                                {
                                    if let Some(position) =
                                        self.props.drawn_analysis_results_for_nodes
                                        .iter()
                                        .position(|data|
                                            data.node_number == node_number)
                                    {
                                        Some(&self.props.drawn_analysis_results_for_nodes[position])
                                    }
                                    else
                                    {
                                        None
                                    }
                                };
                            if analysis_data.is_some()
                            {
                                if analysis_data.unwrap().x_direction_value.is_some()
                                {
                                    x = analysis_data.unwrap().x_direction_value.unwrap();
                                }
                                if analysis_data.unwrap().y_direction_value.is_some()
                                {
                                    y = analysis_data.unwrap().y_direction_value.unwrap();
                                }
                                if analysis_data.unwrap().z_direction_value.is_some()
                                {
                                    z = analysis_data.unwrap().z_direction_value.unwrap();
                                }
                            }

                            let displacement_value = (x * x + y * y + z * z).sqrt();
                            if displacement_value < min_displacement
                            {
                                min_displacement = displacement_value;
                            }
                            if displacement_value > max_displacement
                            {
                                max_displacement = displacement_value;
                            }
                            if displacement_value > 0 as ElementsValues
                            {
                                format!("{:+.3e}", displacement_value)
                            }
                            else
                            {
                                "".to_string()
                            }
                        };
                    add_denotation(&ctx,
                    &[node.x - DRAWN_DEFORMED_SHAPE_NODES_DENOTATION_SHIFT /
                        (1.0 + self.state.d_scale),
                        node.y - DRAWN_DEFORMED_SHAPE_NODES_DENOTATION_SHIFT /
                            (1.0 + self.state.d_scale),
                        node.z,
                        1.0],
                    &matrix,
                    self.props.canvas_width as f32,
                    self.props.canvas_height as f32,
                    &denotation);
                    ctx.stroke();
                }
                ctx.set_fill_style(&HINTS_COLOR.into());
                add_displacements_hints(&ctx, self.props.canvas_width as f32,
                    self.props.canvas_height as f32, min_displacement, max_displacement);
                ctx.stroke();

                if !self.props.drawn_elements.is_empty()
                {
                    for element in self.props.drawn_elements.as_ref()
                    {
                        let denotation_color = define_drawn_object_denotation_color(element.uid,
                            &self.state.selected_color, &self.state.under_cursor_color,
                            CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR);
                        match element.find_denotation_coordinates(&self.state.normalized_nodes)
                        {
                            Ok(coordinates) =>
                                {
                                    ctx.set_fill_style(&denotation_color.into());
                                    add_denotation(&ctx,
                                    &[coordinates[0],
                                        coordinates[1] +
                                            DRAWN_ELEMENTS_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                                        coordinates[2],
                                        coordinates[3]],
                                    &matrix,
                                    self.props.canvas_width as f32,
                                    self.props.canvas_height as f32,
                                    &element.number.to_string());
                                    ctx.stroke();
                                },
                            Err(msg) =>
                                yew::services::DialogService::alert(&format!("{:?}", msg)),
                        }
                    }
                }
            }
            else if self.props.is_plot_reactions_selected
            {
                for node in self.state.normalized_nodes.iter()
                {
                    let denotation_color = define_drawn_object_denotation_color(node.uid,
                        &self.state.selected_color, &self.state.under_cursor_color,
                        CANVAS_DRAWN_NODES_DENOTATION_COLOR);
                    ctx.set_fill_style(&denotation_color.into());
                    add_denotation(&ctx,
                    &[node.x - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                        node.y - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                        node.z,
                        1.0],
                    &matrix,
                    self.props.canvas_width as f32,
                    self.props.canvas_height as f32,
                    &node.number.to_string());
                    ctx.stroke();
                }
                ctx.set_fill_style(&HINTS_COLOR.into());
                add_reactions_hints(&ctx, self.props.canvas_width as f32,
                    self.props.canvas_height as f32);
                ctx.stroke();

                if !self.props.drawn_elements.is_empty()
                {
                    for element in self.props.drawn_elements.as_ref()
                    {
                        let denotation_color = define_drawn_object_denotation_color(element.uid,
                            &self.state.selected_color, &self.state.under_cursor_color,
                            CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR);
                        match element.find_denotation_coordinates(&self.state.normalized_nodes)
                        {
                            Ok(coordinates) =>
                                {
                                    ctx.set_fill_style(&denotation_color.into());
                                    add_denotation(&ctx,
                                    &[coordinates[0],
                                        coordinates[1] +
                                            DRAWN_ELEMENTS_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                                        coordinates[2],
                                        coordinates[3]],
                                    &matrix,
                                    self.props.canvas_width as f32,
                                    self.props.canvas_height as f32,
                                    &element.number.to_string());
                                    ctx.stroke();
                                },
                            Err(msg) =>
                                yew::services::DialogService::alert(&format!("{:?}", msg)),
                        }
                    }
                }
            }
            else if self.props.stress_component_selected.is_some()
            {
                for node in self.state.normalized_nodes.iter()
                {
                    let denotation_color = define_drawn_object_denotation_color(node.uid,
                        &self.state.selected_color, &self.state.under_cursor_color,
                        CANVAS_DRAWN_NODES_DENOTATION_COLOR);
                    ctx.set_fill_style(&denotation_color.into());
                    add_denotation(&ctx,
                    &[node.x - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                        node.y - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                        node.z,
                        1.0],
                    &matrix,
                    self.props.canvas_width as f32,
                    self.props.canvas_height as f32,
                    &node.number.to_string());
                    ctx.stroke();
                }
                let stress_component = self.props.stress_component_selected.unwrap().as_str();
                ctx.set_fill_style(&HINTS_COLOR.into());
                add_stresses_hints(&ctx, self.props.canvas_width as f32,
                    self.props.canvas_height as f32, stress_component);
                ctx.stroke();

                add_color_bar(&ctx, self.props.canvas_width as f32,
                    self.props.canvas_height as f32);


                if !self.props.drawn_elements.is_empty()
                {
                    for element in self.props.drawn_elements.as_ref()
                    {
                        let uid =
                            {
                                if let Some(position) =
                                    self.props.drawn_analysis_results_for_elements
                                        .iter()
                                        .position(|data|
                                            data.element_analysis_data.number_same(element.number))
                                {
                                    self.props.drawn_analysis_results_for_elements[position].uid
                                }
                                else
                                {
                                    element.uid
                                }
                            };
                        let denotation_color = define_drawn_object_denotation_color(uid,
                            &self.state.selected_color, &self.state.under_cursor_color,
                            CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR);
                        match element.find_denotation_coordinates(&self.state.normalized_nodes)
                        {
                            Ok(coordinates) =>
                                {
                                    ctx.set_fill_style(&denotation_color.into());
                                    add_denotation(&ctx,
                                    &[coordinates[0],
                                        coordinates[1] +
                                            DRAWN_ELEMENTS_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                                        coordinates[2],
                                        coordinates[3]],
                                    &matrix,
                                    self.props.canvas_width as f32,
                                    self.props.canvas_height as f32,
                                    &element.number.to_string());
                                    ctx.stroke();
                                },
                            Err(msg) =>
                                yew::services::DialogService::alert(&format!("{:?}", msg)),
                        }
                    }
                }
            }
            else
            {
                for node in self.state.normalized_nodes.iter()
                {
                    let denotation_color = define_drawn_object_denotation_color(node.uid,
                        &self.state.selected_color, &self.state.under_cursor_color,
                        CANVAS_DRAWN_NODES_DENOTATION_COLOR);
                    ctx.set_fill_style(&denotation_color.into());
                    add_denotation(&ctx,
                    &[node.x - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                        node.y - DRAWN_NODES_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                        node.z,
                        1.0],
                    &matrix,
                    self.props.canvas_width as f32,
                    self.props.canvas_height as f32,
                    &node.number.to_string());
                    ctx.stroke();
                }
                if !self.props.drawn_elements.is_empty()
                {
                    for element in self.props.drawn_elements.as_ref()
                    {
                        let denotation_color = define_drawn_object_denotation_color(element.uid,
                            &self.state.selected_color, &self.state.under_cursor_color,
                            CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR);
                        match element.find_denotation_coordinates(&self.state.normalized_nodes)
                        {
                            Ok(coordinates) =>
                                {
                                    ctx.set_fill_style(&denotation_color.into());
                                    add_denotation(&ctx,
                                    &[coordinates[0],
                                        coordinates[1] +
                                            DRAWN_ELEMENTS_DENOTATION_SHIFT / (1.0 + self.state.d_scale),
                                        coordinates[2],
                                        coordinates[3]],
                                    &matrix,
                                    self.props.canvas_width as f32,
                                    self.props.canvas_height as f32,
                                    &element.number.to_string());
                                    ctx.stroke();
                                },
                            Err(msg) =>
                                yew::services::DialogService::alert(&format!("{:?}", msg)),
                        }
                    }
                }
            }

            // if !self.props.drawn_elements.is_empty()
            // {
            //     for element in self.props.drawn_elements.as_ref()
            //     {
            //         match element.find_denotation_coordinates(&normalized_nodes)
            //         {
            //             Ok(coordinates) =>
            //                 {
            //                     ctx.set_fill_style(&CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR.into());
            //                     add_denotation(&ctx,
            //                     &coordinates,
            //                     &matrix,
            //                     self.props.canvas_width as f32,
            //                     self.props.canvas_height as f32,
            //                     &element.number.to_string());
            //                     ctx.stroke();
            //                 },
            //             Err(e) => self.props.add_analysis_message.emit(e),
            //         }
            //     }
            // }

            // if !drawn_displacements.is_empty()
            // {
            //     for displacement in drawn_displacements
            //     {
            //         match displacement.find_denotation_coordinates(&normalized_nodes)
            //         {
            //             Ok(coordinates) =>
            //                 {
            //                     ctx.set_fill_style(&CANVAS_DRAWN_DISPLACEMENTS_DENOTATION_COLOR.into());
            //                     add_denotation(&ctx,
            //                     &[coordinates[0] + DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_X /
            //                         (1.0 + self.state.d_scale),
            //                         coordinates[1] - DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_Y /
            //                         (1.0 + self.state.d_scale),
            //                         coordinates[2], coordinates[3]],
            //                     &matrix,
            //                     self.props.canvas_width as f32,
            //                     self.props.canvas_height as f32,
            //                     &displacement.number.to_string());
            //                     ctx.stroke();
            //                 },
            //             Err(e) => self.props.add_analysis_message.emit(e)
            //         }
            //     }
            // }


            // if !drawn_forces.is_empty()
            // {
            //     for force in drawn_forces
            //     {
            //         match force.find_denotation_coordinates(&normalized_nodes)
            //         {
            //             Ok(coordinates) =>
            //                 {
            //                     ctx.set_fill_style(&CANVAS_DRAWN_FORCES_DENOTATION_COLOR.into());
            //                     add_denotation(&ctx,
            //                     &[coordinates[0] + DRAWN_FORCES_DENOTATION_SHIFT_X /
            //                         (1.0 + self.state.d_scale),
            //                         coordinates[1] + DRAWN_FORCES_DENOTATION_SHIFT_Y /
            //                         (1.0 + self.state.d_scale),
            //                         coordinates[2], coordinates[3]],
            //                     &matrix,
            //                     self.props.canvas_width as f32,
            //                     self.props.canvas_height as f32,
            //                     &format!("#{}", force.number));
            //                     ctx.stroke();
            //                 },
            //             Err(e) => self.props.add_analysis_message.emit(e)
            //         }
            //     }
            // }
        }

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
        let point_size = 5.0 as GLElementsValues;

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
        gl.uniform1f(Some(&shaders_variables.point_size), point_size);
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

        ctx.set_fill_style(&HINTS_COLOR.into());
        add_hints(&ctx, self.props.canvas_width as f32,
            self.props.canvas_height as f32);
        ctx.stroke();

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);

        self.render_loop = Some(handle);
    }
}
