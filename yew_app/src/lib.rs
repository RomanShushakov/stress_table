#![recursion_limit="1024"]
mod math;
use math::math_aux_structs::Coordinates;
mod fe;
use fe::node::FeNode;
use fe::elements::truss::Truss2n2ip;
use fe::elements::element::FElement;
use fe::solver::FeModel;
use fe::fe_aux_structs::{Displacement, AxisComponent};


mod components;
use components::NodesMenu;
use components::Canvas;
use components::ElementsMenu;
mod auxiliary;
use auxiliary::AuxTruss;


use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


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


pub const NUMBER_OF_DOF: i32 = 6;


struct State
{
    canvas_width: u32,
    canvas_height: u32,
    nodes: Vec<FeNode<u16, f64>>,
    truss_elements_prep: Vec<AuxTruss>,
    max_stress: Option<f64>,
    error_message: Option<String>,
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
    AddAuxTrussElement(AuxTruss),
    UpdateAuxTrussElement((usize, AuxTruss)),
    RemoveTrussElement(usize),
    Submit,
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


    fn submit(&mut self) -> Result<f64, String>
    {
        self.state.nodes.sort_unstable_by(|a, b| a.number.partial_cmp(&b.number).unwrap());
        let mut elements: Vec<Rc<RefCell<dyn FElement<_, _, _>>>> = Vec::new();
        for aux_truss_element in &self.state.truss_elements_prep
        {
            let node_1_position = self.state.nodes
                .iter()
                .position(|node| node.number == aux_truss_element.node_1_number)
                .unwrap();
            let node_1 = self.state.nodes[node_1_position].to_owned();
            let node_2_position = self.state.nodes
                .iter()
                .position(|node| node.number == aux_truss_element.node_2_number)
                .unwrap();
            let node_2 = self.state.nodes[node_2_position].to_owned();
            let truss_element = Truss2n2ip::create(
                    aux_truss_element.number, node_1, node_2,
                    aux_truss_element.young_modulus, aux_truss_element.area,
                    None
                );
            elements.push(Rc::new(RefCell::new(truss_element)));
        }
        let mut applied_displacements = HashMap::new();
        applied_displacements.insert(Displacement { component: AxisComponent::U, node_number: 3 }, 0.0);
        applied_displacements.insert(Displacement { component: AxisComponent::V, node_number: 3 }, 0.0);
        applied_displacements.insert(Displacement { component: AxisComponent::U, node_number: 4 }, 0.0);
        applied_displacements.insert(Displacement { component: AxisComponent::V, node_number: 1 }, -0.025);
        let mut model = FeModel::create(self.state.nodes.to_owned(), elements, applied_displacements, None);
        model.compose_global_stiffness_matrix()?;
        model.analyze()?;

        let mut max_stress = 0f64;

        if let Some(ref analysis_result) = model.analysis_result
        {
            yew::services::ConsoleService::log(&format!("Reactions: {:?}", analysis_result.reactions));
            yew::services::ConsoleService::log(&format!("Displacements: {:?}", analysis_result.displacements));
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
            }
        }
        Ok(max_stress)
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
                    error_message: None,
                    nodes: Vec::new(), truss_elements_prep: Vec::new(), // elements: Vec::new(),
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
                    let removed_node = self.state.nodes.remove(position);
                    let mut truss_elements_deletion_positions = Vec::new();
                    for (pos, element) in self.state.truss_elements_prep.iter().enumerate()
                    {
                        if element.node_1_number == removed_node.number
                        {
                            truss_elements_deletion_positions.push(pos);
                        }
                        if element.node_2_number == removed_node.number
                        {
                            truss_elements_deletion_positions.push(pos);
                        }
                    }
                    if !truss_elements_deletion_positions.is_empty()
                    {
                        truss_elements_deletion_positions.sort();
                        for position in truss_elements_deletion_positions.iter().rev()
                        {
                            self.state.truss_elements_prep.remove(*position);
                        }
                    }
                },
            Msg::AddAuxTrussElement(element) =>
                {
                    let node_1_number_position = self.state.nodes
                        .iter()
                        .position(|node| node.number == element.node_1_number);
                    let node_2_number_position = self.state.nodes
                        .iter()
                        .position(|node| node.number == element.node_2_number);
                    if node_1_number_position.is_none() || node_2_number_position.is_none()
                    {
                        yew::services::DialogService::alert(
                            "The selected node or nodes do not exist.");
                    }
                    else
                    {
                        self.state.truss_elements_prep.push(element.to_owned());
                    }
                },
            Msg::UpdateAuxTrussElement(data) =>
                {
                    let node_1_number_position = self.state.nodes
                        .iter()
                        .position(|node| node.number == data.1.node_1_number);
                    let node_2_number_position = self.state.nodes
                        .iter()
                        .position(|node| node.number == data.1.node_2_number);
                    if node_1_number_position.is_none() || node_2_number_position.is_none()
                    {
                        yew::services::DialogService::alert(
                            "The selected node or nodes do not exist.");
                    }
                    else
                    {
                        self.state.truss_elements_prep[data.0] = data.1;
                    }
                },
            Msg::RemoveTrussElement(position) =>
                {
                    self.state.truss_elements_prep.remove(position);
                },
            Msg::Submit =>
                {
                    match self.submit()
                    {
                        Ok(stress) => self.state.max_stress = Some(stress),
                        Err(msg) => self.state.error_message = Some(msg),
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

        let handle_add_aux_truss_element =
            self.link.callback(|truss_element: AuxTruss| Msg::AddAuxTrussElement(truss_element));
        let handle_update_aux_truss_element =
            self.link.callback(|data: (usize, AuxTruss)| Msg::UpdateAuxTrussElement(data));
        let handle_remove_aux_truss_element =
            self.link.callback(|position: usize| Msg::RemoveTrussElement(position));

        html! {
            <div class="container">
                <div class="preprocessor">
                    <div class="menu">
                        <NodesMenu
                            nodes=self.state.nodes.to_owned(), add_node=handle_add_node,
                            update_node=handle_update_node, remove_node=handle_remove_node,
                        />
                        <ElementsMenu
                            truss_elements_prep=self.state.truss_elements_prep.to_owned(),
                            add_aux_truss_element=handle_add_aux_truss_element,
                            update_aux_truss_element=handle_update_aux_truss_element,
                            remove_aux_truss_element=handle_remove_aux_truss_element,
                        />
                        <button class="button">{ "Forces" }</button>
                        <button class="button">{ "Displacements" }</button>
                        <button class="button" onclick=self.link.callback(|_| Msg::Submit)>{ "Submit" }</button>
                    </div>
                    <div class="canvas">
                        <Canvas
                            canvas_width=self.state.canvas_width,
                            canvas_height=self.state.canvas_height,
                            nodes=self.state.nodes.to_owned(),
                            truss_elements_prep=self.state.truss_elements_prep.to_owned(),
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
                {
                    if let Some(error_message) = &self.state.error_message
                    {
                        html!
                        {
                            <p>{ error_message }</p>
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
