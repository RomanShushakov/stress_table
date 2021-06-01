use serde_json::Value;
use wasm_bindgen::prelude::*;

mod external_functions;
use external_functions::common::log;
use external_functions::communication_with_geometry::
{
    add_point_to_geometry, update_point_in_geometry,
    delete_point_from_geometry, restore_point_in_geometry,
    add_line_to_geometry, update_line_in_geometry,
    delete_line_from_geometry, restore_line_in_geometry,
    show_point_info, show_line_info_from_geometry,
    add_whole_geometry_to_preprocessor, clear_geometry_module_by_action_id
};


use external_functions::communication_with_properties::
{
    add_material_to_properties, update_material_in_properties,
    delete_material_from_properties, restore_material_in_properties,
    add_truss_section_to_properties, update_truss_section_in_properties,
    delete_truss_section_from_properties, restore_truss_section_in_properties,
    clear_properties_module_by_action_id,
};

mod action;
use action::{Action, Coordinates};
use action::{GeometryActionType, ActionType, PropertiesActionType};

mod methods_for_geometry_type_actions_handle;

mod methods_for_properties_type_actions_handle;


const ADD_POINT_MESSAGE_HEADER: &str = "add_point";
const UPDATE_POINT_MESSAGE_HEADER: &str = "update_point";
const DELETE_POINT_MESSAGE_HEADER: &str = "delete_point";
const ADD_LINE_MESSAGE_HEADER: &str = "add_line";
const UPDATE_LINE_MESSAGE_HEADER: &str = "update_line";
const DELETE_LINE_MESSAGE_HEADER: &str = "delete_line";

const ADD_MATERIAL_MESSAGE_HEADER: &str = "add_material";
const UPDATE_MATERIAL_MESSAGE_HEADER: &str = "update_material";
const DELETE_MATERIAL_MESSAGE_HEADER: &str = "delete_material";

const ADD_TRUSS_SECTION_MESSAGE_HEADER: &str = "add_truss_section";
const UPDATE_TRUSS_SECTION_MESSAGE_HEADER: &str = "update_truss_section";
const DELETE_TRUSS_SECTION_MESSAGE_HEADER: &str = "delete_truss_section";

const UNDO_MESSAGE_HEADER: &str = "undo";
const REDO_MESSAGE_HEADER: &str = "redo";

const SELECTED_POINT_NUMBER_MESSAGE_HEADER: &str = "selected_point_number";
const SELECTED_LINE_NUMBER_MESSAGE_HEADER: &str = "selected_line_number";

const CHANGE_VIEW_MESSAGE_HEADER: &str = "change_view";



#[wasm_bindgen]
pub struct ActionsRouter
{
    // ( action, is_action_id_should_be_added_to_active_actions )
    current_action: Option<(Action, bool)>,

    active_actions: Vec<Action>,
    undo_actions: Vec<Action>,
}


#[wasm_bindgen]
impl ActionsRouter
{
    pub fn create() -> ActionsRouter
    {
        let current_action = None;
        let active_actions = Vec::new();
        let undo_actions = Vec::new();
        ActionsRouter { current_action, active_actions, undo_actions }
    }


    fn handle_undo_message(&mut self, undo_data: &Value) -> Result<(), JsValue>
    {
        let action_id = undo_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Redo action: \
                Action id could not be converted to u32!")))?;
        if let Some(position) = self.active_actions.iter().rposition(|action|
            action.action_id_same(action_id))
        {
            let undo_action = self.active_actions.remove(position);
            match &undo_action.get_action_type()
            {
                ActionType::GeometryActionType(geometry_action_type) =>
                    {
                        match geometry_action_type
                        {
                            GeometryActionType::AddPoint(
                                point_number,
                                _coordinates,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::DeletePoint(*point_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::UpdatePoint(
                                point_number,
                                old_coordinates,
                                new_coordinates,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::UpdatePoint(*point_number,
                                            new_coordinates.clone(), old_coordinates.clone(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::DeletePoint(
                                point_number,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::RestorePoint(*point_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::RestorePoint(_, _) => (),
                            GeometryActionType::AddLine(
                                line_number,
                                _start_point_number,
                                _end_point_number,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::DeleteLine(*line_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::UpdateLine(
                                line_number,
                                old_start_point_number,
                                old_end_point_number,
                                new_start_point_number,
                                new_end_point_number,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::UpdateLine(*line_number,
                                            *new_start_point_number, *new_end_point_number,
                                            *old_start_point_number, *old_end_point_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::DeleteLine(
                                line_number,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::GeometryActionType(
                                        GeometryActionType::RestoreLine(*line_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::RestoreLine(_, _) => (),
                        }
                    },
                    ActionType::ShowPointInfo(_, _) => (),
                    ActionType::ShowLineInfo(_, _) => (),
                    ActionType::ChangeView(_, _) => (),
                ActionType::PropertiesActionType(properties_action_type) =>
                    {
                        match properties_action_type
                        {
                            PropertiesActionType::AddMaterial(
                                material_name,
                                _young_modulus,
                                _poisson_ratio,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::PropertiesActionType(
                                        PropertiesActionType::DeleteMaterial(material_name.clone(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::UpdateMaterial(
                                material_name,
                                old_young_modulus,
                                old_poisson_ratio,
                                new_young_modulus,
                                new_poisson_ratio,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::PropertiesActionType(
                                        PropertiesActionType::UpdateMaterial(material_name.clone(),
                                            *new_young_modulus, *new_poisson_ratio,
                                            *old_young_modulus, *old_poisson_ratio,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::DeleteMaterial(
                                material_name,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::PropertiesActionType(
                                        PropertiesActionType::RestoreMaterial(
                                            material_name.clone(), is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::RestoreMaterial(_, _) => (),
                            PropertiesActionType::AddTrussSection(
                                truss_section_name,
                                _area,
                                _area2,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::PropertiesActionType(
                                        PropertiesActionType::DeleteTrussSection(
                                            truss_section_name.clone(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::UpdateTrussSection(
                                truss_section_name,
                                old_area,
                                old_area2,
                                new_area,
                                new_area2,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::PropertiesActionType(
                                        PropertiesActionType::UpdateTrussSection(
                                            truss_section_name.clone(),
                                            *new_area, *new_area2,
                                            *old_area, *old_area2,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::DeleteTrussSection(
                                truss_section_name,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::PropertiesActionType(
                                        PropertiesActionType::RestoreTrussSection(
                                            truss_section_name.clone(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::RestoreTrussSection(_, _) => (),
                        }
                    }
            }
            self.undo_actions.push(undo_action);
        }
        Ok(())
    }


    fn handle_redo_message(&mut self, redo_data: &Value) -> Result<(), JsValue>
    {
        let action_id = redo_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Redo action: \
                Action id could not be converted to u32!")))?;
        if let Some(position) = self.undo_actions.iter().position(|action|
            action.action_id_same(action_id))
        {
            let redo_action = self.undo_actions.remove(position);
            let add_to_active_actions = true;
            self.current_action = Some((redo_action, add_to_active_actions));
        }
        Ok(())
    }


    fn handle_selected_point_number_message(&mut self, selected_point_number: &Value,
        show_object_info_handle: &js_sys::Function) -> Result<(), JsValue>
    {
        let point_number = selected_point_number.to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Show point info action: \
                Point number could not be converted to u32!")))?;
        let action_id = 0;
        let action_type = ActionType::ShowPointInfo(point_number, show_object_info_handle.clone());
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = false;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    fn handle_selected_line_number_message(&mut self, selected_line_number: &Value,
        show_object_info_handle: &js_sys::Function) -> Result<(), JsValue>
    {
        let line_number = selected_line_number.to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Show line info action: \
                Line number could not be converted to u32!")))?;
        let action_id = 0;
        let action_type = ActionType::ShowLineInfo(line_number, show_object_info_handle.clone());
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = false;
        self.current_action = Some((action, add_to_active_actions));
        Ok(())
    }


    fn handle_change_view_message(&mut self, view: &Value, change_view_handle: &js_sys::Function)
    {
        let selected_view = view["selectedView"].to_string();
        let action_id = 0;
        let action_type = ActionType::ChangeView(selected_view, change_view_handle.clone());
        let action = Action::create(action_id, action_type);
        let add_to_active_actions = false;
        self.current_action = Some((action, add_to_active_actions));
    }


    fn handle_current_action(&mut self) -> Result<(), JsValue>
    {
        if let Some((action, add_to_active_actions)) =
            &self.current_action
        {
            let action_id = action.get_action_id();
            let action_type = &action.get_action_type();
            match action_type
            {
                ActionType::GeometryActionType(geometry_action_type) =>
                    {
                        match geometry_action_type
                        {
                            GeometryActionType::AddPoint(
                                point_number,
                                coordinates,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_properties_module_by_action_id(action_id);
                                    let x = coordinates.get_x();
                                    let y = coordinates.get_y();
                                    let z = coordinates.get_z();
                                    add_point_to_geometry(action_id, *point_number, x, y, z,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            GeometryActionType::UpdatePoint(
                                point_number,
                                _old_coordinates,
                                new_coordinates,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_properties_module_by_action_id(action_id);
                                    let x = new_coordinates.get_x();
                                    let y = new_coordinates.get_y();
                                    let z = new_coordinates.get_z();
                                    update_point_in_geometry(action_id, *point_number, x, y, z,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            GeometryActionType::DeletePoint(
                                point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_properties_module_by_action_id(action_id);
                                    delete_point_from_geometry(action_id, *point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            GeometryActionType::RestorePoint(
                                point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_point_in_geometry(action_id, *point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            GeometryActionType::AddLine(
                                line_number,
                                start_point_number,
                                end_point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_properties_module_by_action_id(action_id);
                                    add_line_to_geometry(action_id, *line_number,
                                        *start_point_number, *end_point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            GeometryActionType::UpdateLine(
                                line_number,
                                _old_start_point_number,
                                _old_end_point_number,
                                new_start_point_number,
                                new_end_point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_properties_module_by_action_id(action_id);
                                    update_line_in_geometry(action_id, *line_number,
                                        *new_start_point_number, *new_end_point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                }
                            GeometryActionType::DeleteLine(
                                line_number,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_properties_module_by_action_id(action_id);
                                    delete_line_from_geometry(action_id, *line_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            GeometryActionType::RestoreLine(
                                line_number,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_line_in_geometry(action_id, *line_number,
                                                             *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                        }
                    },
                ActionType::ShowPointInfo(
                    point_number,
                    show_object_info_handle) =>
                    {
                        let point_info = show_point_info(*point_number)?;
                        let this = JsValue::null();
                        let _ = show_object_info_handle.call1(&this, &point_info)?;
                    },
                ActionType::ShowLineInfo(
                    line_number,
                    show_object_info_handle) =>
                    {
                        let line_info_from_geometry =
                            show_line_info_from_geometry(*line_number)?;
                        let this = JsValue::null();
                        let _ = show_object_info_handle.call1(&this, &line_info_from_geometry)?;
                    },
                ActionType::ChangeView(
                    selected_view_name,
                    change_view_handle) =>
                    {
                        let view_name = selected_view_name.clone();
                        let this = JsValue::null();
                        let _ = change_view_handle.call1(&this, &JsValue::from(view_name))?;
                    },
                ActionType::PropertiesActionType(properties_action_type) =>
                    {
                        match properties_action_type
                        {
                            PropertiesActionType::AddMaterial(
                                material_name,
                                young_modulus,
                                poisson_ratio,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_geometry_module_by_action_id(action_id);
                                    add_material_to_properties(action_id,
                                        material_name,
                                        *young_modulus, *poisson_ratio,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            PropertiesActionType::UpdateMaterial(
                                material_name,
                                _old_young_modulus,
                                _old_poisson_ratio,
                                new_young_modulus,
                                new_poisson_ratio,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_geometry_module_by_action_id(action_id);
                                    update_material_in_properties(action_id,
                                        material_name,
                                        *new_young_modulus, *new_poisson_ratio,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            PropertiesActionType::DeleteMaterial(
                                material_name,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_geometry_module_by_action_id(action_id);
                                    delete_material_from_properties(action_id,
                                        material_name,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            PropertiesActionType::RestoreMaterial(
                                material_name,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_material_in_properties(action_id,
                                                                   material_name,
                                                                   *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            PropertiesActionType::AddTrussSection(
                                truss_section_name,
                                area,
                                area2,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_geometry_module_by_action_id(action_id);
                                    add_truss_section_to_properties(action_id,
                                        truss_section_name,
                                        *area, *area2,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            PropertiesActionType::UpdateTrussSection(
                                truss_section_name,
                                _old_area,
                                _old_area2,
                                new_area,
                                new_area2,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_geometry_module_by_action_id(action_id);
                                    update_truss_section_in_properties(action_id,
                                        truss_section_name,
                                        *new_area, *new_area2,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            PropertiesActionType::DeleteTrussSection(
                                truss_section_name,
                                is_action_id_should_be_increased) =>
                                {
                                    clear_geometry_module_by_action_id(action_id);
                                    delete_truss_section_from_properties(action_id,
                                        truss_section_name,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                            PropertiesActionType::RestoreTrussSection(
                                truss_section_name,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_truss_section_in_properties(action_id,
                                        truss_section_name,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions == true
                                    {
                                        self.active_actions.push(action.clone());
                                    }
                                },
                        }
                    }
            }
            self.current_action = None;
        }
        Ok(())
    }


    pub fn handle_message(&mut self, message: JsValue, show_object_info_handle: &js_sys::Function,
        change_view_handle: &js_sys::Function) -> Result<(), JsValue>
    {
        let serialized_message: Value = message.into_serde().or(Err(JsValue::from(
            "Actions router: Message could not be serialized!")))?;
        if let Some(point_data) = serialized_message.get(ADD_POINT_MESSAGE_HEADER)
        {
            self.handle_add_point_message(&point_data)?;
        }
        else if let Some(point_data) = serialized_message
            .get(UPDATE_POINT_MESSAGE_HEADER)
        {
            self.handle_update_point_message(&point_data)?;
        }
        else if let Some(point_data) = serialized_message
            .get(DELETE_POINT_MESSAGE_HEADER)
        {
            self.handle_delete_point_message(&point_data)?;
        }
        else if let Some(line_data) = serialized_message.get(ADD_LINE_MESSAGE_HEADER)
        {
            self.handle_add_line_message(&line_data)?;
        }
        else if let Some(line_data) = serialized_message
            .get(UPDATE_LINE_MESSAGE_HEADER)
        {
            self.handle_update_line_message(&line_data)?;
        }
        else if let Some(line_data) = serialized_message
            .get(DELETE_LINE_MESSAGE_HEADER)
        {
            self.handle_delete_line_message(&line_data)?;
        }
        else if let Some(material_data) = serialized_message
            .get(ADD_MATERIAL_MESSAGE_HEADER)
        {
            self.handle_add_material_message(&material_data)?;
        }
        else if let Some(material_data) = serialized_message
            .get(UPDATE_MATERIAL_MESSAGE_HEADER)
        {
            self.handle_update_material_message(&material_data)?;
        }
        else if let Some(material_data) = serialized_message
            .get(DELETE_MATERIAL_MESSAGE_HEADER)
        {
            self.handle_delete_material_message(&material_data)?;
        }
        else if let Some(truss_section_data) = serialized_message
            .get(ADD_TRUSS_SECTION_MESSAGE_HEADER)
        {
            self.handle_add_truss_section_message(&truss_section_data)?;
        }
        else if let Some(truss_section_data) = serialized_message
            .get(UPDATE_TRUSS_SECTION_MESSAGE_HEADER)
        {
            self.handle_update_truss_section_message(&truss_section_data)?;
        }
        else if let Some(truss_section_data) = serialized_message
            .get(DELETE_TRUSS_SECTION_MESSAGE_HEADER)
        {
            self.handle_delete_truss_section_message(&truss_section_data)?;
        }
        else if let Some(undo_data) = serialized_message.get(UNDO_MESSAGE_HEADER)
        {
            self.handle_undo_message(&undo_data)?;
        }
        else if let Some(redo_data) = serialized_message.get(REDO_MESSAGE_HEADER)
        {
            self.handle_redo_message(&redo_data)?;
        }
        else if let Some(selected_point_number) =
            serialized_message.get(SELECTED_POINT_NUMBER_MESSAGE_HEADER)
        {
            self.handle_selected_point_number_message(&selected_point_number,
                show_object_info_handle)?;
        }
        else if let Some(selected_line_number) =
            serialized_message.get(SELECTED_LINE_NUMBER_MESSAGE_HEADER)
        {
            self.handle_selected_line_number_message(&selected_line_number,
                show_object_info_handle)?;
        }
        else if let Some(view) = serialized_message.get(CHANGE_VIEW_MESSAGE_HEADER)
        {
            self.handle_change_view_message(&view, change_view_handle);
        }
        else
        {
            let error_message = "Actions router: Message could not be handled!";
            return Err(JsValue::from(error_message));
        }
        self.handle_current_action()?;

        for action in &self.active_actions
        {
            let action_id = &action.get_action_id();
            let action_type = &action.get_action_type();
            log(&format!("Actions router active actions: Action id: {:?}, action type: {:?}",
                action_id, action_type));
        }
        log(&format!("Actions router: The number of active actions: {}",
            self.active_actions.len()));

        for action in &self.undo_actions
        {
            let action_id = &action.get_action_id();
            let action_type = &action.get_action_type();
            log(&format!("Actions router undo actions: Action id: {:?}, action type: {:?}",
                action_id, action_type));
        }
        log(&format!("Actions router: The number of undo actions: {}",
            self.undo_actions.len()));
        Ok(())
    }


    pub fn add_whole_geometry_to_preprocessor(&self)
    {
        let is_action_id_should_be_increased = false;
        add_whole_geometry_to_preprocessor(is_action_id_should_be_increased);
    }
}
