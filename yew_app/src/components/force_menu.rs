use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;

use crate::fe::fe_node::FeNode;
use crate::{Coordinates, AnalysisType, AuxDisplacement, AuxElement, ElementType, AuxForce};
use crate::auxiliary::AuxDisplacementInputOption;


const FORCE_MENU_ID: &str = "force_menu";
const FORCE_MENU: &str = "force_menu";
const HIDDEN: &str = "hidden";
const FORCE_SELECT_ID: &str = "force_select";
const FORCE_IN_X_DIRECTION_VALUE: &str = "force_x_value";
const FORCE_IN_Y_DIRECTION_VALUE: &str = "force_y_value";
const FORCE_IN_Z_DIRECTION_VALUE: &str = "force_z_value";
const MOMENT_IN_XY_PLANE_VALUE: &str = "moment_in_xy_plane_value";
const MOMENT_IN_YZ_PLANE_VALUE: &str = "moment_in_yz_plane_value";
const MOMENT_IN_ZX_PLANE_VALUE: &str = "moment_in_zx_plane_value";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub analysis_type: Option<AnalysisType>,
    pub aux_elements: Vec<AuxElement>,
    pub aux_forces: Vec<AuxForce>,
    pub add_aux_force: Callback<AuxForce>,
    pub update_aux_force: Callback<(usize, AuxForce)>,
    pub remove_aux_force: Callback<usize>,
}


struct State
{
    selected_force: AuxForce,
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
    RemoveForce,
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
            element.set_class_name(FORCE_MENU);
        }
        else
        {
            element.set_class_name(&(FORCE_MENU.to_owned() + " " + HIDDEN));
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
        options.set_length(self.props.aux_forces.len() as u32 + 1);
        let number =
            {
                let mut n = 0;
                for (i, force) in self.props.aux_forces.iter().enumerate()
                {
                    if let Ok(option) = HtmlOptionElement::new()
                    {
                        option.set_value(&force.number.to_string());
                        option.set_text(&force.number.to_string());
                        options.set(i as u32, Some(&option)).unwrap();
                    }
                    if force.number > n
                    {
                        n = force.number;
                    }
                }
                n + 1
            };
        self.state.selected_force = AuxForce
            {
                number,
                node_number: 1u16,
                is_rotation_stiffness_enabled: false,
                force_x_value: None,
                force_y_value: None,
                force_z_value: None,
                moment_xy_value: None,
                moment_yz_value: None,
                moment_zx_value: None,
            };
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.aux_forces.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.aux_forces.len() as i32).unwrap();
    }


    fn read_inputted_force(&self, input_field: &str) -> Option<f32>
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(force) = input_element.value().parse::<f32>()
        {
            Some(force)
        }
        else
        {
            None
        }
    }


    fn check_rotation_stiffness(&self, node_number: u16) -> bool
    {
        let mut rotational_stiffness_statuses = Vec::new();
        for element in &self.props.aux_elements
        {
            match element.element_type
            {
                ElementType::Truss2n2ip =>
                    {
                        if (element.node_1_number == node_number) ||
                           (element.node_2_number == node_number)
                        {
                            rotational_stiffness_statuses.push(false);
                        }
                    },
                ElementType::OtherType =>
                    {
                        if (element.node_1_number == node_number) ||
                           (element.node_2_number == node_number)
                        {
                            rotational_stiffness_statuses.push(true);
                        }
                    }
            }
        }
        rotational_stiffness_statuses.iter().any(|status| *status == true)
    }


    fn check_inputted_data(&self) -> bool
    {
        if let Some(force_x) = self.state.selected_force.force_x_value
        {
            if force_x != 0f32
            {
                return true;
            }
        }
        if let Some(force_y) = self.state.selected_force.force_y_value
        {
            if force_y != 0f32
            {
                return true;
            }
        }
        if let Some(force_z) = self.state.selected_force.force_z_value
        {
            if force_z != 0f32
            {
                return true;
            }
        }
        if let Some(moment_xy) = self.state.selected_force.moment_xy_value
        {
            if moment_xy != 0f32
            {
                return true;
            }
        }
        if let Some(moment_yz) = self.state.selected_force.moment_yz_value
        {
            if moment_yz != 0f32
            {
                return true;
            }
        }
        if let Some(moment_zx) = self.state.selected_force.moment_zx_value
        {
            if moment_zx != 0f32
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
            AuxForce
            {
                number: 1u16,
                node_number: 1u16,
                is_rotation_stiffness_enabled: false,
                force_x_value: None,
                force_y_value: None,
                force_z_value: None,
                moment_xy_value: None,
                moment_yz_value: None,
                moment_zx_value: None,
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
                                if let Some(position) = self.props.aux_forces
                                        .iter()
                                        .position(|force|
                                            force.number.to_string() == select_force.value())
                                {
                                    self.state.selected_force = self.props.aux_forces[position].to_owned();
                                }
                                else
                                {
                                    let number = select_force.value().parse::<u16>().unwrap();
                                    self.state.selected_force = AuxForce
                                        {
                                            number,
                                            node_number: 1u16,
                                            is_rotation_stiffness_enabled: self.check_rotation_stiffness(1u16),
                                            force_x_value: None,
                                            force_y_value: None,
                                            force_z_value: None,
                                            moment_xy_value: None,
                                            moment_yz_value: None,
                                            moment_zx_value: None,
                                        };
                                }
                            }
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
                        if let None = self.props.aux_elements
                            .iter()
                            .position(|element|
                                {
                                    match element.element_type
                                    {
                                        ElementType::Truss2n2ip =>
                                            {
                                                (element.node_1_number == node_number) ||
                                                (element.node_2_number == node_number)
                                            },
                                        ElementType::OtherType =>
                                            {
                                                (element.node_1_number == node_number) ||
                                                (element.node_2_number == node_number)
                                            },
                                    }
                                })
                        {
                            yew::services::DialogService::alert(
                            "The selected node does not used in any element.");
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
                        yew::services::DialogService::alert(
                            "You use incorrect node number input format.");
                        return false;
                    }
                },
            Msg::ApplyForceDataChange =>
                {
                    if let None = self.props.aux_forces
                        .iter()
                        .position(|force|
                            {
                                (force.node_number == self.state.selected_force.node_number) &&
                                (force.number != self.state.selected_force.number)
                            })
                    {
                        self.state.selected_force.force_x_value =
                            self.read_inputted_force(FORCE_IN_X_DIRECTION_VALUE);
                        self.state.selected_force.force_y_value =
                            self.read_inputted_force(FORCE_IN_Y_DIRECTION_VALUE);
                        if let Some(analysis_type) = &self.props.analysis_type
                        {
                            match analysis_type
                            {
                                AnalysisType::TwoDimensional =>
                                    {
                                        if self.state.selected_force.is_rotation_stiffness_enabled
                                        {
                                            self.state.selected_force.moment_xy_value =
                                                self.read_inputted_force(MOMENT_IN_XY_PLANE_VALUE);
                                        }
                                    },
                                AnalysisType::ThreeDimensional =>
                                    {
                                        self.state.selected_force.force_z_value =
                                            self.read_inputted_force(FORCE_IN_Z_DIRECTION_VALUE);
                                        if self.state.selected_force.is_rotation_stiffness_enabled
                                        {
                                            self.state.selected_force.moment_xy_value =
                                                self.read_inputted_force(MOMENT_IN_XY_PLANE_VALUE);
                                            self.state.selected_force.moment_yz_value =
                                                self.read_inputted_force(MOMENT_IN_YZ_PLANE_VALUE);
                                            self.state.selected_force.moment_zx_value =
                                                self.read_inputted_force(MOMENT_IN_ZX_PLANE_VALUE);
                                        }
                                    }
                            }
                        }

                        if !self.check_inputted_data()
                        {
                            yew::services::DialogService::alert(
                                "The some force value must be specified.");
                            return false;
                        }

                        if let None = self.props.aux_elements
                            .iter()
                            .position(|element|
                                {
                                    match element.element_type
                                    {
                                        ElementType::Truss2n2ip =>
                                            {
                                                (element.node_1_number == self.state.selected_force.node_number) ||
                                                (element.node_2_number == self.state.selected_force.node_number)
                                            },
                                        ElementType::OtherType =>
                                            {
                                                (element.node_1_number == self.state.selected_force.node_number) ||
                                                (element.node_2_number == self.state.selected_force.node_number)
                                            },
                                    }
                                })
                        {
                            yew::services::DialogService::alert(
                                "The selected node does not used in any element.");
                            return false;
                        }
                        if let Some(position) = self.props.aux_forces
                            .iter()
                            .position(|force| force.number == self.state.selected_force.number)
                        {

                            self.props.update_aux_force.emit((position, self.state.selected_force.to_owned()));
                        }
                        else
                        {
                            self.props.add_aux_force.emit(self.state.selected_force.to_owned());
                        }
                    }
                    else
                    {
                        yew::services::DialogService::alert(
                            "The force is already applied to the selected node.");
                    }
                },
            Msg::RemoveForce =>
                {
                    if let Some(position) =
                    self.props.aux_forces
                        .iter()
                        .position(|force| force.number == self.state.selected_force.number)
                    {
                        self.props.remove_aux_force.emit(position);
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
                    class="button", onclick=self.link.callback(|_| Msg::ShowHideForceMenu),
                    disabled={ if self.props.analysis_type.is_some() { false } else { true } },
                >
                    { "Force" }
                </button>
                <div id = { FORCE_MENU_ID } class={ FORCE_MENU.to_owned() + " " + HIDDEN }>
                    <div class="force_menu_input_fields">
                        <ul class="force_menu_input_fields_list">
                            <li>
                                {
                                    html!
                                    {
                                        <select
                                            id={ FORCE_SELECT_ID },
                                            onchange=self.link.callback(|data: ChangeData| Msg::SelectForce(data)),
                                        >
                                            <option value={ self.state.selected_force.number }>
                                                { format!("{} New", self.state.selected_force.number) }
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
                                            <p class="force_menu_input_fields_descriptions">
                                                { "Node number:" }
                                            </p>
                                            <input
                                                value={ self.state.selected_force.node_number },
                                                type="number",
                                                min = { 1 },
                                                oninput=self.link.callback(|d: InputData| Msg::UpdateEditNodeNumber(d.value))
                                            />
                                        </li>
                                        <li>
                                            <p class="force_menu_input_fields_descriptions">
                                                { "Force in the X direction:" }
                                            </p>
                                            <input
                                                id={ FORCE_IN_X_DIRECTION_VALUE },
                                                value=
                                                    {
                                                        if let Some(value) = self.state.selected_force.force_x_value
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
                                            <p class="force_menu_input_fields_descriptions">
                                                { "Force in the Y direction:" }
                                            </p>
                                            <input
                                                id={ FORCE_IN_Y_DIRECTION_VALUE },
                                                value=
                                                    {
                                                        if let Some(value) = self.state.selected_force.force_y_value
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
                                            if let Some(analysis_type) = &self.props.analysis_type
                                            {
                                                match analysis_type
                                                {
                                                    AnalysisType::TwoDimensional =>
                                                        {
                                                            if self.state.selected_force.is_rotation_stiffness_enabled
                                                            {
                                                                html!
                                                                {
                                                                    <li>
                                                                        <p class="force_menu_input_fields_descriptions">
                                                                            { "Moment in the XY plane:" }
                                                                        </p>
                                                                        <input
                                                                            id={ MOMENT_IN_XY_PLANE_VALUE },
                                                                            value=
                                                                                {
                                                                                    if let Some(value) = self.state.selected_force.moment_xy_value
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
                                                                }
                                                            }
                                                            else
                                                            {
                                                                html! {  }
                                                            }
                                                        },
                                                    AnalysisType::ThreeDimensional =>
                                                        {
                                                            html!
                                                            {
                                                                <>
                                                                    <li>
                                                                        <p class="force_menu_input_fields_descriptions">
                                                                            { "Force in the Z direction:" }
                                                                        </p>
                                                                        <input
                                                                            id={ FORCE_IN_Z_DIRECTION_VALUE },
                                                                            value=
                                                                                {
                                                                                    if let Some(value) = self.state.selected_force.force_z_value
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
                                                                                        <p class="force_menu_input_fields_descriptions">
                                                                                            { "Moment in the XY plane:" }
                                                                                        </p>
                                                                                        <input
                                                                                            id={ MOMENT_IN_XY_PLANE_VALUE },
                                                                                            value=
                                                                                                {
                                                                                                    if let Some(value) = self.state.selected_force.moment_xy_value
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
                                                                                        <p class="force_menu_input_fields_descriptions">
                                                                                            { "MOMENT in the YZ plane:" }
                                                                                        </p>
                                                                                        <input
                                                                                            id={ MOMENT_IN_YZ_PLANE_VALUE },
                                                                                            value=
                                                                                                {
                                                                                                    if let Some(value) = self.state.selected_force.moment_yz_value
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
                                                                                        <p class="force_menu_input_fields_descriptions">
                                                                                            { "Moment in the ZX plane:" }
                                                                                        </p>
                                                                                        <input
                                                                                            id={ MOMENT_IN_ZX_PLANE_VALUE },
                                                                                            value=
                                                                                                {
                                                                                                    if let Some(value) = self.state.selected_force.moment_zx_value
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
                    <div class="force_menu_buttons">
                        <button
                            class="force_menu_button",
                            onclick=self.link.callback(|_| Msg::ApplyForceDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class="force_menu_button",
                            onclick=self.link.callback(|_| Msg::RemoveForce),
                        >
                            { "Remove" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
