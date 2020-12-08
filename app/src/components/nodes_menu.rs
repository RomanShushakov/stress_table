use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;



use crate::fe::node::FeNode;
use crate::Coordinates;


const NODES_MENU_CONTAINER_ID: &str = "nodes_menu_container";
const NODES_MENU_CONTAINER: &str = "nodes_menu_container";
const HIDDEN: &str = "hidden";
const NODE_SELECT_ID: &str = "node_select";
const NODE_X_COORD: &str = "node_x_coord";
const NODE_Y_COORD: &str = "node_y_coord";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub nodes: Vec<FeNode<u16, f64>>,
    pub add_node: Callback<FeNode<u16, f64>>,
    pub update_node: Callback<(usize, FeNode<u16, f64>)>,
    pub remove_node: Callback<usize>,
}


struct State
{
    selected_node: FeNode<u16, f64>,
}


pub struct NodesMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideNodesMenu,
    SelectNode(ChangeData),
    ApplyNodeDataChange,
    RemoveNode,
}


impl NodesMenu
{
    fn show_hide_nodes_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(NODES_MENU_CONTAINER_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(NODES_MENU_CONTAINER);
        }
        else
        {
            element.set_class_name(&(NODES_MENU_CONTAINER.to_owned() + " " + HIDDEN));
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
                        option.set_value(&node.number.to_string());
                        option.set_text(&node.number.to_string());
                        options.set(i as u32, Some(&option)).unwrap();
                    }
                    if node.number > n
                    {
                        n = node.number;
                    }
                }
                n + 1
            };
        let (x, y, z) = (0.0, 0.0, 0.0);
        self.state.selected_node = FeNode { number, coordinates: Coordinates { x, y, z } };
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.nodes.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.nodes.len() as i32).unwrap();
    }


    fn read_inputted_coordinate(&self, input_field: &str) -> f64
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(coord) = input_element.value().parse::<f64>()
        {
            coord
        }
        else
        {
            0.0
        }
    }
}


impl Component for NodesMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let selected_node = FeNode { number: 1, coordinates: Coordinates { x: 0.0, y: 0.0, z: 0.0 } };
        Self { props, link, state: State { selected_node } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideNodesMenu => self.show_hide_nodes_menu(),
            Msg::SelectNode(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_node) =>
                            {
                                if let Some(position) = self.props.nodes
                                        .iter()
                                        .position(|node| node.number.to_string() == select_node.value())
                                {
                                    self.state.selected_node = self.props.nodes[position].to_owned();
                                }
                                else
                                {
                                    let number = select_node.value().parse::<u16>().unwrap();
                                    let (x, y, z) = (0.0, 0.0, 0.0);
                                    self.state.selected_node = FeNode { number, coordinates: Coordinates { x, y, z } };
                                }
                            },
                        _ => (),
                    }
                },
            Msg::ApplyNodeDataChange =>
                {
                    self.state.selected_node.coordinates.x = self.read_inputted_coordinate(NODE_X_COORD);
                    self.state.selected_node.coordinates.y = self.read_inputted_coordinate(NODE_Y_COORD);
                    if let Some(position) = self.props.nodes
                        .iter()
                        .position(|node| node.number == self.state.selected_node.number)
                    {
                        self.props.update_node.emit((position, self.state.selected_node.to_owned()));
                    }
                    else
                    {
                        self.props.add_node.emit(self.state.selected_node.to_owned());
                    }
                },
            Msg::RemoveNode =>
                {
                    if let Some(position) =
                    self.props.nodes
                        .iter()
                        .position(|node| node.number == self.state.selected_node.number)
                    {
                        self.props.remove_node.emit(position);
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
                    class="button" onclick=self.link.callback(|_| Msg::ShowHideNodesMenu)
                >
                    { "Nodes" }
                </button>
                <div id = { NODES_MENU_CONTAINER_ID } class={ NODES_MENU_CONTAINER.to_owned() + " " + HIDDEN }>
                    <div>
                        <ul class="nodes_menu">
                            <li>
                                {
                                    html!
                                    {
                                        <select
                                            id={ NODE_SELECT_ID },
                                            onchange=self.link.callback(|data: ChangeData| Msg::SelectNode(data)),
                                        >
                                            <option value={ self.state.selected_node.number }>
                                                { format!("{} New", self.state.selected_node.number) }
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
                                            <p>{ "x coordinate" }</p>
                                            <input
                                                id={ NODE_X_COORD },
                                                value={ self.state.selected_node.coordinates.x },
                                                type="number",
                                            />
                                        </li>
                                        <li>
                                            <p>{ "y coordinate" }</p>
                                            <input
                                                id={ NODE_Y_COORD },
                                                value={ self.state.selected_node.coordinates.y },
                                                type="number",
                                            />
                                        </li>
                                    </>
                                }

                            }
                        </ul>
                    </div>
                    <div>
                        <button
                            class="menu_button",
                            onclick=self.link.callback(|_| Msg::ApplyNodeDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class="menu_button",
                            onclick=self.link.callback(|_| Msg::RemoveNode),
                        >
                            { "Remove" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
