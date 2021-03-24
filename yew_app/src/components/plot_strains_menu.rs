use yew::prelude::*;
use web_sys::DomTokenList;

use crate::pages::POSTPROCESSOR_BUTTON_CLASS;
use crate::fem::StressStrainComponent;


const PLOT_STRAINS_MENU_ID: &str = "plot_strains_menu";
const PLOT_STRAINS_MENU_CLASS: &str = "plot_strains_menu";
const PLOT_STRAINS_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "plot_strains_menu_input_fields_container";
const PLOT_STRAINS_MENU_INPUT_FIELD_CONTAINER_CLASS: &str = "plot_strains_input_field_container";
const PLOT_STRAINS_MENU_BUTTONS_CONTAINER_CLASS: &str = "plot_strains_menu_buttons";
const PLOT_STRAINS_MENU_BUTTON_CLASS: &str = "plot_strains_menu_button";
const HIDDEN: &str = "hidden";
const PLOT_STRAINS_INPUT_NAME: &str = "plot_strains";
const PLOT_STRAIN_XX_ID: &str = "plot_strain_xx";
const PLOT_STRAIN_XY_ID: &str = "plot_strain_xy";
const PLOT_STRAIN_XZ_ID: &str = "plot_strain_xz";
const PLOT_STRAIN_YY_ID: &str = "plot_strain_yy";
const PLOT_STRAIN_YZ_ID: &str = "plot_strain_yz";
const PLOT_STRAIN_ZZ_ID: &str = "plot_strain_zz";
const PLOT_STRAINS_MENU_INPUT_FIELD_CLASS: &str = "plot_strains_menu_input_field";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub strain_component_selected: Option<StressStrainComponent>,
    pub select_strain_component: Callback<StressStrainComponent>,
}


struct State
{
    strain_component_selected: StressStrainComponent,
}


pub struct PlotStrainsMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHidePlotStrainsMenu,
    SelectStrainComponent(ChangeData),
    ApplyStrainComponent,
}


impl PlotStrainsMenu
{
    fn show_hide_plot_strains_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(PLOT_STRAINS_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(PLOT_STRAINS_MENU_CLASS);
        }
        else
        {
            element.set_class_name(&(PLOT_STRAINS_MENU_CLASS.to_owned() + " " + HIDDEN));
        }
    }
}


impl Component for PlotStrainsMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link, state: State { strain_component_selected: StressStrainComponent::XX } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHidePlotStrainsMenu => self.show_hide_plot_strains_menu(),
            Msg::SelectStrainComponent(data) =>
                {
                    match data
                    {
                        ChangeData::Value(stress_select) =>
                            {
                                if stress_select == StressStrainComponent::XX.as_str()
                                {
                                    self.state.strain_component_selected = StressStrainComponent::XX;
                                }
                                if stress_select == StressStrainComponent::XY.as_str()
                                {
                                    self.state.strain_component_selected = StressStrainComponent::XY;
                                }
                                if stress_select == StressStrainComponent::XZ.as_str()
                                {
                                    self.state.strain_component_selected = StressStrainComponent::XZ;
                                }
                                if stress_select == StressStrainComponent::YY.as_str()
                                {
                                    self.state.strain_component_selected = StressStrainComponent::YY;
                                }
                                if stress_select == StressStrainComponent::YZ.as_str()
                                {
                                    self.state.strain_component_selected = StressStrainComponent::YZ;
                                }
                                if stress_select == StressStrainComponent::ZZ.as_str()
                                {
                                    self.state.strain_component_selected = StressStrainComponent::ZZ;
                                }
                                return false;
                            }
                        _ => (),
                    }
                },
            Msg::ApplyStrainComponent =>
                {
                    self.props.select_strain_component.emit(self.state.strain_component_selected.to_owned());
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
                    onclick=self.link.callback(|_| Msg::ShowHidePlotStrainsMenu),
                >
                    { "Plot strains" }
                </button>
                <div id = { PLOT_STRAINS_MENU_ID } class={ PLOT_STRAINS_MENU_CLASS.to_owned() + " " + HIDDEN }>
                    <div class={ PLOT_STRAINS_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <div class={ PLOT_STRAINS_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRAINS_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStrainComponent(data)),
                                type="radio", id={ PLOT_STRAIN_XX_ID },
                                name={ PLOT_STRAINS_INPUT_NAME },
                                value={ StressStrainComponent::XX.as_str() },
                                checked=
                                {
                                    if let Some(strain) = &self.props.strain_component_selected
                                    {
                                        if *strain == StressStrainComponent::XX
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
                            <label for={ PLOT_STRAIN_XX_ID }>
                                { StressStrainComponent::XX.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRAINS_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRAINS_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStrainComponent(data)),
                                type="radio", id={ PLOT_STRAIN_XY_ID },
                                name={ PLOT_STRAINS_INPUT_NAME },
                                value={ StressStrainComponent::XY.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(strain) = &self.props.strain_component_selected
                                    {
                                        if *strain == StressStrainComponent::XY
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
                            <label for={ PLOT_STRAIN_XY_ID }>
                                { StressStrainComponent::XY.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRAINS_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRAINS_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStrainComponent(data)),
                                type="radio", id={ PLOT_STRAIN_XZ_ID },
                                name={ PLOT_STRAINS_INPUT_NAME },
                                value={ StressStrainComponent::XZ.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(strain) = &self.props.strain_component_selected
                                    {
                                        if *strain == StressStrainComponent::XZ
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
                            <label for={ PLOT_STRAIN_XZ_ID }>
                                { StressStrainComponent::XZ.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRAINS_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRAINS_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStrainComponent(data)),
                                type="radio", id={ PLOT_STRAIN_YY_ID },
                                name={ PLOT_STRAINS_INPUT_NAME },
                                value={ StressStrainComponent::YY.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(strain) = &self.props.strain_component_selected
                                    {
                                        if *strain == StressStrainComponent::YY
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
                            <label for={ PLOT_STRAIN_YY_ID }>
                                { StressStrainComponent::YY.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRAINS_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRAINS_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStrainComponent(data)),
                                type="radio", id={ PLOT_STRAIN_YZ_ID },
                                name={ PLOT_STRAINS_INPUT_NAME },
                                value={ StressStrainComponent::YZ.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(strain) = &self.props.strain_component_selected
                                    {
                                        if *strain == StressStrainComponent::YZ
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
                            <label for={ PLOT_STRAIN_YZ_ID }>
                                { StressStrainComponent::YZ.as_str() }
                            </label>
                        </div>
                        <div class={ PLOT_STRAINS_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ PLOT_STRAINS_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectStrainComponent(data)),
                                type="radio", id={ PLOT_STRAIN_ZZ_ID },
                                name={ PLOT_STRAINS_INPUT_NAME },
                                value={ StressStrainComponent::ZZ.as_str() },
                                disabled=true,
                                checked=
                                {
                                    if let Some(strain) = &self.props.strain_component_selected
                                    {
                                        if *strain == StressStrainComponent::ZZ
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
                            <label for={ PLOT_STRAIN_ZZ_ID }>
                                { StressStrainComponent::ZZ.as_str() }
                            </label>
                        </div>
                    </div>
                    <div class={ PLOT_STRAINS_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ PLOT_STRAINS_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyStrainComponent),
                        >
                            { "Apply" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
