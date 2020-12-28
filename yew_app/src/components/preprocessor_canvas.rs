use yew::prelude::*;
use std::f64;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::virtual_dom::VNode;
use web_sys::{ CanvasRenderingContext2d, HtmlCanvasElement };

use crate::fe::fe_node::FeNode;
use crate::auxiliary::{DrawnNode, View, AuxElement, ElementType, AuxDisplacement, AuxForce};


const CANVAS_BACKGROUND_COLOR: &str = "white";
const CANVAS_X_AXIS_COLOR: &str = "red";
const CANVAS_Y_AXIS_COLOR: &str = "green";
const CANVAS_NODES_COLOR: &str = "black";
const CANVAS_ELEMENTS_COLOR: &str = "blue";
const CANVAS_DISPLACEMENTS_COLOR: &str = "orange";
const CANVAS_FORCE_COLOR: &str = "magenta";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub view: View,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub nodes: Vec<FeNode<u16, f64>>,
    pub aux_elements: Vec<AuxElement>,
    pub aux_displacements: Vec<AuxDisplacement>,
    pub aux_forces: Vec<AuxForce>,
}


pub struct PreprocessorCanvas
{
    // link: ComponentLink<Self>,
    props: Props,
}


impl PreprocessorCanvas
{
    fn draw_plane_xy(&self) -> Html
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.create_element("canvas").unwrap();
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

        let x_origin = base_dimension as f64 / 45f64;
        let y_origin = self.props.canvas_height as f64 - base_dimension as f64 / 45f64;
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
                            y: self.props.canvas_height as f64 - y_imaging,
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

            if !self.props.aux_elements.is_empty()
            {
                for aux_element in self.props.aux_elements.iter()
                {
                    match aux_element.element_type
                    {
                        ElementType::Truss2n2ip =>
                            {
                                let node_1_position = drawn_nodes
                                    .iter()
                                    .position(|node| node.number == aux_element.node_1_number).unwrap();
                                let drawn_node_1 = drawn_nodes[node_1_position].to_owned();
                                let node_2_position = drawn_nodes
                                    .iter()
                                    .position(|node| node.number == aux_element.node_2_number).unwrap();
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
                                    aux_element.number.to_string().chars().count() as f64 * axis_line_length / 10f64,
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
                                    &aux_element.number.to_string(),
                                    x_center - axis_line_length / 20f64,
                                    y_center + axis_line_length / 20f64)
                                    .unwrap();
                                context.stroke();
                            },
                        ElementType::OtherType =>
                            {
                                let node_1_position = drawn_nodes
                                    .iter()
                                    .position(|node| node.number == aux_element.node_1_number).unwrap();
                                let drawn_node_1 = drawn_nodes[node_1_position].to_owned();
                                let node_2_position = drawn_nodes
                                    .iter()
                                    .position(|node| node.number == aux_element.node_2_number).unwrap();
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
                                    aux_element.number.to_string().chars().count() as f64 * axis_line_length / 10f64,
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
                                    &aux_element.number.to_string(),
                                    x_center - axis_line_length / 20f64,
                                    y_center + axis_line_length / 20f64)
                                    .unwrap();
                                context.stroke();
                            },
                    }
                }
            }

            if !self.props.aux_displacements.is_empty()
            {
                for aux_displacement in self.props.aux_displacements.iter()
                {
                    let node_position = drawn_nodes
                        .iter()
                        .position(|node| node.number == aux_displacement.node_number).unwrap();
                    let drawn_node = drawn_nodes[node_position].to_owned();

                    context.begin_path();
                    context.move_to(drawn_node.x, drawn_node.y + axis_line_length / 25f64);
                    context.set_stroke_style(&CANVAS_DISPLACEMENTS_COLOR.into());
                    context.line_to(
                        drawn_node.x - axis_line_length / 24f64,
                        drawn_node.y + axis_line_length / 25f64 + axis_line_length / 12f64);
                    context.line_to(
                        drawn_node.x + axis_line_length / 24f64,
                        drawn_node.y + axis_line_length / 25f64 + axis_line_length / 12f64);
                    context.line_to(drawn_node.x, drawn_node.y + axis_line_length / 25f64);
                    context.set_fill_style(&CANVAS_DISPLACEMENTS_COLOR.into());
                    context.fill();

                    context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                    context.fill_text(
                        &aux_displacement.number.to_string(),
                        drawn_node.x + axis_line_length / 10f64,
                        drawn_node.y + axis_line_length / 25f64 + axis_line_length / 10f64)
                        .unwrap();
                    context.stroke();

                }
            }

            if !self.props.aux_forces.is_empty()
            {
                for aux_force in self.props.aux_forces.iter()
                {
                    let node_position = drawn_nodes
                        .iter()
                        .position(|node| node.number == aux_force.node_number).unwrap();
                    let drawn_node = drawn_nodes[node_position].to_owned();

                    context.begin_path();
                    context.set_fill_style(&CANVAS_FORCE_COLOR.into());
                    context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                    context.fill_text(
                        &format!("#{}", aux_force.number),
                        drawn_node.x + axis_line_length / 14f64,
                        drawn_node.y - axis_line_length / 14f64)
                        .unwrap();
                    context.stroke();

                    if let Some(force_x) = aux_force.force_x_value
                    {
                        if force_x > 0f32
                        {
                            context.begin_path();
                            context.move_to(drawn_node.x + axis_line_length / 25f64, drawn_node.y);
                            context.set_stroke_style(&CANVAS_FORCE_COLOR.into());
                            context.line_to(
                                drawn_node.x + axis_line_length / 2f64 - axis_line_length / 10f64,
                                drawn_node.y);
                            context.move_to(drawn_node.x + axis_line_length / 2f64, drawn_node.y);
                            context.line_to(
                                drawn_node.x + axis_line_length / 2f64 - axis_line_length / 10f64,
                                drawn_node.y + axis_line_length / 30f64);
                            context.line_to(
                                drawn_node.x + axis_line_length / 2f64 - axis_line_length / 10f64,
                                drawn_node.y - axis_line_length / 30f64);
                            context.line_to(drawn_node.x + axis_line_length / 2f64, drawn_node.y);
                            context.set_fill_style(&CANVAS_FORCE_COLOR.into());
                            context.fill();
                            context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                            context.fill_text(
                                &format!("{:.2}", force_x),
                                drawn_node.x + axis_line_length / 2f64,
                                drawn_node.y - axis_line_length / 14f64)
                                .unwrap();
                            context.stroke();
                        }
                        if force_x < 0f32
                        {
                            context.begin_path();
                            context.move_to(drawn_node.x - axis_line_length / 25f64, drawn_node.y);
                            context.set_stroke_style(&CANVAS_FORCE_COLOR.into());
                            context.line_to(
                                drawn_node.x - axis_line_length / 2f64 + axis_line_length / 10f64,
                                drawn_node.y);
                            context.move_to(drawn_node.x - axis_line_length / 2f64, drawn_node.y);
                            context.line_to(
                                drawn_node.x - axis_line_length / 2f64 + axis_line_length / 10f64,
                                drawn_node.y + axis_line_length / 30f64);
                            context.line_to(
                                drawn_node.x - axis_line_length / 2f64 + axis_line_length / 10f64,
                                drawn_node.y - axis_line_length / 30f64);
                            context.line_to(drawn_node.x - axis_line_length / 2f64, drawn_node.y);
                            context.set_fill_style(&CANVAS_FORCE_COLOR.into());
                            context.fill();
                            context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                            context.fill_text(
                                &format!("{:.2}", force_x),
                                drawn_node.x - axis_line_length / 2f64 -
                                    (format!("{:.2}", force_x).chars().count() as f64 * axis_line_length / 14f64),
                                drawn_node.y - axis_line_length / 14f64)
                                .unwrap();
                            context.stroke();
                        }
                    }

                    if let Some(force_y) = aux_force.force_y_value
                    {
                        if force_y > 0f32
                        {
                            context.begin_path();
                            context.move_to(drawn_node.x, drawn_node.y - axis_line_length / 25f64);
                            context.set_stroke_style(&CANVAS_FORCE_COLOR.into());
                            context.line_to(
                                drawn_node.x,
                                drawn_node.y - axis_line_length / 2f64 + axis_line_length / 10f64);
                            context.move_to(drawn_node.x, drawn_node.y - axis_line_length / 2f64);
                            context.line_to(
                                drawn_node.x + axis_line_length / 30f64,
                                drawn_node.y - axis_line_length / 2f64 + axis_line_length / 10f64);
                            context.line_to(
                                drawn_node.x - axis_line_length / 30f64,
                                drawn_node.y - axis_line_length / 2f64 + axis_line_length / 10f64);
                            context.line_to(drawn_node.x, drawn_node.y - axis_line_length / 2f64);
                            context.set_fill_style(&CANVAS_FORCE_COLOR.into());
                            context.fill();
                            context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                            context.fill_text(
                                &format!("{:.2}", force_y),
                                drawn_node.x + axis_line_length / 14f64,
                                drawn_node.y - axis_line_length / 2f64)
                                .unwrap();
                            context.stroke();
                        }
                        if force_y < 0f32
                        {
                            context.begin_path();
                            context.move_to(drawn_node.x, drawn_node.y + axis_line_length / 25f64);
                            context.set_stroke_style(&CANVAS_FORCE_COLOR.into());
                            context.line_to(
                                drawn_node.x,
                                drawn_node.y + axis_line_length / 2f64 - axis_line_length / 10f64);
                            context.move_to(drawn_node.x, drawn_node.y + axis_line_length / 2f64);
                            context.line_to(
                                drawn_node.x + axis_line_length / 30f64,
                                drawn_node.y + axis_line_length / 2f64 - axis_line_length / 10f64);
                            context.line_to(
                                drawn_node.x - axis_line_length / 30f64,
                                drawn_node.y + axis_line_length / 2f64 - axis_line_length / 10f64);
                            context.line_to(drawn_node.x, drawn_node.y + axis_line_length / 2f64);
                            context.set_fill_style(&CANVAS_FORCE_COLOR.into());
                            context.fill();
                            context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                            context.fill_text(
                                &format!("{:.2}", force_y),
                                drawn_node.x + axis_line_length / 14f64,
                                drawn_node.y + axis_line_length / 2f64 + axis_line_length / 14f64)
                                .unwrap();
                            context.stroke();
                        }
                    }
                    if let Some(moment_xy) = aux_force.moment_xy_value
                    {
                        if moment_xy > 0f32
                        {
                            context.begin_path();
                            context.set_stroke_style(&CANVAS_FORCE_COLOR.into());
                            context
                                .arc(
                                    drawn_node.x,
                                    drawn_node.y,
                                    axis_line_length / 3f64,
                                    f64::consts::PI * 5f64 / 4f64,
                                    f64::consts::PI / 4f64
                                    )
                                .unwrap();
                            context.stroke();

                            context.begin_path();
                            context.move_to(
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2,
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2);
                            context.line_to(
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 +
                                    axis_line_length / 10f64 * (f64::consts::PI / 8f64).sin(),
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 -
                                    axis_line_length / 10f64 * (f64::consts::PI / 8f64).cos());
                            context.line_to(
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 +
                                    axis_line_length / 10f64 * (f64::consts::PI / 8f64).cos(),
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 -
                                    axis_line_length / 10f64 * (f64::consts::PI / 8f64).sin());
                            context.line_to(
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2,
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2);
                            context.set_fill_style(&CANVAS_FORCE_COLOR.into());
                            context.fill();
                            context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                            context.fill_text(
                                &format!("{:.2}", moment_xy),
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 -
                                    (format!("{:.2}", moment_xy).chars().count() as f64 + 1f64) * axis_line_length / 14f64,
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2)
                                .unwrap();
                            context.stroke();
                        }
                        if moment_xy < 0f32
                        {
                            context.begin_path();
                            context.set_stroke_style(&CANVAS_FORCE_COLOR.into());
                            context
                                .arc(
                                    drawn_node.x,
                                    drawn_node.y,
                                    axis_line_length / 3f64,
                                    f64::consts::PI / 4f64,
                                    f64::consts::PI * 5f64 / 4f64
                                    )
                                .unwrap();

                            context.stroke();
                            context.begin_path();
                            context.move_to(
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2,
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2);
                            context.line_to(
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 -
                                    axis_line_length / 10f64 * (f64::consts::PI / 8f64).sin(),
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 +
                                    axis_line_length / 10f64 * (f64::consts::PI / 8f64).cos());
                            context.line_to(
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 -
                                    axis_line_length / 10f64 * (f64::consts::PI / 8f64).cos(),
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 +
                                    axis_line_length / 10f64 * (f64::consts::PI / 8f64).sin());
                            context.line_to(
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2,
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2);
                            context.set_fill_style(&CANVAS_FORCE_COLOR.into());
                            context.fill();
                            context.set_font(&format!("{}px Serif", axis_line_length / 7f64));
                            context.fill_text(
                                &format!("{:.2}", moment_xy),
                                drawn_node.x - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2 -
                                    (format!("{:.2}", moment_xy).chars().count() as f64 + 1f64) * axis_line_length / 14f64,
                                drawn_node.y - axis_line_length / 3f64 * f64::consts::FRAC_1_SQRT_2)
                                .unwrap();
                            context.stroke();
                        }
                    }
                }
            }
        }
        let node = Node::from(canvas);
        let vnode = VNode::VRef(node);
        vnode
    }
}


impl Component for PreprocessorCanvas
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
