use yew::prelude::*;

use crate::auxiliary::{View, FEDrawnNodeData, FEDrawnElementData, DrawnBCData};
use std::rc::Rc;
use std::cell::RefCell;
use crate::fem::FENode;
use crate::{ElementsNumbers, ElementsValues};

use crate::components::
    {
        NodeMenu, PreprocessorCanvas, ElementMenu,
        ViewMenu, DisplacementMenu, ForceMenu
    };
use yew_router::prelude::{RouterButton};
use crate::route::AppRoute;


const PREPROCESSOR_CLASS: &str = "preprocessor";
const PREPROCESSOR_MENU_CLASS: &str = "preprocessor_menu";
pub const PREPROCESSOR_BUTTON_CLASS: &str = "preprocessor_button";
const PREPROCESSOR_CANVAS_CLASS: &str = "preprocessor_canvas";
const ANALYSIS_ERROR_CLASS: &str = "analysis_error";
const ANALYSIS_ERROR_MESSAGE_CLASS: &str = "analysis_error_message";
const ANALYSIS_ERROR_BUTTON_CLASS: &str = "analysis_error_button";


#[derive(Properties, Clone)]
pub struct Props
{
    pub view: Option<View>,
    pub change_view: Callback<View>,
    pub discard_view: Callback<()>,
    pub is_preprocessor_active: bool,

    pub nodes: Rc<Vec<Rc<RefCell<FENode<ElementsNumbers, ElementsValues>>>>>,
    pub add_node: Callback<FEDrawnNodeData>,
    pub update_node: Callback<FEDrawnNodeData>,
    pub delete_node: Callback<ElementsNumbers>,

    pub drawn_elements: Rc<Vec<FEDrawnElementData>>,
    pub add_element: Callback<FEDrawnElementData>,
    pub update_element: Callback<FEDrawnElementData>,
    pub delete_element: Callback<ElementsNumbers>,

    pub drawn_bcs: Rc<Vec<DrawnBCData>>,
    pub add_bc: Callback<DrawnBCData>,
    pub update_bc: Callback<DrawnBCData>,
    pub delete_bc: Callback<DrawnBCData>,
    pub add_analysis_error_message: Callback<String>,

    pub canvas_width: u32,
    pub canvas_height: u32,
    pub analysis_error_message: Option<String>,

    pub reset_analysis_error_message: Callback<()>,
}


pub struct Preprocessor
{
    link: ComponentLink<Self>,
    props: Props,
}


impl Preprocessor
{
    fn check_preprocessor_data(&self) -> bool
    {
        if self.props.nodes.is_empty()
        {
            return false;
        }
        if self.props.drawn_elements.is_empty()
        {
            return false;
        }
        if self.props.drawn_bcs.is_empty()
        {
            return false;
        }
        true
    }
}


pub enum Msg
{
    ResetAnalysisErrorMessage,
}


impl Component for Preprocessor
{
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ResetAnalysisErrorMessage =>
                self.props.reset_analysis_error_message.emit(()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if (&self.props.view, &self.props.is_preprocessor_active,
            &self.props.canvas_height, &self.props.canvas_width,
            &self.props.analysis_error_message) !=
            (&props.view, &props.is_preprocessor_active,
             &props.canvas_height, &props.canvas_width, &props.analysis_error_message) ||
            !Rc::ptr_eq(&self.props.nodes, &props.nodes) ||
            !Rc::ptr_eq(&self.props.drawn_elements, &props.drawn_elements) ||
            !Rc::ptr_eq(&self.props.drawn_bcs, &props.drawn_bcs)
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
                <div class={ PREPROCESSOR_CLASS }>
                    <div class={ PREPROCESSOR_MENU_CLASS }>
                        <ViewMenu
                            view=self.props.view.to_owned(),
                            change_view=self.props.change_view.to_owned(),
                        />
                        <NodeMenu
                            is_preprocessor_active=self.props.is_preprocessor_active.to_owned(),
                            nodes=Rc::clone(&self.props.nodes),
                            add_node=self.props.add_node.to_owned(),
                            update_node=self.props.update_node.to_owned(),
                            delete_node=self.props.delete_node.to_owned(),
                        />
                        <ElementMenu
                            is_preprocessor_active=self.props.is_preprocessor_active.to_owned(),
                            drawn_elements=Rc::clone(&self.props.drawn_elements),
                            add_element=self.props.add_element.to_owned(),
                            update_element=self.props.update_element.to_owned(),
                            delete_element=self.props.delete_element.to_owned(),
                        />
                        <DisplacementMenu
                            is_preprocessor_active=self.props.is_preprocessor_active.to_owned(),
                            drawn_elements=Rc::clone(&self.props.drawn_elements),
                            drawn_bcs=Rc::clone(&self.props.drawn_bcs),
                            add_bc=self.props.add_bc.to_owned(),
                            update_bc=self.props.update_bc.to_owned(),
                            delete_bc=self.props.delete_bc.to_owned(),
                            add_analysis_error_message=self.props.add_analysis_error_message.to_owned(),
                        />
                        <ForceMenu
                            is_preprocessor_active=self.props.is_preprocessor_active.to_owned(),
                            drawn_elements=Rc::clone(&self.props.drawn_elements),
                            drawn_bcs=Rc::clone(&self.props.drawn_bcs),
                            add_bc=self.props.add_bc.to_owned(),
                            update_bc=self.props.update_bc.to_owned(),
                            delete_bc=self.props.delete_bc.to_owned(),
                            add_analysis_error_message=self.props.add_analysis_error_message.to_owned(),
                        />
                        <Button route=AppRoute::Postprocessor classes={PREPROCESSOR_BUTTON_CLASS }>
                          { "Submit" }
                        </Button>


                        // <button class={ PREPROCESSOR_BUTTON_CLASS }
                        //     // onclick=self.link.callback(|_| Msg::Submit),
                        //     disabled=
                        //         {
                        //             if self.props.is_preprocessor_active && self.check_preprocessor_data()
                        //             {
                        //                 false
                        //             }
                        //             else
                        //             {
                        //                 true
                        //             }
                        //         },
                        // >
                        //     { "Submit" }
                        // </button>
                    </div>
                    <div class={ PREPROCESSOR_CANVAS_CLASS }>
                        <PreprocessorCanvas
                            view=self.props.view.to_owned(),
                            discard_view=self.props.discard_view.to_owned(),
                            canvas_width=self.props.canvas_width.to_owned(),
                            canvas_height=self.props.canvas_height.to_owned(),
                            nodes=Rc::clone(&self.props.nodes),
                            drawn_elements=Rc::clone(&self.props.drawn_elements),
                            add_analysis_error_message=self.props.add_analysis_error_message.to_owned(),
                            drawn_bcs=Rc::clone(&self.props.drawn_bcs),
                        />
                    </div>
                </div>
                {
                    if let Some(error_message) = &self.props.analysis_error_message
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
            </>
        }
    }
}