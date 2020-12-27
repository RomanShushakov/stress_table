use yew::prelude::*;
use web_sys::DomTokenList;

use crate::{ResultView, POSTPROCESSOR_BUTTON_CLASS};


const RESULT_VIEW_MENU_ID: &str = "result_view_menu";
const RESULT_VIEW_MENU_CLASS: &str = "result_view_menu";
const RESULT_VIEW_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "result_view_menu_input_fields_container";
const RESULT_VIEW_MENU_INPUT_FIELD_CONTAINER_CLASS: &str = "result_view_input_field_container";
const RESULT_VIEW_MENU_BUTTONS_CONTAINER_CLASS: &str = "result_view_menu_buttons";
const RESULT_VIEW_MENU_BUTTON_CLASS: &str = "result_view_menu_button";
const HIDDEN: &str = "hidden";
const RESULT_VIEW_INPUT_NAME: &str = "result_view";
const PLOT_STRESSES: &str = "plot_stresses";
const PLOT_REACTIONS: &str = "plot_reactions";
const PRINT_ALL_RESULTS: &str = "print_all_results";
const RESULT_VIEW_MENU_INPUT_FIELD_CLASS: &str = "result_view_menu_input_field";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub result_view: Option<ResultView>,
    pub change_result_view: Callback<ResultView>,
}


struct State
{
    selected_result_view: ResultView,
}


pub struct ResultViewMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideResultViewMenu,
    SelectResultView(ChangeData),
    ApplyResultView,
}


impl ResultViewMenu
{
    fn show_hide_result_view_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(RESULT_VIEW_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(RESULT_VIEW_MENU_CLASS);
        }
        else
        {
            element.set_class_name(&(RESULT_VIEW_MENU_CLASS.to_owned() + " " + HIDDEN));
        }
    }
}


impl Component for ResultViewMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link, state: State { selected_result_view: ResultView::PlotStresses } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideResultViewMenu => self.show_hide_result_view_menu(),
            Msg::SelectResultView(data) =>
                {
                    match data
                    {
                        ChangeData::Value(result_view_select) =>
                            {
                                if result_view_select == ResultView::PlotStresses.as_str()
                                {
                                    self.state.selected_result_view = ResultView::PlotStresses;
                                }
                                if result_view_select == ResultView::PlotReactions.as_str()
                                {
                                    self.state.selected_result_view = ResultView::PlotReactions;
                                }
                                if result_view_select == ResultView::PrintAllResults.as_str()
                                {
                                    self.state.selected_result_view = ResultView::PrintAllResults;
                                }
                                return false;
                            }
                        _ => (),
                    }
                },
            Msg::ApplyResultView =>
                {
                    self.props.change_result_view.emit(self.state.selected_result_view.to_owned());
                },
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
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
        html!
        {
            <>
                <button
                    class={ POSTPROCESSOR_BUTTON_CLASS },
                    // disabled=true,
                    onclick=self.link.callback(|_| Msg::ShowHideResultViewMenu),
                >
                    { "Result" }
                </button>
                <div id = { RESULT_VIEW_MENU_ID } class={ RESULT_VIEW_MENU_CLASS.to_owned() + " " + HIDDEN }>
                    <div class={ RESULT_VIEW_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <div class={ RESULT_VIEW_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ RESULT_VIEW_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectResultView(data)),
                                type="radio", id={ PLOT_STRESSES },
                                name={ RESULT_VIEW_INPUT_NAME },
                                value={ ResultView::PlotStresses.as_str() },
                                checked=
                                    {
                                        if let Some(result_view) = &self.props.result_view
                                        {
                                            match result_view
                                            {
                                                ResultView::PlotStresses => true,
                                                _ => false,
                                            }
                                        }
                                        else
                                        {
                                            false
                                        }
                                    },
                            />
                            <label for={ PLOT_STRESSES }>
                                { ResultView::PlotStresses.as_str() }
                            </label>
                        </div>
                        <div class={ RESULT_VIEW_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ RESULT_VIEW_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectResultView(data)),
                                type="radio", id={ PLOT_REACTIONS },
                                name={ RESULT_VIEW_INPUT_NAME },
                                value={ ResultView::PlotReactions.as_str() },
                                checked=
                                    {
                                        if let Some(result_view) = &self.props.result_view
                                        {
                                            match result_view
                                            {
                                                ResultView::PlotReactions => true,
                                                _ => false,
                                            }
                                        }
                                        else
                                        {
                                            false
                                        }
                                    },
                            />
                            <label for={ PLOT_REACTIONS }>
                                { ResultView::PlotReactions.as_str() }
                            </label>
                        </div>
                        <div class={ RESULT_VIEW_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ RESULT_VIEW_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectResultView(data)),
                                type="radio", id={ PRINT_ALL_RESULTS },
                                name={ RESULT_VIEW_INPUT_NAME },
                                value={ ResultView::PrintAllResults.as_str() },
                                checked=
                                    {
                                        if let Some(result_view) = &self.props.result_view
                                        {
                                            match result_view
                                            {
                                                ResultView::PrintAllResults => true,
                                                _ => false,
                                            }
                                        }
                                        else
                                        {
                                            false
                                        }
                                    },
                            />
                            <label for={ PRINT_ALL_RESULTS }>
                                { ResultView::PrintAllResults.as_str() }
                            </label>
                        </div>
                    </div>
                    <div class={ RESULT_VIEW_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ RESULT_VIEW_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyResultView),
                        >
                            { "Apply" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
