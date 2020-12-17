use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;

use crate::fe::fe_node::FeNode;
use crate::{Coordinates, AnalysisType, AuxDisplacement, AuxElement};


const DISPLACEMENT_MENU_ID: &str = "displacement_menu";
const DISPLACEMENT_MENU: &str = "displacement_menu";
const HIDDEN: &str = "hidden";
const DISPLACEMENT_SELECT_ID: &str = "displacement_select";
const NODE_NUMBER: &str = "node_number";
const DISPLACEMENT_IN_X_DIRECTION: &str = "displacement_x_direction";
const DISPLACEMENT_IN_Y_DIRECTION: &str = "displacement_y_direction";
const DISPLACEMENT_IN_Z_DIRECTION: &str = "displacement_z_direction";
const ROTATION_IN_XY_PLANE: &str = "rotation_in_xy_plane";
const ROTATION_IN_YZ_PLANE: &str = "rotation_in_yz_plane";
const ROTATION_IN_ZX_PLANE: &str = "rotation_in_zx_plane";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub analysis_type: Option<AnalysisType>,
    pub aux_elements: Vec<AuxElement>,
    pub aux_displacements: Vec<AuxDisplacement>,
    pub add_aux_displacement: Callback<AuxDisplacement>,
    pub update_aux_displacement: Callback<(usize, AuxDisplacement)>,
    pub remove_aux_displacement: Callback<usize>,
}


struct State
{
    selected_displacement: AuxDisplacement,

}


pub struct DisplacementMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideNodeMenu,
    SelectDisplacement(ChangeData),
    UpdateEditNodeNumber(String),
    SelectDisplacementXInputOption(ChangeData),
    SelectDisplacementYInputOption(ChangeData),
    SelectDisplacementZInputOption(ChangeData),
    SelectRotationXYInputOption(ChangeData),
    SelectRotationYZInputOption(ChangeData),
    SelectRotationZXInputOption(ChangeData),
    ApplyDisplacementDataChange,
    RemoveDisplacement,
}


impl DisplacementMenu
{
    fn show_hide_displacement_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(DISPLACEMENT_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(DISPLACEMENT_MENU);
        }
        else
        {
            element.set_class_name(&(DISPLACEMENT_MENU.to_owned() + " " + HIDDEN));
        }
    }


    fn update_numbers_in_displacement_menu(&mut self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(DISPLACEMENT_SELECT_ID).unwrap();
        let select = element.dyn_into::<HtmlSelectElement>()
            .map_err(|_| ())
            .unwrap();
        let options: HtmlOptionsCollection = select.options();
        options.set_length(self.props.aux_displacements.len() as u32 + 1);
        let number =
            {
                let mut n = 0;
                for (i, displacement) in self.props.aux_displacements.iter().enumerate()
                {
                    if let Ok(option) = HtmlOptionElement::new()
                    {
                        option.set_value(&displacement.number.to_string());
                        option.set_text(&displacement.number.to_string());
                        options.set(i as u32, Some(&option)).unwrap();
                    }
                    if displacement.number > n
                    {
                        n = displacement.number;
                    }
                }
                n + 1
            };
        self.state.selected_displacement = AuxDisplacement
            {
                number, node_number: 1u16,
                x_direction_value: None, y_direction_value: None, z_direction_value: None,
                xy_plane_value: None, yz_plane_value: None, zx_plane_value: None,
            };
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.aux_displacements.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.aux_displacements.len() as i32).unwrap();
    }


    fn read_inputted_displacement(&self, input_field: &str) -> f64
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


impl Component for DisplacementMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let selected_displacement = AuxDisplacement
            {
                number: 1u16, node_number: 1u16,
                x_direction_value: None, y_direction_value: None, z_direction_value: None,
                xy_plane_value: None, yz_plane_value: None, zx_plane_value: None,
            };
        Self { props, link, state: State { selected_displacement } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideNodeMenu => self.show_hide_displacement_menu(),
            Msg::SelectDisplacement(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_displacement) =>
                            {
                                if let Some(position) = self.props.aux_displacements
                                        .iter()
                                        .position(|displacement| displacement.number.to_string() == select_displacement.value())
                                {
                                    self.state.selected_displacement = self.props.aux_displacements[position].to_owned();
                                }
                                else
                                {
                                    let number = select_displacement.value().parse::<u16>().unwrap();
                                    self.state.selected_displacement = AuxDisplacement
                                        {
                                            number, node_number: 1u16,
                                            x_direction_value: None, y_direction_value: None, z_direction_value: None,
                                            xy_plane_value: None, yz_plane_value: None, zx_plane_value: None,
                                        };
                                }
                            },
                        _ => (),
                    }
                },
            Msg::UpdateEditNodeNumber(value) =>
                {
                    if let Ok(node_number) = value.parse::<u16>()
                    {
                        if node_number <= 0u16
                        {
                            yew::services::DialogService::alert(
                            "Node number cannot be less than 1.");
                            return false;
                        }
                        self.state.selected_displacement.node_number = node_number;
                    }
                    else
                    {
                        yew::services::DialogService::alert(
                            "You use incorrect node number input format.");
                        return false;
                    }
                },
            Msg::ApplyDisplacementDataChange =>
                {
                    self.state.selected_node.coordinates.x = self.read_inputted_displacement(DISPLACEMENT_IN_X_DIRECTION);
                    self.state.selected_node.coordinates.y = self.read_inputted_displacement(NODE_Y_COORD);
                    if let Some(analysis_type) = &self.props.analysis_type
                    {
                        match analysis_type
                        {
                            AnalysisType::ThreeDimensional => self.state.selected_node.coordinates.z =
                                self.read_inputted_displacement(NODE_Z_COORD),
                            _ => (),
                        }
                    }
                    if let None = self.props.nodes
                        .iter()
                        .position(|existed_node|
                            {
                                if let Some(analysis_type) = &self.props.analysis_type
                                {
                                    match analysis_type
                                    {
                                        AnalysisType::ThreeDimensional =>
                                            {
                                                (existed_node.coordinates.x == self.state.selected_node.coordinates.x) &&
                                                (existed_node.coordinates.y == self.state.selected_node.coordinates.y) &&
                                                (existed_node.coordinates.z == self.state.selected_node.coordinates.z)
                                            },
                                        AnalysisType::TwoDimensional =>
                                            {
                                                (existed_node.coordinates.x == self.state.selected_node.coordinates.x) &&
                                                (existed_node.coordinates.y == self.state.selected_node.coordinates.y)
                                            },

                                    }
                                }
                                else
                                {
                                    (existed_node.coordinates.x == self.state.selected_node.coordinates.x) &&
                                    (existed_node.coordinates.y == self.state.selected_node.coordinates.y)
                                }
                            }
                        )
                    {
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
                    }
                    else
                    {
                        yew::services::DialogService::alert(
                            "The node with the same coordinates is already in use.");
                    }
                },
            Msg::RemoveDisplacement =>
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
            self.update_numbers_in_displacement_menu();
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
                    class="button", onclick=self.link.callback(|_| Msg::ShowHideNodeMenu),
                    disabled={ if self.props.analysis_type.is_some() { false } else { true } },
                >
                    { "Node" }
                </button>
                <div id = { NODE_MENU_ID } class={ NODE_MENU.to_owned() + " " + HIDDEN }>
                    <div class="node_menu_input_fields">
                        <ul class="node_menu_input_fields_list">
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
                                            <p class="node_menu_input_fields_descriptions">
                                                { "X coordinate:" }
                                            </p>
                                            <input
                                                id={ NODE_X_COORD },
                                                value={ self.state.selected_node.coordinates.x },
                                                type="number",
                                            />
                                        </li>
                                        <li>
                                            <p class="node_menu_input_fields_descriptions">
                                                { "Y coordinate:" }
                                            </p>
                                            <input
                                                id={ NODE_Y_COORD },
                                                value={ self.state.selected_node.coordinates.y },
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
                                                                    <p class="node_menu_input_fields_descriptions">
                                                                        { "Z coordinate:" }
                                                                    </p>
                                                                    <input
                                                                        id={ NODE_Z_COORD },
                                                                        value={ self.state.selected_node.coordinates.z },
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
                    <div class="node_menu_buttons">
                        <button
                            class="node_menu_button",
                            onclick=self.link.callback(|_| Msg::ApplyNodeDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class="node_menu_button",
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
