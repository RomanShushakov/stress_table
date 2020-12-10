use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::f64;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::virtual_dom::VNode;
use web_sys::{ CanvasRenderingContext2d, HtmlCanvasElement };

use crate::fe::node::FeNode;


const CANVAS_ID: &str = "canvas";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub nodes: Vec<FeNode<u16, f64>>,
}


pub struct Canvas
{
    link: ComponentLink<Self>,
    props: Props,
}


impl Canvas
{
    fn draw_canvas(&self) -> Html
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

        let x_origin = base_dimension as f64 / 20f64;
        let y_origin = self.props.canvas_height as f64 - base_dimension as f64 / 20f64;
        let axis_line_length = base_dimension as f64 / 7f64;
        let axis_line_width = axis_line_length / 50f64;

        context.begin_path();
        context.move_to(x_origin, y_origin);
        context.set_line_width(axis_line_width);
        context.set_stroke_style(&"red".into());
        context.line_to(x_origin + axis_line_length - axis_line_length / 7f64, y_origin);
        context.move_to(x_origin + axis_line_length, y_origin);
        context.line_to(
            x_origin + axis_line_length - axis_line_length / 7f64,
            y_origin - axis_line_length / 25f64);
        context.line_to(
            x_origin + axis_line_length - axis_line_length / 7f64,
            y_origin + axis_line_length / 25f64);
        context.line_to(x_origin + axis_line_length, y_origin);
        context.set_fill_style(&"red".into());
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
        context.set_stroke_style(&"green".into());
        context.line_to(x_origin, y_origin - axis_line_length + axis_line_length / 7f64);
        context.move_to(x_origin, y_origin - axis_line_length);
        context.line_to(
            x_origin - axis_line_length / 25f64,
            y_origin - axis_line_length + axis_line_length / 7f64);
        context.line_to(
            x_origin + axis_line_length / 25f64,
            y_origin - axis_line_length + axis_line_length / 7f64);
        context.line_to(x_origin, y_origin - axis_line_length);
        context.set_fill_style(&"green".into());
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
            let mut imaging_nodes = Vec::new();
            if self.props.nodes.len() == 1
            {
                imaging_nodes.push(
                    (
                        self.props.nodes[0].number,
                        (self.props.canvas_width / 2) as f64,
                        (self.props.canvas_height / 2) as f64,
                    )
                );
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
                    imaging_nodes.push(
                        (
                            node.number,
                            x_imaging,
                            self.props.canvas_height as f64 - y_imaging
                        ));
                }
            }

            for node in imaging_nodes
            {
                context.begin_path();
                context.move_to(node.1, node.2);
                context.set_stroke_style(&"black".into());
                context
                    .arc(
                        node.1 - axis_line_length / 25f64,
                        node.2,
                        axis_line_length / 25f64,
                        0.0,
                        f64::consts::PI * 2.0)
                    .unwrap();
                context.set_fill_style(&"black".into());
                context.fill();

                // context.save();
                // context.translate(node.1 - axis_line_length / 6f64,node.2 + axis_line_length / 6f64).unwrap();
                // context.rotate(f64::consts::PI / 2.0).unwrap();

                context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                context.fill_text(
                    &node.0.to_string(),
                    node.1 - axis_line_length / 6f64,
                    node.2 + axis_line_length / 6f64)
                    .unwrap();
                // context.fill_text(
                //     &node.0.to_string(),
                //     0f64,
                //     0f64)
                //     .unwrap();
                context.stroke();

                // context.restore();
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


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link }
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
        html!
        {
            { self.draw_canvas() }
        }
    }
}
