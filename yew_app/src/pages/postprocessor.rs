use yew::prelude::*;
use yew_router::prelude::RouterButton;
use std::rc::Rc;
use std::cell::RefCell;

use crate::route::AppRoute;
use crate::fem::{GlobalAnalysisResult, FENode, Displacements};
use crate::fem::{StressStrainComponent};
use crate::{ElementsNumbers, ElementsValues, UIDNumbers};

use crate::components::{ViewMenu, PostprocessorCanvas, PlotDisplacementsMenu, PlotStressesMenu};
use crate::auxiliary::{View, FEDrawnNodeData, FEDrawnElementData};
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
    pub is_plot_displacements_selected: bool,
    pub is_plot_reactions_selected: bool,
    pub stress_component_selected: Option<StressStrainComponent>,
}


pub struct Postprocessor
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    AddObjectInfo(String),
    ResetObjectInfo,
    ChangeMagnitude(ElementsValues),
    SelectPlotDisplacements,
    SelectPlotReactions,
    SelectStressComponent(StressStrainComponent),
}


impl Component for Postprocessor
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let state = State
            {
                object_info: None,
                magnitude: 1.0 as ElementsValues,
                is_plot_displacements_selected: false,
                is_plot_reactions_selected: false,
                stress_component_selected: None,
            };
        Self { props, link, state }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ChangeMagnitude(value) => self.state.magnitude = value,
            Msg::SelectPlotDisplacements =>
                {
                    self.state.is_plot_displacements_selected = true;
                    self.state.is_plot_reactions_selected = false;
                    self.state.stress_component_selected = None;
                },
            Msg::SelectPlotReactions =>
                {
                    self.state.is_plot_displacements_selected = false;
                    self.state.is_plot_reactions_selected = true;
                    self.state.stress_component_selected = None;
                },
            Msg::SelectStressComponent(component) =>
                {
                    self.state.is_plot_displacements_selected = false;
                    self.state.is_plot_reactions_selected = false;
                    self.state.stress_component_selected = Some(component);
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
            (&props.view, &props.postproc_init_uid_number, &props.canvas_width, &props.canvas_height) ||
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
        html!
        {
            <>
                {
                    if self.props.global_displacements.as_ref().is_some() &&
                        self.props.reactions.as_ref().is_some()
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
                                        disabled=
                                            {
                                                if self.props.reactions.as_ref().is_some()
                                                {
                                                    false
                                                }
                                                else
                                                {
                                                    true
                                                }
                                            },
                                    >
                                        { "Plot reactions" }
                                    </button>
                                    <PlotStressesMenu
                                        stress_component_selected=self.state.stress_component_selected.to_owned(),
                                        select_stress_component=handle_select_stress_component,
                                    />
                                </div>
                                <div class={ POSTPROCESSOR_CANVAS_CLASS }>
                                    <PostprocessorCanvas
                                        view=self.props.view.to_owned(),
                                        discard_view=self.props.discard_view.to_owned(),
                                        canvas_width=self.props.canvas_width.to_owned(),
                                        canvas_height=self.props.canvas_height.to_owned(),
                                        magnitude=self.state.magnitude.to_owned(),
                                        drawn_nodes=Rc::clone(&self.props.drawn_nodes),
                                        drawn_elements=Rc::clone(&self.props.drawn_elements),
                                        global_displacements=Rc::clone(&self.props.global_displacements),
                                        is_plot_displacements_selected=self.state.is_plot_displacements_selected.to_owned(),
                                        reactions=Rc::clone(&self.props.reactions),
                                        is_plot_reactions_selected=self.state.is_plot_reactions_selected.to_owned(),
                                        stress_component_selected=self.state.stress_component_selected.to_owned(),
                                        elements_analysis_result=Rc::clone(&self.props.elements_analysis_result),
                                        add_object_info=handle_add_object_info.to_owned(),
                                        reset_object_info=handle_reset_object_info.to_owned(),
                                        postproc_init_uid_number=self.props.postproc_init_uid_number.to_owned(),
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
                                // {
                                //     if let Some(result_view) = &self.state.result_view
                                //     {
                                //         match result_view
                                //         {
                                //             ResultView::PrintAllResults =>
                                //                 {
                                //                     html!
                                //                     {
                                //                         <AllResultsTable
                                //                             nodes=self.state.nodes.to_owned(),
                                //                             aux_elements=self.state.aux_elements.to_owned(),
                                //                             aux_displacements=self.state.aux_displacements.to_owned(),
                                //                             analysis_result=analysis_result.to_owned(),
                                //                             canvas_width=self.state.canvas_width,
                                //                         />
                                //                     }
                                //                 },
                                //             _ =>
                                //                 {
                                //                     html!
                                //                     {
                                //                         <div class={ POSTPROCESSOR_CANVAS_CLASS }>
                                //                             <PostprocessorCanvas
                                //                                 view=self.state.view.to_owned(),
                                //                                 canvas_width=self.state.canvas_width,
                                //                                 canvas_height=self.state.canvas_height,
                                //                                 nodes=self.state.nodes.to_owned(),
                                //                                 aux_elements=self.state.aux_elements.to_owned(),
                                //                                 analysis_result=analysis_result,
                                //                                 result_view=result_view,
                                //                             />
                                //                         </div>
                                //                     }
                                //                 }
                                //         }
                                //     }
                                //     else
                                //     {
                                //         html! {  }
                                //     }
                                // }
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