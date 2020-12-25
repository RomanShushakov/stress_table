use yew::prelude::*;
use web_sys::DomTokenList;

use crate::{View, PREPROCESSOR_BUTTON_CLASS};


const VIEW_MENU_ID: &str = "view_menu";
const VIEW_MENU_CLASS: &str = "view_menu";
const VIEW_MENU_INPUT_FIELDS_CONTAINER_CLASS: &str = "view_menu_input_fields_container";
const VIEW_MENU_INPUT_FIELD_CONTAINER_CLASS: &str = "view_input_field_container";
const VIEW_MENU_BUTTONS_CONTAINER_CLASS: &str = "view_menu_buttons";
const VIEW_MENU_BUTTON_CLASS: &str = "view_menu_button";
const HIDDEN: &str = "hidden";
const VIEW_INPUT_NAME: &str = "view";
const PLANE_XY_VIEW_ID: &str = "plane_xy_view";
const PLANE_ZY_VIEW_ID: &str = "plane_zy_view";
const PLANE_XZ_VIEW_ID: &str = "plane_xz_view";
const ISOMETRIC_VIEW_ID: &str = "isometric_view";
const VIEW_MENU_INPUT_FIELD_CLASS: &str = "view_menu_input_field";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub view: View,
    pub change_view: Callback<View>,
}


struct State
{
    selected_view: View,
}


pub struct ViewMenu
{
    link: ComponentLink<Self>,
    props: Props,
    state: State,
}


pub enum Msg
{
    ShowHideViewMenu,
    SelectView(ChangeData),
    ApplyView,
}


impl ViewMenu
{
    fn show_hide_view_menu(&self)
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id(VIEW_MENU_ID).unwrap();
        let class_list: DomTokenList = element.class_list();
        if class_list.contains(HIDDEN)
        {
            element.set_class_name(VIEW_MENU_CLASS);
        }
        else
        {
            element.set_class_name(&(VIEW_MENU_CLASS.to_owned() + " " + HIDDEN));
        }
    }
}


impl Component for ViewMenu
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link, state: State { selected_view: View::PlaneXY } }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ShowHideViewMenu => self.show_hide_view_menu(),
            Msg::SelectView(data) =>
                {
                    match data
                    {
                        ChangeData::Value(view_select) =>
                            {
                                if view_select == View::PlaneXY.as_str()
                                {
                                    self.state.selected_view = View::PlaneXY;
                                }
                                if view_select == View::PlaneZY.as_str()
                                {
                                    self.state.selected_view = View::PlaneZY;
                                }
                                if view_select == View::PlaneXZ.as_str()
                                {
                                    self.state.selected_view = View::PlaneXZ;
                                }
                                if view_select == View::Isometric.as_str()
                                {
                                    self.state.selected_view = View::Isometric;
                                }
                                return false;
                            },
                        _ => (),
                    }
                },
            Msg::ApplyView =>
                {
                    self.props.change_view.emit(self.state.selected_view.to_owned());
                },
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
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
                    // disabled=true,
                    onclick=self.link.callback(|_| Msg::ShowHideViewMenu),
                >
                    { "View" }
                </button>
                <div id = { VIEW_MENU_ID } class={ VIEW_MENU_ID.to_owned() + " " + HIDDEN }>
                    <div class={ VIEW_MENU_INPUT_FIELDS_CONTAINER_CLASS }>
                        <div class={ VIEW_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ VIEW_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectView(data)),
                                type="radio", id={ PLANE_XY_VIEW_ID },
                                name={ VIEW_INPUT_NAME },
                                value={ View::PlaneXY.as_str() },
                                checked={ self.props.view == View::PlaneXY },
                            />
                            <label for={ PLANE_XY_VIEW_ID }>
                                { View::PlaneXY.as_str() }
                            </label>
                        </div>
                        <div class={ VIEW_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ VIEW_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectView(data)),
                                type="radio", id={ PLANE_ZY_VIEW_ID },
                                name={ VIEW_INPUT_NAME },
                                value={ View::PlaneZY.as_str() },
                                checked={ self.props.view == View::PlaneZY },
                            />
                            <label for={ PLANE_ZY_VIEW_ID }>
                                { View::PlaneZY.as_str() }
                            </label>
                        </div>
                        <div class={ VIEW_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ VIEW_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectView(data)),
                                type="radio", id={ PLANE_XZ_VIEW_ID },
                                name={ VIEW_INPUT_NAME },
                                value={ View::PlaneXZ.as_str() },
                                checked={ self.props.view == View::PlaneXZ },
                            />
                            <label for={ PLANE_XZ_VIEW_ID }>
                                { View::PlaneXZ.as_str() }
                            </label>
                        </div>
                        <div class={ VIEW_MENU_INPUT_FIELD_CONTAINER_CLASS }>
                            <input
                                class={ VIEW_MENU_INPUT_FIELD_CLASS },
                                onchange=self.link.callback(|data: ChangeData| Msg::SelectView(data)),
                                type="radio", id={ ISOMETRIC_VIEW_ID },
                                name={ VIEW_INPUT_NAME },
                                value={ View::Isometric.as_str() },
                                checked={ self.props.view == View::Isometric },
                            />
                            <label for={ ISOMETRIC_VIEW_ID }>
                                { View::Isometric.as_str() }
                            </label>
                        </div>
                    </div>
                    <div class={ VIEW_MENU_BUTTONS_CONTAINER_CLASS }>
                        <button
                            class={ VIEW_MENU_BUTTON_CLASS },
                            onclick=self.link.callback(|_| Msg::ApplyView),
                        >
                            { "Apply" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
