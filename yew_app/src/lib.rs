#![recursion_limit="2048"]
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

mod math;
use math::math_aux_structs::Coordinates;
mod fe;
use fe::fe_node::FeNode;
use fe::elements::truss_element::Truss2n2ip;
use fe::elements::f_element::FElement;
use fe::fe_solver::FeModel;
use fe::fe_aux_structs::{Displacement, AxisComponent};

mod components;
use components::{AnalysisTypeMenu, NodeMenu, Canvas, ElementMenu, ViewMenu, DisplacementMenu};
mod auxiliary;
use auxiliary::{AuxElement, AnalysisType, View, ElementType, AuxDisplacement};


pub const NUMBER_OF_DOF: i32 = 6;


struct State
{
    analysis_type: Option<AnalysisType>,
    view: View,
    canvas_width: u32,
    canvas_height: u32,
    nodes: Vec<FeNode<u16, f64>>,
    aux_elements: Vec<AuxElement>,
    aux_displacements: Vec<AuxDisplacement>,
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
    AddAnalysisType(AnalysisType),
    ChangeView(View),
    AddNode(FeNode<u16, f64>),
    UpdateNode((usize, FeNode<u16, f64>)),
    RemoveNode(usize),
    AddAuxElement(AuxElement),
    UpdateAuxElement((usize, AuxElement)),
    RemoveAuxElement(usize),
    AddAuxDisplacement(AuxDisplacement),
    UpdateAuxDisplacement((usize, AuxDisplacement)),
    RemoveAuxDisplacement(usize),
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
        for aux_element in &self.state.aux_elements
        {
            match aux_element.element_type
            {
                ElementType::Truss2n2ip =>
                    {
                        let node_1_position = self.state.nodes
                            .iter()
                            .position(|node| node.number == aux_element.node_1_number)
                            .unwrap();
                        let node_1 = self.state.nodes[node_1_position].to_owned();
                        let node_2_position = self.state.nodes
                            .iter()
                            .position(|node| node.number == aux_element.node_2_number)
                            .unwrap();
                        let node_2 = self.state.nodes[node_2_position].to_owned();
                        let truss_element = Truss2n2ip::create(
                                aux_element.number, node_1, node_2,
                                aux_element.young_modulus, aux_element.area,
                                aux_element.area_2,
                            );
                        elements.push(Rc::new(RefCell::new(truss_element)));
                    },
                // ElementType::OtherType => (),
            }
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
                    analysis_type: None, view: View::PlaneXY,
                    canvas_width: width, canvas_height: height,
                    nodes: Vec::new(), aux_elements: Vec::new(),
                    aux_displacements: Vec::new(),
                    max_stress: None, error_message: None,
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
            Msg::AddAnalysisType(analysis_type) => self.state.analysis_type = Some(analysis_type),
            Msg::ChangeView(view) => self.state.view = view,
            Msg::AddNode(node) => self.state.nodes.push(node),
            Msg::UpdateNode(data) => self.state.nodes[data.0] = data.1,
            Msg::RemoveNode(position) =>
                {
                    let removed_node = self.state.nodes.remove(position);
                    let mut aux_elements_deletion_positions = Vec::new();
                    for (pos, element) in self.state.aux_elements.iter().enumerate()
                    {
                        if element.node_1_number == removed_node.number
                        {
                            aux_elements_deletion_positions.push(pos);
                        }
                        if element.node_2_number == removed_node.number
                        {
                            aux_elements_deletion_positions.push(pos);
                        }
                    }
                    if !aux_elements_deletion_positions.is_empty()
                    {
                        aux_elements_deletion_positions.sort();
                        for position in aux_elements_deletion_positions.iter().rev()
                        {
                            self.state.aux_elements.remove(*position);
                        }
                    }
                    let mut i = (self.state.aux_displacements.len() - 1) as i32;
                    while i >= 0
                    {
                        if let None = self.state.aux_elements
                            .iter()
                            .position(|element|
                                {
                                    match element.element_type
                                    {
                                        ElementType::Truss2n2ip =>
                                            {
                                                (element.node_1_number == self.state.aux_displacements[i as usize].node_number) ||
                                                (element.node_2_number == self.state.aux_displacements[i as usize].node_number)
                                            },
                                    }
                                })
                        {
                            self.state.aux_displacements.remove(i as usize);
                        }
                        i -= 1;
                    }
                },
            Msg::AddAuxElement(element) => self.state.aux_elements.push(element),
            Msg::UpdateAuxElement(data) => self.state.aux_elements[data.0] = data.1,
            Msg::RemoveAuxElement(position) =>
                {
                    self.state.aux_elements.remove(position);
                    let mut i = (self.state.aux_displacements.len() - 1) as i32;
                    while i > 0
                    {
                        if let None = self.state.aux_elements
                            .iter()
                            .position(|element|
                                {
                                    match element.element_type
                                    {
                                        ElementType::Truss2n2ip =>
                                            {
                                                (element.node_1_number == self.state.aux_displacements[i as usize].node_number) ||
                                                (element.node_2_number == self.state.aux_displacements[i as usize].node_number)
                                            },
                                    }
                                })
                        {
                            self.state.aux_displacements.remove(i as usize);
                        }
                        i -= 1;
                    }
                },
            Msg::AddAuxDisplacement(displacement) => self.state.aux_displacements.push(displacement),
            Msg::UpdateAuxDisplacement(data) => self.state.aux_displacements[data.0] = data.1,
            Msg::RemoveAuxDisplacement(position) =>
                {
                    self.state.aux_displacements.remove(position);
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
        let handle_add_analysis_type = self.link.callback(|analysis_type: AnalysisType| Msg::AddAnalysisType(analysis_type));
        let handle_change_view = self.link.callback(|view: View| Msg::ChangeView(view));
        let handle_add_node = self.link.callback(|node: FeNode<u16, f64>| Msg::AddNode(node));
        let handle_update_node = self.link.callback(|data: (usize, FeNode<u16, f64>)| Msg::UpdateNode(data));
        let handle_remove_node = self.link.callback(|position: usize| Msg::RemoveNode(position));
        let handle_add_aux_element =
            self.link.callback(|element: AuxElement| Msg::AddAuxElement(element));
        let handle_update_aux_element =
            self.link.callback(|data: (usize, AuxElement)| Msg::UpdateAuxElement(data));
        let handle_remove_aux_element =
            self.link.callback(|position: usize| Msg::RemoveAuxElement(position));
        let handle_add_aux_displacement =
            self.link.callback(|displacement: AuxDisplacement| Msg::AddAuxDisplacement(displacement));
        let handle_update_aux_displacement =
            self.link.callback(|data: (usize, AuxDisplacement)| Msg::UpdateAuxDisplacement(data));
        let handle_remove_aux_displacement =
            self.link.callback(|position: usize| Msg::RemoveAuxDisplacement(position));
        html! {
            <div class="container">
                <div class="preprocessor">
                    <div class="menu">
                        <AnalysisTypeMenu
                            analysis_type=self.state.analysis_type.to_owned(),
                            add_analysis_type=handle_add_analysis_type,
                        />
                        <ViewMenu
                            view=self.state.view.to_owned(),
                            change_view=handle_change_view,
                        />
                        <NodeMenu
                            analysis_type=self.state.analysis_type.to_owned(),
                            nodes=self.state.nodes.to_owned(), add_node=handle_add_node,
                            update_node=handle_update_node, remove_node=handle_remove_node,
                        />
                        <ElementMenu
                            analysis_type=self.state.analysis_type.to_owned(),
                            nodes=self.state.nodes.to_owned(),
                            aux_elements=self.state.aux_elements.to_owned(),
                            add_aux_element=handle_add_aux_element,
                            update_aux_element=handle_update_aux_element,
                            remove_aux_element=handle_remove_aux_element,
                        />
                        <DisplacementMenu
                            analysis_type=self.state.analysis_type.to_owned(),
                            aux_elements=self.state.aux_elements.to_owned(),
                            aux_displacements=self.state.aux_displacements.to_owned(),
                            add_aux_displacement=handle_add_aux_displacement,
                            update_aux_displacement=handle_update_aux_displacement,
                            remove_aux_displacement=handle_remove_aux_displacement,
                        />
                        <button class="button">{ "Force" }</button>
                        <button class="button" onclick=self.link.callback(|_| Msg::Submit)>{ "Submit" }</button>
                    </div>
                    <div class="canvas">
                        <Canvas
                            view=self.state.view.to_owned(),
                            canvas_width=self.state.canvas_width,
                            canvas_height=self.state.canvas_height,
                            nodes=self.state.nodes.to_owned(),
                            aux_elements=self.state.aux_elements.to_owned(),
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
