use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;

use self::ElementType::*;

use crate::{AnalysisType, AuxElement, ElementType};
use crate::fe::fe_node::FeNode;
use crate::PREPROCESSOR_BUTTON_CLASS;


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


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub analysis_type: Option<AnalysisType>,
    pub is_preprocessor_active: bool,
    pub nodes: Vec<FeNode<u16, f64>>,
    pub aux_elements: Vec<AuxElement>,
    pub add_aux_element: Callback<AuxElement>,
    pub update_aux_element: Callback<(usize, AuxElement)>,
    pub remove_aux_element: Callback<usize>,

}


struct State
{
    new_element_number: u16,
    selected_element_number: u16,
    selected_element: AuxElement,
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
    SelectAuxElementType(ChangeData),
    SelectAuxElementByNumber(ChangeData),
    ApplyAuxElementDataChange,
    RemoveAuxElement,
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
        options.set_length(self.props.aux_elements.len() as u32 + 1);
        let number =
            {
                let mut n = 0;
                for (i, element) in self.props.aux_elements.iter().enumerate()
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
        let new_element = AuxElement
        {
            element_type: ElementType::Truss2n2ip,
            number, node_1_number: 1u16, node_2_number: 2u16,
            young_modulus: 1f32, area: 1f32, area_2: None,

            moment_of_inertia_about_z_axis: None,
            moment_of_inertia_about_y_axis: None,
            torsion_constant: None,
        };
        self.state.selected_element = new_element;
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.aux_elements.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.aux_elements.len() as i32).unwrap();
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
                for (i, element_type) in ElementType::iterator().enumerate()
                {
                    if element_type == &self.state.selected_element.element_type
                    {
                        n = i;
                    }
                }
                n
            };
        options.set_selected_index(selected_index_number as i32).unwrap();
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


    fn read_inputted_properties(&self, input_field: &str) -> f32
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
        let default_element_number = 1u16;
        let default_element = AuxElement
            {
                element_type: default_element_type.to_owned(),
                number: default_element_number,
                node_1_number: 1u16, node_2_number: 2u16,
                young_modulus: 1f32,
                area: 1f32, area_2: None,

                moment_of_inertia_about_z_axis: None,
                moment_of_inertia_about_y_axis: None,
                torsion_constant: None,
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
            Msg::SelectAuxElementType(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_element_type) =>
                            {
                                if select_element_type.value() == ElementType::Truss2n2ip.as_str()
                                {
                                    self.state.selected_element.element_type = ElementType::Truss2n2ip;
                                }
                                if select_element_type.value() == ElementType::OtherType.as_str()
                                {
                                    self.state.selected_element.element_type = ElementType::OtherType;
                                }
                            },
                        _ => (),
                    }
                },
            Msg::SelectAuxElementByNumber(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_element) =>
                            {
                                let selected_element_number = select_element.value().parse::<u16>().unwrap();
                                self.state.selected_element_number = selected_element_number;
                                if let Some(position) = self.props.aux_elements
                                        .iter()
                                        .position(|element|
                                            element.number.to_string() == select_element.value())
                                {
                                    self.state.selected_element =
                                        self.props.aux_elements[position].to_owned();
                                }
                                else
                                {
                                    self.state.new_element_number = selected_element_number;
                                    let new_element = AuxElement
                                        {
                                            element_type: ElementType::Truss2n2ip,
                                            number: selected_element_number,
                                            node_1_number: 1u16,
                                            node_2_number: 2u16,
                                            young_modulus: 1f32,
                                            area: 1f32,
                                            area_2: None,

                                            moment_of_inertia_about_z_axis: None,
                                            moment_of_inertia_about_y_axis: None,
                                            torsion_constant: None,
                                        };
                                    self.state.selected_element = new_element;
                                }
                            },
                        _ => (),
                    }
                    self.update_selected_type_in_element_menu();
                },
            Msg::ApplyAuxElementDataChange =>
                {
                    let selected_element_node_1_inputted_number = self.read_inputted_node_number(NODE_1_NUMBER);
                    let selected_element_node_2_inputted_number = self.read_inputted_node_number(NODE_2_NUMBER);
                    if selected_element_node_1_inputted_number == selected_element_node_2_inputted_number
                    {
                        yew::services::DialogService::alert(
                            "The element's node 1 and node 2 are the same.");
                        return false;
                    }
                    let selected_element_young_modulus = self.read_inputted_properties(YOUNG_MODULUS);
                    if selected_element_young_modulus <= 0f32
                    {
                        yew::services::DialogService::alert(
                            "The element's Young's modulus should be greater than 0.");
                        return false;
                    }
                    let selected_element_area = self.read_inputted_properties(AREA);
                    if selected_element_area <= 0f32
                    {
                        yew::services::DialogService::alert(
                            "The element's area should be greater than 0.");
                        return false;
                    }
                    match self.state.selected_element.element_type
                    {
                        Truss2n2ip =>
                            {
                                let selected_element_area_2 = self.read_inputted_properties(AREA_2);
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
                                if let Some(_) = self.props.aux_elements
                                    .iter()
                                    .position(|existed_element|
                                        {
                                            (existed_element.node_1_number == selected_element_node_1_inputted_number) &&
                                            (existed_element.node_2_number == selected_element_node_2_inputted_number) &&
                                            (existed_element.number != self.state.selected_element.number)
                                        })
                                {
                                    yew::services::DialogService::alert(
                                        "The element with the same type and with the same nodes set is already in use.");
                                    return false;
                                }
                                self.state.selected_element.node_1_number = selected_element_node_1_inputted_number;
                                self.state.selected_element.node_2_number = selected_element_node_2_inputted_number;
                                self.state.selected_element.young_modulus = selected_element_young_modulus;
                                self.state.selected_element.area = selected_element_area;
                                self.state.selected_element.area_2 =
                                    {
                                        if selected_element_area_2 != 0f32
                                        {
                                            if selected_element_area_2 < 0f32
                                            {
                                                yew::services::DialogService::alert(
                                                    "The element's area 2 should be greater than 0.");
                                                return false;
                                            }
                                            else
                                            {
                                                Some(selected_element_area_2)
                                            }
                                        }
                                        else
                                        {
                                            None
                                        }
                                    };
                                if let Some(position) = self.props.aux_elements
                                    .iter()
                                    .position(|element|
                                        {
                                            element.number ==
                                            self.state.selected_element.number
                                        })
                                {
                                    self.props.update_aux_element.emit(
                                        (
                                                position,
                                                self.state.selected_element.to_owned()
                                        ));
                                }
                                else
                                {
                                    self.props.add_aux_element.emit(self.state.selected_element.to_owned());
                                }
                            },
                        OtherType =>
                            {
                                let selected_element_moment_of_inertia_z =
                                    self.read_inputted_properties(MOMENT_OF_INERTIA_ABOUT_Z_AXIS);
                                let selected_element_moment_of_inertia_y =
                                    self.read_inputted_properties(MOMENT_OF_INERTIA_ABOUT_Y_AXIS);
                                let selected_element_torsion_constant =
                                    self.read_inputted_properties(TORSION_CONSTANT);
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
                                if let Some(_) = self.props.aux_elements
                                    .iter()
                                    .position(|existed_element|
                                        {
                                            (existed_element.node_1_number == selected_element_node_1_inputted_number) &&
                                            (existed_element.node_2_number == selected_element_node_2_inputted_number) &&
                                            (existed_element.number != self.state.selected_element.number)
                                        })
                                {
                                    yew::services::DialogService::alert(
                                        "The element with the same type and with the same nodes set is already in use.");
                                    return false;
                                }
                                self.state.selected_element.node_1_number = selected_element_node_1_inputted_number;
                                self.state.selected_element.node_2_number = selected_element_node_2_inputted_number;
                                self.state.selected_element.young_modulus = selected_element_young_modulus;
                                self.state.selected_element.area = selected_element_area;
                                self.state.selected_element.moment_of_inertia_about_z_axis =
                                    {
                                        if selected_element_moment_of_inertia_z > 0f32
                                        {
                                            Some(selected_element_moment_of_inertia_z)
                                        }
                                        else
                                        {
                                            yew::services::DialogService::alert(
                                            "The element's moment of inertia about x axis should be greater than 0.");
                                            return false;
                                        }
                                    };
                                self.state.selected_element.moment_of_inertia_about_y_axis =
                                    {
                                        if selected_element_moment_of_inertia_y > 0f32
                                        {
                                            Some(selected_element_moment_of_inertia_y)
                                        }
                                        else
                                        {
                                            yew::services::DialogService::alert(
                                            "The element's moment of inertia about y axis should be greater than 0.");
                                            return false;
                                        }
                                    };
                                self.state.selected_element.torsion_constant =
                                    {
                                        if selected_element_torsion_constant > 0f32
                                        {
                                            Some(selected_element_torsion_constant)
                                        }
                                        else
                                        {
                                            yew::services::DialogService::alert(
                                            "The element's torsion constant should be greater than 0.");
                                            return false;
                                        }
                                    };
                                if let Some(position) = self.props.aux_elements
                                    .iter()
                                    .position(|element|
                                        {
                                            element.number ==
                                            self.state.selected_element.number
                                        })
                                {
                                    self.props.update_aux_element.emit(
                                        (
                                                position,
                                                self.state.selected_element.to_owned()
                                        ));
                                }
                                else
                                {
                                    self.props.add_aux_element.emit(self.state.selected_element.to_owned());
                                }
                            },
                    }
                },
            Msg::RemoveAuxElement =>
                {
                    if let Some(position) = self.props.aux_elements
                        .iter()
                        .position(|element|
                            {
                                element.number ==
                                self.state.selected_element.number
                            })
                    {
                        self.props.remove_aux_element.emit(position);
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
                            if self.props.analysis_type.is_some() && self.props.is_preprocessor_active
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
                                            onchange=self.link.callback(|data: ChangeData| Msg::SelectAuxElementByNumber(data)),
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
                                    onchange=self.link.callback(|data: ChangeData| Msg::SelectAuxElementType(data)),
                                    disabled={ self.state.selected_element_number != self.state.new_element_number },
                                >
                                    {
                                        for ElementType::iterator().map(|element_type|
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
                                    value={ self.state.selected_element.node_1_number },
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
                                    value={ self.state.selected_element.node_2_number },
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
                                    value={ self.state.selected_element.young_modulus },
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
                                    value={ self.state.selected_element.area },
                                    type="number",
                                    min={ 0 },
                                />
                            </li>
                            {
                                match self.state.selected_element.element_type
                                {
                                    ElementType::Truss2n2ip =>
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
                                                                if let Some(area_2) = self.state.selected_element.area_2
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
                                            }
                                        },
                                    ElementType::OtherType =>
                                        {
                                            html!
                                            {
                                                <>
                                                    <li>
                                                        <p class={ ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                            { "The moment of inertia about z axis:" }
                                                        </p>
                                                        <input
                                                            id={ MOMENT_OF_INERTIA_ABOUT_Z_AXIS },
                                                            value=
                                                                {
                                                                    if let Some(moment_of_inertia_z) =
                                                                        self.state.selected_element.moment_of_inertia_about_z_axis
                                                                    {
                                                                        moment_of_inertia_z.to_string()
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
                                                    <li>
                                                        <p class={ ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                            { "The moment of inertia about y axis:" }
                                                        </p>
                                                        <input
                                                            id={ MOMENT_OF_INERTIA_ABOUT_Y_AXIS },
                                                            value=
                                                                {
                                                                    if let Some(moment_of_inertia_y) =
                                                                        self.state.selected_element.moment_of_inertia_about_y_axis
                                                                    {
                                                                        moment_of_inertia_y.to_string()
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
                                                    <li>
                                                        <p class={ ELEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                            { "The torsion constant:" }
                                                        </p>
                                                        <input
                                                            id={ TORSION_CONSTANT },
                                                            value=
                                                                {
                                                                    if let Some(torsion_constant) =
                                                                        self.state.selected_element.torsion_constant
                                                                    {
                                                                        torsion_constant.to_string()
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
                                        },
                                }
                            }
                        </ul>
                    </div>
                    <div class={ ELEMENT_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ ELEMENT_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyAuxElementDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class={ ELEMENT_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::RemoveAuxElement),
                        >
                            { "Remove" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
