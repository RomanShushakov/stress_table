use yew::prelude::*;
use web_sys::
    {
        HtmlSelectElement, HtmlOptionElement, HtmlOptionsCollection,
        DomTokenList, HtmlInputElement
    };
use wasm_bindgen::JsCast;

use crate::
    {
        AnalysisType, AuxDisplacement, AuxElement, ElementType,
        PREPROCESSOR_BUTTON_CLASS
    };
use crate::auxiliary::AuxDisplacementInputOption;


const DISPLACEMENT_MENU_ID: &str = "displacement_menu";
const DISPLACEMENT_MENU_CLASS: &str = "displacement_menu";
const DISPLACEMENT_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "displacement_menu_input_fields_container";
const DISPLACEMENT_MENU_INPUT_FIELDS_LIST_CLASS: &str = "displacement_menu_input_fields_list";
const DISPLACEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS: &str = "displacement_menu_input_fields_descriptions";
const DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS: &str = "displacement_input_field_container";
const HIDDEN: &str = "hidden";
const DISPLACEMENT_SELECT_ID: &str = "displacement_select";
const DISPLACEMENT_MENU_INPUT_FIELD_CLASS: &str = "displacement_menu_input_field";
const DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME: &str = "displacement_x_direction_input_name";
const DISPLACEMENT_IN_X_DIRECTION_VALUE: &str = "displacement_x_direction_value";
const DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME: &str = "displacement_y_direction_input_name";
const DISPLACEMENT_IN_Y_DIRECTION_VALUE: &str = "displacement_y_direction_value";
const DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME: &str = "displacement_z_direction_input_name";
const DISPLACEMENT_IN_Z_DIRECTION_VALUE: &str = "displacement_z_direction_value";
const ROTATION_IN_XY_PLANE_INPUT_NAME: &str = "rotation_in_xy_plane_input_name";
const ROTATION_IN_XY_PLANE_VALUE: &str = "rotation_in_xy_plane_value";
const ROTATION_IN_YZ_PLANE_INPUT_NAME: &str = "rotation_in_yz_plane_input_name";
const ROTATION_IN_YZ_PLANE_VALUE: &str = "rotation_in_yz_plane_value";
const ROTATION_IN_ZX_PLANE_INPUT_NAME: &str = "rotation_in_zx_plane_input_name";
const ROTATION_IN_ZX_PLANE_VALUE: &str = "rotation_in_zx_plane_value";
const DISPLACEMENT_MENU_BUTTONS_CONTAINER_CLASS: &str = "displacement_menu_buttons";
const DISPLACEMENT_MENU_BUTTON_CLASS: &str = "displacement_menu_button";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub analysis_type: Option<AnalysisType>,
    pub is_preprocessor_active: bool,
    pub aux_elements: Vec<AuxElement>,
    pub aux_displacements: Vec<AuxDisplacement>,
    pub add_aux_displacement: Callback<AuxDisplacement>,
    pub update_aux_displacement: Callback<(usize, AuxDisplacement)>,
    pub remove_aux_displacement: Callback<usize>,
}


struct State
{
    selected_displacement: AuxDisplacement,
    displacement_x_is_active: bool,
    displacement_y_is_active: bool,
    displacement_z_is_active: bool,
    rotation_xy_is_active: bool,
    rotation_yz_is_active: bool,
    rotation_zx_is_active: bool,
}


pub struct DisplacementMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideDisplacementMenu,
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
            element.set_class_name(DISPLACEMENT_MENU_CLASS);
        }
        else
        {
            element.set_class_name(&(DISPLACEMENT_MENU_CLASS.to_owned() + " " + HIDDEN));
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
                number,
                node_number: 1u16,
                is_rotation_stiffness_enabled: false,
                x_direction_value: None,
                y_direction_value: None,
                z_direction_value: None,
                xy_plane_value: None,
                yz_plane_value: None,
                zx_plane_value: None,
            };
        self.state.displacement_x_is_active = false;
        self.state.displacement_y_is_active = false;
        self.state.displacement_z_is_active = false;
        self.state.rotation_xy_is_active = false;
        self.state.rotation_yz_is_active = false;
        self.state.rotation_zx_is_active = false;
        if let Ok(option) = HtmlOptionElement::new()
        {
            option.set_value(&number.to_string());
            option.set_text(&format!("{} New", number));
            options.set(self.props.aux_displacements.len() as u32, Some(&option)).unwrap();
        }
        options.set_selected_index(self.props.aux_displacements.len() as i32).unwrap();
    }


    fn read_inputted_displacement(&self, input_field: &str) -> Option<f32>
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(input_field).unwrap();
        let input_element = element.dyn_into::<HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        if let Ok(displacement) = input_element.value().parse::<f32>()
        {
            Some(displacement)
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
                // ElementType::OtherType =>
                //     {
                //         if (element.node_1_number == node_number) ||
                //            (element.node_2_number == node_number)
                //         {
                //             rotational_stiffness_statuses.push(true);
                //         }
                //     }
            }
        }
        rotational_stiffness_statuses.iter().any(|status| *status == true)
    }


    fn check_inputted_data(&self) -> bool
    {
        self.state.selected_displacement.x_direction_value.is_some() ||
        self.state.selected_displacement.y_direction_value.is_some() ||
        self.state.selected_displacement.z_direction_value.is_some() ||
        self.state.selected_displacement.xy_plane_value.is_some() ||
        self.state.selected_displacement.yz_plane_value.is_some() ||
        self.state.selected_displacement.zx_plane_value.is_some()
    }
}


impl Component for DisplacementMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let selected_displacement =
            AuxDisplacement
            {
                number: 1u16,
                node_number: 1u16,
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
                selected_displacement,
                displacement_x_is_active: false,
                displacement_y_is_active: false,
                displacement_z_is_active: false,
                rotation_xy_is_active: false,
                rotation_yz_is_active: false,
                rotation_zx_is_active: false,
            }
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideDisplacementMenu => self.show_hide_displacement_menu(),
            Msg::SelectDisplacement(data) =>
                {
                    match data
                    {
                        ChangeData::Select(select_displacement) =>
                            {
                                if let Some(position) = self.props.aux_displacements
                                        .iter()
                                        .position(|displacement|
                                            displacement.number.to_string() == select_displacement.value())
                                {
                                    self.state.selected_displacement = self.props.aux_displacements[position].to_owned();
                                    if let Some(displacement_x) =
                                        self.props.aux_displacements[position].to_owned().x_direction_value
                                    {
                                        if displacement_x != 0f32
                                        {
                                            self.state.displacement_x_is_active = true;
                                        }
                                        else
                                        {
                                            self.state.displacement_x_is_active = false;
                                        }
                                    }
                                    else
                                    {
                                        self.state.displacement_x_is_active = false;
                                    }
                                    if let Some(displacement_y) =
                                        self.props.aux_displacements[position].to_owned().y_direction_value
                                    {
                                        if displacement_y != 0f32
                                        {
                                            self.state.displacement_y_is_active = true;
                                        }
                                        else
                                        {
                                            self.state.displacement_y_is_active = false;
                                        }
                                    }
                                    else
                                    {
                                        self.state.displacement_y_is_active = false;
                                    }
                                    if let Some(displacement_z) =
                                        self.props.aux_displacements[position].to_owned().z_direction_value
                                    {
                                        if displacement_z != 0f32
                                        {
                                            self.state.displacement_z_is_active = true;
                                        }
                                        else
                                        {
                                            self.state.displacement_z_is_active = false;
                                        }
                                    }
                                    else
                                    {
                                        self.state.displacement_z_is_active = false;
                                    }
                                    if let Some(rotation_xy) =
                                        self.props.aux_displacements[position].to_owned().xy_plane_value
                                    {
                                        if rotation_xy != 0f32
                                        {
                                            self.state.rotation_xy_is_active = true;
                                        }
                                        else
                                        {
                                            self.state.rotation_xy_is_active = false;
                                        }
                                    }
                                    else
                                    {
                                        self.state.rotation_xy_is_active = false;
                                    }
                                    if let Some(rotation_yz) =
                                        self.props.aux_displacements[position].to_owned().yz_plane_value
                                    {
                                        if rotation_yz != 0f32
                                        {
                                            self.state.rotation_yz_is_active = true;
                                        }
                                        else
                                        {
                                            self.state.rotation_yz_is_active = false;
                                        }
                                    }
                                    else
                                    {
                                        self.state.rotation_yz_is_active = false;
                                    }
                                    if let Some(rotation_zx) =
                                        self.props.aux_displacements[position].to_owned().zx_plane_value
                                    {
                                        if rotation_zx != 0f32
                                        {
                                            self.state.rotation_zx_is_active = true;
                                        }
                                        else
                                        {
                                            self.state.rotation_zx_is_active = false;
                                        }
                                    }
                                    else
                                    {
                                        self.state.rotation_zx_is_active = false;
                                    }
                                }
                                else
                                {
                                    let number = select_displacement.value().parse::<u16>().unwrap();
                                    self.state.selected_displacement = AuxDisplacement
                                        {
                                            number,
                                            node_number: 1u16,
                                            is_rotation_stiffness_enabled: self.check_rotation_stiffness(1u16),
                                            x_direction_value: None,
                                            y_direction_value: None,
                                            z_direction_value: None,
                                            xy_plane_value: None,
                                            yz_plane_value: None,
                                            zx_plane_value: None,
                                        };
                                    self.state.displacement_x_is_active = false;
                                    self.state.displacement_y_is_active = false;
                                    self.state.displacement_z_is_active = false;
                                    self.state.rotation_xy_is_active = false;
                                    self.state.rotation_yz_is_active = false;
                                    self.state.rotation_zx_is_active = false;
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
                                        // ElementType::OtherType =>
                                        //     {
                                        //         (element.node_1_number == node_number) ||
                                        //         (element.node_2_number == node_number)
                                        //     },
                                    }
                                })
                        {
                            yew::services::DialogService::alert(
                            "The selected node does not used in any element.");
                            return false;
                        }
                        if self.check_rotation_stiffness(node_number)
                        {
                            self.state.selected_displacement.is_rotation_stiffness_enabled = true;
                        }
                        else
                        {
                            self.state.selected_displacement.is_rotation_stiffness_enabled = false;
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
            Msg::SelectDisplacementXInputOption(data) =>
                {
                    match data
                    {
                        ChangeData::Value(x_input_option) =>
                            {
                                if x_input_option == AuxDisplacementInputOption::Free.as_str()
                                {
                                    self.state.displacement_x_is_active = false;
                                    self.state.selected_displacement.x_direction_value = None;
                                }
                                if x_input_option == AuxDisplacementInputOption::Restrained.as_str()
                                {
                                    self.state.displacement_x_is_active = false;
                                    self.state.selected_displacement.x_direction_value = Some(0f32);
                                }
                                if x_input_option == AuxDisplacementInputOption::Value.as_str()
                                {
                                    self.state.displacement_x_is_active = true;
                                }
                            },
                        _ => (),
                    }
                },
            Msg::SelectDisplacementYInputOption(data) =>
                {
                    match data
                    {
                        ChangeData::Value(y_input_option) =>
                            {
                                if y_input_option == AuxDisplacementInputOption::Free.as_str()
                                {
                                    self.state.displacement_y_is_active = false;
                                    self.state.selected_displacement.y_direction_value = None;
                                }
                                if y_input_option == AuxDisplacementInputOption::Restrained.as_str()
                                {
                                    self.state.displacement_y_is_active = false;
                                    self.state.selected_displacement.y_direction_value = Some(0f32);
                                }
                                if y_input_option == AuxDisplacementInputOption::Value.as_str()
                                {
                                    self.state.displacement_y_is_active = true;
                                }
                            }
                        _ => (),
                    }
                },
            Msg::SelectDisplacementZInputOption(data) =>
                {
                    match data
                    {
                        ChangeData::Value(z_input_option) =>
                            {
                                if z_input_option == AuxDisplacementInputOption::Free.as_str()
                                {
                                    self.state.displacement_z_is_active = false;
                                    self.state.selected_displacement.z_direction_value = None;
                                }
                                if z_input_option == AuxDisplacementInputOption::Restrained.as_str()
                                {
                                    self.state.displacement_z_is_active = false;
                                    self.state.selected_displacement.z_direction_value = Some(0f32);
                                }
                                if z_input_option == AuxDisplacementInputOption::Value.as_str()
                                {
                                    self.state.displacement_z_is_active = true;
                                }
                            }
                        _ => (),
                    }
                },
            Msg::SelectRotationXYInputOption(data) =>
                {
                    match data
                    {
                        ChangeData::Value(xy_input_option) =>
                            {
                                if xy_input_option == AuxDisplacementInputOption::Free.as_str()
                                {
                                    self.state.rotation_xy_is_active = false;
                                    self.state.selected_displacement.xy_plane_value = None;
                                }
                                if xy_input_option == AuxDisplacementInputOption::Restrained.as_str()
                                {
                                    self.state.rotation_xy_is_active = false;
                                    self.state.selected_displacement.xy_plane_value = Some(0f32);
                                }
                                if xy_input_option == AuxDisplacementInputOption::Value.as_str()
                                {
                                    self.state.rotation_xy_is_active = true;
                                }
                            }
                        _ => (),
                    }
                },
            Msg::SelectRotationYZInputOption(data) =>
                {
                    match data
                    {
                        ChangeData::Value(yz_input_option) =>
                            {
                                if yz_input_option == AuxDisplacementInputOption::Free.as_str()
                                {
                                    self.state.rotation_yz_is_active = false;
                                    self.state.selected_displacement.yz_plane_value = None;
                                }
                                if yz_input_option == AuxDisplacementInputOption::Restrained.as_str()
                                {
                                    self.state.rotation_yz_is_active = false;
                                    self.state.selected_displacement.yz_plane_value = Some(0f32);
                                }
                                if yz_input_option == AuxDisplacementInputOption::Value.as_str()
                                {
                                    self.state.rotation_yz_is_active = true;
                                }
                            }
                        _ => (),
                    }
                },
            Msg::SelectRotationZXInputOption(data) =>
                {
                    match data
                    {
                        ChangeData::Value(zx_input_option) =>
                            {
                                if zx_input_option == AuxDisplacementInputOption::Free.as_str()
                                {
                                    self.state.rotation_zx_is_active = false;
                                    self.state.selected_displacement.zx_plane_value = None;
                                }
                                if zx_input_option == AuxDisplacementInputOption::Restrained.as_str()
                                {
                                    self.state.rotation_zx_is_active = false;
                                    self.state.selected_displacement.zx_plane_value = Some(0f32);
                                }
                                if zx_input_option == AuxDisplacementInputOption::Value.as_str()
                                {
                                    self.state.rotation_zx_is_active = true;
                                }
                            }
                        _ => (),
                    }
                },
            Msg::ApplyDisplacementDataChange =>
                {
                    if let None = self.props.aux_displacements
                        .iter()
                        .position(|displacement|
                            {
                                (displacement.node_number == self.state.selected_displacement.node_number) &&
                                (displacement.number != self.state.selected_displacement.number)
                            })
                    {
                        if self.state.displacement_x_is_active
                        {
                            self.state.selected_displacement.x_direction_value =
                                self.read_inputted_displacement(DISPLACEMENT_IN_X_DIRECTION_VALUE);
                        }
                        if self.state.displacement_y_is_active
                        {
                            self.state.selected_displacement.y_direction_value =
                                self.read_inputted_displacement(DISPLACEMENT_IN_Y_DIRECTION_VALUE);
                        }
                        if self.state.displacement_z_is_active
                        {
                            self.state.selected_displacement.z_direction_value =
                                self.read_inputted_displacement(DISPLACEMENT_IN_Z_DIRECTION_VALUE);
                        }
                        if self.state.rotation_xy_is_active
                        {
                            self.state.selected_displacement.xy_plane_value =
                                self.read_inputted_displacement(ROTATION_IN_XY_PLANE_VALUE);
                        }
                        if self.state.rotation_yz_is_active
                        {
                            self.state.selected_displacement.yz_plane_value =
                                self.read_inputted_displacement(ROTATION_IN_YZ_PLANE_VALUE);
                        }
                       if self.state.rotation_zx_is_active
                        {
                            self.state.selected_displacement.zx_plane_value =
                                self.read_inputted_displacement(ROTATION_IN_ZX_PLANE_VALUE);
                        }

                        if !self.check_inputted_data()
                        {
                            yew::services::DialogService::alert(
                                "The some displacement value must be specified.");
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
                                                  (element.node_1_number == self.state.selected_displacement.node_number) ||
                                                  (element.node_2_number == self.state.selected_displacement.node_number)
                                              },
                                          // ElementType::OtherType =>
                                          //     {
                                          //         (element.node_1_number == self.state.selected_displacement.node_number) ||
                                          //         (element.node_2_number == self.state.selected_displacement.node_number)
                                          //     },
                                      }
                                  })
                        {
                            yew::services::DialogService::alert(
                                "The selected node does not used in any element.");
                            return false;
                        }
                        if let Some(position) = self.props.aux_displacements
                            .iter()
                            .position(|displacement| displacement.number == self.state.selected_displacement.number)
                        {

                            self.props.update_aux_displacement.emit((position, self.state.selected_displacement.to_owned()));
                        }
                        else
                        {
                            self.props.add_aux_displacement.emit(self.state.selected_displacement.to_owned());
                        }
                    }
                    else
                    {
                        yew::services::DialogService::alert(
                            "The displacement is already applied to the selected node.");
                    }
                },
            Msg::RemoveDisplacement =>
                {
                    if let Some(position) =
                    self.props.aux_displacements
                        .iter()
                        .position(|displacement| displacement.number == self.state.selected_displacement.number)
                    {
                        self.props.remove_aux_displacement.emit(position);
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
            if self.check_rotation_stiffness(self.state.selected_displacement.node_number)
            {
                self.state.selected_displacement.is_rotation_stiffness_enabled = true;
            }
            else
            {
                self.state.selected_displacement.is_rotation_stiffness_enabled = false;
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
                    onclick=self.link.callback(|_| Msg::ShowHideDisplacementMenu),
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
                    { "Displacement" }
                </button>
                <div id = { DISPLACEMENT_MENU_ID } class={ DISPLACEMENT_MENU_CLASS.to_owned() + " " + HIDDEN }>
                    <div class={ DISPLACEMENT_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <ul class={ DISPLACEMENT_MENU_INPUT_FIELDS_LIST_CLASS }>
                            <li>
                                {
                                    html!
                                    {
                                        <select
                                            id={ DISPLACEMENT_SELECT_ID },
                                            onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacement(data)),
                                        >
                                            <option value={ self.state.selected_displacement.number }>
                                                { format!("{} New", self.state.selected_displacement.number) }
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
                                            <p class={ DISPLACEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                { "Node number:" }
                                            </p>
                                            <input
                                                value={ self.state.selected_displacement.node_number },
                                                type="number",
                                                min = { 1 },
                                                oninput=self.link.callback(|d: InputData| Msg::UpdateEditNodeNumber(d.value))
                                            />
                                        </li>
                                        <li>
                                            <p class={ DISPLACEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                { "Displacement in the X direction:" }
                                            </p>
                                            <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                <input
                                                    class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                    onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacementXInputOption(data)),
                                                    type="radio", id={ DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME },
                                                    name={ DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME },
                                                    value={ AuxDisplacementInputOption::Free.as_str() },
                                                    checked={ self.state.selected_displacement.x_direction_value.is_none() },
                                                />
                                                <label for={ DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME }>
                                                    { AuxDisplacementInputOption::Free.as_str() }
                                                </label>
                                            </div>
                                            <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                <input
                                                    class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                    onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacementXInputOption(data)),
                                                    type="radio", id={ DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME },
                                                    name={ DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME },
                                                    value={ AuxDisplacementInputOption::Restrained.as_str() },
                                                    checked=
                                                        {
                                                            if let Some(value) = self.state.selected_displacement.x_direction_value
                                                            {
                                                                if value == 0f32
                                                                {
                                                                    true
                                                                }
                                                                else
                                                                {
                                                                    false
                                                                }
                                                            }
                                                            else
                                                            {
                                                                false
                                                            }
                                                        },
                                                />
                                                <label for={ DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME }>
                                                    { AuxDisplacementInputOption::Restrained.as_str() }
                                                </label>
                                            </div>
                                            <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                <input
                                                    class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                    onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacementXInputOption(data)),
                                                    type="radio", id={ DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME },
                                                    name={ DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME },
                                                    value={ AuxDisplacementInputOption::Value.as_str() },
                                                    checked={ self.state.displacement_x_is_active },
                                                />
                                                <label for={ DISPLACEMENT_IN_X_DIRECTION_INPUT_NAME }>
                                                    { AuxDisplacementInputOption::Value.as_str() }
                                                </label>
                                            </div>
                                            <input
                                                id={ DISPLACEMENT_IN_X_DIRECTION_VALUE },
                                                value=
                                                    {
                                                        if let Some(value) = self.state.selected_displacement.x_direction_value
                                                        {
                                                            value.to_string()
                                                        }
                                                        else
                                                        {
                                                            "".to_string()
                                                        }
                                                    },
                                                type="number",
                                                disabled=
                                                    {
                                                        !self.state.displacement_x_is_active
                                                    },
                                            />
                                        </li>
                                        <li>
                                            <p class={ DISPLACEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                { "Displacement in the Y direction:" }
                                            </p>
                                            <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                <input
                                                    class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                    onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacementYInputOption(data)),
                                                    type="radio", id={ DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME },
                                                    name={ DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME },
                                                    value={ AuxDisplacementInputOption::Free.as_str() },
                                                    checked={ self.state.selected_displacement.y_direction_value.is_none() },
                                                />
                                                <label for={ DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME }>
                                                    { AuxDisplacementInputOption::Free.as_str() }
                                                </label>
                                            </div>
                                            <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                <input
                                                    class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                    onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacementYInputOption(data)),
                                                    type="radio", id={ DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME },
                                                    name={ DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME },
                                                    value={ AuxDisplacementInputOption::Restrained.as_str() },
                                                    checked=
                                                        {
                                                            if let Some(value) = self.state.selected_displacement.y_direction_value
                                                            {
                                                                if value == 0f32
                                                                {
                                                                    true
                                                                }
                                                                else
                                                                {
                                                                    false
                                                                }
                                                            }
                                                            else
                                                            {
                                                                false
                                                            }
                                                        },
                                                />
                                                <label for={ DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME }>
                                                    { AuxDisplacementInputOption::Restrained.as_str() }
                                                </label>
                                            </div>
                                            <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                <input
                                                    class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                    onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacementYInputOption(data)),
                                                    type="radio", id={ DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME },
                                                    name={ DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME },
                                                    value={ AuxDisplacementInputOption::Value.as_str() },
                                                    checked={ self.state.displacement_y_is_active },
                                                />
                                                <label for={ DISPLACEMENT_IN_Y_DIRECTION_INPUT_NAME }>
                                                    { AuxDisplacementInputOption::Value.as_str() }
                                                </label>
                                            </div>
                                            <input
                                                id={ DISPLACEMENT_IN_Y_DIRECTION_VALUE },
                                                value=
                                                    {
                                                        if let Some(value) = self.state.selected_displacement.y_direction_value
                                                        {
                                                            value.to_string()
                                                        }
                                                        else
                                                        {
                                                            "".to_string()
                                                        }
                                                    },
                                                type="number",
                                                disabled=
                                                    {
                                                        !self.state.displacement_y_is_active
                                                    },
                                            />
                                        </li>
                                        {
                                            if let Some(analysis_type) = &self.props.analysis_type
                                            {
                                                match analysis_type
                                                {
                                                    AnalysisType::TwoDimensional =>
                                                        {
                                                            if self.state.selected_displacement.is_rotation_stiffness_enabled
                                                            {
                                                                html!
                                                                {
                                                                    <li>
                                                                        <p class={ DISPLACEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                                            { "Rotation in the XY plane:" }
                                                                        </p>
                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                            <input
                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationXYInputOption(data)),
                                                                                type="radio",
                                                                                id={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                name={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                value={ AuxDisplacementInputOption::Free.as_str() },
                                                                                checked={ self.state.selected_displacement.xy_plane_value.is_none() },
                                                                            />
                                                                            <label for={ ROTATION_IN_XY_PLANE_INPUT_NAME }>
                                                                                { AuxDisplacementInputOption::Free.as_str() }
                                                                            </label>
                                                                        </div>
                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                            <input
                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationXYInputOption(data)),
                                                                                type="radio",
                                                                                id={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                name={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                value={ AuxDisplacementInputOption::Restrained.as_str() },
                                                                                checked=
                                                                                    {
                                                                                        if let Some(value) = self.state.selected_displacement.xy_plane_value
                                                                                        {
                                                                                            if value == 0f32
                                                                                            {
                                                                                                true
                                                                                            }
                                                                                            else
                                                                                            {
                                                                                                false
                                                                                            }
                                                                                        }
                                                                                        else
                                                                                        {
                                                                                            false
                                                                                        }
                                                                                    },
                                                                            />
                                                                            <label for={ ROTATION_IN_XY_PLANE_INPUT_NAME }>
                                                                                { AuxDisplacementInputOption::Restrained.as_str() }
                                                                            </label>
                                                                        </div>
                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                            <input
                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationXYInputOption(data)),
                                                                                type="radio",
                                                                                id={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                name={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                value={ AuxDisplacementInputOption::Value.as_str() },
                                                                                checked={ self.state.rotation_xy_is_active },
                                                                            />
                                                                            <label for={ ROTATION_IN_XY_PLANE_INPUT_NAME }>
                                                                                { AuxDisplacementInputOption::Value.as_str() }
                                                                            </label>
                                                                        </div>
                                                                        <input
                                                                            id={ ROTATION_IN_XY_PLANE_VALUE },
                                                                            value=
                                                                                {
                                                                                    if let Some(value) = self.state.selected_displacement.xy_plane_value
                                                                                    {
                                                                                        value.to_string()
                                                                                    }
                                                                                    else
                                                                                    {
                                                                                        "".to_string()
                                                                                    }
                                                                                },
                                                                            type="number",
                                                                            disabled=
                                                                                {
                                                                                    !self.state.rotation_xy_is_active
                                                                                },
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
                                                                        <p class={ DISPLACEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                                            { "Displacement in the Z direction:" }
                                                                        </p>
                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                            <input
                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacementZInputOption(data)),
                                                                                type="radio",
                                                                                id={ DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME },
                                                                                name={ DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME },
                                                                                value={ AuxDisplacementInputOption::Free.as_str() },
                                                                                checked={ self.state.selected_displacement.z_direction_value.is_none() },
                                                                            />
                                                                            <label for={ DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME }>
                                                                                { AuxDisplacementInputOption::Free.as_str() }
                                                                            </label>
                                                                        </div>
                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                            <input
                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacementZInputOption(data)),
                                                                                type="radio",
                                                                                id={ DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME },
                                                                                name={ DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME },
                                                                                value={ AuxDisplacementInputOption::Restrained.as_str() },
                                                                                checked=
                                                                                    {
                                                                                        if let Some(value) = self.state.selected_displacement.z_direction_value
                                                                                        {
                                                                                            if value == 0f32
                                                                                            {
                                                                                                true
                                                                                            }
                                                                                            else
                                                                                            {
                                                                                                false
                                                                                            }
                                                                                        }
                                                                                        else
                                                                                        {
                                                                                            false
                                                                                        }
                                                                                    },
                                                                            />
                                                                            <label for={ DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME }>
                                                                                { AuxDisplacementInputOption::Restrained.as_str() }
                                                                            </label>
                                                                        </div>
                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                            <input
                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectDisplacementZInputOption(data)),
                                                                                type="radio",
                                                                                id={ DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME },
                                                                                name={ DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME },
                                                                                value={ AuxDisplacementInputOption::Value.as_str() },
                                                                                checked={ self.state.displacement_z_is_active },
                                                                            />
                                                                            <label for={ DISPLACEMENT_IN_Z_DIRECTION_INPUT_NAME }>
                                                                                { AuxDisplacementInputOption::Value.as_str() }
                                                                            </label>
                                                                        </div>
                                                                        <input
                                                                            id={ DISPLACEMENT_IN_Z_DIRECTION_VALUE },
                                                                            value=
                                                                                {
                                                                                    if let Some(value) = self.state.selected_displacement.z_direction_value
                                                                                    {
                                                                                        value.to_string()
                                                                                    }
                                                                                    else
                                                                                    {
                                                                                        "".to_string()
                                                                                    }
                                                                                },
                                                                            type="number",
                                                                            disabled=
                                                                                {
                                                                                    !self.state.displacement_z_is_active
                                                                                },
                                                                        />
                                                                    </li>
                                                                    {
                                                                        if self.state.selected_displacement.is_rotation_stiffness_enabled
                                                                        {
                                                                            html!
                                                                            {
                                                                                <>
                                                                                    <li>
                                                                                        <p class={ DISPLACEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                                                            { "Rotation in the XY plane:" }
                                                                                        </p>
                                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                                            <input
                                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationXYInputOption(data)),
                                                                                                type="radio",
                                                                                                id={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                                name={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                                value={ AuxDisplacementInputOption::Free.as_str() },
                                                                                                checked={ self.state.selected_displacement.xy_plane_value.is_none() },
                                                                                            />
                                                                                            <label for={ ROTATION_IN_XY_PLANE_INPUT_NAME }>
                                                                                                { AuxDisplacementInputOption::Free.as_str() }
                                                                                            </label>
                                                                                        </div>
                                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                                            <input
                                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationXYInputOption(data)),
                                                                                                type="radio",
                                                                                                id={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                                name={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                                value={ AuxDisplacementInputOption::Restrained.as_str() },
                                                                                                checked=
                                                                                                    {
                                                                                                        if let Some(value) = self.state.selected_displacement.xy_plane_value
                                                                                                        {
                                                                                                            if value == 0f32
                                                                                                            {
                                                                                                                true
                                                                                                            }
                                                                                                            else
                                                                                                            {
                                                                                                                false
                                                                                                            }
                                                                                                        }
                                                                                                        else
                                                                                                        {
                                                                                                            false
                                                                                                        }
                                                                                                    },
                                                                                            />
                                                                                            <label for={ ROTATION_IN_XY_PLANE_INPUT_NAME }>
                                                                                                { AuxDisplacementInputOption::Restrained.as_str() }
                                                                                            </label>
                                                                                        </div>
                                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                                            <input
                                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationXYInputOption(data)),
                                                                                                type="radio",
                                                                                                id={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                                name={ ROTATION_IN_XY_PLANE_INPUT_NAME },
                                                                                                value={ AuxDisplacementInputOption::Value.as_str() },
                                                                                                checked={ self.state.rotation_xy_is_active },
                                                                                            />
                                                                                            <label for={ ROTATION_IN_XY_PLANE_INPUT_NAME }>
                                                                                                { AuxDisplacementInputOption::Value.as_str() }
                                                                                            </label>
                                                                                        </div>
                                                                                        <input
                                                                                            id={ ROTATION_IN_XY_PLANE_VALUE },
                                                                                            value=
                                                                                                {
                                                                                                    if let Some(value) = self.state.selected_displacement.xy_plane_value
                                                                                                    {
                                                                                                        value.to_string()
                                                                                                    }
                                                                                                    else
                                                                                                    {
                                                                                                        "".to_string()
                                                                                                    }
                                                                                                },
                                                                                            type="number",
                                                                                            disabled=
                                                                                                {
                                                                                                    !self.state.rotation_xy_is_active
                                                                                                },
                                                                                        />
                                                                                    </li>
                                                                                    <li>
                                                                                        <p class={ DISPLACEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                                                            { "Rotation in the YZ plane:" }
                                                                                        </p>
                                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                                            <input
                                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationYZInputOption(data)),
                                                                                                type="radio",
                                                                                                id={ ROTATION_IN_YZ_PLANE_INPUT_NAME },
                                                                                                name={ ROTATION_IN_YZ_PLANE_INPUT_NAME },
                                                                                                value={ AuxDisplacementInputOption::Free.as_str() },
                                                                                                checked={ self.state.selected_displacement.yz_plane_value.is_none() },
                                                                                            />
                                                                                            <label for={ ROTATION_IN_YZ_PLANE_INPUT_NAME }>
                                                                                                { AuxDisplacementInputOption::Free.as_str() }
                                                                                            </label>
                                                                                        </div>
                                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                                            <input
                                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationYZInputOption(data)),
                                                                                                type="radio",
                                                                                                id={ ROTATION_IN_YZ_PLANE_INPUT_NAME },
                                                                                                name={ ROTATION_IN_YZ_PLANE_INPUT_NAME },
                                                                                                value={ AuxDisplacementInputOption::Restrained.as_str() },
                                                                                                checked=
                                                                                                    {
                                                                                                        if let Some(value) = self.state.selected_displacement.yz_plane_value
                                                                                                        {
                                                                                                            if value == 0f32
                                                                                                            {
                                                                                                                true
                                                                                                            }
                                                                                                            else
                                                                                                            {
                                                                                                                false
                                                                                                            }
                                                                                                        }
                                                                                                        else
                                                                                                        {
                                                                                                            false
                                                                                                        }
                                                                                                    },
                                                                                            />
                                                                                            <label for={ ROTATION_IN_YZ_PLANE_INPUT_NAME }>
                                                                                                { AuxDisplacementInputOption::Restrained.as_str() }
                                                                                            </label>
                                                                                        </div>
                                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                                            <input
                                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationYZInputOption(data)),
                                                                                                type="radio",
                                                                                                id={ ROTATION_IN_YZ_PLANE_INPUT_NAME },
                                                                                                name={ ROTATION_IN_YZ_PLANE_INPUT_NAME },
                                                                                                value={ AuxDisplacementInputOption::Value.as_str() },
                                                                                                checked={ self.state.rotation_yz_is_active },
                                                                                            />
                                                                                            <label for={ ROTATION_IN_YZ_PLANE_INPUT_NAME }>
                                                                                                { AuxDisplacementInputOption::Value.as_str() }
                                                                                            </label>
                                                                                        </div>
                                                                                        <input
                                                                                            id={ ROTATION_IN_YZ_PLANE_VALUE },
                                                                                            value=
                                                                                                {
                                                                                                    if let Some(value) = self.state.selected_displacement.yz_plane_value
                                                                                                    {
                                                                                                        value.to_string()
                                                                                                    }
                                                                                                    else
                                                                                                    {
                                                                                                        "".to_string()
                                                                                                    }
                                                                                                },
                                                                                            type="number",
                                                                                            disabled=
                                                                                                {
                                                                                                    !self.state.rotation_yz_is_active
                                                                                                },
                                                                                        />
                                                                                    </li>
                                                                                    <li>
                                                                                        <p class={ DISPLACEMENT_MENU_INPUT_FIELDS_DESCRIPTIONS_CLASS }>
                                                                                            { "Rotation in the ZX plane:" }
                                                                                        </p>
                                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                                            <input
                                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationZXInputOption(data)),
                                                                                                type="radio",
                                                                                                id={ ROTATION_IN_ZX_PLANE_INPUT_NAME },
                                                                                                name={ ROTATION_IN_ZX_PLANE_INPUT_NAME },
                                                                                                value={ AuxDisplacementInputOption::Free.as_str() },
                                                                                                checked={ self.state.selected_displacement.zx_plane_value.is_none() },
                                                                                            />
                                                                                            <label for={ ROTATION_IN_ZX_PLANE_INPUT_NAME }>
                                                                                                { AuxDisplacementInputOption::Free.as_str() }
                                                                                            </label>
                                                                                        </div>
                                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                                            <input
                                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationZXInputOption(data)),
                                                                                                type="radio",
                                                                                                id={ ROTATION_IN_ZX_PLANE_INPUT_NAME },
                                                                                                name={ ROTATION_IN_ZX_PLANE_INPUT_NAME },
                                                                                                value={ AuxDisplacementInputOption::Restrained.as_str() },
                                                                                                checked=
                                                                                                    {
                                                                                                        if let Some(value) = self.state.selected_displacement.zx_plane_value
                                                                                                        {
                                                                                                            if value == 0f32
                                                                                                            {
                                                                                                                true
                                                                                                            }
                                                                                                            else
                                                                                                            {
                                                                                                                false
                                                                                                            }
                                                                                                        }
                                                                                                        else
                                                                                                        {
                                                                                                            false
                                                                                                        }
                                                                                                    },
                                                                                            />
                                                                                            <label for={ ROTATION_IN_ZX_PLANE_INPUT_NAME }>
                                                                                                { AuxDisplacementInputOption::Restrained.as_str() }
                                                                                            </label>
                                                                                        </div>
                                                                                        <div class={ DISPLACEMENT_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                                                                                            <input
                                                                                                class={ DISPLACEMENT_MENU_INPUT_FIELD_CLASS },
                                                                                                onchange=self.link.callback(|data: ChangeData| Msg::SelectRotationZXInputOption(data)),
                                                                                                type="radio",
                                                                                                id={ ROTATION_IN_ZX_PLANE_INPUT_NAME },
                                                                                                name={ ROTATION_IN_ZX_PLANE_INPUT_NAME },
                                                                                                value={ AuxDisplacementInputOption::Value.as_str() },
                                                                                                checked={ self.state.rotation_zx_is_active },
                                                                                            />
                                                                                            <label for={ ROTATION_IN_ZX_PLANE_INPUT_NAME }>
                                                                                                { AuxDisplacementInputOption::Value.as_str() }
                                                                                            </label>
                                                                                        </div>
                                                                                        <input
                                                                                            id={ ROTATION_IN_ZX_PLANE_VALUE },
                                                                                            value=
                                                                                                {
                                                                                                    if let Some(value) = self.state.selected_displacement.zx_plane_value
                                                                                                    {
                                                                                                        value.to_string()
                                                                                                    }
                                                                                                    else
                                                                                                    {
                                                                                                        "".to_string()
                                                                                                    }
                                                                                                },
                                                                                            type="number",
                                                                                            disabled=
                                                                                                {
                                                                                                    !self.state.rotation_zx_is_active
                                                                                                },
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
                    <div class={ DISPLACEMENT_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ DISPLACEMENT_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyDisplacementDataChange),
                        >
                            { "Apply" }
                        </button>
                        <button
                            class={ DISPLACEMENT_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::RemoveDisplacement),
                        >
                            { "Remove" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
