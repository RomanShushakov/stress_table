#![recursion_limit="16384"]
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
        CanvasRenderingContext2d, HtmlSelectElement, HtmlOptionElement,
        HtmlCanvasElement, HtmlOptionsCollection, DomTokenList,
    };
use yew::services::resize::{WindowDimensions, ResizeTask, ResizeService};

mod math;
use math::math_aux_structs::Coordinates;
mod fe;
use fe::fe_node::FeNode;
use fe::elements::truss_element::Truss2n2ip;
use fe::elements::f_element::FElement;
use fe::fe_solver::FeModel;
use fe::fe_aux_structs::{Displacement, AxisComponent, Force, StrainStressComponent};

mod components;
use components::
    {
        AnalysisTypeMenu, NodeMenu, PreprocessorCanvas, ElementMenu,
        ViewMenu, DisplacementMenu, ForceMenu, ResultViewMenu, PostprocessorCanvas,
    };
mod auxiliary;
use auxiliary::
    {
        AuxElement, AnalysisType, View, ElementType, AuxDisplacement,
        AuxForce, AnalysisResult, ResultView, MinMaxValues
    };


pub const NUMBER_OF_DOF: i32 = 6;

const MAIN_CLASS: &str = "main";
const MAIN_CONTAINER_CLASS: &str = "main_container";
const PREPROCESSOR_CLASS: &str = "preprocessor";
const PREPROCESSOR_MENU_CLASS: &str = "preprocessor_menu";
pub const PREPROCESSOR_BUTTON_CLASS: &str = "preprocessor_button";
const PREPROCESSOR_CANVAS_CLASS: &str = "preprocessor_canvas";
const ANALYSIS_ERROR_CLASS: &str = "analysis_error";
const ANALYSIS_ERROR_MESSAGE_CLASS: &str = "analysis_error_message";
const ANALYSIS_ERROR_BUTTON_CLASS: &str = "analysis_error_button";
const POSTPROCESSOR_CLASS: &str = "postprocessor";
const POSTPROCESSOR_MENU_CLASS: &str = "postprocessor_menu";
pub const POSTPROCESSOR_BUTTON_CLASS: &str = "postprocessor_button";
const POSTPROCESSOR_CANVAS_CLASS: &str = "postprocessor_canvas";


struct State
{
    analysis_type: Option<AnalysisType>,
    view: View,
    canvas_width: u32,
    canvas_height: u32,
    nodes: Vec<FeNode<u16, f64>>,
    aux_elements: Vec<AuxElement>,
    aux_displacements: Vec<AuxDisplacement>,
    aux_forces: Vec<AuxForce>,
    analysis_error_message: Option<String>,
    analysis_result: Option<AnalysisResult>,
    result_view: Option<ResultView>,
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
    AddAuxForce(AuxForce),
    UpdateAuxForce((usize, AuxForce)),
    RemoveAuxForce(usize),
    Submit,
    ResetAnalysisErrorMessage,
    ChangeResultView(ResultView),
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


    fn remove_uncoupled_displacements(&mut self)
    {
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
                            ElementType::OtherType =>
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
    }


    fn remove_uncoupled_forces(&mut self)
    {
        let mut i = (self.state.aux_forces.len() - 1) as i32;
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
                                    (element.node_1_number == self.state.aux_forces[i as usize].node_number) ||
                                    (element.node_2_number == self.state.aux_forces[i as usize].node_number)
                                },
                            ElementType::OtherType =>
                                {
                                    (element.node_1_number == self.state.aux_forces[i as usize].node_number) ||
                                    (element.node_2_number == self.state.aux_forces[i as usize].node_number)
                                },
                        }
                    })
            {
                self.state.aux_forces.remove(i as usize);
            }
            i -= 1;
        }
    }


    fn check_preprocessor_data(&self) -> bool
    {
        if self.state.nodes.is_empty()
        {
            return false;
        }
        if self.state.aux_elements.is_empty()
        {
            return false;
        }
        if self.state.aux_displacements.is_empty()
        {
            return false;
        }
        true
    }


    fn submit(&mut self) -> Result<AnalysisResult, String>
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
                ElementType::OtherType => (),
            }
        }
        let mut applied_displacements = HashMap::new();
        for aux_displacement in &self.state.aux_displacements
        {
            let node_number = aux_displacement.node_number;
            if let Some(u_displacement) = aux_displacement.x_direction_value
            {
                applied_displacements.insert(Displacement { component: AxisComponent::U, node_number }, u_displacement);
            }
            if let Some(v_displacement) = aux_displacement.y_direction_value
            {
                applied_displacements.insert(Displacement { component: AxisComponent::V, node_number }, v_displacement);
            }
            if let Some(w_displacement) = aux_displacement.z_direction_value
            {
                applied_displacements.insert(Displacement { component: AxisComponent::W, node_number }, w_displacement);
            }
            if let Some(theta_u) = aux_displacement.yz_plane_value
            {
                applied_displacements.insert(Displacement { component: AxisComponent::ThetaU, node_number }, theta_u);
            }
            if let Some(theta_v) = aux_displacement.zx_plane_value
            {
                applied_displacements.insert(Displacement { component: AxisComponent::ThetaV, node_number }, theta_v);
            }
            if let Some(theta_w) = aux_displacement.xy_plane_value
            {
                applied_displacements.insert(Displacement { component: AxisComponent::ThetaW, node_number }, theta_w);
            }
        }

        let mut applied_forces = HashMap::new();
        for aux_force in &self.state.aux_forces
        {
            let node_number = aux_force.node_number;
            if let Some(force_x) = aux_force.force_x_value
            {
                applied_forces.insert(Force { component: AxisComponent::U, node_number }, force_x);
            }
            if let Some(force_y) = aux_force.force_y_value
            {
                applied_forces.insert(Force { component: AxisComponent::V, node_number }, force_y);
            }
            if let Some(force_z) = aux_force.force_z_value
            {
                applied_forces.insert(Force { component: AxisComponent::W, node_number }, force_z);
            }
            if let Some(moment_xy) = aux_force.moment_xy_value
            {
                applied_forces.insert(Force { component: AxisComponent::ThetaW, node_number }, moment_xy);
            }
            if let Some(moment_yz) = aux_force.moment_yz_value
            {
                applied_forces.insert(Force { component: AxisComponent::ThetaU, node_number }, moment_yz);
            }
            if let Some(moment_zx) = aux_force.moment_zx_value
            {
                applied_forces.insert(Force { component: AxisComponent::ThetaV, node_number }, moment_zx);
            }
        }
        let mut model = FeModel::create(
            self.state.nodes.to_owned(), elements, applied_displacements,
            if !applied_forces.is_empty() { Some(applied_forces) } else { None });
        model.compose_global_stiffness_matrix()?;
        model.calculate_reactions_and_displacements()?;

        if let Some(ref global_analysis_result) = model.global_analysis_result
        {
            let displacements = global_analysis_result.displacements.to_owned();
            let reactions = global_analysis_result.reactions.to_owned();
            yew::services::ConsoleService::log(&format!("Reactions: {:?}", reactions));
            yew::services::ConsoleService::log(&format!("Displacements: {:?}", displacements));

            let mut all_strains_and_stresses = HashMap::new();
            let mut min_max_stress_values: HashMap<StrainStressComponent, MinMaxValues> = HashMap::new();
            for element in model.elements
            {
                let element_strains_and_stresses =
                    element
                        .borrow_mut()
                        .calculate_strains_and_stresses(&displacements)?;

                for (element_number, strains_stresses) in element_strains_and_stresses
                {
                    yew::services::ConsoleService::log(&format!("Strains and stresses: {:?}, {:?}", element_number, strains_stresses));
                    all_strains_and_stresses.insert(element_number, strains_stresses.to_owned());
                    for strain_stress in &strains_stresses
                    {
                        let current_stress_component = strain_stress.stress.component;
                        let current_stress_value = strain_stress.stress.value;
                        if let Some(min_max_values) = min_max_stress_values.get_mut(&current_stress_component)
                        {
                            if current_stress_value < min_max_values.min_value
                            {
                                min_max_values.min_value = current_stress_value;
                            }
                            if current_stress_value > min_max_values.max_value
                            {
                                min_max_values.max_value = current_stress_value;
                            }
                        }
                        else
                        {
                            min_max_stress_values.insert(
                                current_stress_component,
                                MinMaxValues
                                    {
                                        min_value: current_stress_value,
                                        max_value: current_stress_value,
                                    }
                                );
                        }
                    }
                }
            }
            yew::services::ConsoleService::log(&format!("Min max stress values: {:?}", min_max_stress_values));
            Ok(AnalysisResult
                {
                    displacements,
                    reactions,
                    strains_and_stresses: all_strains_and_stresses,
                    min_max_stress_values
                })
        }
        else
        {
            Err("Global analysis results could not be extracted".to_string())
        }
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
                    analysis_type: None,
                    view: View::PlaneXY,
                    canvas_width: width,
                    canvas_height: height,
                    nodes: Vec::new(),
                    aux_elements: Vec::new(),
                    aux_displacements: Vec::new(),
                    aux_forces: Vec::new(),
                    analysis_error_message: None,
                    analysis_result: None,
                    result_view: None,
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
                        match element.element_type
                        {
                            ElementType::Truss2n2ip =>
                                {
                                    if element.node_1_number == removed_node.number
                                    {
                                        aux_elements_deletion_positions.push(pos);
                                    }
                                    if element.node_2_number == removed_node.number
                                    {
                                        aux_elements_deletion_positions.push(pos);
                                    }
                                },
                            ElementType::OtherType =>
                                {
                                    if element.node_1_number == removed_node.number
                                    {
                                        aux_elements_deletion_positions.push(pos);
                                    }
                                    if element.node_2_number == removed_node.number
                                    {
                                        aux_elements_deletion_positions.push(pos);
                                    }
                                },
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
                    self.remove_uncoupled_displacements();
                    self.remove_uncoupled_forces();
                },
            Msg::AddAuxElement(element) => self.state.aux_elements.push(element),
            Msg::UpdateAuxElement(data) => self.state.aux_elements[data.0] = data.1,
            Msg::RemoveAuxElement(position) =>
                {
                    self.state.aux_elements.remove(position);
                    self.remove_uncoupled_displacements();
                    self.remove_uncoupled_forces();
                },
            Msg::AddAuxDisplacement(displacement) => self.state.aux_displacements.push(displacement),
            Msg::UpdateAuxDisplacement(data) => self.state.aux_displacements[data.0] = data.1,
            Msg::RemoveAuxDisplacement(position) =>
                {
                    self.state.aux_displacements.remove(position);
                },
            Msg::AddAuxForce(force) => self.state.aux_forces.push(force),
            Msg::UpdateAuxForce(data) => self.state.aux_forces[data.0] = data.1,
            Msg::RemoveAuxForce(position) =>
                {
                    self.state.aux_forces.remove(position);
                },
            Msg::Submit =>
                {
                    match self.submit()
                    {
                        Ok(analysis_result) => self.state.analysis_result = Some(analysis_result),
                        Err(msg) =>
                            {
                                self.state.analysis_error_message = Some(msg);
                                self.state.analysis_result = None;
                                self.state.result_view = None;
                            },
                    }
                },
            Msg::ResetAnalysisErrorMessage => self.state.analysis_error_message = None,
            Msg::ChangeResultView(result_view) => self.state.result_view = Some(result_view),
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
        let handle_add_aux_force = self.link.callback(|force: AuxForce| Msg::AddAuxForce(force));
        let handle_update_aux_force = self.link.callback(|data: (usize, AuxForce)| Msg::UpdateAuxForce(data));
        let handle_remove_aux_force = self.link.callback(|position: usize| Msg::RemoveAuxForce(position));
        let handle_change_result_view = self.link.callback(|result_view: ResultView| Msg::ChangeResultView(result_view));
        html!
        {
            <main class={ MAIN_CLASS }>
                <div class={ MAIN_CONTAINER_CLASS }>
                    <div class={ PREPROCESSOR_CLASS }>
                        <div class={ PREPROCESSOR_MENU_CLASS }>
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
                            <ForceMenu
                                analysis_type=self.state.analysis_type.to_owned(),
                                aux_elements=self.state.aux_elements.to_owned(),
                                aux_forces=self.state.aux_forces.to_owned(),
                                add_aux_force=handle_add_aux_force,
                                update_aux_force=handle_update_aux_force,
                                remove_aux_force=handle_remove_aux_force,
                            />
                            <button class={ PREPROCESSOR_BUTTON_CLASS }
                                onclick=self.link.callback(|_| Msg::Submit),
                                disabled={ !self.check_preprocessor_data() },
                            >
                                { "Submit" }
                            </button>
                        </div>
                        <div class={ PREPROCESSOR_CANVAS_CLASS }>
                            <PreprocessorCanvas
                                view=self.state.view.to_owned(),
                                canvas_width=self.state.canvas_width,
                                canvas_height=self.state.canvas_height,
                                nodes=self.state.nodes.to_owned(),
                                aux_elements=self.state.aux_elements.to_owned(),
                                aux_displacements=self.state.aux_displacements.to_owned(),
                                aux_forces=self.state.aux_forces.to_owned(),
                            />
                        </div>
                    </div>
                    {
                        if let Some(error_message) = &self.state.analysis_error_message
                        {
                            html!
                            {
                                <div class={ ANALYSIS_ERROR_CLASS }>
                                    <p class={ ANALYSIS_ERROR_MESSAGE_CLASS }>{ error_message }</p>
                                    <button
                                        class={ ANALYSIS_ERROR_BUTTON_CLASS },
                                        onclick=self.link.callback(|_| Msg::ResetAnalysisErrorMessage)
                                    >
                                        { "Hide message" }
                                    </button>
                                </div>
                            }
                        }
                        else
                        {
                            html! {}
                        }
                    }
                    {
                        if let Some(analysis_result) = &self.state.analysis_result
                        {
                            html!
                            {
                                <div class={ POSTPROCESSOR_CLASS }>
                                    <div class={ POSTPROCESSOR_MENU_CLASS }>
                                        <ResultViewMenu
                                            result_view=self.state.result_view.to_owned(),
                                            change_result_view=handle_change_result_view,
                                        />
                                    </div>
                                    {
                                        if let Some(result_view) = &self.state.result_view
                                        {
                                            match result_view
                                            {
                                                ResultView::PrintAllResults =>
                                                    {
                                                        html! {  }
                                                    },
                                                _ =>
                                                    {
                                                        html!
                                                        {
                                                            <div class={ POSTPROCESSOR_CANVAS_CLASS }>
                                                                <PostprocessorCanvas
                                                                    view=self.state.view.to_owned(),
                                                                    canvas_width=self.state.canvas_width,
                                                                    canvas_height=self.state.canvas_height,
                                                                    nodes=self.state.nodes.to_owned(),
                                                                    aux_elements=self.state.aux_elements.to_owned(),
                                                                    analysis_result=analysis_result,
                                                                    result_view=result_view,
                                                                />
                                                            </div>
                                                        }
                                                    }
                                            }
                                        }
                                        else
                                        {
                                            html! {  }
                                        }
                                    }
                                </div>
                            }
                        }
                        else
                        {
                            html! {}
                        }
                    }
                </div>
            </main>
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
