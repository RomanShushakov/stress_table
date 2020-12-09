use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;

use crate::fe::node::FeNode;
use crate::fe::elements::element::FElement;
use crate::fe::elements::truss::Truss2n2ip;
use crate::Coordinates;


const ELEMENTS_MENU_ID: &str = "elements_menu";
const ELEMENTS_MENU: &str = "elements_menu";
const HIDDEN: &str = "hidden";
const ELEMENT_SELECT_ID: &str = "element_select";
const NODE_1_NUMBER: &str = "first_node_number";
const NODE_2_NUMBER: &str = "second_node_number";
const YOUNG_S_MODULUS: &str = "young_s_modulus";
const AREA: &str = "area";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub nodes: Vec<FeNode<u16, f64>>,
    pub elements: Vec<Rc<RefCell<dyn FElement<u16, f64, f32>>>>,
    pub add_element: Callback<Rc<RefCell<dyn FElement<u16, f64, f32>>>>,
    pub update_element: Callback<(usize, Rc<RefCell<dyn FElement<u16, f64, f32>>>)>,
    pub remove_element: Callback<usize>,
}


struct State
{
    selected_element: Rc<RefCell<dyn FElement<u16, f64, f32>>>,
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
    SelectElement(ChangeData),
    ApplyElementDataChange,
    RemoveElement,
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
        options.set_length(self.props.elements.len() as u32 + 1);
        let number =
            {
                let mut n = 0;
                for (i, element) in self.props.elements.iter().enumerate()
                {
                    if let Ok(option) = HtmlOptionElement::new()
                    {
                        option.set_value(&element.borrow().show_info().number.to_string());
                        option.set_text(&element.borrow().show_info().number.to_string());
                        options.set(i as u32, Some(&option)).unwrap();
                    }
                    if element.borrow().show_info().number > n
                    {
                        n = element.borrow().show_info().number;
                    }
                }
                n + 1
            };
        let default_node_1 = FeNode { number: 1, coordinates: Coordinates { x: 1.0, y: 0.0, z: 0.0 } };
        let default_node_2 = FeNode { number: 2, coordinates: Coordinates { x: 2.0, y: 0.0, z: 0.0 } };
        let new_element = Truss2n2ip::create
        (
            number, default_node_1, default_node_2,
            1f32, 1f32, None
        );
        self.state.selected_element = Rc::new(RefCell::new(new_element));
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.elements.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.elements.len() as i32).unwrap();
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
            0
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
            0.0
        }
    }
}


impl Component for ElementsMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let default_node_1 = FeNode { number: 1, coordinates: Coordinates { x: 1.0, y: 0.0, z: 0.0 } };
        let default_node_2 = FeNode { number: 2, coordinates: Coordinates { x: 2.0, y: 0.0, z: 0.0 } };
        let default_element = Truss2n2ip::create
        (
            1u16, default_node_1, default_node_2,
            1f32, 1f32, None
        );
        let selected_element = Rc::new(RefCell::new(default_element));
        Self { props, link, state: State { selected_element } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideElementsMenu => self.show_hide_elements_menu(),
            Msg::SelectElement(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_element) =>
                            {
                                if let Some(position) = self.props.elements
                                        .iter()
                                        .position(|element|
                                            element.borrow().show_info().number.to_string() == select_element.value())
                                {
                                    self.state.selected_element = self.props.elements[position].to_owned();
                                }
                                else
                                {
                                    let number = select_element.value().parse::<u16>().unwrap();
                                    let default_node_1 = FeNode
                                        {
                                            number: 1,
                                            coordinates: Coordinates { x: 1.0, y: 0.0, z: 0.0 }
                                        };
                                    let default_node_2 = FeNode
                                        {
                                            number: 2,
                                            coordinates: Coordinates { x: 2.0, y: 0.0, z: 0.0 }
                                        };
                                    let new_element = Truss2n2ip::create
                                    (
                                        number, default_node_1, default_node_2,
                                        1f32, 1f32, None
                                    );
                                    self.state.selected_element = Rc::new(RefCell::new(new_element));
                                }
                            },
                        _ => (),
                    }
                },
            Msg::ApplyElementDataChange =>
                {
                    let selected_element_node_1_number = self.read_inputted_node_number(NODE_1_NUMBER);
                    let selected_element_node_2_number = self.read_inputted_node_number(NODE_2_NUMBER);
                    let selected_element_young_s_modulus = self.read_inputted_data(YOUNG_S_MODULUS);
                    let selected_element_area = self.read_inputted_data(AREA);
                    if selected_element_node_1_number == selected_element_node_2_number
                    {
                        yew::services::DialogService::alert(
                            "The element's node 1 and node 2 are the same.");
                        return false;
                    }
                    let selected_element_node_1 =
                        {
                            if let Some(position) = self.props.nodes
                                .iter()
                                .position(|node| node.number == selected_element_node_1_number)
                            {
                                Some(self.props.nodes[position].to_owned())
                            }
                            else
                            {
                                None
                            }
                        };
                    let selected_element_node_2 =
                        {
                            if let Some(position) = self.props.nodes
                                .iter()
                                .position(|node| node.number == selected_element_node_2_number)
                            {
                                Some(self.props.nodes[position].to_owned())
                            }
                            else
                            {
                                None
                            }
                        };
                    if selected_element_node_1.is_none() || selected_element_node_2.is_none()
                    {
                        yew::services::DialogService::alert(
                            "The selected node or nodes do not exist.");
                        return false;
                    }
                    let created_element = Truss2n2ip::create(
                        self.state.selected_element.borrow().show_info().number.to_owned(),
                        selected_element_node_1.unwrap(),
                        selected_element_node_2.unwrap(),
                        selected_element_young_s_modulus,
                        selected_element_area,
                        None);
                    if let Some(position) = self.props.elements
                        .iter()
                        .position(|element|
                            {
                                element.borrow().show_info().number ==
                                self.state.selected_element.borrow().show_info().number
                            })
                    {
                        self.props.update_element.emit((position, Rc::new(RefCell::new(created_element))));
                    }
                    else
                    {
                        self.props.add_element.emit(Rc::new(RefCell::new(created_element)));
                    }
                },
            Msg::RemoveElement =>
                {
                    if let Some(position) = self.props.elements
                        .iter()
                        .position(|element|
                            {
                                element.borrow().show_info().number ==
                                self.state.selected_element.borrow().show_info().number
                            })
                    {
                        self.props.remove_element.emit(position);
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
                                            onchange=self.link.callback(|data: ChangeData| Msg::SelectElement(data)),
                                        >
                                            <option value={ self.state.selected_element.borrow().show_info().number }>
                                                { format!("{} New", self.state.selected_element.borrow().show_info().number) }
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
                                                value={ self.state.selected_element.borrow().show_info().nodes_numbers[0] },
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
                                                value={ self.state.selected_element.borrow().show_info().nodes_numbers[1] },
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
                                                id={ YOUNG_S_MODULUS },
                                                value={ self.state.selected_element.borrow().show_info().stiffness_properties[0] },
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
                                                value={ self.state.selected_element.borrow().show_info().stiffness_properties[1] },
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
                            onclick=self.link.callback(|_| Msg::ApplyElementDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class="elements_menu_button",
                            onclick=self.link.callback(|_| Msg::RemoveElement),
                        >
                            { "Remove" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
