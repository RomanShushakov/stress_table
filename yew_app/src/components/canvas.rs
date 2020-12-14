use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::f64;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::virtual_dom::VNode;
use web_sys::{ CanvasRenderingContext2d, HtmlCanvasElement };

use crate::fe::node::FeNode;
use crate::auxiliary::{AuxTruss, DrawnNode, View};


const CANVAS_ID: &str = "canvas";
const CANVAS_BACKGROUND_COLOR: &str = "white";
const CANVAS_X_AXIS_COLOR: &str = "red";
const CANVAS_Y_AXIS_COLOR: &str = "green";
const CANVAS_NODES_COLOR: &str = "black";
const CANVAS_ELEMENTS_COLOR: &str = "blue";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub view: View,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub nodes: Vec<FeNode<u16, f64>>,
    pub truss_elements_prep: Vec<AuxTruss>,
}


pub struct Canvas
{
    // link: ComponentLink<Self>,
    props: Props,
}


impl Canvas
{
    fn draw_plane_xy(&self) -> Html
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.create_element("canvas").unwrap();
        element.set_id(CANVAS_ID);
        let canvas = element.dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        canvas.set_width(self.props.canvas_width);
        canvas.set_height(self.props.canvas_height);
        let base_dimension =
            {
                if self.props.canvas_width < self.props.canvas_height
                {
                    self.props.canvas_width
                }
                else
                {
                    self.props.canvas_height
                }
            };
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let x_origin = base_dimension as f64 / 30f64;
        let y_origin = self.props.canvas_height as f64 - base_dimension as f64 / 30f64;
        let axis_line_length = base_dimension as f64 / 7f64;
        let axis_line_width = axis_line_length / 50f64;

        context.begin_path();
        context.move_to(x_origin, y_origin);
        context.set_line_width(axis_line_width);
        context.set_stroke_style(&CANVAS_X_AXIS_COLOR.into());
        context.line_to(x_origin + axis_line_length - axis_line_length / 7f64, y_origin);
        context.move_to(x_origin + axis_line_length, y_origin);
        context.line_to(
            x_origin + axis_line_length - axis_line_length / 7f64,
            y_origin - axis_line_length / 25f64);
        context.line_to(
            x_origin + axis_line_length - axis_line_length / 7f64,
            y_origin + axis_line_length / 25f64);
        context.line_to(x_origin + axis_line_length, y_origin);
        context.set_fill_style(&CANVAS_X_AXIS_COLOR.into());
        context.fill();
        context.set_font(&format!("{}px Serif", axis_line_length as i32 / 6));
        context.fill_text(
            "X",
            x_origin + axis_line_length + axis_line_length / 10f64,
            y_origin + axis_line_length / 7f64)
            .unwrap();
        context.stroke();

        context.begin_path();
        context.move_to(x_origin, y_origin);
        context.set_stroke_style(&CANVAS_Y_AXIS_COLOR.into());
        context.line_to(x_origin, y_origin - axis_line_length + axis_line_length / 7f64);
        context.move_to(x_origin, y_origin - axis_line_length);
        context.line_to(
            x_origin - axis_line_length / 25f64,
            y_origin - axis_line_length + axis_line_length / 7f64);
        context.line_to(
            x_origin + axis_line_length / 25f64,
            y_origin - axis_line_length + axis_line_length / 7f64);
        context.line_to(x_origin, y_origin - axis_line_length);
        context.set_fill_style(&CANVAS_Y_AXIS_COLOR.into());
        context.fill();
        context.set_font(&format!("{}px Serif", axis_line_length as i32 / 6));
        context.fill_text(
            "Y",
            x_origin - axis_line_length / 7f64,
            y_origin - axis_line_length - axis_line_length / 10f64)
            .unwrap();
        context.stroke();

        if !self.props.nodes.is_empty()
        {
            let mut drawn_nodes = Vec::new();
            if self.props.nodes.len() == 1
            {
                drawn_nodes.push(DrawnNode
                    {
                        number: self.props.nodes[0].number,
                        x: (self.props.canvas_width / 2) as f64,
                        y: (self.props.canvas_height / 2) as f64,
                    });
            }
            else
            {
                let (mut x_min, mut x_max, mut y_min, mut y_max) =
                (
                    self.props.nodes[0].coordinates.x, self.props.nodes[0].coordinates.x,
                    self.props.nodes[0].coordinates.y, self.props.nodes[0].coordinates.y
                );
                for node in &self.props.nodes
                {
                    if node.coordinates.x < x_min
                    {
                        x_min = node.coordinates.x;
                    }
                    if node.coordinates.x > x_max
                    {
                        x_max = node.coordinates.x;
                    }
                    if node.coordinates.y < y_min
                    {
                        y_min = node.coordinates.y;
                    }
                    if node.coordinates.y > y_max
                    {
                        y_max = node.coordinates.y;
                    }
                }
                for node in &self.props.nodes
                {
                    let x_inter = 0.1 * self.props.canvas_width as f64 + (node.coordinates.x - x_min) *
                        0.8 * self.props.canvas_width as f64 / (x_max - x_min);
                    let y_inter = 0.1 * self.props.canvas_height as f64 + (node.coordinates.y - y_min) *
                        0.8 * self.props.canvas_height as f64 / (y_max - y_min);
                    let x_imaging =
                        {
                            if !x_inter.is_nan()
                            {
                                x_inter
                            }
                            else
                            {
                                (self.props.canvas_width / 2) as f64
                            }
                        };
                    let y_imaging =
                        {
                            if !y_inter.is_nan()
                            {
                                y_inter
                            }
                            else
                            {
                                (self.props.canvas_height / 2) as f64
                            }
                        };
                    drawn_nodes.push(DrawnNode
                        {
                            number: node.number,
                            x: x_imaging,
                            y: self.props.canvas_height as f64 - y_imaging
                        });
                }
            }

            for node in drawn_nodes.iter()
            {
                context.begin_path();
                context.move_to(node.x, node.y);
                context.set_stroke_style(&CANVAS_NODES_COLOR.into());
                context
                    .arc(
                        node.x,
                        node.y,
                        axis_line_length / 25f64,
                        0.0,
                        f64::consts::PI * 2.0)
                    .unwrap();
                context.set_fill_style(&CANVAS_NODES_COLOR.into());
                context.fill();

                // context.save();
                // context.translate(node.1 - axis_line_length / 6f64,node.2 + axis_line_length / 6f64).unwrap();
                // context.rotate(f64::consts::PI / 2.0).unwrap();

                context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                context.fill_text(
                    &node.number.to_string(),
                    node.x - axis_line_length / 6f64,
                    node.y + axis_line_length / 6f64)
                    .unwrap();
                // context.fill_text(
                //     &node.0.to_string(),
                //     0f64,
                //     0f64)
                //     .unwrap();
                context.stroke();

                // context.restore();
            }

            if !self.props.truss_elements_prep.is_empty()
            {
                for truss_element in self.props.truss_elements_prep.iter()
                {
                    let node_1_position = drawn_nodes
                        .iter()
                        .position(|node| node.number == truss_element.node_1_number).unwrap();
                    let drawn_node_1 = drawn_nodes[node_1_position].to_owned();
                    let node_2_position = drawn_nodes
                        .iter()
                        .position(|node| node.number == truss_element.node_2_number).unwrap();
                    let drawn_node_2 = drawn_nodes[node_2_position].to_owned();

                    context.begin_path();
                    context.move_to(drawn_node_1.x, drawn_node_1.y);
                    context.set_stroke_style(&CANVAS_ELEMENTS_COLOR.into());
                    context.line_to(drawn_node_2.x, drawn_node_2.y);
                    context.stroke();

                    let x_center = (drawn_node_1.x + drawn_node_2.x) / 2f64;
                    let y_center = (drawn_node_1.y + drawn_node_2.y) / 2f64;

                    context.begin_path();
                    context.set_stroke_style(&CANVAS_BACKGROUND_COLOR.into());
                    context
                    .arc(
                        x_center,
                        y_center,
                        axis_line_length / 10f64,
                        0.0,
                        f64::consts::PI * 2.0)
                    .unwrap();
                    context.set_fill_style(&CANVAS_BACKGROUND_COLOR.into());
                    context.fill();
                    context.stroke();

                    context.begin_path();
                    context.set_fill_style(&CANVAS_ELEMENTS_COLOR.into());
                    context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                    context.fill_text(
                        &truss_element.number.to_string(),
                        x_center - axis_line_length / 20f64,
                        y_center + axis_line_length / 20f64)
                        .unwrap();
                    context.stroke();
                }
            }
        }

        let node = Node::from(canvas);
        let vnode = VNode::VRef(node);
        vnode
    }
}


impl Component for Canvas
{
    type Message = ();
    type Properties = Props;


    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self
    {
        Self { props }
    }


    fn update(&mut self, _msg: Self::Message) -> ShouldRender
    {
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            true
        }
        else
        {
            false
        }
    }


    fn view(&self) -> Html
    {
        match self.props.view
        {
            View::PlaneXY =>
                html!
                {
                    { self.draw_plane_xy() }
                },
            _ => html! {}
        }

    }
}
