use yew::prelude::*;
use yew_router::prelude::RouterButton;
use std::rc::Rc;
use std::cell::RefCell;

use crate::route::AppRoute;
use crate::fem::{GlobalAnalysisResult, FENode};
use crate::{ElementsNumbers, ElementsValues};

use crate::components::{ViewMenu, PostprocessorCanvas, PlotDisplacementsMenu};
use crate::auxiliary::{View, FEDrawnNodeData};


const POSTPROCESSOR_CLASS: &str = "postprocessor";
const POSTPROCESSOR_MENU_CLASS: &str = "postprocessor_menu";
pub const POSTPROCESSOR_BUTTON_CLASS: &str = "postprocessor_button";
const POSTPROCESSOR_CANVAS_CLASS: &str = "postprocessor_canvas";


#[derive(Properties, Clone)]
pub struct Props
{
    pub global_analysis_result: Rc<Option<GlobalAnalysisResult<ElementsNumbers, ElementsValues>>>,
    pub view: Option<View>,
    pub change_view: Callback<View>,
    pub discard_view: Callback<()>,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub nodes: Rc<Vec<FEDrawnNodeData>>,
}


pub struct State
{
    pub magnitude: ElementsValues,
    pub is_plot_displacements_selected: bool,
}


pub struct Postprocessor
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ChangeMagnitude(ElementsValues),
    SelectPlotDisplacements,
}


impl Component for Postprocessor
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let state = State
            {
                magnitude: 1.0 as ElementsValues,
                is_plot_displacements_selected: false,
            };
        Self { props, link, state }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ChangeMagnitude(value) => self.state.magnitude = value,
            Msg::SelectPlotDisplacements => self.state.is_plot_displacements_selected = true,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {

        if &self.props.view != &props.view ||
            !Rc::ptr_eq(&self.props.global_analysis_result,
                &props.global_analysis_result)
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

        let handle_change_magnitude =
            self.link.callback(|value: ElementsValues| Msg::ChangeMagnitude(value));

        let handle_select_plot_displacements =
            self.link.callback(|_| Msg::SelectPlotDisplacements);
        html!
        {
            <>
                {
                    if self.props.global_analysis_result.as_ref().is_some()
                    {
                        html!
                        {
                            <div class={ POSTPROCESSOR_CLASS }>
                                <div class={ POSTPROCESSOR_MENU_CLASS }>

                                    <Button route=AppRoute::Preprocessor
                                        classes={POSTPROCESSOR_BUTTON_CLASS }
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

                                    // <ResultViewMenu
                                    //     result_view=self.state.result_view.to_owned(),
                                    //     change_result_view=handle_change_result_view,
                                    // />
                                </div>
                                <div class={ POSTPROCESSOR_CANVAS_CLASS }>
                                    <PostprocessorCanvas
                                        view=self.props.view.to_owned(),
                                        discard_view=self.props.discard_view.to_owned(),
                                        canvas_width=self.props.canvas_width.to_owned(),
                                        canvas_height=self.props.canvas_height.to_owned(),
                                        magnitude=self.state.magnitude.to_owned(),
                                        nodes=Rc::clone(&self.props.nodes),
                                        global_analysis_result=Rc::clone(&self.props.global_analysis_result),
                                        is_plot_displacements_selected=self.state.is_plot_displacements_selected.to_owned(),
                                        // drawn_elements=Rc::clone(&self.props.drawn_elements),
                                        // add_analysis_message=self.props.add_analysis_message.to_owned(),
                                        // drawn_bcs=Rc::clone(&self.props.drawn_bcs),
                                    />
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