use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;

use crate::fem::{FENode};
use crate::auxiliary::FEDrawnNodeData;
use crate::{ElementsNumbers, ElementsValues};
use crate::pages::POSTPROCESSOR_BUTTON_CLASS;


const PLOT_DISPLACEMENTS_MENU_ID: &str = "plot_displacements_menu";
const PLOT_DISPLACEMENTS_MENU: &str = "plot_displacements_menu";
const PLOT_DISPLACEMENTS_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str =
    "plot_displacements_menu_input_fields_container";
const PLOT_DISPLACEMENTS_MENU_INPUT_FIELDS_LIST_CLASS: &str =
    "plot_displacements_menu_input_fields_list";
const PLOT_DISPLACEMENTS_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS: &str =
    "plot_displacements_menu_input_fields_descriptions";
const PLOT_DISPLACEMENTS_MENU_BUTTONS_CONTAINER_CLASS: &str = "plot_displacements_menu_buttons";
const PLOT_DISPLACEMENTS_MENU_BUTTON_CLASS: &str = "plot_displacements_menu_button";
const HIDDEN: &str = "hidden";
const MAGNITUDE: &str = "magnitude";


#[derive(Properties, Clone)]
pub struct Props
{
    pub magnitude: ElementsValues,
    pub change_magnitude: Callback<ElementsValues>,
    pub select_plot_displacements: Callback<()>,
}


pub struct PlotDisplacementsMenu
{
    link: ComponentLink<Self>,
    props: Props,
}


pub enum Msg
{
    ShowHidePlotDisplacementsMenu,
    ChangeMagnitude,
}


impl PlotDisplacementsMenu
{
    fn show_hide_plot_displacements_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(PLOT_DISPLACEMENTS_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(PLOT_DISPLACEMENTS_MENU);
        }
        else
        {
            element.set_class_name(&(PLOT_DISPLACEMENTS_MENU.to_owned() + " " + HIDDEN));
        }
    }


    fn read_inputted_magnitude(&self, input_field: &str) -> ElementsValues
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(coord) = input_element.value().parse::<ElementsValues>()
        {
            if coord < 0.0
            {
                0.0
            }
            else
            {
                coord
            }
        }
        else
        {
            0.0
        }
    }
}


impl Component for PlotDisplacementsMenu
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
            Msg::ShowHidePlotDisplacementsMenu => self.show_hide_plot_displacements_menu(),
            Msg::ChangeMagnitude =>
                {
                    let magnitude = self.read_inputted_magnitude(MAGNITUDE);
                    self.props.select_plot_displacements.emit(());
                    self.props.change_magnitude.emit(magnitude);
                },
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if &self.props.magnitude != &props.magnitude
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
                    class={ POSTPROCESSOR_BUTTON_CLASS }, onclick=self.link.callback(|_|
                        Msg::ShowHidePlotDisplacementsMenu),
                >
                    { "Plot displacements" }
                </button>
                <div
                    id = { PLOT_DISPLACEMENTS_MENU_ID },
                    class={ PLOT_DISPLACEMENTS_MENU.to_owned() + " " + HIDDEN }
                >
                    <div class={ PLOT_DISPLACEMENTS_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <ul class={ PLOT_DISPLACEMENTS_MENU_INPUT_FIELDS_LIST_CLASS }>
                            <li>
                                <p class={ PLOT_DISPLACEMENTS_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                    { "Magnitude:" }
                                </p>
                                <input
                                    id={ MAGNITUDE },
                                    value={ self.props.magnitude },
                                    type="number",
                                    min=1.0,
                                />
                            </li>
                        </ul>
                    </div>
                    <div class={ PLOT_DISPLACEMENTS_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ PLOT_DISPLACEMENTS_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ChangeMagnitude),
                        >
                            { "Apply" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
