use yew::prelude::*;
use web_sys::DomTokenList;

use crate::pages::POSTPROCESSOR_BUTTON_CLASS;
use crate::fem::StressStrainComponent;


const PLOT_STRESSES_MENU_ID: &str = "plot_stresses_menu";
const PLOT_STRESSES_MENU_CLASS: &str = "plot_stresses_menu";
const PLOT_STRESSES_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "plot_stresses_menu_input_fields_container";
const PLOT_STRESSES_MENU_INPUT_FIELD_CONTAINER_CLASS: &str = "plot_stresses_input_field_container";
const PLOT_STRESSES_MENU_BUTTONS_CONTAINER_CLASS: &str = "plot_stresses_menu_buttons";
const PLOT_STRESSES_MENU_BUTTON_CLASS: &str = "plot_stresses_menu_button";
const HIDDEN: &str = "hidden";
const PLOT_STRESSES_INPUT_NAME: &str = "plot_stresses";
const PLOT_STRESS_XX_ID: &str = "plot_stress_xx";
const PLOT_STRESS_XY_ID: &str = "plot_stress_xy";
const PLOT_STRESS_XZ_ID: &str = "plot_stress_xz";
const PLOT_STRESS_YY_ID: &str = "plot_stress_yy";
const PLOT_STRESS_YZ_ID: &str = "plot_stress_yz";
const PLOT_STRESS_ZZ_ID: &str = "plot_stress_zz";
const PLOT_STRESSES_MENU_INPUT_FIELD_CLASS: &str = "plot_stresses_menu_input_field";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub stress_component_selected: Option<StressStrainComponent>,
    pub select_stress_component: Callback<StressStrainComponent>,
}


struct State
{
    stress_component_selected: StressStrainComponent,
}


pub struct PlotStressesMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHidePlotStressesMenu,
    SelectStressComponent(ChangeData),
    ApplyStressComponent,
}


impl PlotStressesMenu
{
    fn show_hide_plot_stresses_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(PLOT_STRESSES_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(PLOT_STRESSES_MENU_CLASS);
        }
        else
        {
            element.set_class_name(&(PLOT_STRESSES_MENU_CLASS.to_owned() + " " + HIDDEN));
        }
    }
}


impl Component for PlotStressesMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link, state: State { stress_component_selected: StressStrainComponent::XX } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHidePlotStressesMenu => self.show_hide_plot_stresses_menu(),
            Msg::SelectStressComponent(data) =>
                {
                    match data
                    {
                        ChangeData::Value(stress_select) =>
                            {
                                if stress_select == StressStrainComponent::XX.as_str()
                                {
                                    self.state.stress_component_selected = StressStrainComponent::XX;
                                }
                                if stress_select == StressStrainComponent::XY.as_str()
                                {
                                    self.state.stress_component_selected = StressStrainComponent::XY;
                                }
                                if stress_select == StressStrainComponent::XZ.as_str()
                                {
                                    self.state.stress_component_selected = StressStrainComponent::XZ;
                                }
                                if stress_select == StressStrainComponent::YY.as_str()
                                {
                                    self.state.stress_component_selected = StressStrainComponent::YY;
                                }
                                if stress_select == StressStrainComponent::YZ.as_str()
                                {
                                    self.state.stress_component_selected = StressStrainComponent::YZ;
                                }
                                if stress_select == StressStrainComponent::ZZ.as_str()
                                {
                                    self.state.stress_component_selected = StressStrainComponent::ZZ;
                                }
                                return false;
                            }
                        _ => (),
                    }
                },
            Msg::ApplyStressComponent =>
                {
                    self.props.select_stress_component.emit(self.state.stress_component_selected.to_owned());
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
                    onclick=self.link.callback(|_| Msg::ShowHidePlotStressesMenu),
                >
                    { "Plot stresses" }
                </button>
                <div id = { PLOT_STRESSES_MENU_ID } class={ PLOT_STRESSES_MENU_CLASS.to_owned() + " " + HIDDEN }>
                    <div class={ PLOT_STRESSES_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <div class={ PLOT_STRESSES_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRESSES_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStressComponent(data)),
                                type="radio", id={ PLOT_STRESS_XX_ID },
                                name={ PLOT_STRESSES_INPUT_NAME },
                                value={ StressStrainComponent::XX.as_str() },
                                checked=
                                {
                                    if let Some(stress) = &self.props.stress_component_selected
                                    {
                                        if *stress == StressStrainComponent::XX
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
                            <label for={ PLOT_STRESS_XX_ID }>
                                { StressStrainComponent::XX.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRESSES_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRESSES_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStressComponent(data)),
                                type="radio", id={ PLOT_STRESS_XY_ID },
                                name={ PLOT_STRESSES_INPUT_NAME },
                                value={ StressStrainComponent::XY.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(stress) = &self.props.stress_component_selected
                                    {
                                        if *stress == StressStrainComponent::XY
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
                            <label for={ PLOT_STRESS_XY_ID }>
                                { StressStrainComponent::XY.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRESSES_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRESSES_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStressComponent(data)),
                                type="radio", id={ PLOT_STRESS_XZ_ID },
                                name={ PLOT_STRESSES_INPUT_NAME },
                                value={ StressStrainComponent::XZ.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(stress) = &self.props.stress_component_selected
                                    {
                                        if *stress == StressStrainComponent::XZ
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
                            <label for={ PLOT_STRESS_XZ_ID }>
                                { StressStrainComponent::XZ.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRESSES_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRESSES_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStressComponent(data)),
                                type="radio", id={ PLOT_STRESS_YY_ID },
                                name={ PLOT_STRESSES_INPUT_NAME },
                                value={ StressStrainComponent::YY.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(stress) = &self.props.stress_component_selected
                                    {
                                        if *stress == StressStrainComponent::YY
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
                            <label for={ PLOT_STRESS_YY_ID }>
                                { StressStrainComponent::YY.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRESSES_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRESSES_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStressComponent(data)),
                                type="radio", id={ PLOT_STRESS_YZ_ID },
                                name={ PLOT_STRESSES_INPUT_NAME },
                                value={ StressStrainComponent::YZ.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(stress) = &self.props.stress_component_selected
                                    {
                                        if *stress == StressStrainComponent::YZ
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
                            <label for={ PLOT_STRESS_YZ_ID }>
                                { StressStrainComponent::YZ.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRESSES_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRESSES_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStressComponent(data)),
                                type="radio", id={ PLOT_STRESS_ZZ_ID },
                                name={ PLOT_STRESSES_INPUT_NAME },
                                value={ StressStrainComponent::ZZ.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(stress) = &self.props.stress_component_selected
                                    {
                                        if *stress == StressStrainComponent::ZZ
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
                            <label for={ PLOT_STRESS_ZZ_ID }>
                                { StressStrainComponent::ZZ.as_str() }
                            </label>
                        </div>
                    </div>
                    <div class={ PLOT_STRESSES_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ PLOT_STRESSES_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyStressComponent),
                        >
                            { "Apply" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
