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
use crate::{AnalysisType};
use crate::pages::PREPROCESSOR_BUTTON_CLASS;


const NODE_MENU_ID: &str = "node_menu";
const NODE_MENU: &str = "node_menu";
const NODE_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "node_menu_input_fields_container";
const NODE_MENU_INPUT_FIELDS_LIST_CLASS: &str = "node_menu_input_fields_list";
const NODE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS: &str = "node_menu_input_fields_descriptions";
const NODE_MENU_BUTTONS_CONTAINER_CLASS: &str = "node_menu_buttons";
const NODE_MENU_BUTTON_CLASS: &str = "node_menu_button";
const HIDDEN: &str = "hidden";
const NODE_SELECT_ID: &str = "node_select";
const NODE_X_COORD: &str = "node_x_coord";
const NODE_Y_COORD: &str = "node_y_coord";
const NODE_Z_COORD: &str = "node_z_coord";


#[derive(Properties, Clone)]
pub struct Props
{
    pub analysis_type: Option<AnalysisType>,
    pub is_preprocessor_active: bool,
    pub nodes: Rc<Vec<Rc<RefCell<FENode<ElementsNumbers, ElementsValues>>>>>,
    pub add_node: Callback<FEDrawnNodeData>,
    pub update_node: Callback<FEDrawnNodeData>,
    pub delete_node: Callback<ElementsNumbers>,
}


struct State
{
    selected_node: FEDrawnNodeData,
}


pub struct NodeMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideNodeMenu,
    SelectNode(ChangeData),
    ApplyNodeDataChange,
    DeleteNode,
}


impl NodeMenu
{
    fn show_hide_node_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(NODE_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(NODE_MENU);
        }
        else
        {
            element.set_class_name(&(NODE_MENU.to_owned() + " " + HIDDEN));
        }
    }


    fn update_node_menu(&mut self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(NODE_SELECT_ID).unwrap();
        let select = element.dyn_into::<HtmlSelectElement>()
            .map_err(|_| ())
            .unwrap();
        let options: HtmlOptionsCollection = select.options();
        options.set_length(self.props.nodes.len() as u32 + 1);
        let number =
            {
                let mut n = 0;
                for (i, node) in self.props.nodes.iter().enumerate()
                {
                    if let Ok(option) = HtmlOptionElement::new()
                    {
                        option.set_value(&node.borrow().number.to_string());
                        option.set_text(&node.borrow().number.to_string());
                        options.set(i as u32, Some(&option)).unwrap();
                    }
                    if node.borrow().number > n
                    {
                        n = node.borrow().number;
                    }
                }
                n + 1
            };
        let (x, y, z) = (0.0, 0.0, 0.0);
        self.state.selected_node = FEDrawnNodeData { number, x, y, z };
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.nodes.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.nodes.len() as i32).unwrap();
    }


    fn read_inputted_coordinate(&self, input_field: &str) -> ElementsValues
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(coord) = input_element.value().parse::<ElementsValues>()
        {
            coord
        }
        else
        {
            0.0
        }
    }
}


impl Component for NodeMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let selected_node = FEDrawnNodeData { number: 1, x: 0.0, y: 0.0, z: 0.0 };
        Self { props, link, state: State { selected_node } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideNodeMenu => self.show_hide_node_menu(),
            Msg::SelectNode(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_node) =>
                            {
                                if let Some(position) = self.props.nodes
                                    .iter()
                                    .position(|node|
                                        node.borrow().number.to_string() == select_node.value())
                                {
                                    let number = self.props.nodes[position].borrow().number;
                                    let x = self.props.nodes[position].borrow().coordinates.x;
                                    let y = self.props.nodes[position].borrow().coordinates.y;
                                    let z = self.props.nodes[position].borrow().coordinates.z;
                                    self.state.selected_node = FEDrawnNodeData { number, x, y, z };
                                }
                                else
                                {
                                    let number = select_node.value()
                                        .parse::<ElementsNumbers>().unwrap();
                                    let (x, y, z) = (0.0, 0.0, 0.0);
                                    self.state.selected_node = FEDrawnNodeData { number, x, y, z };
                                }
                            },
                        _ => (),
                    }
                },
            Msg::ApplyNodeDataChange =>
                {
                    self.state.selected_node.x =
                        self.read_inputted_coordinate(NODE_X_COORD);
                    self.state.selected_node.y =
                        self.read_inputted_coordinate(NODE_Y_COORD);
                    if let Some(analysis_type) = &self.props.analysis_type
                    {
                        match analysis_type
                        {
                            AnalysisType::ThreeDimensional =>
                                self.state.selected_node.z =
                                self.read_inputted_coordinate(NODE_Z_COORD),
                            _ => (),
                        }
                    }
                    let number = self.state.selected_node.number;
                    let x = self.state.selected_node.x;
                    let y = self.state.selected_node.y;
                    let z = self.state.selected_node.z;
                    if self.props.nodes.iter().position(|node|
                        node.borrow().number == number).is_none()
                    {
                        self.props.add_node.emit(FEDrawnNodeData { number, x, y, z });
                    }
                    else
                    {
                        self.props.update_node.emit(FEDrawnNodeData { number, x, y, z });
                    }
                },
            Msg::DeleteNode =>
                {
                    if self.props.nodes.iter().position(|node|
                        node.borrow().number == self.state.selected_node.number).is_some()
                    {
                        self.props.delete_node.emit(self.state.selected_node.number);
                    }
                },
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if (&self.props.is_preprocessor_active, &self.props.analysis_type) !=
            (&props.is_preprocessor_active, &props.analysis_type) ||
            !Rc::ptr_eq(&self.props.nodes, &props.nodes)
        {
            self.props = props;
            self.update_node_menu();
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
                    class={ PREPROCESSOR_BUTTON_CLASS }, onclick=self.link.callback(|_|
                        Msg::ShowHideNodeMenu),
                    disabled=
                        {
                            if self.props.analysis_type.is_some() &&
                                self.props.is_preprocessor_active
                            {
                                false
                            }
                            else
                            {
                                true
                            }
                        },
                >
                    { "Node" }
                </button>
                <div id = { NODE_MENU_ID } class={ NODE_MENU.to_owned() + " " + HIDDEN }>
                    <div class={ NODE_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <ul class={ NODE_MENU_INPUT_FIELDS_LIST_CLASS }>
                            <li>
                                {
                                    html!
                                    {
                                        <select
                                            id={ NODE_SELECT_ID },
                                            onchange=self.link.callback(|data: ChangeData|
                                                Msg::SelectNode(data)),
                                        >
                                            <option value={ self.state.selected_node.number }>
                                                { format!("{} New",
                                                    self.state.selected_node.number) }
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
                                            <p class={ NODE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                { "X coordinate:" }
                                            </p>
                                            <input
                                                id={ NODE_X_COORD },
                                                value={ self.state.selected_node.x },
                                                type="number",
                                            />
                                        </li>
                                        <li>
                                            <p class={ NODE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                { "Y coordinate:" }
                                            </p>
                                            <input
                                                id={ NODE_Y_COORD },
                                                value={ self.state.selected_node.y },
                                                type="number",
                                            />
                                        </li>
                                        {
                                            if let Some(analysis_type) = &self.props.analysis_type
                                            {
                                                match analysis_type
                                                {
                                                    AnalysisType::ThreeDimensional =>
                                                        {
                                                            html!
                                                            {
                                                                <li>
                                                                    <p class={ NODE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                                        { "Z coordinate:" }
                                                                    </p>
                                                                    <input
                                                                        id={ NODE_Z_COORD },
                                                                        value={ self.state.selected_node.z },
                                                                        type="number",
                                                                    />
                                                                </li>
                                                            }
                                                        },
                                                    AnalysisType::TwoDimensional => html! {},
                                                }
                                            }
                                            else
                                            {
                                                html! {}
                                            }
                                        }
                                    </>
                                }
                            }
                        </ul>
                    </div>
                    <div class={ NODE_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ NODE_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyNodeDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class={ NODE_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::DeleteNode),
                        >
                            { "Delete" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
