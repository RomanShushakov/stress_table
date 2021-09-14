use serde_json::json;
use wasm_bindgen::JsValue;
use serde::Serialize;
use std::hash::Hash;
use std::fmt::Debug;

use crate::preprocessor::boundary_conditions::boundary_conditions::BoundaryConditions;
use crate::preprocessor::boundary_conditions::boundary_condition::{BoundaryCondition, DeletedBoundaryCondition};
use crate::preprocessor::boundary_conditions::consts::
{
    ADD_BOUNDARY_CONDITION_EVENT_NAME, UPDATE_BOUNDARY_CONDITION_EVENT_NAME,
    DELETE_BOUNDARY_CONDITION_EVENT_NAME,
};

use crate::traits::ClearByActionIdTrait;
use crate::consts::EVENT_TARGET;
use crate::functions::dispatch_custom_event;


impl<T, V> BoundaryConditions<T, V>
    where T: Copy + Debug + Serialize + Hash + Eq + PartialOrd,
          V: Copy + Debug + Serialize + PartialEq,
{
    pub fn add_boundary_condition(&mut self, action_id: T, point_number: T, optional_ux: Option<V>,
        optional_uy: Option<V>, optional_uz: Option<V>, optional_rx: Option<V>,
        optional_ry: Option<V>, optional_rz: Option<V>, is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if self.boundary_conditions.contains_key(&point_number)
        {
            let error_message = &format!("Boundary conditions: Add boundary condition \
                action: Boundary condition was already applied to point with number {:?}!",
                point_number);
            return Err(JsValue::from(error_message));
        }
        let boundary_condition = BoundaryCondition::create(optional_ux,
            optional_uy, optional_uz, optional_rx, optional_ry, optional_rz);
        self.boundary_conditions.insert(point_number, boundary_condition);
        let detail = json!({ "boundary_condition_data":
            { "point_number": point_number, "optional_ux": optional_ux,
                "optional_uy": optional_uy, "optional_uz": optional_uz,
                "optional_rx": optional_rx, "optional_ry": optional_ry,
                "optional_rz": optional_rz },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_BOUNDARY_CONDITION_EVENT_NAME,
            EVENT_TARGET)?;
        self.logging();
        Ok(())
    }


    pub fn update_boundary_condition(&mut self, action_id: T, point_number: T,
        optional_ux: Option<V>, optional_uy: Option<V>, optional_uz: Option<V>,
        optional_rx: Option<V>, optional_ry: Option<V>, optional_rz: Option<V>,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if let Some(boundary_condition) = self.boundary_conditions
            .get_mut(&point_number)
        {
            boundary_condition.update(optional_ux, optional_uy, optional_uz, optional_rx,
                optional_ry, optional_rz);
            let detail = json!({ "boundary_condition_data": { "point_number": point_number,
                "optional_ux": optional_ux, "optional_uy": optional_uy,
                "optional_uz": optional_uz, "optional_rx": optional_rx,
                "optional_ry": optional_ry, "optional_rz": optional_rz },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_BOUNDARY_CONDITION_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = format!("Boundary conditions: Update boundary condition \
                action: The boundary condition applied to point with number {:?} does not exist!",
                point_number);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_boundary_condition(&mut self, action_id: T, point_number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if let Some((point_number, boundary_condition)) =
            self.boundary_conditions.remove_entry(&point_number)
        {
            let deleted_boundary_condition =
                DeletedBoundaryCondition::create(point_number, boundary_condition);
            self.deleted_boundary_conditions.insert(action_id, deleted_boundary_condition);
            let detail = json!({ "boundary_condition_data": { "point_number": point_number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_BOUNDARY_CONDITION_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Boundary conditions: Delete boundary condition \
                action: The boundary condition applied to point with number {:?} does not exist!",
                point_number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_boundary_condition(&mut self, action_id: T, point_number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_boundary_condition) =
            self.deleted_boundary_conditions.remove(&action_id)
        {
            let (deleted_boundary_condition_point_number,
                optional_ux, optional_uy, optional_uz,
                optional_rx, optional_ry, optional_rz) =
                    deleted_boundary_condition.copy_point_number_and_optional_components();
            if deleted_boundary_condition_point_number != point_number
            {
                let error_message = &format!("Boundary conditions: Restore boundary \
                    condition action: The boundary condition applied to point with number {:?} \
                    does not exist!", point_number);
                return Err(JsValue::from(error_message));
            }
            let detail = json!({ "boundary_condition_data":
                {
                    "point_number": deleted_boundary_condition_point_number,
                    "optional_ux": optional_ux, "optional_uy": optional_uy,
                    "optional_uz": optional_uz, "optional_rx": optional_rx,
                    "optional_ry": optional_ry, "optional_rz": optional_rz,
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_BOUNDARY_CONDITION_EVENT_NAME,
                EVENT_TARGET)?;
            self.boundary_conditions.insert(deleted_boundary_condition_point_number,
                BoundaryCondition::create(optional_ux, optional_uy, optional_uz, optional_rx,
                    optional_ry, optional_rz));

            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Boundary conditions: Restore boundary condition \
                action: The boundary condition applied to point with number {:?} does not exist!",
                point_number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn delete_boundary_condition_applied_to_point(&mut self, action_id: T, point_number: T)
        -> Result<(), JsValue>
    {
        if let Some((point_number, boundary_condition)) =
            self.boundary_conditions.remove_entry(&point_number)
        {
            let deleted_boundary_condition =
                DeletedBoundaryCondition::create(point_number, boundary_condition);
            self.deleted_boundary_conditions.insert(action_id, deleted_boundary_condition);
            let detail = json!({ "boundary_condition_data": { "point_number": point_number },
                "is_action_id_should_be_increased": false });
            dispatch_custom_event(detail, DELETE_BOUNDARY_CONDITION_EVENT_NAME,
                EVENT_TARGET)?;
        }
        Ok(())
    }


    pub fn restore_boundary_condition_applied_to_point(&mut self, action_id: T, point_number: T)
        -> Result<(), JsValue>
    {
        if let Some(deleted_boundary_condition) =
            self.deleted_boundary_conditions.remove(&action_id)
        {
            let (deleted_boundary_condition_point_number,
                optional_ux, optional_uy, optional_uz,
                optional_rx, optional_ry, optional_rz) =
                    deleted_boundary_condition.copy_point_number_and_optional_components();
            if deleted_boundary_condition_point_number != point_number
            {
                let error_message = &format!("Boundary conditions: Restore boundary \
                    condition action: The boundary condition applied to point with number {:?} \
                    does not exist!", point_number);
                return Err(JsValue::from(error_message));
            }
            let detail = json!({ "boundary_condition_data":
                {
                    "point_number": deleted_boundary_condition_point_number,
                    "optional_ux": optional_ux, "optional_uy": optional_uy,
                    "optional_uz": optional_uz, "optional_rx": optional_rx,
                    "optional_ry": optional_ry, "optional_rz": optional_rz,
                },
                "is_action_id_should_be_increased": false });
            dispatch_custom_event(detail, ADD_BOUNDARY_CONDITION_EVENT_NAME,
                EVENT_TARGET)?;
            self.boundary_conditions.insert(deleted_boundary_condition_point_number,
                BoundaryCondition::create(optional_ux, optional_uy, optional_uz, optional_rx,
                    optional_ry, optional_rz));
        }
        Ok(())
    }


    pub fn show_boundary_condition_info(&mut self, point_number: T, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        return if let Some(boundary_condition) =
            self.boundary_conditions.get(&point_number)
        {
            let (optional_ux, optional_uy, optional_uz) =
                boundary_condition.copy_optional_displacement_components();
            let (optional_rx, optional_ry, optional_rz) =
                boundary_condition.copy_optional_rotation_components();
            let boundary_condition_info_json = json!({ "boundary_condition_data":
                {
                    "point_number": point_number,
                    "optional_ux": optional_ux, "optional_uy": optional_uy,
                    "optional_uz": optional_uz, "optional_rx": optional_rx,
                    "optional_ry": optional_ry, "optional_rz": optional_rz,
                } });
            let boundary_condition_info = JsValue::from_serde(&boundary_condition_info_json)
                .or(Err(JsValue::from("Boundary conditions: Show boundary condition info: \
                    Boundary condition info could not be composed!")))?;
            let this = JsValue::null();
            let _ = handler.call1(&this, &boundary_condition_info)?;
            Ok(())
        }
        else
        {
            let error_message = &format!("Loads: Show boundary condition info action: \
                The boundary condition applied to point with number {:?} does not exist!",
                point_number);
            Err(JsValue::from(error_message))
        }
    }


    pub fn extract_boundary_conditions(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_boundary_conditions = json!({ "extracted_boundary_conditions":
            self.boundary_conditions });
        let composed_extracted_boundary_conditions =
            JsValue::from_serde(&extracted_boundary_conditions)
                .or(Err(JsValue::from("Preprocessor: Extract boundary conditions: \
                    Boundary conditions could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_boundary_conditions);
        Ok(())
    }
}
