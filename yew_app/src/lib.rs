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
use yew_router::prelude::*;
use yew_router::agent::RouteRequest;

mod fem;
mod extended_matrix;

mod components;
use components::
    {
        AnalysisTypeMenu, NodeMenu, PreprocessorCanvas, ElementMenu,
        ViewMenu, DisplacementMenu, ForceMenu, // ResultViewMenu, PostprocessorCanvas,
        // AllResultsTable,
    };


mod auxiliary;
use auxiliary::
    {
        AnalysisType, View, FEDrawnNodeData, DrawnBCData,
        ResultView, MinMaxValues, // AnalysisResult,
    };
use crate::fem::{FEModel, FEData};
use crate::fem::{GlobalDOFParameter, BCType};
use crate::fem::GLOBAL_DOF;
use crate::auxiliary::FEDrawnElementData;

mod route;
use route::AppRoute;

mod pages;
use pages::{Preprocessor, Postprocessor};
use yew_router::router::Render;


pub type ElementsNumbers = u32;
pub type ElementsValues = f32;
pub type GLElementsNumbers = u16;
pub type GLElementsValues = f32;

pub const TOLERANCE: ElementsValues = 1e-6;

const MAIN_CLASS: &str = "main";
const MAIN_CONTAINER_CLASS: &str = "main_container";

const POSTPROCESSOR_CLASS: &str = "postprocessor";
const POSTPROCESSOR_MENU_CLASS: &str = "postprocessor_menu";
pub const POSTPROCESSOR_BUTTON_CLASS: &str = "postprocessor_button";
const POSTPROCESSOR_CANVAS_CLASS: &str = "postprocessor_canvas";


struct State
{
    analysis_type: Option<AnalysisType>,
    view: Option<View>,
    canvas_width: u32,
    canvas_height: u32,
    is_preprocessor_active: bool,
    fem: FEModel<ElementsNumbers, ElementsValues>,
    analysis_error_message: Option<String>,
    // analysis_result: Option<AnalysisResult>,
    result_view: Option<ResultView>,
}


struct Model
{
    link: ComponentLink<Self>,
    state: State,
    resize_task: Option<ResizeTask>,
}


enum Msg
{
    ExtractWindowDimensions(WindowDimensions),
    AddAnalysisType(AnalysisType),
    ChangeView(View),
    DiscardView,
    AddNode(FEDrawnNodeData),
    UpdateNode(FEDrawnNodeData),
    DeleteNode(ElementsNumbers),
    AddElement(FEDrawnElementData),
    UpdateElement(FEDrawnElementData),
    DeleteElement(ElementsNumbers),
    AddBC(DrawnBCData),
    UpdateBC(DrawnBCData),
    DeleteBC(DrawnBCData),
    // Submit,
    AddAnalysisErrorMessage(String),
    ResetAnalysisErrorMessage,
    EditStructure,
    ChangeResultView(ResultView),
}


impl Model
{
    fn follow_window_dimensions(&mut self)
    {
        let mut resize_service = ResizeService::new();
        let callback: Callback<WindowDimensions> = self.link
            .callback(|dimensions| Msg::ExtractWindowDimensions(dimensions));
        let task = ResizeService::register(&mut resize_service, callback);
        self.resize_task = Some(task);
    }


    fn extract_window_dimensions(&mut self, dimensions: WindowDimensions)
    {
        self.state.canvas_width = (dimensions.width as f32 * 0.8) as u32;
        self.state.canvas_height = (dimensions.height as f32 * 0.8) as u32;
    }


    // fn submit(&mut self) -> Result<AnalysisResult, String>
    // {
    //     self.state.nodes.sort_unstable_by(|a, b| a.number.partial_cmp(&b.number).unwrap());
    //     let mut elements: Vec<Rc<RefCell<dyn FElement<_, _, _>>>> = Vec::new();
    //     for aux_element in &self.state.aux_elements
    //     {
    //         match aux_element.element_type
    //         {
    //             ElementType::Truss2n2ip =>
    //                 {
    //                     let node_1_position = self.state.nodes
    //                         .iter()
    //                         .position(|node| node.number == aux_element.node_1_number)
    //                         .unwrap();
    //                     let node_1 = self.state.nodes[node_1_position].to_owned();
    //                     let node_2_position = self.state.nodes
    //                         .iter()
    //                         .position(|node| node.number == aux_element.node_2_number)
    //                         .unwrap();
    //                     let node_2 = self.state.nodes[node_2_position].to_owned();
    //                     let truss_element = Truss2n2ip::create(
    //                             aux_element.number, node_1, node_2,
    //                             aux_element.young_modulus, aux_element.area,
    //                             aux_element.area_2,
    //                         );
    //                     elements.push(Rc::new(RefCell::new(truss_element)));
    //                 },
    //             // ElementType::OtherType => (),
    //         }
    //     }
    //     let mut applied_displacements = HashMap::new();
    //     for aux_displacement in &self.state.aux_displacements
    //     {
    //         let node_number = aux_displacement.node_number;
    //         if let Some(u_displacement) = aux_displacement.x_direction_value
    //         {
    //             applied_displacements.insert(Displacement { component: AxisComponent::U, node_number }, u_displacement);
    //         }
    //         if let Some(v_displacement) = aux_displacement.y_direction_value
    //         {
    //             applied_displacements.insert(Displacement { component: AxisComponent::V, node_number }, v_displacement);
    //         }
    //         if let Some(w_displacement) = aux_displacement.z_direction_value
    //         {
    //             applied_displacements.insert(Displacement { component: AxisComponent::W, node_number }, w_displacement);
    //         }
    //         if let Some(theta_u) = aux_displacement.yz_plane_value
    //         {
    //             applied_displacements.insert(Displacement { component: AxisComponent::ThetaU, node_number }, theta_u);
    //         }
    //         if let Some(theta_v) = aux_displacement.zx_plane_value
    //         {
    //             applied_displacements.insert(Displacement { component: AxisComponent::ThetaV, node_number }, theta_v);
    //         }
    //         if let Some(theta_w) = aux_displacement.xy_plane_value
    //         {
    //             applied_displacements.insert(Displacement { component: AxisComponent::ThetaW, node_number }, theta_w);
    //         }
    //     }
    //
    //     let mut applied_forces = HashMap::new();
    //     for aux_force in &self.state.aux_forces
    //     {
    //         let node_number = aux_force.node_number;
    //         if let Some(force_x) = aux_force.force_x_value
    //         {
    //             applied_forces.insert(Force { component: AxisComponent::U, node_number }, force_x);
    //         }
    //         if let Some(force_y) = aux_force.force_y_value
    //         {
    //             applied_forces.insert(Force { component: AxisComponent::V, node_number }, force_y);
    //         }
    //         if let Some(force_z) = aux_force.force_z_value
    //         {
    //             applied_forces.insert(Force { component: AxisComponent::W, node_number }, force_z);
    //         }
    //         if let Some(moment_xy) = aux_force.moment_xy_value
    //         {
    //             applied_forces.insert(Force { component: AxisComponent::ThetaW, node_number }, moment_xy);
    //         }
    //         if let Some(moment_yz) = aux_force.moment_yz_value
    //         {
    //             applied_forces.insert(Force { component: AxisComponent::ThetaU, node_number }, moment_yz);
    //         }
    //         if let Some(moment_zx) = aux_force.moment_zx_value
    //         {
    //             applied_forces.insert(Force { component: AxisComponent::ThetaV, node_number }, moment_zx);
    //         }
    //     }
    //     let mut model = FeModel::create(
    //         self.state.nodes.to_owned(), elements, applied_displacements,
    //         if !applied_forces.is_empty() { Some(applied_forces) } else { None });
    //     model.update_fe_model_state()?;
    //     model.calculate_reactions_and_displacements()?;
    //
    //     if let Some(ref global_analysis_result) = model.global_analysis_result
    //     {
    //         let displacements = global_analysis_result.displacements.to_owned();
    //         let reactions = global_analysis_result.reactions.to_owned();
    //         yew::services::ConsoleService::log(&format!("Reactions: {:?}", reactions));
    //         yew::services::ConsoleService::log(&format!("Displacements: {:?}", displacements));
    //
    //         let mut all_strains_and_stresses = HashMap::new();
    //         let mut min_max_stress_values: HashMap<StrainStressComponent, MinMaxValues> = HashMap::new();
    //         for element in model.elements
    //         {
    //             let element_strains_and_stresses =
    //                 element
    //                     .borrow_mut()
    //                     .calculate_strains_and_stresses(&displacements)?;
    //
    //             for (element_number, strains_stresses) in element_strains_and_stresses
    //             {
    //                 yew::services::ConsoleService::log(&format!("Strains and stresses: {:?}, {:?}", element_number, strains_stresses));
    //                 all_strains_and_stresses.insert(element_number, strains_stresses.to_owned());
    //                 for strain_stress in &strains_stresses
    //                 {
    //                     let current_stress_component = strain_stress.stress.component;
    //                     let current_stress_value = strain_stress.stress.value;
    //                     if let Some(min_max_values) = min_max_stress_values.get_mut(&current_stress_component)
    //                     {
    //                         if current_stress_value < min_max_values.min_value
    //                         {
    //                             min_max_values.min_value = current_stress_value;
    //                         }
    //                         if current_stress_value > min_max_values.max_value
    //                         {
    //                             min_max_values.max_value = current_stress_value;
    //                         }
    //                     }
    //                     else
    //                     {
    //                         min_max_stress_values.insert(
    //                             current_stress_component,
    //                             MinMaxValues
    //                                 {
    //                                     min_value: current_stress_value,
    //                                     max_value: current_stress_value,
    //                                 }
    //                             );
    //                     }
    //                 }
    //             }
    //         }
    //         yew::services::ConsoleService::log(&format!("Min max stress values: {:?}", min_max_stress_values));
    //         Ok(AnalysisResult
    //             {
    //                 displacements,
    //                 reactions,
    //                 strains_and_stresses: all_strains_and_stresses,
    //                 min_max_stress_values
    //             })
    //     }
    //     else
    //     {
    //         Err("Global analysis results could not be extracted".to_string())
    //     }
    // }
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
        let fem = FEModel::create();
        Self
        {
            link,
            state: State
                {
                    analysis_type: None,
                    view: None,
                    canvas_width: width,
                    canvas_height: height,
                    is_preprocessor_active: true,
                    fem,
                    analysis_error_message: None,
                    // analysis_result: None,
                    result_view: None,
                },
            resize_task: None,
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ExtractWindowDimensions(window_dimensions) =>
                self.extract_window_dimensions(window_dimensions),
            Msg::AddAnalysisType(analysis_type) =>
                self.state.analysis_type = Some(analysis_type),
            Msg::ChangeView(view) => self.state.view = Some(view),
            Msg::DiscardView => self.state.view = None,
            Msg::AddNode(data) =>
                {
                    match self.state.fem.add_node(data.number, data.x, data.y, data.z)
                    {
                        Err(e) => self.state.analysis_error_message = Some(e),
                        _ => (),
                    }
                },
            Msg::UpdateNode(data) => match self.state.fem
                    .update_node(data.number, data.x, data.y, data.z)
                {
                    Err(e) => self.state.analysis_error_message = Some(e),
                    _ => (),
                },
            Msg::DeleteNode(number) => match self.state.fem.delete_node(number)
                {
                    Err(e) => self.state.analysis_error_message = Some(e),
                    _ => (),
                },
            Msg::AddElement(data) =>
                {
                    let fe_type = data.fe_type;
                    let nodes_numbers = data.nodes_numbers;
                    let number = data.number;
                    let properties = data.properties;
                    match self.state.fem
                        .add_element(fe_type, nodes_numbers,
                                     FEData { number, nodes: Vec::new(), properties })
                    {
                        Err(e) => self.state.analysis_error_message = Some(e),
                        _ => ()
                    }
                },
            Msg::UpdateElement(data) =>
                {
                    let nodes_numbers = data.nodes_numbers;
                    let number = data.number;
                    let properties = data.properties;
                    match self.state.fem
                        .update_element(nodes_numbers,
                            FEData { number, nodes: Vec::new(), properties })
                    {
                        Err(e) => self.state.analysis_error_message = Some(e),
                        _ => ()
                    }
                },
            Msg::DeleteElement(number) => match self.state.fem.delete_element(number)
                {
                    Err(e) => self.state.analysis_error_message = Some(e),
                    _ => (),
                },
            Msg::AddBC(data) =>
                {
                    let node_number = data.node_number;
                    let bcs = vec![
                        data.x_direction_value, data.y_direction_value,
                        data.z_direction_value, data.yz_plane_value,
                        data.zx_plane_value, data.xy_plane_value];
                    for i in 0..bcs.len()
                    {
                        if let Some(value) = bcs[i]
                        {
                            let bc_type = data.bc_type;
                            let number = data.number * GLOBAL_DOF + i as ElementsNumbers;
                            let dof_parameter =
                                GlobalDOFParameter::iterator().nth(i).unwrap();
                            match self.state.fem.add_bc(
                                bc_type, number, node_number, *dof_parameter, value)
                            {
                                Err(e) => self.state.analysis_error_message = Some(e),
                                _ => ()
                            }
                        }
                    }
                },
            Msg::UpdateBC(data) =>
                {
                    let node_number = data.node_number;
                    let bcs = vec![
                        data.x_direction_value, data.y_direction_value,
                        data.z_direction_value, data.yz_plane_value,
                        data.zx_plane_value, data.xy_plane_value];
                    for i in 0..bcs.len()
                    {
                        let bc_type = data.bc_type;
                        let number = data.number * GLOBAL_DOF + i as ElementsNumbers;
                        let dof_parameter =
                            GlobalDOFParameter::iterator().nth(i).unwrap();
                        if let Some(value) = bcs[i]
                        {
                            if self.state.fem.boundary_conditions
                                .iter()
                                .position(|model_bc|
                                    model_bc.number_same(number) && model_bc.type_same(bc_type))
                                .is_some()
                            {
                                match self.state.fem.update_bc(bc_type, number, node_number,
                                    *dof_parameter, value)
                                {
                                    Err(e) => self.state.analysis_error_message = Some(e),
                                    _ => (),
                                }
                            }
                            else
                            {
                                match self.state.fem.add_bc(
                                    bc_type, number, node_number, *dof_parameter, value)
                                {
                                    Err(e) =>
                                        self.state.analysis_error_message = Some(e),
                                    _ => (),
                                }
                            }
                        }
                        else
                        {
                            if self.state.fem.boundary_conditions
                                .iter()
                                .position(|bc|
                                    bc.number_same(number) &&
                                    bc.type_same(bc_type))
                                .is_some()
                            {
                                match self.state.fem.delete_bc(bc_type, number)
                                {
                                    Err(e) => self.state.analysis_error_message = Some(e),
                                    _ => ()
                                }
                            }
                        }
                    }
                },
            Msg::DeleteBC(data) =>
                {
                    let bcs = vec![
                        data.x_direction_value, data.y_direction_value,
                        data.z_direction_value, data.yz_plane_value,
                        data.zx_plane_value, data.xy_plane_value];
                    for i in 0..bcs.len()
                    {
                        if bcs[i].is_some()
                        {
                            let bc_type = data.bc_type;
                            let number = data.number * GLOBAL_DOF + i as ElementsNumbers;
                            match self.state.fem.delete_bc(bc_type, number)
                            {
                                Err(e) => self.state.analysis_error_message = Some(e),
                                _ => ()
                            }
                        }
                    }
                },
            Msg::AddAnalysisErrorMessage(msg) => self.state.analysis_error_message = Some(msg),
            Msg::ResetAnalysisErrorMessage => self.state.analysis_error_message = None,
            Msg::EditStructure =>
                {
                    self.state.is_preprocessor_active = true;
                    // self.state.analysis_result = None;
                    self.state.result_view = None;
                },
            Msg::ChangeResultView(result_view) =>
                self.state.result_view = Some(result_view),
        }
        true
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }


    fn view(&self) -> Html
    {
        let handle_add_analysis_type =
            self.link.callback(|analysis_type: AnalysisType| Msg::AddAnalysisType(analysis_type));

        let handle_add_analysis_error_message =
            self.link.callback(|msg: String| Msg::AddAnalysisErrorMessage(msg));

        let handle_change_view = self.link.callback(|view: View| Msg::ChangeView(view));
        let handle_discard_view = self.link.callback(|_| Msg::DiscardView);

        let nodes = self.state.fem.nodes_rc_clone();
        let handle_add_node =
            self.link
                .callback(|data: FEDrawnNodeData| Msg::AddNode(data));
        let handle_update_node =
            self.link
                .callback(|data: FEDrawnNodeData| Msg::UpdateNode(data));
        let handle_delete_node =
            self.link.callback(|number: ElementsNumbers| Msg::DeleteNode(number));

        let drawn_elements = self.state.fem.drawn_elements_rc();
        let handle_add_element =
            self.link.callback(|data: FEDrawnElementData| Msg::AddElement(data));
        let handle_update_element =
            self.link.callback(|data: FEDrawnElementData| Msg::UpdateElement(data));
        let handle_delete_element =
            self.link.callback(|number: ElementsNumbers| Msg::DeleteElement(number));

        let drawn_bcs = self.state.fem.drawn_bcs_rc();
        let handle_add_bc = self.link.callback(|data: DrawnBCData| Msg::AddBC(data));
        let handle_update_bc = self.link.callback(|data: DrawnBCData|
            Msg::UpdateBC(data));
        let handle_delete_bc = self.link.callback(|data: DrawnBCData|
            Msg::DeleteBC(data));

        let handle_change_result_view =
            self.link.callback(|result_view: ResultView| Msg::ChangeResultView(result_view));

        let handle_reset_analysis_error_message =
            self.link.callback(|_| Msg::ResetAnalysisErrorMessage);

        let analysis_type = self.state.analysis_type.to_owned();
        let view = self.state.view.to_owned();
        let preprocessor_is_active = self.state.is_preprocessor_active.to_owned();

        let canvas_width = self.state.canvas_width.to_owned();
        let canvas_height = self.state.canvas_height.to_owned();
        let analysis_error_message = self.state.analysis_error_message.to_owned();

        let render = Router::render(move |switch: AppRoute| match switch
        {
            AppRoute::Preprocessor =>
                html!
                {
                    <Preprocessor analysis_type=analysis_type.to_owned(),
                        add_analysis_type=handle_add_analysis_type.to_owned(),
                        view=view.to_owned(),
                        change_view=handle_change_view.to_owned(),
                        discard_view=handle_discard_view.to_owned(),
                        is_preprocessor_active=preprocessor_is_active.to_owned(),

                        nodes=Rc::clone(&nodes),
                        add_node=handle_add_node.to_owned(),
                        update_node=handle_update_node.to_owned(),
                        delete_node=handle_delete_node.to_owned(),

                        drawn_elements=Rc::clone(&drawn_elements),
                        add_element=handle_add_element.to_owned(),
                        update_element=handle_update_element.to_owned(),
                        delete_element=handle_delete_element.to_owned(),

                        drawn_bcs=Rc::clone(&drawn_bcs),
                        add_bc=handle_add_bc.to_owned(),
                        update_bc=handle_update_bc.to_owned(),
                        delete_bc=handle_delete_bc.to_owned(),
                        add_analysis_error_message=handle_add_analysis_error_message.to_owned(),

                        canvas_width=canvas_width.to_owned(),
                        canvas_height=canvas_height.to_owned(),
                        analysis_error_message=analysis_error_message.to_owned(),

                        reset_analysis_error_message=handle_reset_analysis_error_message.to_owned(),
                    />
                },
            AppRoute::Postprocessor => html! { <Postprocessor /> },
            AppRoute::HomePage =>
                html!
                {
                    <Preprocessor analysis_type=analysis_type.to_owned(),
                        add_analysis_type=handle_add_analysis_type.to_owned(),
                        view=view.to_owned(),
                        change_view=handle_change_view.to_owned(),
                        discard_view=handle_discard_view.to_owned(),
                        is_preprocessor_active=preprocessor_is_active.to_owned(),

                        nodes=Rc::clone(&nodes),
                        add_node=handle_add_node.to_owned(),
                        update_node=handle_update_node.to_owned(),
                        delete_node=handle_delete_node.to_owned(),

                        drawn_elements=Rc::clone(&drawn_elements),
                        add_element=handle_add_element.to_owned(),
                        update_element=handle_update_element.to_owned(),
                        delete_element=handle_delete_element.to_owned(),

                        drawn_bcs=Rc::clone(&drawn_bcs),
                        add_bc=handle_add_bc.to_owned(),
                        update_bc=handle_update_bc.to_owned(),
                        delete_bc=handle_delete_bc.to_owned(),
                        add_analysis_error_message=handle_add_analysis_error_message.to_owned(),

                        canvas_width=canvas_width.to_owned(),
                        canvas_height=canvas_height.to_owned(),
                        analysis_error_message=analysis_error_message.to_owned(),

                        reset_analysis_error_message=handle_reset_analysis_error_message.to_owned(),
                    />
                },
        });

        html!
        {
            <main class={ MAIN_CLASS }>
                <div class={ MAIN_CONTAINER_CLASS }>
                    <Router<AppRoute, ()> render=render />
                    // {
                    //     if let Some(analysis_result) = &self.state.analysis_result
                    //     {
                    //         html!
                    //         {
                    //             <div class={ POSTPROCESSOR_CLASS }>
                    //                 <div class={ POSTPROCESSOR_MENU_CLASS }>
                    //                     <button
                    //                         class={ POSTPROCESSOR_BUTTON_CLASS },
                    //                         onclick=self.link.callback(|_| Msg::EditStructure),
                    //                     >
                    //                         { "Edit model" }
                    //                     </button>
                    //                     <ResultViewMenu
                    //                         result_view=self.state.result_view.to_owned(),
                    //                         change_result_view=handle_change_result_view,
                    //                     />
                    //                 </div>
                    //                 {
                    //                     if let Some(result_view) = &self.state.result_view
                    //                     {
                    //                         match result_view
                    //                         {
                    //                             ResultView::PrintAllResults =>
                    //                                 {
                    //                                     html!
                    //                                     {
                    //                                         <AllResultsTable
                    //                                             nodes=self.state.nodes.to_owned(),
                    //                                             aux_elements=self.state.aux_elements.to_owned(),
                    //                                             aux_displacements=self.state.aux_displacements.to_owned(),
                    //                                             analysis_result=analysis_result.to_owned(),
                    //                                             canvas_width=self.state.canvas_width,
                    //                                         />
                    //                                     }
                    //                                 },
                    //                             _ =>
                    //                                 {
                    //                                     html!
                    //                                     {
                    //                                         <div class={ POSTPROCESSOR_CANVAS_CLASS }>
                    //                                             <PostprocessorCanvas
                    //                                                 view=self.state.view.to_owned(),
                    //                                                 canvas_width=self.state.canvas_width,
                    //                                                 canvas_height=self.state.canvas_height,
                    //                                                 nodes=self.state.nodes.to_owned(),
                    //                                                 aux_elements=self.state.aux_elements.to_owned(),
                    //                                                 analysis_result=analysis_result,
                    //                                                 result_view=result_view,
                    //                                             />
                    //                                         </div>
                    //                                     }
                    //                                 }
                    //                         }
                    //                     }
                    //                     else
                    //                     {
                    //                         html! {  }
                    //                     }
                    //                 }
                    //             </div>
                    //         }
                    //     }
                    //     else
                    //     {
                    //         html! {}
                    //     }
                    // }
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
