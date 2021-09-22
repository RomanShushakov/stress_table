use serde_json::Value;
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Request, RequestInit, Response};

mod external_functions;
use external_functions::common::log;
use external_functions::communication_with_geometry::
{
    add_point_to_geometry, update_point_in_geometry,
    delete_point_from_geometry, restore_point_in_geometry,
    add_line_to_geometry, update_line_in_geometry,
    delete_line_from_geometry, restore_line_in_geometry,
    show_point_info, show_line_info,
    extract_points, extract_lines,
};
use external_functions::communication_with_properties::
{
    add_material_to_properties, update_material_in_properties,
    delete_material_from_properties, restore_material_in_properties,
    add_truss_section_to_properties, update_truss_section_in_properties,
    delete_truss_section_from_properties, restore_truss_section_in_properties,
    add_beam_section_to_properties, update_beam_section_in_properties,
    delete_beam_section_from_properties, restore_beam_section_in_properties,
    add_properties_to_properties, update_properties_in_properties,
    delete_properties_from_properties, restore_properties_in_properties,
    add_assigned_properties_to_lines_to_properties, update_assigned_properties_to_lines_in_properties,
    delete_assigned_properties_to_lines_from_properties, restore_assigned_properties_to_lines_in_properties,
    add_beam_section_local_axis_1_direction_to_properties,
    remove_beam_section_local_axis_1_direction_from_properties,
    restore_beam_section_local_axis_1_direction_in_properties,
    update_beam_section_orientation_data_in_properties,
    extract_materials, extract_truss_sections, extract_beam_sections,
    extract_properties, extract_assigned_properties_to_lines,
    extract_beam_sections_local_axis_1_directions,
};
use external_functions::communication_with_loads::
{
    add_concentrated_load_to_loads, update_concentrated_load_in_loads,
    delete_concentrated_load_from_loads, restore_concentrated_load_in_loads,
    add_distributed_line_load_to_loads, update_distributed_line_load_in_loads,
    delete_distributed_line_load_from_loads, restore_distributed_line_load_in_loads,
    extract_concentrated_loads, extract_distributed_line_loads,
    show_concentrated_load_info, show_distributed_line_load_info,
};
use external_functions::communication_with_boundary_conditions::
{
    add_boundary_condition_to_boundary_conditions, update_boundary_condition_in_boundary_conditions,
    delete_boundary_condition_from_boundary_conditions,
    restore_boundary_condition_in_boundary_conditions, extract_boundary_conditions,
    show_boundary_condition_info,
};
use external_functions::communication_with_solver::
{
    submit_job,
};
use external_functions::communication_with_postprocessor::
{
    show_job_analysis_result, delete_job, extract_job_ids
};

mod action;
use action::
{
    Action, GeometryActionType, ActionType, PropertiesActionType, LoadsActionType, Coordinates,
    ConcentratedLoad, DistributedLineLoad, BoundaryCondition, BoundaryConditionsActionType,
};

mod types;
use types::{FEUInt};

mod consts;
use consts::
{
    ADD_POINT_MESSAGE_HEADER, UPDATE_POINT_MESSAGE_HEADER, DELETE_POINT_MESSAGE_HEADER,
    ADD_LINE_MESSAGE_HEADER, UPDATE_LINE_MESSAGE_HEADER, DELETE_LINE_MESSAGE_HEADER,
    ADD_MATERIAL_MESSAGE_HEADER, UPDATE_MATERIAL_MESSAGE_HEADER, DELETE_MATERIAL_MESSAGE_HEADER,
    ADD_TRUSS_SECTION_MESSAGE_HEADER, UPDATE_TRUSS_SECTION_MESSAGE_HEADER,
    DELETE_TRUSS_SECTION_MESSAGE_HEADER, ADD_BEAM_SECTION_MESSAGE_HEADER,
    UPDATE_BEAM_SECTION_MESSAGE_HEADER, DELETE_BEAM_SECTION_MESSAGE_HEADER,
    ADD_PROPERTIES_MESSAGE_HEADER, UPDATE_PROPERTIES_MESSAGE_HEADER,
    DELETE_PROPERTIES_MESSAGE_HEADER, ADD_ASSIGNED_PROPERTIES_TO_LINES_MESSAGE_HEADER,
    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_MESSAGE_HEADER,
    DELETE_ASSIGNED_PROPERTIES_TO_LINES_MESSAGE_HEADER,
    ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER,
    REMOVE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER,
    UPDATE_BEAM_SECTION_ORIENTATION_DATA_MESSAGE_HEADER,
    ADD_CONCENTRATED_LOAD_MESSAGE_HEADER, UPDATE_CONCENTRATED_LOAD_MESSAGE_HEADER,
    DELETE_CONCENTRATED_LOAD_MESSAGE_HEADER, ADD_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER,
    UPDATE_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER, DELETE_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER,
    ADD_BOUNDARY_CONDITION_MESSAGE_HEADER, UPDATE_BOUNDARY_CONDITION_MESSAGE_HEADER,
    DELETE_BOUNDARY_CONDITION_MESSAGE_HEADER, UNDO_MESSAGE_HEADER, REDO_MESSAGE_HEADER,
};

mod methods_for_geometry_type_actions_handle;

mod methods_for_properties_type_actions_handle;

mod methods_for_loads_type_actions_handle;

mod methods_for_boundary_condition_type_actions_handle;


async fn add_to_cache(message: JsValue) -> Result<(), JsValue>
{
    let msg =  js_sys::JSON::stringify(&message)?;
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&msg));
    let url = "/cache/update";
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request
        .headers()
        .set("Content-Type", "text/plain")?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    if resp.ok()
    {
        log("Cache updated");
        return Ok(());
    }
    Err(JsValue::from("Actions router: Update cache: Message could not be cached!"))
}


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
        ActionsRouter
        {
            current_action,
            active_actions,
            undo_actions,
        }
    }


    fn handle_undo_message(&mut self, undo_data: &Value) -> Result<(), JsValue>
    {
        let action_id = undo_data["actionId"].to_string()
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Redo action: \
                Action id could not be converted to FEUInt!")))?;
        if let Some(position) = self.active_actions.iter().rposition(|action|
            action.is_action_id_same(action_id))
        {
            let undo_action = self.active_actions.remove(position);
            match &undo_action.ref_action_type()
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
                                    let action_type = ActionType::from(
                                        GeometryActionType::DeletePoint(
                                            *point_number,
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
                                    let action_type = ActionType::from(
                                        GeometryActionType::UpdatePoint(
                                            *point_number,
                                            new_coordinates.to_owned(),
                                            old_coordinates.to_owned(),
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
                                    let action_type = ActionType::from(
                                        GeometryActionType::RestorePoint(
                                            *point_number,
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
                                    let action_type = ActionType::from(
                                        GeometryActionType::DeleteLine(
                                            *line_number,
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
                                    let action_type = ActionType::from(
                                        GeometryActionType::UpdateLine(
                                            *line_number,
                                            *new_start_point_number,
                                            *new_end_point_number,
                                            *old_start_point_number,
                                            *old_end_point_number,
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
                                    let action_type = ActionType::from(
                                        GeometryActionType::RestoreLine(
                                            *line_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            GeometryActionType::RestoreLine(_, _) => (),
                        }
                    },
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
                                    let action_type = ActionType::from(
                                        PropertiesActionType::DeleteMaterial(
                                            material_name.to_owned(),
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
                                    let action_type = ActionType::from(
                                        PropertiesActionType::UpdateMaterial(
                                            material_name.to_owned(),
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
                                    let action_type = ActionType::from(
                                        PropertiesActionType::RestoreMaterial(
                                            material_name.to_owned(),
                                            is_action_id_should_be_increased));
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
                                    let action_type = ActionType::from(
                                        PropertiesActionType::DeleteTrussSection(
                                            truss_section_name.to_owned(),
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
                                    let action_type = ActionType::from(
                                        PropertiesActionType::UpdateTrussSection(
                                            truss_section_name.to_owned(),
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
                                    let action_type = ActionType::from(
                                        PropertiesActionType::RestoreTrussSection(
                                            truss_section_name.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::RestoreTrussSection(_, _) => (),
                            PropertiesActionType::AddBeamSection(
                                beam_section_name,
                                _area,
                                _i11,
                                _i22,
                                _i12,
                                _it,
                                _shear_factor,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::DeleteBeamSection(
                                            beam_section_name.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::UpdateBeamSection(
                                beam_section_name,
                                old_area,
                                old_i11,
                                old_i22,
                                old_i12,
                                old_it,
                                old_shear_factor,
                                new_area,
                                new_i11,
                                new_i22,
                                new_i12,
                                new_it,
                                new_shear_factor,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::UpdateBeamSection(
                                            beam_section_name.to_owned(),
                                            *new_area, *new_i11,
                                            *new_i22, *new_i12,
                                            *new_it, *new_shear_factor,
                                            *old_area, *old_i11,
                                            *old_i22, *old_i12,
                                            *old_it, *old_shear_factor,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::DeleteBeamSection(
                                beam_section_name,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::RestoreBeamSection(
                                            beam_section_name.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::RestoreBeamSection(_, _) => (),
                            PropertiesActionType::AddProperties(
                                properties_name,
                                _material_name,
                                _cross_section_name,
                                _cross_section_type,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::DeleteProperties(
                                            properties_name.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::UpdateProperties(
                                properties_name,
                                old_material_name,
                                old_cross_section_name,
                                old_cross_section_type,
                                new_material_name,
                                new_cross_section_name,
                                new_cross_section_type,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::UpdateProperties(
                                            properties_name.to_owned(),
                                            new_material_name.to_owned(),
                                            new_cross_section_name.to_owned(),
                                            new_cross_section_type.to_owned(),
                                            old_material_name.to_owned(),
                                            old_cross_section_name.to_owned(),
                                            old_cross_section_type.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::DeleteProperties(
                                properties_name,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::RestoreProperties(
                                            properties_name.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::RestoreProperties(_, _) => (),
                            PropertiesActionType::AddAssignedPropertiesToLines(
                                assigned_properties_name,
                                _line_numbers,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::DeleteAssignedPropertiesToLines(
                                            assigned_properties_name.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::UpdateAssignedPropertiesToLines(
                                assigned_properties_name,
                                old_line_numbers,
                                new_line_numbers,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::UpdateAssignedPropertiesToLines(
                                            assigned_properties_name.to_owned(),
                                            new_line_numbers.to_owned(),
                                            old_line_numbers.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::DeleteAssignedPropertiesToLines(
                                assigned_properties_name,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::RestoreAssignedPropertiesToLines(
                                            assigned_properties_name.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::RestoreAssignedPropertiesToLines(_, _) => (),
                            PropertiesActionType::AddBeamSectionLocalAxis1Direction(
                                local_axis_1_direction,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::
                                            RemoveBeamSectionLocalAxis1Direction(
                                                local_axis_1_direction.to_owned(),
                                                is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::RemoveBeamSectionLocalAxis1Direction(
                                local_axis_1_direction,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::
                                            RestoreBeamSectionLocalAxis1Direction(
                                                local_axis_1_direction.to_owned(),
                                                is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            PropertiesActionType::RestoreBeamSectionLocalAxis1Direction(_, _) => (),
                            PropertiesActionType::UpdateBeamSectionOrientationData(
                                local_axis_1_direction,
                                old_line_numbers,
                                new_line_numbers,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        PropertiesActionType::
                                        UpdateBeamSectionOrientationData(
                                            local_axis_1_direction.to_owned(),
                                            new_line_numbers.to_owned(),
                                            old_line_numbers.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                        }
                    },
                ActionType::LoadsActionType(loads_action_type) =>
                    {
                        match loads_action_type
                        {
                            LoadsActionType::AddConcentratedLoad(
                                point_number,
                                _concentrated_load,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        LoadsActionType::DeleteConcentratedLoad(
                                            *point_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            LoadsActionType::UpdateConcentratedLoad(
                                point_number,
                                old_concentrated_load,
                                new_concentrated_load,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        LoadsActionType::UpdateConcentratedLoad(
                                            *point_number,
                                            new_concentrated_load.to_owned(),
                                            old_concentrated_load.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            LoadsActionType::DeleteConcentratedLoad(
                                point_number,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        LoadsActionType::RestoreConcentratedLoad(
                                            *point_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            LoadsActionType::RestoreConcentratedLoad(_, _) => (),
                            LoadsActionType::AddDistributedLineLoad(
                                line_number,
                                _distributed_line_load,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        LoadsActionType::DeleteDistributedLineLoad(
                                            *line_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            LoadsActionType::UpdateDistributedLineLoad(
                                line_number,
                                old_distributed_line_load,
                                new_distributed_line_load,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        LoadsActionType::UpdateDistributedLineLoad(
                                            *line_number,
                                            new_distributed_line_load.to_owned(),
                                            old_distributed_line_load.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            LoadsActionType::DeleteDistributedLineLoad(
                                line_number,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        LoadsActionType::RestoreDistributedLineLoad(
                                            *line_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            LoadsActionType::RestoreDistributedLineLoad(_, _) => (),
                        }
                    },
                ActionType::BoundaryConditionsActionType(boundary_conditions_action_type) =>
                    {
                        match boundary_conditions_action_type
                        {
                            BoundaryConditionsActionType::AddBoundaryCondition(
                                point_number,
                                _boundary_condition,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        BoundaryConditionsActionType::DeleteBoundaryCondition(
                                            *point_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            BoundaryConditionsActionType::UpdateBoundaryCondition(
                                point_number,
                                old_boundary_condition,
                                new_boundary_condition,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        BoundaryConditionsActionType::UpdateBoundaryCondition(
                                            *point_number,
                                            new_boundary_condition.to_owned(),
                                            old_boundary_condition.to_owned(),
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            BoundaryConditionsActionType::DeleteBoundaryCondition(
                                point_number,
                                _is_action_id_should_be_increased) =>
                                {
                                    let is_action_id_should_be_increased = false;
                                    let action_type = ActionType::from(
                                        BoundaryConditionsActionType::RestoreBoundaryCondition(
                                            *point_number,
                                            is_action_id_should_be_increased));
                                    let action = Action::create(action_id, action_type);
                                    let add_to_active_actions = false;
                                    self.current_action = Some((action, add_to_active_actions));
                                },
                            BoundaryConditionsActionType::RestoreBoundaryCondition(_, _) => (),
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
            .parse::<FEUInt>()
            .or(Err(JsValue::from("Actions router: Redo action: \
                Action id could not be converted to FEUInt!")))?;
        if let Some(position) = self.undo_actions.iter().position(|action|
            action.is_action_id_same(action_id))
        {
            let redo_action = self.undo_actions.remove(position);
            let add_to_active_actions = true;
            self.current_action = Some((redo_action, add_to_active_actions));
        }
        Ok(())
    }

    fn handle_current_action(&mut self) -> Result<(), JsValue>
    {
        if let Some((action, add_to_active_actions)) = &self.current_action
        {
            let ref_action_id = action.ref_action_id();
            let ref_action_type = action.ref_action_type();
            match ref_action_type
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
                                    let Coordinates { x, y, z } =
                                        coordinates;
                                    add_point_to_geometry(*ref_action_id, *point_number,
                                        *x, *y, *z, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            GeometryActionType::UpdatePoint(
                                point_number,
                                _old_coordinates,
                                new_coordinates,
                                is_action_id_should_be_increased) =>
                                {
                                    let Coordinates { x, y, z } =
                                        new_coordinates;
                                    update_point_in_geometry(*ref_action_id, *point_number,
                                        *x, *y, *z, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            GeometryActionType::DeletePoint(
                                point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_point_from_geometry(*ref_action_id, *point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            GeometryActionType::RestorePoint(
                                point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_point_in_geometry(*ref_action_id, *point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            GeometryActionType::AddLine(
                                line_number,
                                start_point_number,
                                end_point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    add_line_to_geometry(*ref_action_id, *line_number,
                                        *start_point_number, *end_point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
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
                                    update_line_in_geometry(*ref_action_id, *line_number,
                                        *new_start_point_number, *new_end_point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                }
                            GeometryActionType::DeleteLine(
                                line_number,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_line_from_geometry(*ref_action_id, *line_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            GeometryActionType::RestoreLine(
                                line_number,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_line_in_geometry(*ref_action_id, *line_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                        }
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
                                    add_material_to_properties(*ref_action_id,
                                        material_name, *young_modulus, *poisson_ratio,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
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
                                    update_material_in_properties(*ref_action_id,
                                        material_name, *new_young_modulus, *new_poisson_ratio,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::DeleteMaterial(
                                material_name,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_material_from_properties(*ref_action_id,
                                        material_name, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::RestoreMaterial(
                                material_name,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_material_in_properties(*ref_action_id,
                                        material_name, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::AddTrussSection(
                                truss_section_name,
                                area,
                                area2,
                                is_action_id_should_be_increased) =>
                                {
                                    add_truss_section_to_properties(*ref_action_id,
                                        truss_section_name, *area, *area2,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
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
                                    update_truss_section_in_properties(*ref_action_id,
                                        truss_section_name, *new_area, *new_area2,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::DeleteTrussSection(
                                truss_section_name,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_truss_section_from_properties(*ref_action_id,
                                        truss_section_name, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::RestoreTrussSection(
                                truss_section_name,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_truss_section_in_properties(*ref_action_id,
                                        truss_section_name, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::AddBeamSection(
                                beam_section_name,
                                area,
                                i11,
                                i22,
                                i12,
                                it,
                                shear_factor,
                                is_action_id_should_be_increased) =>
                                {
                                    add_beam_section_to_properties(*ref_action_id,
                                        beam_section_name, *area, *i11, *i22, *i12, *it,
                                        *shear_factor, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::UpdateBeamSection(
                                beam_section_name,
                                _old_area,
                                _old_i11,
                                _old_i22,
                                _old_i12,
                                _old_it,
                                _old_shear_factor,
                                new_area,
                                new_i11,
                                new_i22,
                                new_i12,
                                new_it,
                                new_shear_factor,
                                is_action_id_should_be_increased) =>
                                {
                                    update_beam_section_in_properties(*ref_action_id,
                                        beam_section_name, *new_area, *new_i11, *new_i22, *new_i12,
                                        *new_it, *new_shear_factor,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::DeleteBeamSection(
                                beam_section_name,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_beam_section_from_properties(*ref_action_id,
                                        beam_section_name, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::RestoreBeamSection(
                                beam_section_name,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_beam_section_in_properties(*ref_action_id,
                                        beam_section_name, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::AddProperties(
                                properties_name,
                                material_name,
                                cross_section_name,
                                cross_section_type,
                                is_action_id_should_be_increased) =>
                                {
                                    add_properties_to_properties(*ref_action_id, properties_name,
                                        material_name, cross_section_name, cross_section_type,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::UpdateProperties(
                                properties_name,
                                _old_material_name,
                                _old_cross_section_name,
                                _old_cross_section_type,
                                new_material_name,
                                new_cross_section_name,
                                new_cross_section_type,
                                is_action_id_should_be_increased) =>
                                {
                                    update_properties_in_properties(*ref_action_id,
                                        properties_name, new_material_name, new_cross_section_name,
                                        new_cross_section_type, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::DeleteProperties(
                                properties_name,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_properties_from_properties(*ref_action_id,
                                        properties_name, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::RestoreProperties(
                                properties_name,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_properties_in_properties(*ref_action_id,
                                        properties_name, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::AddAssignedPropertiesToLines(
                                properties_name,
                                line_numbers,
                                is_action_id_should_be_increased) =>
                                {
                                    add_assigned_properties_to_lines_to_properties(*ref_action_id,
                                        properties_name, line_numbers,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::UpdateAssignedPropertiesToLines(
                                properties_name,
                                _old_line_numbers,
                                new_line_numbers,
                                is_action_id_should_be_increased) =>
                                {
                                    update_assigned_properties_to_lines_in_properties(
                                        *ref_action_id, properties_name, new_line_numbers,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::DeleteAssignedPropertiesToLines(
                                properties_name,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_assigned_properties_to_lines_from_properties(
                                        *ref_action_id, properties_name,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::RestoreAssignedPropertiesToLines(
                                assigned_properties_name,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_assigned_properties_to_lines_in_properties(
                                        *ref_action_id, assigned_properties_name,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::AddBeamSectionLocalAxis1Direction(
                                local_axis_1_direction,
                                is_action_id_should_be_increased) =>
                                {
                                    add_beam_section_local_axis_1_direction_to_properties(
                                        *ref_action_id, local_axis_1_direction,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::RemoveBeamSectionLocalAxis1Direction(
                                local_axis_1_direction,
                                is_action_id_should_be_increased) =>
                                {
                                    remove_beam_section_local_axis_1_direction_from_properties(
                                        *ref_action_id, local_axis_1_direction,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::RestoreBeamSectionLocalAxis1Direction(
                                local_axis_1_direction,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_beam_section_local_axis_1_direction_in_properties(
                                        *ref_action_id, local_axis_1_direction,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            PropertiesActionType::UpdateBeamSectionOrientationData(
                                local_axis_1_direction,
                                _old_line_numbers,
                                new_line_numbers,
                                is_action_id_should_be_increased) =>
                                {
                                    update_beam_section_orientation_data_in_properties(
                                        *ref_action_id, local_axis_1_direction, new_line_numbers,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                        }
                    },
                ActionType::LoadsActionType(loads_action_type) =>
                    {
                        match loads_action_type
                        {
                            LoadsActionType::AddConcentratedLoad(
                                point_number,
                                concentrated_load,
                                is_action_id_should_be_increased) =>
                                {
                                    let ConcentratedLoad { fx, fy, fz,
                                        mx, my, mz } = concentrated_load;
                                    add_concentrated_load_to_loads(*ref_action_id, *point_number,
                                        *fx, *fy, *fz, *mx, *my, *mz,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            LoadsActionType::UpdateConcentratedLoad(
                                point_number,
                                _old_concentrated_load,
                                new_concentrated_load,
                                is_action_id_should_be_increased) =>
                                {
                                    let ConcentratedLoad { fx, fy, fz,
                                        mx, my, mz } =
                                            new_concentrated_load;
                                    update_concentrated_load_in_loads(*ref_action_id,
                                        *point_number, *fx, *fy, *fz, *mx, *my, *mz,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            LoadsActionType::DeleteConcentratedLoad(
                                point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_concentrated_load_from_loads(*ref_action_id,
                                        *point_number, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            LoadsActionType::RestoreConcentratedLoad(
                                point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_concentrated_load_in_loads(*ref_action_id,
                                        *point_number, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            LoadsActionType::AddDistributedLineLoad(
                                line_number,
                                distributed_line_load,
                                is_action_id_should_be_increased) =>
                                {
                                    let DistributedLineLoad { qx, qy,
                                        qz } = distributed_line_load;
                                    add_distributed_line_load_to_loads(*ref_action_id, *line_number,
                                        *qx, *qy, *qz, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            LoadsActionType::UpdateDistributedLineLoad(
                                line_number,
                                _old_distributed_line_load,
                                new_distributed_line_load,
                                is_action_id_should_be_increased) =>
                                {
                                    let DistributedLineLoad { qx, qy,
                                        qz } = new_distributed_line_load;
                                    update_distributed_line_load_in_loads(*ref_action_id,
                                        *line_number, *qx, *qy, *qz,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            LoadsActionType::DeleteDistributedLineLoad(
                                line_number,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_distributed_line_load_from_loads(*ref_action_id,
                                        *line_number, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            LoadsActionType::RestoreDistributedLineLoad(
                                line_number,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_distributed_line_load_in_loads(*ref_action_id,
                                        *line_number, *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                        }
                    },
                ActionType::BoundaryConditionsActionType(boundary_conditions_action_type) =>
                    {
                        match boundary_conditions_action_type
                        {
                            BoundaryConditionsActionType::AddBoundaryCondition(
                                point_number,
                                boundary_condition,
                                is_action_id_should_be_increased) =>
                                {
                                    let BoundaryCondition { optional_ux,
                                        optional_uy, optional_uz,
                                        optional_rx, optional_ry,
                                        optional_rz } = boundary_condition;
                                    add_boundary_condition_to_boundary_conditions(*ref_action_id,
                                        *point_number,
                                        *optional_ux, *optional_uy, *optional_uz,
                                        *optional_rx, *optional_ry, *optional_rz,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            BoundaryConditionsActionType::UpdateBoundaryCondition(
                                point_number,
                                _old_boundary_condition,
                                new_boundary_condition,
                                is_action_id_should_be_increased) =>
                                {
                                    let BoundaryCondition { optional_ux,
                                        optional_uy, optional_uz,
                                        optional_rx, optional_ry,
                                        optional_rz } = new_boundary_condition;
                                    update_boundary_condition_in_boundary_conditions(*ref_action_id,
                                        *point_number, *optional_ux, *optional_uy, *optional_uz,
                                        *optional_rx, *optional_ry, *optional_rz,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            BoundaryConditionsActionType::DeleteBoundaryCondition(
                                point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    delete_boundary_condition_from_boundary_conditions(
                                        *ref_action_id, *point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                            BoundaryConditionsActionType::RestoreBoundaryCondition(
                                point_number,
                                is_action_id_should_be_increased) =>
                                {
                                    restore_boundary_condition_in_boundary_conditions(
                                        *ref_action_id, *point_number,
                                        *is_action_id_should_be_increased)?;
                                    if *add_to_active_actions
                                    {
                                        self.active_actions.push(action.to_owned());
                                    }
                                },
                        }
                    }
            }
            self.current_action = None;
        }
        Ok(())
    }


    pub fn handle_message(&mut self, message: JsValue, to_cache: bool) -> Result<(), JsValue>
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
        else if let Some(beam_section_data) = serialized_message
            .get(ADD_BEAM_SECTION_MESSAGE_HEADER)
        {
            self.handle_add_beam_section_message(&beam_section_data)?;
        }
        else if let Some(beam_section_data) = serialized_message
            .get(UPDATE_BEAM_SECTION_MESSAGE_HEADER)
        {
            self.handle_update_beam_section_message(&beam_section_data)?;
        }
        else if let Some(beam_section_data) = serialized_message
            .get(DELETE_BEAM_SECTION_MESSAGE_HEADER)
        {
            self.handle_delete_beam_section_message(&beam_section_data)?;
        }
        else if let Some(properties_data) = serialized_message
            .get(ADD_PROPERTIES_MESSAGE_HEADER)
        {
            self.handle_add_properties_message(&properties_data)?;
        }
        else if let Some(properties_data) = serialized_message
            .get(UPDATE_PROPERTIES_MESSAGE_HEADER)
        {
            self.handle_update_properties_message(&properties_data)?;
        }
        else if let Some(properties_data) = serialized_message
            .get(DELETE_PROPERTIES_MESSAGE_HEADER)
        {
            self.handle_delete_properties_message(&properties_data)?;
        }
        else if let Some(assigned_properties_to_lines_data) = serialized_message
            .get(ADD_ASSIGNED_PROPERTIES_TO_LINES_MESSAGE_HEADER)
        {
            self.handle_add_assigned_properties_to_lines_message(&assigned_properties_to_lines_data)?;
        }
        else if let Some(assigned_properties_to_lines_data) = serialized_message
            .get(UPDATE_ASSIGNED_PROPERTIES_TO_LINES_MESSAGE_HEADER)
        {
            self.handle_update_assigned_properties_to_lines_message(&assigned_properties_to_lines_data)?;
        }
        else if let Some(assigned_properties_to_lines_data) = serialized_message
            .get(DELETE_ASSIGNED_PROPERTIES_TO_LINES_MESSAGE_HEADER)
        {
            self.handle_delete_assigned_properties_to_lines_message(&assigned_properties_to_lines_data)?;
        }
        else if let Some(local_axis_1_direction_data) = serialized_message
            .get(ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER)
        {
            self.handle_add_beam_section_local_axis_1_direction_message(
                &local_axis_1_direction_data)?;
        }
        else if let Some(local_axis_1_direction_data) = serialized_message
            .get(REMOVE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER)
        {
            self.handle_remove_beam_section_local_axis_1_direction_message(
                &local_axis_1_direction_data)?;
        }
        else if let Some(beam_section_orientation_data) = serialized_message
            .get(UPDATE_BEAM_SECTION_ORIENTATION_DATA_MESSAGE_HEADER)
        {
            self.handle_update_beam_section_orientation_data_message(
                &beam_section_orientation_data)?;
        }
        else if let Some(concentrated_load_data) = serialized_message.get(
            ADD_CONCENTRATED_LOAD_MESSAGE_HEADER)
        {
            self.handle_add_concentrated_load_message(&concentrated_load_data)?;
        }
        else if let Some(concentrated_load_data) = serialized_message.get(
            UPDATE_CONCENTRATED_LOAD_MESSAGE_HEADER)
        {
            self.handle_update_concentrated_load_message(&concentrated_load_data)?;
        }
        else if let Some(concentrated_load_data) = serialized_message.get(
            DELETE_CONCENTRATED_LOAD_MESSAGE_HEADER)
        {
            self.handle_delete_concentrated_load_message(&concentrated_load_data)?;
        }
        else if let Some(distributed_line_load_data) = serialized_message.get(
            ADD_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER)
        {
            self.handle_add_distributed_line_load_message(&distributed_line_load_data)?;
        }
        else if let Some(distributed_line_load_data) = serialized_message.get(
            UPDATE_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER)
        {
            self.handle_update_distributed_line_load_message(&distributed_line_load_data)?;
        }
        else if let Some(distributed_line_load_data) = serialized_message.get(
            DELETE_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER)
        {
            self.handle_delete_distributed_line_load_message(&distributed_line_load_data)?;
        }
        else if let Some(boundary_condition_data) = serialized_message.get(
            ADD_BOUNDARY_CONDITION_MESSAGE_HEADER)
        {
            self.handle_add_boundary_condition_message(&boundary_condition_data)?;
        }
        else if let Some(boundary_condition_data) = serialized_message.get(
            UPDATE_BOUNDARY_CONDITION_MESSAGE_HEADER)
        {
            self.handle_update_boundary_condition_message(&boundary_condition_data)?;
        }
        else if let Some(boundary_condition_data) = serialized_message.get(
            DELETE_BOUNDARY_CONDITION_MESSAGE_HEADER)
        {
            self.handle_delete_boundary_condition_message(&boundary_condition_data)?;
        }
        else if let Some(undo_data) = serialized_message.get(UNDO_MESSAGE_HEADER)
        {
            self.handle_undo_message(&undo_data)?;
        }
        else if let Some(redo_data) = serialized_message.get(REDO_MESSAGE_HEADER)
        {
            self.handle_redo_message(&redo_data)?;
        }
        else
        {
            let error_message = "Actions router: Message could not be handled!";
            return Err(JsValue::from(error_message));
        }
        self.handle_current_action()?;

        // if to_cache
        // {
        //     spawn_local(async
        //     {
        //         add_to_cache(message).await.unwrap_throw();
        //     });
        // }

        for action in &self.active_actions
        {
            let action_id = &action.ref_action_id();
            let action_type = &action.ref_action_type();
            log(&format!("Actions router active actions: \n
                Action id: {:?}, action type: {:?} \n",
                action_id, action_type));
        }
        log(&format!("Actions router: The number of active actions: {}",
            self.active_actions.len()));

        for action in &self.undo_actions
        {
            let action_id = &action.ref_action_id();
            let action_type = &action.ref_action_type();
            log(&format!("Actions router undo actions: \n
                Action id: {:?}, action type: {:?} \n",
                action_id, action_type));
        }
        log(&format!("Actions router: The number of undo actions: {}",
            self.undo_actions.len()));

        Ok(())
    }


    pub fn submit_job(&self, job_name: &str) -> Result<(), JsValue>
    {
        submit_job(job_name)
    }


    pub fn show_job_analysis_result(&self, job_name: &str, job_id: u32) -> Result<(), JsValue>
    {
        show_job_analysis_result(job_name, job_id)
    }


    pub fn delete_job(&self, job_name: &str) -> Result<(), JsValue>
    {
        delete_job(job_name)
    }


    pub fn extract_points(&self, handler: js_sys::Function)
    {
        extract_points(handler);
    }


    pub fn extract_lines(&self, handler: js_sys::Function)
    {
        extract_lines(handler);
    }


    pub fn extract_materials(&self, handler: js_sys::Function)
    {
        extract_materials(handler);
    }


    pub fn extract_truss_sections(&self, handler: js_sys::Function)
    {
        extract_truss_sections(handler);
    }


    pub fn extract_beam_sections(&self, handler: js_sys::Function)
    {
        extract_beam_sections(handler);
    }


    pub fn extract_properties(&self, handler: js_sys::Function)
    {
        extract_properties(handler);
    }


    pub fn extract_assigned_properties_to_lines(&self, handler: js_sys::Function)
    {
        extract_assigned_properties_to_lines(handler);
    }


    pub fn extract_beam_sections_local_axis_1_directions(&self, handler: js_sys::Function)
    {
        extract_beam_sections_local_axis_1_directions(handler);
    }


    pub fn extract_concentrated_loads(&self, handler: js_sys::Function)
    {
        extract_concentrated_loads(handler);
    }


    pub fn extract_distributed_line_loads(&self, handler: js_sys::Function)
    {
        extract_distributed_line_loads(handler);
    }


    pub fn extract_boundary_conditions(&self, handler: js_sys::Function)
    {
        extract_boundary_conditions(handler);
    }


    pub fn extract_job_ids(&self, handler: js_sys::Function)
    {
        extract_job_ids(handler);
    }


    pub fn show_boundary_condition_info(&self, point_number: FEUInt, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        show_boundary_condition_info(point_number, handler)
    }


    pub fn show_point_info(&self, number: FEUInt, handler: js_sys::Function) -> Result<(), JsValue>
    {
        show_point_info(number, handler)
    }


    pub fn show_line_info(&self, number: FEUInt, handler: js_sys::Function) -> Result<(), JsValue>
    {
        show_line_info(number, handler)
    }


    pub fn show_concentrated_load_info(&self, point_number: FEUInt, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        show_concentrated_load_info(point_number, handler)
    }


    pub fn show_distributed_line_load_info(&self, line_number: FEUInt, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        show_distributed_line_load_info(line_number, handler)
    }
}
