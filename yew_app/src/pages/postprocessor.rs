use yew::prelude::*;
use yew_router::prelude::RouterButton;
use std::rc::Rc;

use crate::route::AppRoute;
use crate::fem::GlobalAnalysisResult;
use crate::{ElementsNumbers, ElementsValues};

use crate::components::ViewMenu;
use crate::auxiliary::View;


const POSTPROCESSOR_CLASS: &str = "postprocessor";
const POSTPROCESSOR_MENU_CLASS: &str = "postprocessor_menu";
pub const POSTPROCESSOR_BUTTON_CLASS: &str = "postprocessor_button";


#[derive(Properties, Clone)]
pub struct Props
{
    pub global_analysis_result: Rc<Option<GlobalAnalysisResult<ElementsNumbers, ElementsValues>>>,
    pub view: Option<View>,
    pub change_view: Callback<View>,
}


pub struct Postprocessor
{
    link: ComponentLink<Self>,
    props: Props,
}


impl Component for Postprocessor
{
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender
    {
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

                                    // <ResultViewMenu
                                    //     result_view=self.state.result_view.to_owned(),
                                    //     change_result_view=handle_change_result_view,
                                    // />
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