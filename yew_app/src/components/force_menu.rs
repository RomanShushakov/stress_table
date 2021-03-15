use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;
use std::rc::Rc;

use crate::
    {
        FEDrawnElementData, DrawnBCData,
        ElementsNumbers, ElementsValues,
    };
use crate::pages::PREPROCESSOR_BUTTON_CLASS;
use crate::fem::{FEType, BCType};


const FORCE_MENU_ID: &str = "force_menu";
const FORCE_MENU_CLASS: &str = "force_menu";
const FORCE_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "force_menu_input_fields_container";
const FORCE_MENU_INPUT_FIELDS_LIST_CLASS: &str = "force_menu_input_fields_list";
const FORCE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS: &str = "force_menu_input_fields_descriptions";
const FORCE_MENU_BUTTONS_CONTAINER_CLASS: &str = "force_menu_buttons";
const FORCE_MENU_BUTTON_CLASS: &str = "force_menu_button";
const HIDDEN: &str = "hidden";
const FORCE_SELECT_ID: &str = "force_select";
const FORCE_IN_X_DIRECTION_VALUE: &str = "force_x_value";
const FORCE_IN_Y_DIRECTION_VALUE: &str = "force_y_value";
const FORCE_IN_Z_DIRECTION_VALUE: &str = "force_z_value";
const MOMENT_IN_XY_PLANE_VALUE: &str = "moment_in_xy_plane_value";
const MOMENT_IN_YZ_PLANE_VALUE: &str = "moment_in_yz_plane_value";
const MOMENT_IN_ZX_PLANE_VALUE: &str = "moment_in_zx_plane_value";


#[derive(Properties, Clone)]
pub struct Props
{
    pub is_preprocessor_active: bool,
    pub drawn_elements: Rc<Vec<FEDrawnElementData>>,
    pub drawn_bcs: Rc<Vec<DrawnBCData>>,
    pub add_bc: Callback<DrawnBCData>,
    pub update_bc: Callback<DrawnBCData>,
    pub delete_bc: Callback<DrawnBCData>,
    pub add_analysis_message: Callback<String>,
}


struct State
{
    selected_force: DrawnBCData,
}


pub struct ForceMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideForceMenu,
    SelectForce(ChangeData),
    UpdateEditNodeNumber(String),
    ApplyForceDataChange,
    DeleteForce,
}


impl ForceMenu
{
    fn show_hide_force_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(FORCE_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(FORCE_MENU_CLASS);
        }
        else
        {
            element.set_class_name(&(FORCE_MENU_CLASS.to_owned() + " " + HIDDEN));
        }
    }


    fn update_numbers_in_force_menu(&mut self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(FORCE_SELECT_ID).unwrap();
        let select = element.dyn_into::<HtmlSelectElement>()
            .map_err(|_| ())
            .unwrap();
        let options: HtmlOptionsCollection = select.options();
        options.set_length(self.props.drawn_bcs
            .iter()
            .filter(|bc| bc.bc_type == BCType::Force)
            .collect::<Vec<&DrawnBCData>>().len() as u32 + 1);
        let number =
            {
                let mut n = 0;
                for (i, bc) in self.props.drawn_bcs
                    .iter()
                    .filter(|bc| bc.bc_type == BCType::Force)
                    .enumerate()
                {
                    if let Ok(option) = HtmlOptionElement::new()
                    {
                        option.set_value(&bc.number.to_string());
                        option.set_text(&bc.number.to_string());
                        options.set(i as u32, Some(&option)).unwrap();
                    }
                    if bc.number > n
                    {
                        n = bc.number;
                    }
                }
                n + 1
            };
        self.state.selected_force = DrawnBCData
            {
                bc_type: BCType::Force,
                number: number as ElementsNumbers,
                node_number: 1 as ElementsNumbers,
                is_rotation_stiffness_enabled: false,
                x_direction_value: None,
                y_direction_value: None,
                z_direction_value: None,
                xy_plane_value: None,
                yz_plane_value: None,
                zx_plane_value: None,
            };
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.drawn_bcs
                .iter()
                .filter(|bc| bc.bc_type == BCType::Force)
                .collect::<Vec<&DrawnBCData>>().len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.drawn_bcs
            .iter()
            .filter(|bc| bc.bc_type == BCType::Force)
            .collect::<Vec<&DrawnBCData>>().len() as i32)
            .unwrap();
    }


    fn read_inputted_force(&self, input_field: &str) -> Option<ElementsValues>
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(force) = input_element.value().parse::<ElementsValues>()
        {
            Some(force)
        }
        else
        {
            None
        }
    }


    fn check_rotation_stiffness(&self, node_number: ElementsNumbers) -> bool
    {
        let mut rotational_stiffness_statuses = Vec::new();
        for element in self.props.drawn_elements.as_ref()
        {
            match element.fe_type
            {
                FEType::Truss2n2ip =>
                    {
                        if (element.nodes_numbers[0] == node_number) ||
                           (element.nodes_numbers[1] == node_number)
                        {
                            rotational_stiffness_statuses.push(false);
                        }
                    },
            }
        }
        rotational_stiffness_statuses.iter().any(|status| *status == true)
    }


    fn check_inputted_data(&self) -> bool
    {
        if let Some(force_x) = self.state.selected_force.x_direction_value
        {
            if force_x != 0.0 as ElementsValues
            {
                return true;
            }
        }
        if let Some(force_y) = self.state.selected_force.y_direction_value
        {
            if force_y != 0.0 as ElementsValues
            {
                return true;
            }
        }
        if let Some(force_z) = self.state.selected_force.z_direction_value
        {
            if force_z != 0.0 as ElementsValues
            {
                return true;
            }
        }
        if let Some(moment_xy) = self.state.selected_force.xy_plane_value
        {
            if moment_xy != 0.0 as ElementsValues
            {
                return true;
            }
        }
        if let Some(moment_yz) = self.state.selected_force.yz_plane_value
        {
            if moment_yz != 0.0 as ElementsValues
            {
                return true;
            }
        }
        if let Some(moment_zx) = self.state.selected_force.zx_plane_value
        {
            if moment_zx != 0.0 as ElementsValues
            {
                return true;
            }
        }
        false
    }
}


impl Component for ForceMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let selected_force =
            DrawnBCData
            {
                bc_type: BCType::Force,
                number: 1 as ElementsNumbers,
                node_number: 1 as ElementsNumbers,
                is_rotation_stiffness_enabled: false,
                x_direction_value: None,
                y_direction_value: None,
                z_direction_value: None,
                xy_plane_value: None,
                yz_plane_value: None,
                zx_plane_value: None,
            };
        Self
        {
            props, link, state:
            State
            {
                selected_force,
            }
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideForceMenu => self.show_hide_force_menu(),
            Msg::SelectForce(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_force) =>
                            {
                                if let Some(position) = self.props.drawn_bcs
                                    .iter()
                                    .position(|bc|
                                        bc.number.to_string() == select_force.value() &&
                                        bc.bc_type == BCType::Force)
                                {
                                    self.state.selected_force =
                                        self.props.drawn_bcs[position].to_owned();
                                }
                                else
                                {
                                    let bc_type = BCType::Force;
                                    let number = select_force.value()
                                        .parse::<ElementsNumbers>().unwrap();
                                    let node_number = 1 as ElementsNumbers;
                                    self.state.selected_force = DrawnBCData
                                        {
                                            bc_type,
                                            number,
                                            node_number,
                                            is_rotation_stiffness_enabled:
                                                self.check_rotation_stiffness(node_number),
                                            x_direction_value: None,
                                            y_direction_value: None,
                                            z_direction_value: None,
                                            xy_plane_value: None,
                                            yz_plane_value: None,
                                            zx_plane_value: None,
                                        };
                                }
                            }
                        _ => (),
                    }
                },
            Msg::UpdateEditNodeNumber(value) =>
                {
                    if let Ok(node_number) = value.parse::<ElementsNumbers>()
                    {
                        if node_number <= 0 as ElementsNumbers
                        {
                            self.props.add_analysis_message.emit("Node menu: Node \
                                number cannot be less than 1.".to_string());
                            return false;
                        }
                        if self.check_rotation_stiffness(node_number)
                        {
                            self.state.selected_force.is_rotation_stiffness_enabled = true;
                        }
                        else
                        {
                            self.state.selected_force.is_rotation_stiffness_enabled = false;
                        }
                        self.state.selected_force.node_number = node_number;
                    }
                    else
                    {
                        self.props.add_analysis_message.emit("Node menu: You use \
                            incorrect node number input format.".to_string());
                        return false;
                    }
                },
            Msg::ApplyForceDataChange =>
                {
                    if let None = self.props.drawn_bcs
                        .iter()
                        .position(|bc|
                            {
                                (bc.node_number == self.state.selected_force.node_number) &&
                                (bc.number != self.state.selected_force.number) &&
                                (bc.bc_type == self.state.selected_force.bc_type)
                            })
                    {
                        self.state.selected_force.x_direction_value =
                            self.read_inputted_force(FORCE_IN_X_DIRECTION_VALUE);
                        self.state.selected_force.y_direction_value =
                            self.read_inputted_force(FORCE_IN_Y_DIRECTION_VALUE);
                        if self.state.selected_force.is_rotation_stiffness_enabled
                        {
                            self.state.selected_force.xy_plane_value =
                                self.read_inputted_force(
                                    MOMENT_IN_XY_PLANE_VALUE);
                        }
                        self.state.selected_force.z_direction_value =
                            self.read_inputted_force(
                                FORCE_IN_Z_DIRECTION_VALUE);
                        if self.state.selected_force.is_rotation_stiffness_enabled
                        {
                            self.state.selected_force.xy_plane_value =
                                self.read_inputted_force(
                                    MOMENT_IN_XY_PLANE_VALUE);
                            self.state.selected_force.yz_plane_value =
                                self.read_inputted_force(
                                    MOMENT_IN_YZ_PLANE_VALUE);
                            self.state.selected_force.zx_plane_value =
                                self.read_inputted_force(
                                    MOMENT_IN_ZX_PLANE_VALUE);
                        }
                        if !self.check_inputted_data()
                        {
                            self.props.add_analysis_message.emit("Node menu: The some \
                                force value must be specified.".to_string());
                            return false;
                        }
                        if let Some(position) = self.props.drawn_bcs
                            .iter()
                            .position(|bc|
                                bc.number == self.state.selected_force.number &&
                                bc.bc_type == self.state.selected_force.bc_type)
                        {

                            self.props.update_bc.emit(self.state.selected_force.to_owned());
                        }
                        else
                        {
                            self.props.add_bc.emit(self.state.selected_force.to_owned());
                        }
                    }
                    else
                    {
                        self.props.add_analysis_message.emit("Node menu: The force is \
                            already applied to the selected node.".to_string());
                        return false;
                    }
                },
            Msg::DeleteForce =>
                {
                    if self.props.drawn_bcs
                        .iter()
                        .position(|bc|
                            bc.number == self.state.selected_force.number &&
                            bc.bc_type == self.state.selected_force.bc_type)
                        .is_some()
                    {
                        self.props.delete_bc.emit(self.state.selected_force.to_owned());
                    }
                },
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if &self.props.is_preprocessor_active != &props.is_preprocessor_active ||
            !Rc::ptr_eq(&self.props.drawn_elements, &props.drawn_elements) ||
            !Rc::ptr_eq(&self.props.drawn_bcs, &props.drawn_bcs)
        {
            self.props = props;
            self.update_numbers_in_force_menu();
            if self.check_rotation_stiffness(self.state.selected_force.node_number)
            {
                self.state.selected_force.is_rotation_stiffness_enabled = true;
            }
            else
            {
                self.state.selected_force.is_rotation_stiffness_enabled = false;
            }
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
                    class={ PREPROCESSOR_BUTTON_CLASS },
                        onclick=self.link.callback(|_| Msg::ShowHideForceMenu),
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
                    { "Force" }
                </button>
                <div id = { FORCE_MENU_ID } class={ FORCE_MENU_CLASS.to_owned() + " " + HIDDEN }>
                    <div class={ FORCE_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <ul class={ FORCE_MENU_INPUT_FIELDS_LIST_CLASS }>
                            <li>
                                {
                                    html!
                                    {
                                        <select
                                            id={ FORCE_SELECT_ID },
                                            onchange=self.link.callback(|data: ChangeData|
                                                Msg::SelectForce(data)),
                                        >
                                            <option value={ self.state.selected_force.number }>
                                                { format!("{} New",
                                                    self.state.selected_force.number) }
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
                                            <p class={ FORCE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                { "Node number:" }
                                            </p>
                                            <input
                                                value={ self.state.selected_force.node_number },
                                                type="number",
                                                min = { 1 },
                                                oninput=self.link.callback(|d: InputData|
                                                    Msg::UpdateEditNodeNumber(d.value))
                                            />
                                        </li>
                                        <li>
                                            <p class={ FORCE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                { "Force in the X direction:" }
                                            </p>
                                            <input
                                                id={ FORCE_IN_X_DIRECTION_VALUE },
                                                value=
                                                    {
                                                        if let Some(value) =
                                                            self.state.selected_force.x_direction_value
                                                        {
                                                            value.to_string()
                                                        }
                                                        else
                                                        {
                                                            "".to_string()
                                                        }
                                                    },
                                                type="number",
                                            />
                                        </li>
                                        <li>
                                            <p class={ FORCE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                { "Force in the Y direction:" }
                                            </p>
                                            <input
                                                id={ FORCE_IN_Y_DIRECTION_VALUE },
                                                value=
                                                    {
                                                        if let Some(value) = self.state.selected_force.y_direction_value
                                                        {
                                                            value.to_string()
                                                        }
                                                        else
                                                        {
                                                            "".to_string()
                                                        }
                                                    },
                                                type="number",
                                            />
                                        </li>
                                        <li>
                                            <p class={ FORCE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                { "Force in the Z direction:" }
                                            </p>
                                            <input
                                                id={ FORCE_IN_Z_DIRECTION_VALUE },
                                                value=
                                                    {
                                                        if let Some(value) = self.state.selected_force.z_direction_value
                                                        {
                                                            value.to_string()
                                                        }
                                                        else
                                                        {
                                                            "".to_string()
                                                        }
                                                    },
                                                type="number",
                                            />
                                        </li>
                                        {
                                            if self.state.selected_force.is_rotation_stiffness_enabled
                                            {
                                                html!
                                                {
                                                    <>
                                                        <li>
                                                            <p class={ FORCE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                                { "Moment in the XY plane:" }
                                                            </p>
                                                            <input
                                                                id={ MOMENT_IN_XY_PLANE_VALUE },
                                                                value=
                                                                    {
                                                                        if let Some(value) = self.state.selected_force.xy_plane_value
                                                                        {
                                                                            value.to_string()
                                                                        }
                                                                        else
                                                                        {
                                                                            "".to_string()
                                                                        }
                                                                    },
                                                                type="number",
                                                            />
                                                        </li>
                                                        <li>
                                                            <p class={ FORCE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                                { "MOMENT in the YZ plane:" }
                                                            </p>
                                                            <input
                                                                id={ MOMENT_IN_YZ_PLANE_VALUE },
                                                                value=
                                                                    {
                                                                        if let Some(value) = self.state.selected_force.yz_plane_value
                                                                        {
                                                                            value.to_string()
                                                                        }
                                                                        else
                                                                        {
                                                                            "".to_string()
                                                                        }
                                                                    },
                                                                type="number",
                                                            />
                                                        </li>
                                                        <li>
                                                            <p class={ FORCE_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                                { "Moment in the ZX plane:" }
                                                            </p>
                                                            <input
                                                                id={ MOMENT_IN_ZX_PLANE_VALUE },
                                                                value=
                                                                    {
                                                                        if let Some(value) = self.state.selected_force.zx_plane_value
                                                                        {
                                                                            value.to_string()
                                                                        }
                                                                        else
                                                                        {
                                                                            "".to_string()
                                                                        }
                                                                    },
                                                                type="number",
                                                            />
                                                        </li>
                                                    </>
                                                }
                                            }
                                            else
                                            {
                                                html! {  }
                                            }
                                        }
                                    </>
                                }
                            }
                        </ul>
                    </div>
                    <div class={ FORCE_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ FORCE_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyForceDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class={ FORCE_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::DeleteForce),
                        >
                            { "Delete" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
