#![recursion_limit="256"]

mod math;
use math::math_aux_structs::Coordinates;


mod fe;
use fe::node::FeNode;
use fe::elements::truss::Truss2n2ip;
use fe::elements::element::FElement;
use fe::solver::FeModel;
use std::rc::Rc;
use std::cell::RefCell;
use crate::fe::fe_aux_structs::{Displacement, AxisComponent};
use std::collections::HashMap;


pub const NUMBER_OF_DOF: i32 = 6;


fn result_extract() -> Result<f64, String>
{
    let node_3 = FeNode { number: 3, coordinates: Coordinates { x: 0.0, y: 0.0, z: 0.0 } };
    let node_4 = FeNode { number: 4, coordinates: Coordinates { x: 0.0, y: 3.0, z: 0.0 } };
    let node_2 = FeNode { number: 2, coordinates: Coordinates { x: 4.0, y: 3.0, z: 0.0 } };
    let node_1 = FeNode { number: 1, coordinates: Coordinates { x: 4.0, y: 0.0, z: 0.0 } };
    let mut nodes = vec![node_2.to_owned(), node_1.to_owned(), node_3.to_owned(), node_4.to_owned()];
    nodes.sort_unstable_by(|a, b| a.number.partial_cmp(&b.number).unwrap());

    let element_1 = Truss2n2ip::create
        (
            1, node_2.to_owned(), node_1.to_owned(),
            128000000.0, 0.0625, None
        );
    let element_2 = Truss2n2ip::create
        (
            2, node_2.to_owned(), node_3.to_owned(),
            128000000.0, 0.0625, None
        );
    let element_3 = Truss2n2ip::create
        (
            3, node_2.to_owned(), node_4.to_owned(),
            128000000.0, 0.0625, None
        );

    let mut elements: Vec<Rc<RefCell<dyn FElement<_, _, _>>>> = Vec::new();
    elements.push(Rc::new(RefCell::new(element_1)));
    elements.push(Rc::new(RefCell::new(element_2)));
    elements.push(Rc::new(RefCell::new(element_3)));

    let mut applied_displacements = HashMap::new();
    applied_displacements.insert(Displacement { component: AxisComponent::U, node_number: 3 }, 0.0);
    applied_displacements.insert(Displacement { component: AxisComponent::V, node_number: 3 }, 0.0);
    applied_displacements.insert(Displacement { component: AxisComponent::U, node_number: 4 }, 0.0);
    // applied_displacements.insert(Displacement { component: Component::V, node_number: 4 }, 0.0);
    applied_displacements.insert(Displacement { component: AxisComponent::V, node_number: 1 }, -0.025);

    // let mut applied_forces = HashMap::new();
    // applied_forces.insert(Force { component: Component::V, node_number: 1 }, -100.0);
    // applied_forces.insert(Force { component: Component::V, node_number: 1 }, 100);
    // applied_forces.insert(Force { component: Component::W, node_number: 1 }, 100);

    let mut model = FeModel::create(nodes, elements, applied_displacements, None);

    model.compose_global_stiffness_matrix()?;
    // if let Some(ref state) = model.state
    // {
    //     println!("{:?}", state.displacements_indexes);
    //     println!("{:?}", state.forces_indexes);
    //     println!("{:?}", state.stiffness_matrix);
    // }
    model.analyze()?;

    let mut max_stress = 0f64;

    if let Some(ref analysis_result) = model.analysis_result
    {
        println!("Reactions: {:?}", analysis_result.reactions);
        println!("Displacements: {:?}", analysis_result.displacements);
    }
    for element in model.elements
    {
        let global_displacements =
            &model.analysis_result.as_ref().unwrap().displacements;
        let strains_and_stresses =
            element
                .borrow_mut()
                .calculate_strains_and_stresses(global_displacements)?;
        for (k, v) in strains_and_stresses
        {
            for stress_strain in v
            {
                if stress_strain.stress.value > max_stress
                {
                    max_stress = stress_strain.stress.value;
                }
            }
            // println!("For element: {:?}, strains and stresses are: {:?}", k, v);
        }
    }
    Ok(max_stress)
}


use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::f64;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::virtual_dom::VNode;


use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::__rt::core::cmp::min;


struct State
{
    canvas_width: u32,
    canvas_height: u32,
    max_stress: Option<f64>,
}


struct Model
{
    link: ComponentLink<Self>,
    state: State,
}


enum Msg
{
    SelectCanvasDimensions(ChangeData),
    ShowResult,
}


impl Model
{
    fn draw_right_eye(&self, ctx: &CanvasRenderingContext2d)
    {
        ctx.move_to(95.0, 65.0);
        ctx
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();
    }


    fn draw_coordinate_axes(&self) -> Html
    {
        let document = web_sys::window().unwrap().document().unwrap();
        // let canvas = document.get_element_by_id("canvas").unwrap();
        let element = document.create_element("canvas").unwrap();

        element.set_id("canvas");

        let canvas: web_sys::HtmlCanvasElement = element.dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_width(self.state.canvas_width);
        canvas.set_height(self.state.canvas_height);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let x_origin = self.state.canvas_height as f64 / 20f64;
        let y_origin = self.state.canvas_height as f64 - self.state.canvas_height as f64 / 20f64;
        let axis_line_length = self.state.canvas_height as f64 / 5f64;
        let axis_line_width = axis_line_length / 50f64;

        context.begin_path();
        context.move_to(x_origin, y_origin);
        context.set_line_width(axis_line_width);
        context.set_stroke_style(&"red".into());
        context.line_to(x_origin + axis_line_length, y_origin);
        context.line_to(x_origin + axis_line_length - axis_line_length / 5f64, y_origin - axis_line_length / 15f64);
        context.line_to(x_origin + axis_line_length - axis_line_length / 5f64, y_origin + axis_line_length / 15f64);
        context.line_to(x_origin + axis_line_length, y_origin);
        context.set_fill_style(&"red".into());
        context.fill();
        context.set_font(&format!("{}px Times New Roman", axis_line_length as i32 / 4));
        context.fill_text("X", x_origin + axis_line_length + axis_line_length / 8f64, y_origin + axis_line_length / 5f64).unwrap();
        context.stroke();

        context.begin_path();
        context.move_to(x_origin, y_origin);
        context.set_stroke_style(&"green".into());
        context.line_to(x_origin, y_origin - axis_line_length);
        context.line_to(x_origin - axis_line_length / 15f64, y_origin - axis_line_length + axis_line_length / 5f64);
        context.line_to(x_origin + axis_line_length / 15f64, y_origin - axis_line_length + axis_line_length / 5f64);
        context.line_to(x_origin, y_origin - axis_line_length);
        context.set_fill_style(&"green".into());
        context.fill();
        context.set_font(&format!("{}px Times New Roman", axis_line_length as i32 / 4));
        context.fill_text("Y", x_origin - axis_line_length / 5f64, y_origin - axis_line_length - axis_line_length / 8f64).unwrap();
        context.stroke();





        // Draw the outer circle.
        context.move_to(125.0, 75.0);
        context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

        // Draw the left eye.
        context.move_to(65.0, 65.0);
        context
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();


        for _ in 0..2
        {
            self.draw_right_eye(&context);
        }

        context.stroke();


        let node = Node::from(canvas);
        let vnode = VNode::VRef(node);
        vnode
    }
}


impl Component for Model
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self
    {

        Self
        {
            link,
            state: State { canvas_width: 320, canvas_height: 240, max_stress: None },
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::SelectCanvasDimensions(value) =>
                {
                    match value
                    {
                        ChangeData::Select(select_element) =>
                            {
                                let screen_size_values = select_element.value();
                                if screen_size_values == "640x480"
                                {
                                    self.state.canvas_width = 640;
                                    self.state.canvas_height = 480;
                                }
                                else if screen_size_values == "800x600"
                                {
                                    self.state.canvas_width = 800;
                                    self.state.canvas_height = 600;
                                }
                                else if screen_size_values == "1024x768"
                                {
                                    self.state.canvas_width = 1024;
                                    self.state.canvas_height = 768;
                                }
                                else
                                {
                                    self.state.canvas_width = 320;
                                    self.state.canvas_height = 240;
                                }
                            },
                        _ => (),
                    }
                },
            Msg::ShowResult =>
                {
                    if let Ok(stress) = result_extract()
                    {
                        self.state.max_stress = Some(stress);
                    }
                }
        }
        true
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }


    fn view(&self) -> Html
    {
        html!
        {
            <div>
                <select id="select" onchange=self.link.callback(|data: ChangeData| Msg::SelectCanvasDimensions(data))>
                    <option value="">{ "Select screen dimensions" }</option>
                    <option value="640x480">{ "640x480" }</option>
                    <option value="800x600">{ "800x600" }</option>
                    <option value="1024x768">{ "1024x768" }</option>
                </select>
                { self.draw_coordinate_axes() }
                <button onclick=self.link.callback(|_| Msg::ShowResult)>{ "Analyze" }</button>
                {
                    if let Some(max_stress) = self.state.max_stress
                    {
                        html!
                        {
                            <p>{ max_stress }</p>
                        }
                    }
                    else
                    {
                        html! {}
                    }
                }
            </div>
        }
    }


    fn rendered(&mut self, first_render: bool)
    {
        if first_render
        {
            self.draw_coordinate_axes();
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app()
{
    App::<Model>::new().mount_to_body();
}
