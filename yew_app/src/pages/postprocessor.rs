use yew::prelude::*;
use yew_router::prelude::RouterButton;
use std::rc::Rc;

use crate::route::AppRoute;
use crate::fem::EARComponentTrait;
use crate::fem::{Displacements};
use crate::fem::{GlobalDOFParameter, BCType, EARType, StressStrainComponent, ForceComponent};
use crate::{ElementsNumbers, ElementsValues, UIDNumbers};

use crate::components::
    {
        ViewMenu, PostprocessorCanvas, PlotDisplacementsMenu, PlotStressesMenu, PlotStrainsMenu,
        PlotForcesMenu,
    };
use crate::auxiliary::
    {
        View, FEDrawnNodeData, FEDrawnElementData, FEDrawnAnalysisResultNodeData,
        DrawnAnalysisResultElementData
    };
use crate::fem::global_analysis::fe_global_analysis_result::Reactions;
use crate::fem::element_analysis::fe_element_analysis_result::ElementsAnalysisResult;


const POSTPROCESSOR_CLASS: &str = "postprocessor";
const POSTPROCESSOR_MENU_CLASS: &str = "postprocessor_menu";
pub const POSTPROCESSOR_BUTTON_CLASS: &str = "postprocessor_button";
const POSTPROCESSOR_CANVAS_CLASS: &str = "postprocessor_canvas";
const ANALYSIS_INFO_CLASS: &str = "analysis_info";
const ANALYSIS_MESSAGE_CLASS: &str = "analysis_message";


#[derive(Properties, Clone)]
pub struct Props
{
    pub view: Option<View>,
    pub change_view: Callback<View>,
    pub discard_view: Callback<()>,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub drawn_nodes: Rc<Vec<FEDrawnNodeData>>,
    pub drawn_elements: Rc<Vec<FEDrawnElementData>>,
    pub postproc_init_uid_number: UIDNumbers,
    pub global_displacements: Rc<Option<Displacements<ElementsNumbers, ElementsValues>>>,
    pub reactions: Rc<Option<Reactions<ElementsNumbers, ElementsValues>>>,
    pub elements_analysis_result: Rc<Option<ElementsAnalysisResult<ElementsNumbers, ElementsValues>>>,
}


pub struct State
{
    object_info: Option<String>,
    pub magnitude: ElementsValues,
    pub drawn_nodes_extended: Vec<FEDrawnNodeData>,
    pub drawn_analysis_results_for_nodes: Vec<FEDrawnAnalysisResultNodeData>,
    pub drawn_analysis_results_for_elements: Vec<DrawnAnalysisResultElementData>,
    pub is_plot_displacements_selected: bool,
    pub is_plot_reactions_selected: bool,
    pub stress_component_selected: Option<StressStrainComponent>,
    pub strain_component_selected: Option<StressStrainComponent>,
    pub force_component_selected: Option<ForceComponent>,
    pub min_selected_value: Option<ElementsValues>,
    pub max_selected_value: Option<ElementsValues>,
}


pub struct Postprocessor
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


impl Postprocessor
{
    fn default_drawn_nodes_extended_and_analysis_results_for_nodes(&mut self)
    {
        self.state.drawn_analysis_results_for_nodes = Vec::new();
        self.state.drawn_nodes_extended = if !self.props.drawn_nodes.is_empty()
        {
            (*self.props.drawn_nodes.as_ref()).clone()
        }
        else
        {
            Vec::new()
        };
    }


    fn default_analysis_results_for_elements(&mut self)
    {
        self.state.drawn_analysis_results_for_elements = Vec::new();
    }


    fn extend_by_deformed_shape_nodes(&mut self)
    {
        self.default_drawn_nodes_extended_and_analysis_results_for_nodes();
        if let Some(global_displacements) =
            self.props.global_displacements.as_ref()
        {
            let mut uid = self.props.postproc_init_uid_number;
            for drawn_node in self.props.drawn_nodes.iter()
            {
                uid += 1;
                let initial_node_number = drawn_node.number;
                let mut drawn_analysis_result_node_data = FEDrawnAnalysisResultNodeData
                    {
                        uid, bc_type: BCType::Displacement,
                        node_number: initial_node_number,
                        x_direction_value: None, y_direction_value: None, z_direction_value: None,
                        xy_plane_value: None, yz_plane_value: None, zx_plane_value: None,
                    };
                let deformed_shape_node_number = initial_node_number +
                    self.props.drawn_nodes.len() as ElementsNumbers;
                let initial_node_x = drawn_node.x;
                let deformed_shape_node_x =
                {
                    if let Some(position) = global_displacements.dof_parameters_data
                        .iter()
                        .position(|displacement|
                            displacement.same(GlobalDOFParameter::X,
                                initial_node_number))
                    {
                        drawn_analysis_result_node_data.x_direction_value =
                            Some(global_displacements.displacements_values[position]);
                        initial_node_x + global_displacements.displacements_values[position] *
                            self.state.magnitude
                    }
                    else
                    {
                        initial_node_x
                    }
                };
                let initial_node_y = drawn_node.y;
                let deformed_shape_node_y =
                {
                    if let Some(position) = global_displacements.dof_parameters_data
                        .iter()
                        .position(|displacement|
                            displacement.same(GlobalDOFParameter::Y,
                                initial_node_number))
                    {
                        drawn_analysis_result_node_data.y_direction_value =
                            Some(global_displacements.displacements_values[position]);
                        initial_node_y + global_displacements.displacements_values[position] *
                            self.state.magnitude
                    }
                    else
                    {
                        initial_node_y
                    }
                };
                let initial_node_z = drawn_node.z;
                let deformed_shape_node_z =
                {
                    if let Some(position) = global_displacements.dof_parameters_data
                        .iter()
                        .position(|displacement|
                            displacement.same(GlobalDOFParameter::Z,
                                initial_node_number))
                    {
                        drawn_analysis_result_node_data.z_direction_value =
                            Some(global_displacements.displacements_values[position]);
                        initial_node_z + global_displacements.displacements_values[position] *
                            self.state.magnitude
                    }
                    else
                    {
                        initial_node_z
                    }
                };
                if let Some(position) = global_displacements.dof_parameters_data
                    .iter()
                    .position(|displacement|
                        displacement.same(GlobalDOFParameter::ThX,
                            initial_node_number))
                {
                    drawn_analysis_result_node_data.yz_plane_value =
                        Some(global_displacements.displacements_values[position]);
                }
                if let Some(position) = global_displacements.dof_parameters_data
                    .iter()
                    .position(|displacement|
                        displacement.same(GlobalDOFParameter::ThY,
                            initial_node_number))
                {
                    drawn_analysis_result_node_data.zx_plane_value =
                        Some(global_displacements.displacements_values[position]);
                }
                if let Some(position) = global_displacements.dof_parameters_data
                    .iter()
                    .position(|displacement|
                        displacement.same(GlobalDOFParameter::ThZ,
                            initial_node_number))
                {
                    drawn_analysis_result_node_data.xy_plane_value =
                        Some(global_displacements.displacements_values[position]);
                }
                self.state.drawn_analysis_results_for_nodes.push(
                    drawn_analysis_result_node_data);
                let deformed_shape_node =
                    FEDrawnNodeData { uid, number: deformed_shape_node_number,
                        x: deformed_shape_node_x, y: deformed_shape_node_y,
                        z: deformed_shape_node_z };
                self.state.drawn_nodes_extended.push(deformed_shape_node);
            }
        }
    }


    fn extend_by_reactions(&mut self)
    {
        self.default_drawn_nodes_extended_and_analysis_results_for_nodes();
        if let Some(reactions) = self.props.reactions.as_ref()
        {
            let mut uid = self.props.postproc_init_uid_number;
            for drawn_node in self.props.drawn_nodes.iter()
            {
                uid += 1;
                let mut drawn_analysis_result_node_data = FEDrawnAnalysisResultNodeData
                    {
                        uid, bc_type: BCType::Force,
                        node_number: drawn_node.number,
                        x_direction_value: None, y_direction_value: None, z_direction_value: None,
                        xy_plane_value: None, yz_plane_value: None, zx_plane_value: None,
                    };
                if let Some(position) = reactions.dof_parameters_data
                    .iter()
                    .position(|reaction|
                        reaction.same(GlobalDOFParameter::X,
                            drawn_node.number))
                {
                    drawn_analysis_result_node_data.x_direction_value =
                        Some(reactions.reactions_values[position]);
                }
                if let Some(position) = reactions.dof_parameters_data
                    .iter()
                    .position(|reaction|
                        reaction.same(GlobalDOFParameter::Y,
                            drawn_node.number))
                {
                    drawn_analysis_result_node_data.y_direction_value =
                        Some(reactions.reactions_values[position]);
                }
                if let Some(position) = reactions.dof_parameters_data
                    .iter()
                    .position(|reaction|
                        reaction.same(GlobalDOFParameter::Z,
                            drawn_node.number))
                {
                    drawn_analysis_result_node_data.z_direction_value =
                        Some(reactions.reactions_values[position]);
                }
                if let Some(position) = reactions.dof_parameters_data
                    .iter()
                    .position(|displacement|
                        displacement.same(GlobalDOFParameter::ThX,
                            drawn_node.number))
                {
                    drawn_analysis_result_node_data.yz_plane_value =
                        Some(reactions.reactions_values[position]);
                }
                if let Some(position) = reactions.dof_parameters_data
                    .iter()
                    .position(|reaction|
                        reaction.same(GlobalDOFParameter::ThY,
                            drawn_node.number))
                {
                    drawn_analysis_result_node_data.zx_plane_value =
                        Some(reactions.reactions_values[position]);
                }
                if let Some(position) = reactions.dof_parameters_data
                    .iter()
                    .position(|reaction|
                        reaction.same(GlobalDOFParameter::ThZ,
                            drawn_node.number))
                {
                    drawn_analysis_result_node_data.xy_plane_value =
                        Some(reactions.reactions_values[position]);
                }
                self.state.drawn_analysis_results_for_nodes.push(
                    drawn_analysis_result_node_data);
            }
        }
    }


    fn extend_by_analysis_result_element_data(&mut self)
    {
        self.default_analysis_results_for_elements();
        if let Some(elements_analysis_result) =
            self.props.elements_analysis_result.as_ref()
        {
            let mut uid = self.props.postproc_init_uid_number;
            let elements_analysis_data =
                elements_analysis_result.extract_elements_analysis_data();
            for data in elements_analysis_data.iter()
            {
                uid += 1;

                let drawn_analysis_result_element_data = DrawnAnalysisResultElementData
                    {
                        uid, element_analysis_data: data.to_owned()
                    };
                self.state.drawn_analysis_results_for_elements.push(
                    drawn_analysis_result_element_data);
            }
        }
    }


    fn extract_min_max_values_for_component(&self, ear_type: &EARType,
        component: &Box<dyn EARComponentTrait>)
        -> (Option<ElementsValues>, Option<ElementsValues>)
    {
        let first_appropriate_value =
            {
                match ear_type
                {
                    EARType::Stress =>
                        {
                            if let Some(position) =
                                self.state.drawn_analysis_results_for_elements[0]
                                    .element_analysis_data
                                    .extract_stresses().stresses_components.iter()
                                    .position(|c| c.same(component))
                            {
                                Some(self.state.drawn_analysis_results_for_elements[0]
                                    .element_analysis_data
                                    .extract_stresses().stresses_values[position])
                            }
                            else
                            {
                                None
                            }
                        },
                    EARType::Strain =>
                        {
                            if let Some(position) =
                                self.state.drawn_analysis_results_for_elements[0]
                                    .element_analysis_data
                                    .extract_strains().strains_components.iter()
                                    .position(|c| c.same(component) )
                            {
                                Some(self.state.drawn_analysis_results_for_elements[0]
                                    .element_analysis_data
                                    .extract_strains().strains_values[position])
                            }
                            else
                            {
                                None
                            }
                        },
                    EARType::Force =>
                        {
                            if let Some(position) =
                                self.state.drawn_analysis_results_for_elements[0]
                                    .element_analysis_data
                                    .extract_forces().forces_components.iter()
                                    .position(|c| c.same(component) )
                            {
                                Some(self.state.drawn_analysis_results_for_elements[0]
                                    .element_analysis_data
                                    .extract_forces().forces_values[position])
                            }
                            else
                            {
                                None
                            }
                        }
                }
            };
        let mut min_value = first_appropriate_value;
        let mut max_value = first_appropriate_value;
        for data in self.state.drawn_analysis_results_for_elements.iter()
        {
            let (values, components) =
                {
                    match ear_type
                    {
                        EARType::Stress =>
                            {
                                let mut boxed_components: Vec<Box<dyn EARComponentTrait>> = Vec::new();
                                let element_stresses =
                                    data.element_analysis_data.extract_stresses();
                                for component in element_stresses.stresses_components.iter()
                                {
                                    boxed_components.push(Box::new(*component))
                                }
                                (element_stresses.stresses_values, boxed_components)
                            },
                        EARType::Strain =>
                            {
                                let mut boxed_components: Vec<Box<dyn EARComponentTrait>> = Vec::new();
                                let element_strains =
                                    data.element_analysis_data.extract_strains();
                                for component in element_strains.strains_components.iter()
                                {
                                    boxed_components.push(Box::new(*component))
                                }
                                (element_strains.strains_values, boxed_components)
                            },
                        EARType::Force =>
                            {
                                let mut boxed_components: Vec<Box<dyn EARComponentTrait>> = Vec::new();
                                let element_forces =
                                    data.element_analysis_data.extract_forces();
                                for component in element_forces.forces_components.iter()
                                {
                                    boxed_components.push(Box::new(*component))
                                }
                                (element_forces.forces_values, boxed_components)
                            }
                    }

                };

            for (c, value) in components.iter().zip(values)
            {
                if c.same(component)
                {
                    if min_value.is_some()
                    {
                        if value < min_value.unwrap()
                        {
                            min_value = Some(value);
                        }
                    }
                    if max_value.is_some()
                    {
                        if value > max_value.unwrap()
                        {
                            max_value = Some(value);
                        }
                    }
                }
            }
        }
        (min_value, max_value)
    }
}


pub enum Msg
{
    AddObjectInfo(String),
    ResetObjectInfo,
    ChangeMagnitude(ElementsValues),
    SelectPlotDisplacements,
    SelectPlotReactions,
    SelectStressComponent(StressStrainComponent),
    SelectStrainComponent(StressStrainComponent),
    SelectForceComponent(ForceComponent),
}


impl Component for Postprocessor
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let drawn_nodes_extended = if !props.drawn_nodes.is_empty()
            {
                (*props.drawn_nodes.as_ref()).clone()
            }
            else
            {
                Vec::new()
            };
        let state = State
            {
                object_info: None,
                magnitude: 1.0 as ElementsValues,
                drawn_nodes_extended,
                drawn_analysis_results_for_nodes: Vec::new(),
                drawn_analysis_results_for_elements: Vec::new(),
                is_plot_displacements_selected: false,
                is_plot_reactions_selected: false,
                stress_component_selected: None,
                strain_component_selected: None,
                force_component_selected: None,
                min_selected_value: None,
                max_selected_value: None,
            };
        Self { props, link, state }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ChangeMagnitude(value) =>
                {
                    self.state.magnitude = value;
                    if self.state.is_plot_displacements_selected
                    {
                        self.extend_by_deformed_shape_nodes();
                    }
                },
            Msg::SelectPlotDisplacements =>
                {
                    self.extend_by_deformed_shape_nodes();
                    self.state.is_plot_displacements_selected = true;
                    self.state.is_plot_reactions_selected = false;
                    self.state.stress_component_selected = None;
                    self.state.strain_component_selected = None;
                    self.state.force_component_selected = None;
                    self.state.min_selected_value = None;
                    self.state.max_selected_value = None;
                },
            Msg::SelectPlotReactions =>
                {
                    self.extend_by_reactions();
                    self.state.is_plot_displacements_selected = false;
                    self.state.is_plot_reactions_selected = true;
                    self.state.stress_component_selected = None;
                    self.state.strain_component_selected = None;
                    self.state.force_component_selected = None;
                    self.state.min_selected_value = None;
                    self.state.max_selected_value = None;
                },
            Msg::SelectStressComponent(component) =>
                {
                    self.default_drawn_nodes_extended_and_analysis_results_for_nodes();
                    self.extend_by_analysis_result_element_data();
                    self.state.is_plot_displacements_selected = false;
                    self.state.is_plot_reactions_selected = false;
                    self.state.stress_component_selected = Some(component);
                    let boxed_component: Box<dyn EARComponentTrait> = Box::new(component);
                    let (min_value, max_value) =
                        self.extract_min_max_values_for_component(&EARType::Stress,
                            &boxed_component);
                    self.state.strain_component_selected = None;
                    self.state.force_component_selected = None;
                    self.state.min_selected_value = min_value;
                    self.state.max_selected_value = max_value;
                },
            Msg::SelectStrainComponent(component) =>
                {
                    self.default_drawn_nodes_extended_and_analysis_results_for_nodes();
                    self.extend_by_analysis_result_element_data();
                    self.state.is_plot_displacements_selected = false;
                    self.state.is_plot_reactions_selected = false;
                    self.state.stress_component_selected = None;
                    self.state.strain_component_selected = Some(component);
                    let boxed_component: Box<dyn EARComponentTrait> = Box::new(component);
                    let (min_value, max_value) =
                        self.extract_min_max_values_for_component(&EARType::Strain,
                            &boxed_component);
                    self.state.force_component_selected = None;
                    self.state.min_selected_value = min_value;
                    self.state.max_selected_value = max_value;
                },
            Msg::SelectForceComponent(component) =>
                {
                    self.default_drawn_nodes_extended_and_analysis_results_for_nodes();
                    self.extend_by_analysis_result_element_data();
                    self.state.is_plot_displacements_selected = false;
                    self.state.is_plot_reactions_selected = false;
                    self.state.stress_component_selected = None;
                    self.state.strain_component_selected = None;
                    let boxed_component: Box<dyn EARComponentTrait> = Box::new(component);
                    let (min_value, max_value) =
                        self.extract_min_max_values_for_component(&EARType::Force,
                            &boxed_component);
                    self.state.force_component_selected = Some(component);
                    self.state.min_selected_value = min_value;
                    self.state.max_selected_value = max_value;
                },
            Msg::AddObjectInfo(info) => self.state.object_info = Some(info),
            Msg::ResetObjectInfo => self.state.object_info = None,
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if (&self.props.view, &self.props.postproc_init_uid_number, &self.props.canvas_width,
            &self.props.canvas_height) !=
            (&props.view, &props.postproc_init_uid_number, &props.canvas_width,
             &props.canvas_height) ||
            !Rc::ptr_eq(&self.props.global_displacements, &props.global_displacements) ||
            !Rc::ptr_eq(&self.props.reactions, &props.reactions) ||
            !Rc::ptr_eq(&self.props.elements_analysis_result,
                        &props.elements_analysis_result)
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
        type Button = RouterButton<AppRoute>;

        let handle_add_object_info =
            self.link.callback(|info: String| Msg::AddObjectInfo(info));
        let handle_reset_object_info = self.link.callback(|_| Msg::ResetObjectInfo);

        let handle_change_magnitude =
            self.link.callback(|value: ElementsValues| Msg::ChangeMagnitude(value));

        let handle_select_plot_displacements =
            self.link.callback(|_| Msg::SelectPlotDisplacements);

        let handle_select_stress_component =
            self.link.callback(|stress_component|
                Msg::SelectStressComponent(stress_component));

        let handle_select_strain_component =
            self.link.callback(|strain_component|
                Msg::SelectStrainComponent(strain_component));

        let handle_select_force_component =
            self.link.callback(|force_component|
                Msg::SelectForceComponent(force_component));
        html!
        {
            <>
                {
                    if self.props.global_displacements.as_ref().is_some() &&
                        self.props.reactions.as_ref().is_some() &&
                        self.props.elements_analysis_result.as_ref().is_some()
                    {
                        html!
                        {
                            <div class={ POSTPROCESSOR_CLASS }>
                                <div class={ POSTPROCESSOR_MENU_CLASS }>

                                    <Button route=AppRoute::Preprocessor
                                        classes={ POSTPROCESSOR_BUTTON_CLASS }
                                    >
                                      { "FEM" }
                                    </Button>
                                    <ViewMenu
                                        view=self.props.view.to_owned(),
                                        change_view=self.props.change_view.to_owned(),
                                    />
                                    <PlotDisplacementsMenu
                                        magnitude=self.state.magnitude.to_owned(),
                                        change_magnitude=handle_change_magnitude,
                                        select_plot_displacements=handle_select_plot_displacements,
                                    />

                                    <button class={ POSTPROCESSOR_BUTTON_CLASS }
                                        onclick=self.link.callback(|_| Msg::SelectPlotReactions),
                                    >
                                        { "Plot reactions" }
                                    </button>
                                    <PlotStressesMenu
                                        stress_component_selected=self.state.stress_component_selected.to_owned(),
                                        select_stress_component=handle_select_stress_component,
                                    />
                                    <PlotStrainsMenu
                                        strain_component_selected=self.state.strain_component_selected.to_owned(),
                                        select_strain_component=handle_select_strain_component,
                                    />
                                     <PlotForcesMenu
                                        force_component_selected=self.state.force_component_selected.to_owned(),
                                        select_force_component=handle_select_force_component,
                                    />
                                </div>
                                <div class={ POSTPROCESSOR_CANVAS_CLASS }>
                                    <PostprocessorCanvas
                                        view=self.props.view.to_owned(),
                                        discard_view=self.props.discard_view.to_owned(),
                                        canvas_width=self.props.canvas_width.to_owned(),
                                        canvas_height=self.props.canvas_height.to_owned(),
                                        drawn_elements=Rc::clone(&self.props.drawn_elements),
                                        drawn_nodes_extended=self.state.drawn_nodes_extended.to_owned(),
                                        drawn_analysis_results_for_nodes=self.state.drawn_analysis_results_for_nodes.to_owned(),
                                        drawn_analysis_results_for_elements=self.state.drawn_analysis_results_for_elements.to_owned(),
                                        is_plot_displacements_selected=self.state.is_plot_displacements_selected.to_owned(),
                                        is_plot_reactions_selected=self.state.is_plot_reactions_selected.to_owned(),
                                        stress_component_selected=self.state.stress_component_selected.to_owned(),
                                        strain_component_selected=self.state.strain_component_selected.to_owned(),
                                        force_component_selected=self.state.force_component_selected.to_owned(),
                                        min_selected_value=self.state.min_selected_value.to_owned(),
                                        max_selected_value=self.state.max_selected_value.to_owned(),
                                        add_object_info=handle_add_object_info.to_owned(),
                                        reset_object_info=handle_reset_object_info.to_owned(),
                                    />
                                    {
                                        if let Some(info) = &self.state.object_info
                                        {
                                            html!
                                            {
                                                <div class={ ANALYSIS_INFO_CLASS }>
                                                    <p class={ ANALYSIS_MESSAGE_CLASS }>{ &format!("Object: {}", info) }</p>
                                                </div>
                                            }
                                        }
                                        else
                                        {
                                            html!
                                            {
                                                <div class={ ANALYSIS_INFO_CLASS }>
                                                    <p class={ ANALYSIS_MESSAGE_CLASS }>{ "Object: " }</p>
                                                </div>
                                            }
                                        }
                                    }
                                </div>
                            </div>
                        }
                    }
                    else
                    {
                        html! {}
                    }
                }
            </>
        }
    }
}
