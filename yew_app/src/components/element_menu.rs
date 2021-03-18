use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;
use std::rc::Rc;
use crate::fem::FEType;

use crate::auxiliary::{FEDrawnElementData};
use crate::{ElementsNumbers, ElementsValues, UIDNumbers};
use crate::pages::PREPROCESSOR_BUTTON_CLASS;


const ELEMENT_TYPE_SELECT_ID: &str = "element_type_select";
const ELEMENT_MENU_ID: &str = "element_menu";
const ELEMENT_MENU_CLASS: &str = "element_menu";
const ELEMENT_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "element_menu_input_fields_container";
const ELEMENT_MENU_INPUT_FIELDS_LIST_CLASS: &str = "element_menu_input_fields_list";
const ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS: &str = "element_menu_input_fields_descriptions";
const ELEMENT_MENU_BUTTONS_CONTAINER_CLASS: &str = "element_menu_buttons";
const ELEMENT_MENU_BUTTON_CLASS: &str = "element_menu_button";
const HIDDEN: &str = "hidden";
const ELEMENT_NUMBER_SELECT_ID: &str = "element_number_select";
const NODE_1_NUMBER: &str = "first_node_number";
const NODE_2_NUMBER: &str = "second_node_number";
const YOUNG_MODULUS: &str = "young_modulus";
const AREA: &str = "area";
const AREA_2: &str = "area_2";

const MOMENT_OF_INERTIA_ABOUT_Z_AXIS: &str = "moment_of_inertia_about_z_axis";
const MOMENT_OF_INERTIA_ABOUT_Y_AXIS: &str = "moment_of_inertia_about_y_axis";
const TORSION_CONSTANT: &str = "torsion_constant";


#[derive(Properties, Clone)]
pub struct Props
{
    pub is_preprocessor_active: bool,
    pub drawn_elements: Rc<Vec<FEDrawnElementData>>,
    pub add_element: Callback<FEDrawnElementData>,
    pub update_element: Callback<FEDrawnElementData>,
    pub delete_element: Callback<ElementsNumbers>,
}


struct State
{
    new_element_number: ElementsNumbers,
    selected_element_number: ElementsNumbers,
    selected_element: FEDrawnElementData,
}


pub struct ElementMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideElementMenu,
    SelectElementType(ChangeData),
    SelectElementByNumber(ChangeData),
    ApplyElementDataChange,
    DeleteElement,
}


impl ElementMenu
{
    fn show_hide_element_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(ELEMENT_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(ELEMENT_MENU_CLASS);
        }
        else
        {
            element.set_class_name(&(ELEMENT_MENU_CLASS.to_owned() + " " + HIDDEN));
        }
    }


    fn update_numbers_in_element_menu(&mut self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(ELEMENT_NUMBER_SELECT_ID).unwrap();
        let select = element.dyn_into::<HtmlSelectElement>()
            .map_err(|_| ())
            .unwrap();
        let options: HtmlOptionsCollection = select.options();
        options.set_length(self.props.drawn_elements.len() as u32 + 1);
        let uid = UIDNumbers::default();
        let number =
            {
                let mut n = 0;
                for (i, element) in self.props.drawn_elements.iter().enumerate()
                {
                    if let Ok(option) = HtmlOptionElement::new()
                    {
                        option.set_value(&element.number.to_string());
                        option.set_text(&element.number.to_string());
                        options.set(i as u32, Some(&option)).unwrap();
                    }
                    if element.number > n
                    {
                        n = element.number;
                    }
                }
                n + 1
            };
        self.state.new_element_number = number;
        self.state.selected_element_number = number;
        let new_element = FEDrawnElementData
        {
            uid, fe_type: FEType::Truss2n2ip,
            number, nodes_numbers: vec![1 as ElementsNumbers, 2 as ElementsNumbers],
            properties: vec![1e6 as ElementsValues, 1.0 as ElementsValues],
        };
        self.state.selected_element = new_element;
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.drawn_elements.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.drawn_elements.len() as i32).unwrap();
    }


    fn update_selected_type_in_element_menu(&mut self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(ELEMENT_TYPE_SELECT_ID).unwrap();
        let select = element.dyn_into::<HtmlSelectElement>()
            .map_err(|_| ())
            .unwrap();
        let options: HtmlOptionsCollection = select.options();
        let selected_index_number =
            {
                let mut n = 0;
                for (i, element_type) in FEType::iterator().enumerate()
                {
                    if element_type == &self.state.selected_element.fe_type
                    {
                        n = i;
                    }
                }
                n
            };
        options.set_selected_index(selected_index_number as i32).unwrap();
    }


    fn read_inputted_node_number(&self, input_field: &str) -> ElementsNumbers
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(data) = input_element.value().parse::<ElementsNumbers>()
        {
            data
        }
        else
        {
            0 as ElementsNumbers
        }
    }


    fn read_inputted_properties(&self, input_field: &str) -> ElementsValues
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(data) = input_element.value().parse::<ElementsValues>()
        {
            data
        }
        else
        {
            0.0 as ElementsValues
        }
    }
}


impl Component for ElementMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let uid = UIDNumbers::default();
        let default_element_type = FEType::Truss2n2ip;
        let default_element_number = 1 as ElementsNumbers;
        let default_element = FEDrawnElementData
            {
                uid,
                fe_type: default_element_type.to_owned(),
                number: default_element_number,
                nodes_numbers: vec![1 as ElementsNumbers, 2 as ElementsNumbers],
                properties: vec![1e6 as ElementsValues, 1.0 as ElementsValues],
            };
        Self
        {
            props,
            link,
            state: State
                {
                    new_element_number: default_element_number,
                    selected_element_number: default_element_number,
                    selected_element: default_element,
                }
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideElementMenu => self.show_hide_element_menu(),
            Msg::SelectElementType(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_element_type) =>
                            {
                                if select_element_type.value() == FEType::Truss2n2ip.as_str()
                                {
                                    self.state.selected_element.fe_type = FEType::Truss2n2ip;
                                }
                            },
                        _ => (),
                    }
                },
            Msg::SelectElementByNumber(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_element) =>
                            {
                                let selected_element_number = select_element.value()
                                    .parse::<ElementsNumbers>().unwrap();
                                self.state.selected_element_number = selected_element_number;
                                if let Some(position) = self.props.drawn_elements
                                        .iter()
                                        .position(|element|
                                            element.number.to_string() == select_element.value())
                                {
                                    self.state.selected_element =
                                        self.props.drawn_elements[position].to_owned();
                                }
                                else
                                {
                                    let uid = UIDNumbers::default();
                                    self.state.new_element_number = selected_element_number;
                                    let new_element = FEDrawnElementData
                                        {
                                            uid,
                                            fe_type: FEType::Truss2n2ip,
                                            number: selected_element_number,
                                            nodes_numbers: vec![1 as ElementsNumbers,
                                                                2 as ElementsNumbers],
                                            properties: vec![1e6 as ElementsValues,
                                                             1.0 as ElementsValues],
                                        };
                                    self.state.selected_element = new_element;
                                }
                            },
                        _ => (),
                    }
                    self.update_selected_type_in_element_menu();
                },
            Msg::ApplyElementDataChange =>
                {
                    let selected_element_node_1_inputted_number =
                        self.read_inputted_node_number(NODE_1_NUMBER);
                    let selected_element_node_2_inputted_number =
                        self.read_inputted_node_number(NODE_2_NUMBER);
                    let selected_element_young_modulus =
                        self.read_inputted_properties(YOUNG_MODULUS);
                    let selected_element_area = self.read_inputted_properties(AREA);

                    match self.state.selected_element.fe_type
                    {
                        FEType::Truss2n2ip =>
                            {
                                let selected_element_area_2 =
                                    self.read_inputted_properties(AREA_2);
                                self.state.selected_element.nodes_numbers[0] =
                                    selected_element_node_1_inputted_number;
                                self.state.selected_element.nodes_numbers[1] =
                                    selected_element_node_2_inputted_number;
                                self.state.selected_element.properties[0] =
                                    selected_element_young_modulus;
                                self.state.selected_element.properties[1] = selected_element_area;
                                if selected_element_area_2 != 0.0 as ElementsValues
                                {
                                    if self.state.selected_element.properties.len() == 3
                                    {
                                        self.state.selected_element.properties[2] =
                                            selected_element_area_2
                                    }
                                    else
                                    {
                                        self.state.selected_element.properties.push(
                                            selected_element_area_2)
                                    }
                                }
                                else
                                {
                                    if self.state.selected_element.properties.len() == 3
                                    {
                                        self.state.selected_element.properties.remove(2);
                                    }
                                }
                                if self.props.drawn_elements
                                    .iter()
                                    .position(|element|
                                        {
                                            element.number ==
                                            self.state.selected_element.number
                                        }).is_some()
                                {
                                    self.props.update_element.emit(
                                        self.state.selected_element.to_owned());
                                }
                                else
                                {
                                    self.props.add_element.emit(
                                        self.state.selected_element.to_owned());
                                }
                            },
                    }
                },
            Msg::DeleteElement =>
                {
                    if let Some(position) = self.props.drawn_elements
                        .iter()
                        .position(|element|
                            {
                                element.number ==
                                self.state.selected_element.number
                            })
                    {
                        let element_number = self.state.selected_element.number;
                        self.props.delete_element.emit(element_number);
                    }
                },
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if &self.props.is_preprocessor_active != &props.is_preprocessor_active ||
            !Rc::ptr_eq(&self.props.drawn_elements, &props.drawn_elements)
        {
            self.props = props;
            self.update_numbers_in_element_menu();
            self.update_selected_type_in_element_menu();
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
                    class={ PREPROCESSOR_BUTTON_CLASS }, onclick=self.link.callback(|_| Msg::ShowHideElementMenu),
                    disabled=
                        {
                            if self.props.is_preprocessor_active
                            {
                                false
                            }
                            else
                            {
                                true
                            }
                        },
                >
                    { "Element" }
                </button>
                <div id = { ELEMENT_MENU_ID } class={ ELEMENT_MENU_CLASS.to_owned() + " " + HIDDEN }>
                    <div class={ ELEMENT_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <ul class={ ELEMENT_MENU_INPUT_FIELDS_LIST_CLASS }>
                            <li>
                                {
                                    html!
                                    {
                                        <select
                                            id={ ELEMENT_NUMBER_SELECT_ID },
                                            onchange=self.link.callback(|data: ChangeData| Msg::SelectElementByNumber(data)),
                                        >
                                            <option value={ self.state.selected_element.number }>
                                                { format!("{} New", self.state.selected_element.number) }
                                            </option>
                                        </select>
                                    }
                                }
                            </li>
                            <li>
                                <p class={ ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                    { "Element type:" }
                                </p>
                                <select
                                    id={ ELEMENT_TYPE_SELECT_ID },
                                    onchange=self.link.callback(|data: ChangeData| Msg::SelectElementType(data)),
                                    disabled={ self.state.selected_element_number != self.state.new_element_number },
                                >
                                    {
                                        for FEType::iterator().map(|element_type|
                                            html!
                                            {
                                                <option
                                                    value={ element_type.as_str() },
                                                >
                                                    { element_type.as_str() }
                                                </option>
                                            }
                                        )
                                    }
                                </select>
                            </li>
                            <li>
                                <p class={ ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                    { "1st node number:" }
                                </p>
                                <input
                                    id={ NODE_1_NUMBER },
                                    value={ self.state.selected_element.nodes_numbers[0] },
                                    type="number",
                                    min={ 1 },
                                    step={ 1 },

                                />
                            </li>
                            <li>
                                <p class={ ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                    { "2nd node number:" }
                                </p>
                                <input
                                    id={ NODE_2_NUMBER },
                                    value={ self.state.selected_element.nodes_numbers[1] },
                                    type="number",
                                    min={ 1 },
                                    step={ 1 },
                                />
                            </li>
                            <li>
                                <p class={ ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                    { "Young's modulus:" }
                                </p>
                                <input
                                    id={ YOUNG_MODULUS },
                                    value={ self.state.selected_element.properties[0] },
                                    type="number",
                                    min={ 0 },
                                />
                            </li>
                            <li>
                                <p class={ ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                    { "Cross section area:" }
                                </p>
                                <input
                                    id={ AREA },
                                    value={ self.state.selected_element.properties[1] },
                                    type="number",
                                    min={ 0 },
                                />
                            </li>
                            {
                                match self.state.selected_element.fe_type
                                {
                                    FEType::Truss2n2ip =>
                                        {
                                            html!
                                            {
                                                <li>
                                                    <p class={ ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                        { "Cross section area 2 (Optional value used for tapered element):" }
                                                    </p>
                                                    <input
                                                        id={ AREA_2 },
                                                        value=
                                                            {
                                                                if self.state.selected_element.properties.len() == 3
                                                                {
                                                                    self.state.selected_element.properties[2].to_string()
                                                                }
                                                                else
                                                                {
                                                                    "".to_string()
                                                                }
                                                            },
                                                        type="number",
                                                        min={ 0 },
                                                    />
                                                </li>
                                            }
                                        },
                                }
                            }
                        </ul>
                    </div>
                    <div class={ ELEMENT_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ ELEMENT_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyElementDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class={ ELEMENT_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::DeleteElement),
                        >
                            { "Delete" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
