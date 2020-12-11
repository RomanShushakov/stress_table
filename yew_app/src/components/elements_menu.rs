use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;

use crate::fe::node::FeNode;
use crate::Coordinates;
use crate::AuxTruss;


const ELEMENTS_MENU_ID: &str = "elements_menu";
const ELEMENTS_MENU: &str = "elements_menu";
const HIDDEN: &str = "hidden";
const ELEMENT_SELECT_ID: &str = "element_select";
const NODE_1_NUMBER: &str = "first_node_number";
const NODE_2_NUMBER: &str = "second_node_number";
const YOUNG_MODULUS: &str = "young_modulus";
const AREA: &str = "area";
const AREA_2: &str = "area_2";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub nodes: Vec<FeNode<u16, f64>>,
    pub truss_elements_prep: Vec<AuxTruss>,
    pub add_aux_truss_element: Callback<AuxTruss>,
    pub update_aux_truss_element: Callback<(usize, AuxTruss)>,
    pub remove_aux_truss_element: Callback<usize>,
}


struct State
{
    selected_aux_truss_element: AuxTruss,
}


pub struct ElementsMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideElementsMenu,
    SelectAuxTrussElement(ChangeData),
    ApplyAuxTrussElementDataChange,
    RemoveAuxTrussElement,
}


impl ElementsMenu
{
    fn show_hide_elements_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(ELEMENTS_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(ELEMENTS_MENU);
        }
        else
        {
            element.set_class_name(&(ELEMENTS_MENU.to_owned() + " " + HIDDEN));
        }
    }


    fn update_element_menu(&mut self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(ELEMENT_SELECT_ID).unwrap();
        let select = element.dyn_into::<HtmlSelectElement>()
            .map_err(|_| ())
            .unwrap();
        let options: HtmlOptionsCollection = select.options();
        options.set_length(self.props.truss_elements_prep.len() as u32 + 1);
        let number =
            {
                let mut n = 0;
                for (i, element) in self.props.truss_elements_prep.iter().enumerate()
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
        let new_element = AuxTruss
        {
            number, node_1_number: 1u16, node_2_number: 2u16,
            young_modulus: 1f32, area: 1f32, area_2: None,
        };

        self.state.selected_aux_truss_element = new_element;
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.truss_elements_prep.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.truss_elements_prep.len() as i32).unwrap();
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


impl Component for ElementsMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let default_element = AuxTruss
        {
            number: 1u16, node_1_number: 1u16, node_2_number: 2u16,
            young_modulus: 1f32, area: 1f32, area_2: None,
        };
        Self { props, link, state: State { selected_aux_truss_element: default_element } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideElementsMenu => self.show_hide_elements_menu(),
            Msg::SelectAuxTrussElement(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_element) =>
                            {
                                if let Some(position) = self.props.truss_elements_prep
                                        .iter()
                                        .position(|truss_element|
                                            truss_element.number.to_string() == select_element.value())
                                {
                                    self.state.selected_aux_truss_element =
                                        self.props.truss_elements_prep[position].to_owned();
                                }
                                else
                                {
                                    let number = select_element.value().parse::<u16>().unwrap();
                                    let new_element = AuxTruss
                                        {
                                            number, node_1_number: 1u16, node_2_number: 2u16,
                                            young_modulus: 1f32, area: 1f32, area_2: None,
                                        };
                                    self.state.selected_aux_truss_element = new_element;
                                }
                            },
                        _ => (),
                    }
                },
            Msg::ApplyAuxTrussElementDataChange =>
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
                    if let Some(_) = self.props.truss_elements_prep
                        .iter()
                        .position(|existed_element|
                            {
                                (existed_element.node_1_number == selected_element_node_1_inputted_number) &&
                                (existed_element.node_2_number == selected_element_node_2_inputted_number)
                            })
                    {
                        yew::services::DialogService::alert(
                            "The element with the same type and with the same nodes set is already in use.");
                        return false;
                    }
                    self.state.selected_aux_truss_element.node_1_number = selected_element_node_1_inputted_number;
                    self.state.selected_aux_truss_element.node_2_number = selected_element_node_2_inputted_number;
                    self.state.selected_aux_truss_element.young_modulus = selected_element_young_modulus;
                    self.state.selected_aux_truss_element.area = selected_element_area;
                    self.state.selected_aux_truss_element.area_2 =
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
                    if let Some(position) = self.props.truss_elements_prep
                        .iter()
                        .position(|truss_element|
                            {
                                truss_element.number ==
                                self.state.selected_aux_truss_element.number
                            })
                    {
                        self.props.update_aux_truss_element.emit(
                            (
                                    position,
                                    self.state.selected_aux_truss_element.to_owned()
                            ));
                    }
                    else
                    {
                        self.props.add_aux_truss_element.emit(self.state.selected_aux_truss_element.to_owned());
                    }
                },
            Msg::RemoveAuxTrussElement =>
                {
                    if let Some(position) = self.props.truss_elements_prep
                        .iter()
                        .position(|truss_element|
                            {
                                truss_element.number ==
                                self.state.selected_aux_truss_element.number
                            })
                    {
                        self.props.remove_aux_truss_element.emit(position);
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
            self.update_element_menu();
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
                    class="button" onclick=self.link.callback(|_| Msg::ShowHideElementsMenu)
                >
                    { "Elements" }
                </button>
                <div id = { ELEMENTS_MENU_ID } class={ ELEMENTS_MENU.to_owned() + " " + HIDDEN }>
                    <div class="elements_menu_input_fields">
                        <ul class="elements_menu_input_fields_list">
                            <li>
                                {
                                    html!
                                    {
                                        <select
                                            id={ ELEMENT_SELECT_ID },
                                            onchange=self.link.callback(|data: ChangeData| Msg::SelectAuxTrussElement(data)),
                                        >
                                            <option value={ self.state.selected_aux_truss_element.number }>
                                                { format!("{} New", self.state.selected_aux_truss_element.number) }
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
                                            <p class="elements_menu_input_fields_descriptions">
                                                { "1st node number:" }
                                            </p>
                                            <input
                                                id={ NODE_1_NUMBER },
                                                value={ self.state.selected_aux_truss_element.node_1_number },
                                                type="number",
                                                min={ 1 },
                                                step={ 1 },
                                            />
                                        </li>
                                        <li>
                                            <p class="elements_menu_input_fields_descriptions">
                                                { "2nd node number:" }
                                            </p>
                                            <input
                                                id={ NODE_2_NUMBER },
                                                value={ self.state.selected_aux_truss_element.node_2_number },
                                                type="number",
                                                min={ 1 },
                                                step={ 1 },
                                            />
                                        </li>
                                        <li>
                                            <p class="elements_menu_input_fields_descriptions">
                                                { "Young's modulus:" }
                                            </p>
                                            <input
                                                id={ YOUNG_MODULUS },
                                                value={ self.state.selected_aux_truss_element.young_modulus },
                                                type="number",
                                                min={ 0 },
                                            />
                                        </li>
                                        <li>
                                            <p class="elements_menu_input_fields_descriptions">
                                                { "Cross section area:" }
                                            </p>
                                            <input
                                                id={ AREA },
                                                value={ self.state.selected_aux_truss_element.area },
                                                type="number",
                                                min={ 0 },
                                            />
                                        </li>
                                        <li>
                                            <p class="elements_menu_input_fields_descriptions">
                                                { "Cross section area 2 (Optional value used for tapered element):" }
                                            </p>
                                            <input
                                                id={ AREA_2 },
                                                value=
                                                    {
                                                        if let Some(area_2) = self.state.selected_aux_truss_element.area_2
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
                        </ul>
                    </div>
                    <div class="elements_menu_buttons">
                        <button
                            class="elements_menu_button",
                            onclick=self.link.callback(|_| Msg::ApplyAuxTrussElementDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class="elements_menu_button",
                            onclick=self.link.callback(|_| Msg::RemoveAuxTrussElement),
                        >
                            { "Remove" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
