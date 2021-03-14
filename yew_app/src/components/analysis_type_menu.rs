use yew::prelude::*;
use web_sys::DomTokenList;

use crate::pages::PREPROCESSOR_BUTTON_CLASS;
use crate::{AnalysisType};


const ANALYSIS_TYPE_MENU_ID: &str = "analysis_type_menu";
const ANALYSIS_TYPE_MENU_CLASS: &str = "analysis_type_menu";
const ANALYSIS_TYPE_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "analysis_type_menu_input_fields_container";
const ANALYSIS_TYPE_MENU_INPUT_FIELD_CONTAINER_CLASS: &str = "analysis_type_menu_input_field_container";
const ANALYSIS_TYPE_MENU_INPUT_FIELD_CLASS: &str = "analysis_type_menu_input_field";
const ANALYSIS_TYPE_MENU_BUTTONS_CONTAINER_CLASS: &str = "analysis_type_menu_buttons";
const ANALYSIS_TYPE_MENU_BUTTON_CLASS: &str = "analysis_type_menu_button";
const HIDDEN: &str = "hidden";
const ANALYSIS_TYPE_INPUT_NAME: &str = "analysis_type";
const TWO_DIMENSIONAL_ANALYSIS_TYPE_ID: &str = "two_dimensional_analysis";
const THREE_DIMENSIONAL_ANALYSIS_TYPE_ID: &str = "three_dimensional_analysis";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub analysis_type: Option<AnalysisType>,
    pub add_analysis_type: Callback<AnalysisType>,
}


struct State
{
    selected_analysis_type: AnalysisType,
}


pub struct AnalysisTypeMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideAnalysisTypeMenu,
    SelectAnalysisType(ChangeData),
    ApplyAnalysisType,
}


impl AnalysisTypeMenu
{
    fn show_hide_analysis_type_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(ANALYSIS_TYPE_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(ANALYSIS_TYPE_MENU_CLASS);
        }
        else
        {
            element.set_class_name(&(ANALYSIS_TYPE_MENU_CLASS.to_owned() + " " + HIDDEN));
        }
    }


    fn hide_analysis_type_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(ANALYSIS_TYPE_MENU_ID).unwrap();
        element.set_class_name(&(ANALYSIS_TYPE_MENU_CLASS.to_owned() + " " + HIDDEN));
    }
}


impl Component for AnalysisTypeMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link, state: State { selected_analysis_type: AnalysisType::TwoDimensional } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideAnalysisTypeMenu => self.show_hide_analysis_type_menu(),
            Msg::SelectAnalysisType(data) =>
                {
                    match data
                    {
                        ChangeData::Value(analysis_type_select) =>
                            {
                                if analysis_type_select == AnalysisType::TwoDimensional.as_str()
                                {
                                    self.state.selected_analysis_type = AnalysisType::TwoDimensional;
                                }
                                if analysis_type_select == AnalysisType::ThreeDimensional.as_str()
                                {
                                    self.state.selected_analysis_type = AnalysisType::ThreeDimensional;
                                }
                                return false;
                            },
                        _ => (),
                    }
                },
            Msg::ApplyAnalysisType =>
                {
                    self.props.add_analysis_type.emit(self.state.selected_analysis_type.to_owned());
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
                    class={ PREPROCESSOR_BUTTON_CLASS },
                    // disabled=
                    //     {
                    //         if self.props.analysis_type.is_some()
                    //         {
                    //             self.hide_analysis_type_menu();
                    //             true
                    //         }
                    //         else
                    //         {
                    //             false
                    //         }
                    //      },
                    onclick=self.link.callback(|_| Msg::ShowHideAnalysisTypeMenu),
                >
                    { "Analysis type" }
                </button>
                <div
                    id={ ANALYSIS_TYPE_MENU_ID },
                    class={ ANALYSIS_TYPE_MENU_CLASS.to_owned() + " " + HIDDEN }
                >
                    <div class={ ANALYSIS_TYPE_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <div class={ ANALYSIS_TYPE_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ ANALYSIS_TYPE_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectAnalysisType(data)),
                                type="radio",
                                id={ TWO_DIMENSIONAL_ANALYSIS_TYPE_ID },
                                name={ ANALYSIS_TYPE_INPUT_NAME },
                                value={ AnalysisType::TwoDimensional.as_str() },
                                // checked=true
                            />
                            <label for={ TWO_DIMENSIONAL_ANALYSIS_TYPE_ID }>
                                { AnalysisType::TwoDimensional.as_str() }
                            </label>
                        </div>
                        <div class={ ANALYSIS_TYPE_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ ANALYSIS_TYPE_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectAnalysisType(data)),
                                type="radio",
                                id={ THREE_DIMENSIONAL_ANALYSIS_TYPE_ID },
                                name={ ANALYSIS_TYPE_INPUT_NAME },
                                value={ AnalysisType::ThreeDimensional.as_str() },
                                // disabled=true,
                            />
                            <label for={ THREE_DIMENSIONAL_ANALYSIS_TYPE_ID }>
                                { AnalysisType::ThreeDimensional.as_str() }
                            </label>
                        </div>
                    </div>
                    <div class={ ANALYSIS_TYPE_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ ANALYSIS_TYPE_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyAnalysisType)
                        >
                            { "Apply" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
