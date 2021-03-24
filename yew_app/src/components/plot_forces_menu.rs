use yew::prelude::*;
use web_sys::DomTokenList;

use crate::pages::POSTPROCESSOR_BUTTON_CLASS;
use crate::fem::ForceComponent;


const PLOT_FORCES_MENU_ID: &str = "plot_forces_menu";
const PLOT_FORCES_MENU_CLASS: &str = "plot_forces_menu";
const PLOT_FORCES_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "plot_forces_menu_input_fields_container";
const PLOT_FORCES_MENU_INPUT_FIELD_CONTAINER_CLASS: &str = "plot_forces_input_field_container";
const PLOT_FORCES_MENU_BUTTONS_CONTAINER_CLASS: &str = "plot_forces_menu_buttons";
const PLOT_FORCES_MENU_BUTTON_CLASS: &str = "plot_forces_menu_button";
const HIDDEN: &str = "hidden";
const PLOT_FORCES_INPUT_NAME: &str = "plot_forces";
const PLOT_FORCE_AXIAL_ID: &str = "plot_force_axial";
const PLOT_FORCES_MENU_INPUT_FIELD_CLASS: &str = "plot_forces_menu_input_field";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub force_component_selected: Option<ForceComponent>,
    pub select_force_component: Callback<ForceComponent>,
}


struct State
{
    force_component_selected: ForceComponent,
}


pub struct PlotForcesMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHidePlotForcesMenu,
    SelectForceComponent(ChangeData),
    ApplyForceComponent,
}


impl PlotForcesMenu
{
    fn show_hide_plot_forces_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(PLOT_FORCES_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(PLOT_FORCES_MENU_CLASS);
        }
        else
        {
            element.set_class_name(&(PLOT_FORCES_MENU_CLASS.to_owned() + " " + HIDDEN));
        }
    }
}


impl Component for PlotForcesMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link, state: State { force_component_selected: ForceComponent::Axial } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHidePlotForcesMenu => self.show_hide_plot_forces_menu(),
            Msg::SelectForceComponent(data) =>
                {
                    match data
                    {
                        ChangeData::Value(stress_select) =>
                            {
                                if stress_select == ForceComponent::Axial.as_str()
                                {
                                    self.state.force_component_selected = ForceComponent::Axial;
                                }
                                return false;
                            }
                        _ => (),
                    }
                },
            Msg::ApplyForceComponent =>
                {
                    self.props.select_force_component.emit(self.state.force_component_selected.to_owned());
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
                    onclick=self.link.callback(|_| Msg::ShowHidePlotForcesMenu),
                >
                    { "Plot forces" }
                </button>
                <div id = { PLOT_FORCES_MENU_ID } class={ PLOT_FORCES_MENU_CLASS.to_owned() + " " + HIDDEN }>
                    <div class={ PLOT_FORCES_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <div class={ PLOT_FORCES_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_FORCES_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectForceComponent(data)),
                                type="radio", id={ PLOT_FORCE_AXIAL_ID },
                                name={ PLOT_FORCES_INPUT_NAME },
                                value={ ForceComponent::Axial.as_str() },
                                checked=
                                {
                                    if let Some(force) = &self.props.force_component_selected
                                    {
                                        if *force == ForceComponent::Axial
                                        {
                                            true
                                        }
                                        else
                                        {
                                            false
                                        }
                                    }
                                    else
                                    {
                                        false
                                    }
                                },
                            />
                            <label for={ PLOT_FORCE_AXIAL_ID }>
                                { ForceComponent::Axial.as_str() }
                            </label>
                        </div>
                    </div>
                    <div class={ PLOT_FORCES_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ PLOT_FORCES_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyForceComponent),
                        >
                            { "Apply" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
