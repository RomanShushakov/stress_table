use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;

use std::slice::Iter;
use self::ElementType::*;

use crate::AnalysisType;
use crate::fe::node::FeNode;
use crate::Coordinates;
use crate::AuxTruss2n2ip;


#[derive(PartialEq)]
enum ElementType
{
    Truss2n2ip,
    Other,
}


impl ElementType
{
    fn as_str(&self) -> String
    {
        match self
        {
            ElementType::Truss2n2ip => String::from("Truss2n2ip"),
            ElementType::Other => String::from("Other"),
        }
    }


    pub fn iterator() -> Iter<'static, ElementType>
    {
        static TYPES: [ElementType; 2] =
            [
                Truss2n2ip, Other,
            ];
        TYPES.iter()
    }

}


const ELEMENT_TYPE_SELECT_ID: &str = "element_type_select";
const ELEMENT_MENU_ID: &str = "element_menu";
const ELEMENT_MENU: &str = "element_menu";
const HIDDEN: &str = "hidden";
const ELEMENT_NUMBER_SELECT_ID: &str = "element_number_select";
const NODE_1_NUMBER: &str = "first_node_number";
const NODE_2_NUMBER: &str = "second_node_number";
const YOUNG_MODULUS: &str = "young_modulus";
const AREA: &str = "area";
const AREA_2: &str = "area_2";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub analysis_type: Option<AnalysisType>,
    pub nodes: Vec<FeNode<u16, f64>>,
    pub aux_truss2n2ip_elements: Vec<AuxTruss2n2ip>,
    pub add_aux_truss2n2ip_element: Callback<AuxTruss2n2ip>,
    pub update_aux_truss2n2ip_element: Callback<(usize, AuxTruss2n2ip)>,
    pub remove_aux_truss2n2ip_element: Callback<usize>,
}


struct State
{
    selected_element_type: ElementType,
    selected_aux_truss2n2ip_element: Option<AuxTruss2n2ip>,
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
    SelectAuxTruss2n2ipElement(ChangeData),
    ApplyAuxTruss2n2ipElementDataChange,
    RemoveAuxTruss2n2ipElement,
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
            element.set_class_name(ELEMENT_MENU);
        }
        else
        {
            element.set_class_name(&(ELEMENT_MENU.to_owned() + " " + HIDDEN));
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
        options.set_length(self.props.aux_truss2n2ip_elements.len() as u32 + 1);
        let number =
            {
                let mut n = 0;
                for (i, element) in self.props.aux_truss2n2ip_elements.iter().enumerate()
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

        let new_element = AuxTruss2n2ip
        {
            number, node_1_number: 1u16, node_2_number: 2u16,
            young_modulus: 1f32, area: 1f32, area_2: None,
        };

        self.state.selected_aux_truss2n2ip_element = Some(new_element);
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.aux_truss2n2ip_elements.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.aux_truss2n2ip_elements.len() as i32).unwrap();
    }


    fn read_inputted_node_number(&self, input_field: &str) -> u16
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(data) = input_element.value().parse::<u16>()
        {
            data
        }
        else
        {
            0u16
        }
    }


    fn read_inputted_data(&self, input_field: &str) -> f32
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(data) = input_element.value().parse::<f32>()
        {
            data
        }
        else
        {
            0f32
        }
    }
}


impl Component for ElementMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let default_element_type = ElementType::Truss2n2ip;
        let default_element = AuxTruss2n2ip
        {
            number: 1u16, node_1_number: 1u16, node_2_number: 2u16,
            young_modulus: 1f32, area: 1f32, area_2: None,
        };
        Self { props, link, state:
                State
                {
                    selected_element_type: default_element_type,
                    selected_aux_truss2n2ip_element: Some(default_element)
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
                                if select_element_type.value() == ElementType::Truss2n2ip.as_str()
                                {
                                    self.state.selected_element_type = ElementType::Truss2n2ip;
                                }
                                if select_element_type.value() == ElementType::Other.as_str()
                                {
                                    self.state.selected_element_type = ElementType::Other;
                                }
                            },
                        _ => (),
                    }
                },
            Msg::SelectAuxTruss2n2ipElement(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_element) =>
                            {
                                if let Some(position) = self.props.aux_truss2n2ip_elements
                                        .iter()
                                        .position(|truss_element|
                                            truss_element.number.to_string() == select_element.value())
                                {
                                    self.state.selected_aux_truss2n2ip_element =
                                        Some(self.props.aux_truss2n2ip_elements[position].to_owned());
                                }
                                else
                                {
                                    let number = select_element.value().parse::<u16>().unwrap();
                                    let new_element = AuxTruss2n2ip
                                        {
                                            number, node_1_number: 1u16, node_2_number: 2u16,
                                            young_modulus: 1f32, area: 1f32, area_2: None,
                                        };
                                    self.state.selected_aux_truss2n2ip_element = Some(new_element);
                                }
                            },
                        _ => (),
                    }
                },
            Msg::ApplyAuxTruss2n2ipElementDataChange =>
                {
                    let selected_element_node_1_inputted_number = self.read_inputted_node_number(NODE_1_NUMBER);
                    let selected_element_node_2_inputted_number = self.read_inputted_node_number(NODE_2_NUMBER);
                    if selected_element_node_1_inputted_number == selected_element_node_2_inputted_number
                    {
                        yew::services::DialogService::alert(
                            "The element's node 1 and node 2 are the same.");
                        return false;
                    }
                    let selected_element_young_modulus = self.read_inputted_data(YOUNG_MODULUS);
                    if selected_element_young_modulus == 0f32
                    {
                        yew::services::DialogService::alert(
                            "The element's Young's modulus should be greater than 0.");
                        return false;
                    }
                    let selected_element_area = self.read_inputted_data(AREA);
                    if selected_element_area == 0f32
                    {
                        yew::services::DialogService::alert(
                            "The element's area should be greater than 0.");
                        return false;
                    }
                    let selected_element_area_2 = self.read_inputted_data(AREA_2);
                    let node_1_number_position = self.props.nodes
                        .iter()
                        .position(|node| node.number == selected_element_node_1_inputted_number);
                    let node_2_number_position = self.props.nodes
                        .iter()
                        .position(|node| node.number == selected_element_node_2_inputted_number);
                    if node_1_number_position.is_none() || node_2_number_position.is_none()
                    {
                        yew::services::DialogService::alert(
                            "The selected node(or nodes) doesn't(or don't) exist.");
                        return false;
                    }
                    if let Some(_) = self.props.aux_truss2n2ip_elements
                        .iter()
                        .position(|existed_element|
                            {
                                (existed_element.node_1_number == selected_element_node_1_inputted_number) &&
                                (existed_element.node_2_number == selected_element_node_2_inputted_number) &&
                                (existed_element.number != self.state.selected_aux_truss2n2ip_element.as_ref().unwrap().number)
                            })
                    {
                        yew::services::DialogService::alert(
                            "The element with the same type and with the same nodes set is already in use.");
                        return false;
                    }
                    self.state.selected_aux_truss2n2ip_element.as_mut().unwrap().node_1_number = selected_element_node_1_inputted_number;
                    self.state.selected_aux_truss2n2ip_element.as_mut().unwrap().node_2_number = selected_element_node_2_inputted_number;
                    self.state.selected_aux_truss2n2ip_element.as_mut().unwrap().young_modulus = selected_element_young_modulus;
                    self.state.selected_aux_truss2n2ip_element.as_mut().unwrap().area = selected_element_area;
                    self.state.selected_aux_truss2n2ip_element.as_mut().unwrap().area_2 =
                        {
                            if selected_element_area_2 != 0f32
                            {
                                Some(selected_element_area_2)
                            }
                            else
                            {
                                None
                            }
                        };
                    if let Some(position) = self.props.aux_truss2n2ip_elements
                        .iter()
                        .position(|truss_element|
                            {
                                truss_element.number ==
                                self.state.selected_aux_truss2n2ip_element.as_ref().unwrap().number
                            })
                    {
                        self.props.update_aux_truss2n2ip_element.emit(
                            (
                                    position,
                                    self.state.selected_aux_truss2n2ip_element.as_ref().unwrap().to_owned()
                            ));
                    }
                    else
                    {
                        self.props.add_aux_truss2n2ip_element.emit(self.state.selected_aux_truss2n2ip_element.as_ref().unwrap().to_owned());
                    }
                },
            Msg::RemoveAuxTruss2n2ipElement =>
                {
                    if let Some(position) = self.props.aux_truss2n2ip_elements
                        .iter()
                        .position(|truss_element|
                            {
                                truss_element.number ==
                                self.state.selected_aux_truss2n2ip_element.as_ref().unwrap().number
                            })
                    {
                        self.props.remove_aux_truss2n2ip_element.emit(position);
                    }
                },
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            self.update_numbers_in_element_menu();
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
                    class="button", onclick=self.link.callback(|_| Msg::ShowHideElementMenu),
                    disabled={ if self.props.analysis_type.is_some() { false } else { true } },
                >
                    { "Element" }
                </button>
                <div id = { ELEMENT_MENU_ID } class={ ELEMENT_MENU.to_owned() + " " + HIDDEN }>
                    <div class="element_menu_input_fields">
                        <ul class="element_menu_input_fields_list">
                            <li>
                                <select
                                    id={ ELEMENT_TYPE_SELECT_ID },
                                    onchange=self.link.callback(|data: ChangeData| Msg::SelectElementType(data)),
                                >
                                    {
                                        for ElementType::iterator().map(|element_type|
                                            html!
                                            {
                                                <option
                                                    value={ element_type.as_str() },
                                                    selected={ element_type == &self.state.selected_element_type }
                                                >
                                                    { element_type.as_str() }
                                                </option>
                                            }
                                        )
                                    }
                                </select>
                            </li>
                            {
                                if let Some(aux_truss2n2ip_element) = &self.state.selected_aux_truss2n2ip_element
                                {
                                    html!
                                    {
                                        <>
                                            <li>
                                                {
                                                    html!
                                                    {
                                                        <select
                                                            id={ ELEMENT_NUMBER_SELECT_ID },
                                                            onchange=self.link.callback(|data: ChangeData| Msg::SelectAuxTruss2n2ipElement(data)),
                                                        >
                                                            <option value={ aux_truss2n2ip_element.number }>
                                                                { format!("{} New", aux_truss2n2ip_element.number) }
                                                            </option>
                                                        </select>
                                                    }
                                                }
                                            </li>
                                            {
                                                html!
                                                {
                                                    <>
                                                        <li>
                                                            <p class="element_menu_input_fields_descriptions">
                                                                { "1st node number:" }
                                                            </p>
                                                            <input
                                                                id={ NODE_1_NUMBER },
                                                                value={ aux_truss2n2ip_element.node_1_number },
                                                                type="number",
                                                                min={ 1 },
                                                                step={ 1 },

                                                            />
                                                        </li>
                                                        <li>
                                                            <p class="element_menu_input_fields_descriptions">
                                                                { "2nd node number:" }
                                                            </p>
                                                            <input
                                                                id={ NODE_2_NUMBER },
                                                                value={ aux_truss2n2ip_element.node_2_number },
                                                                type="number",
                                                                min={ 1 },
                                                                step={ 1 },
                                                            />
                                                        </li>
                                                        <li>
                                                            <p class="element_menu_input_fields_descriptions">
                                                                { "Young's modulus:" }
                                                            </p>
                                                            <input
                                                                id={ YOUNG_MODULUS },
                                                                value={ aux_truss2n2ip_element.young_modulus },
                                                                type="number",
                                                                min={ 0 },
                                                            />
                                                        </li>
                                                        <li>
                                                            <p class="element_menu_input_fields_descriptions">
                                                                { "Cross section area:" }
                                                            </p>
                                                            <input
                                                                id={ AREA },
                                                                value={ aux_truss2n2ip_element.area },
                                                                type="number",
                                                                min={ 0 },
                                                            />
                                                        </li>
                                                        <li>
                                                            <p class="element_menu_input_fields_descriptions">
                                                                { "Cross section area 2 (Optional value used for tapered element):" }
                                                            </p>
                                                            <input
                                                                id={ AREA_2 },
                                                                value=
                                                                    {
                                                                        if let Some(area_2) = aux_truss2n2ip_element.area_2
                                                                        {
                                                                            area_2.to_string()
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
                                                    </>
                                                }
                                            }
                                        </>
                                    }
                                }
                                else
                                {
                                    html! { }
                                }
                            }
                        </ul>
                    </div>
                    <div class="element_menu_buttons">
                        {
                            if let Some(aux_truss2n2ip_element) = &self.state.selected_aux_truss2n2ip_element
                            {
                                html!
                                {
                                    <>
                                        <button
                                            class="element_menu_button",
                                            onclick=self.link.callback(|_| Msg::ApplyAuxTruss2n2ipElementDataChange),
                                        >
                                            { "Apply" }
                                        </button>
                                        <button
                                            class="element_menu_button",
                                            onclick=self.link.callback(|_| Msg::RemoveAuxTruss2n2ipElement),
                                        >
                                            { "Remove" }
                                        </button>
                                    </>
                                }
                            }
                            else
                            {
                                html! { }
                            }
                        }
                    </div>
                </div>
            </>
        }
    }
}
