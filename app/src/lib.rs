#![recursion_limit="1024"]

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
            1u16, node_2.to_owned(), node_1.to_owned(),
            128000000.0, 0.0625, None
        );
    let element_2 = Truss2n2ip::create
        (
            2u16, node_2.to_owned(), node_3.to_owned(),
            128000000.0, 0.0625, None
        );
    let element_3 = Truss2n2ip::create
        (
            3u16, node_2.to_owned(), node_4.to_owned(),
            128000000.0, 0.0625, None
        );

    let mut elements: Vec<Rc<RefCell<dyn FElement<_, _, _>>>> = Vec::new();
    elements.push(Rc::new(RefCell::new(element_1)));
    elements.push(Rc::new(RefCell::new(element_2)));
    elements.push(Rc::new(RefCell::new(element_3)));

    // yew::services::ConsoleService::log(&format!("{:?}", elements[0].borrow().show_info().stiffness_properties));

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
use web_sys::
    {
        CanvasRenderingContext2d, HtmlSelectElement, HtmlOptionElement, HtmlCanvasElement,
        HtmlOptionsCollection, DomTokenList,
    };
use yew::services::resize::{WindowDimensions, ResizeTask, ResizeService};

mod components;
use components::NodesMenu;
use components::Canvas;
use components::ElementsMenu;


const CANVAS_ID: &str = "canvas";


struct State
{
    canvas_width: u32,
    canvas_height: u32,
    nodes: Vec<FeNode<u16, f64>>,
    elements: Vec<Rc<RefCell<dyn FElement<u16, f64, f32>>>>,
    max_stress: Option<f64>,
}


struct Model
{
    link: ComponentLink<Self>,
    state: State,
    resize_task: Option<ResizeTask>,
    resize_service: ResizeService,
}


enum Msg
{
    ExtractWindowDimensions(WindowDimensions),
    AddNode(FeNode<u16, f64>),
    UpdateNode((usize, FeNode<u16, f64>)),
    RemoveNode(usize),
    AddElement(Rc<RefCell<dyn FElement<u16, f64, f32>>>),
    UpdateElement((usize, Rc<RefCell<dyn FElement<u16, f64, f32>>>)),
    RemoveElement(usize),
    ShowResult,
}


impl Model
{
    fn follow_window_dimensions(&mut self)
    {
        let callback: Callback<WindowDimensions> = self.link
            .callback(|dimensions| Msg::ExtractWindowDimensions(dimensions));
        let task = ResizeService::register(&mut self.resize_service, callback);
        self.resize_task = Some(task);
    }


    fn extract_window_dimensions(&mut self, dimensions: WindowDimensions)
    {
        self.state.canvas_width = (dimensions.width as f32 * 0.8) as u32;
        self.state.canvas_height = (dimensions.height as f32 * 0.8) as u32;
    }
}


impl Component for Model
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let (width, height) =
            {
                let mut width = 320u32;
                let mut height = 240u32;
                let window = web_sys::window().unwrap();
                if let Ok(w) = window.inner_width()
                {
                    if let Some(w) = w.as_f64()
                    {
                        width = (w * 0.8) as u32;
                    }
                }
                if let Ok(h) = window.inner_height()
                {
                    if let Some(h) = h.as_f64()
                    {
                        height = (h * 0.8) as u32;
                    }
                }
                (width, height)
            };
        Self
        {
            link,
            state: State
                {
                    canvas_width: width, canvas_height: height, max_stress: None,
                    nodes: Vec::new(), elements: Vec::new(),
                },
            resize_task: None, resize_service: ResizeService::new(),
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ExtractWindowDimensions(window_dimensions) =>
                self.extract_window_dimensions(window_dimensions),
            Msg::AddNode(node) =>
                {
                    if let None = self.state.nodes
                        .iter()
                        .position(|existed_node|
                            {
                                (existed_node.coordinates.x == node.coordinates.x) &&
                                (existed_node.coordinates.y == node.coordinates.y)
                            }
                        )
                    {
                        self.state.nodes.push(node);
                    }
                    else
                    {
                        yew::services::DialogService::alert(
                            "The node with the same coordinates is already in use.");
                    }
                },
            Msg::UpdateNode(data) =>
                {
                    self.state.nodes[data.0] = data.1;
                },
            Msg::RemoveNode(position) =>
                {
                    self.state.nodes.remove(position);
                },
            Msg::AddElement(element) =>
                {
                    self.state.elements.push(element)
                },
            Msg::UpdateElement(data) =>
                {
                    self.state.elements[data.0] = data.1;
                },
            Msg::RemoveElement(position) =>
                {
                    self.state.elements.remove(position);
                },
            Msg::ShowResult =>
                {
                    if let Ok(stress) = result_extract()
                    {
                        self.state.max_stress = Some(stress);
                    }
                },
        }
        true
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }


    fn view(&self) -> Html
    {
        let handle_add_node = self.link.callback(|node: FeNode<u16, f64>| Msg::AddNode(node));
        let handle_update_node = self.link.callback(|data: (usize, FeNode<u16, f64>)| Msg::UpdateNode(data));
        let handle_remove_node = self.link.callback(|position: usize| Msg::RemoveNode(position));

        let handle_add_element = self.link.callback(|element: Rc<RefCell<dyn FElement<u16, f64, f32>>>| Msg::AddElement(element));
        let handle_update_element = self.link.callback(|data: (usize, Rc<RefCell<dyn FElement<u16, f64, f32>>>)| Msg::UpdateElement(data));
        let handle_remove_element = self.link.callback(|position: usize| Msg::RemoveElement(position));

        html! {
            <div class="container">
                <div class="preprocessor">
                    <div class="menu">
                        <NodesMenu
                            nodes=self.state.nodes.to_owned(), add_node=handle_add_node,
                            update_node=handle_update_node, remove_node=handle_remove_node,
                        />
                        <ElementsMenu
                            nodes=self.state.nodes.to_owned(), elements=self.state.elements.to_owned(),
                            add_element=handle_add_element, update_element=handle_update_element,
                            remove_element=handle_remove_element,
                        />
                        // <button class="button">{ "Elements" }</button>
                        <button class="button">{ "Forces" }</button>
                        <button class="button">{ "Displacements" }</button>
                        <button class="button" onclick=self.link.callback(|_| Msg::ShowResult)>{ "Analyze" }</button>
                    </div>
                    <div class="canvas">
                        <Canvas
                            canvas_width=self.state.canvas_width,
                            canvas_height=self.state.canvas_height,
                            nodes=self.state.nodes.to_owned(),
                        />
                    </div>
                </div>
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
            self.follow_window_dimensions();
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app()
{
    App::<Model>::new().mount_to_body();
}
