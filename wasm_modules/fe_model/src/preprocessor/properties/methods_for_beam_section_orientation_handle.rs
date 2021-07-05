use wasm_bindgen::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::beam_section_orientation::{BeamSectionOrientation};
use crate::preprocessor::properties::consts::
{
    ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
};

use crate::types::{FEUInt, FEFloat};

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event};


impl Properties
{
    pub fn add_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_deleted_materials_by_action_id(action_id);
        self.clear_deleted_truss_sections_by_action_id(action_id);
        self.clear_deleted_beam_sections_by_action_id(action_id);
        self.clear_deleted_properties_by_action_id(action_id);
        self.clear_deleted_assigned_properties_by_action_id(action_id);
        self.clear_changed_assigned_properties_by_action_id(action_id);

        let converted_local_axis_1_direction = <[FEFloat; 3]>::try_from(local_axis_1_direction).unwrap();
        if self.beam_sections_orientations.iter()
            .position(|beam_section_orientation|
                beam_section_orientation.is_local_axis_1_direction_same(&
                    converted_local_axis_1_direction))
            .is_some()
        {
            let error_message = &format!("Properties: Add beam section local axis 1 \
                direction action: Local axis 1 direction {:?} does already exist!",
                local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }
        let line_numbers = Vec::new();
        let beam_section_orientation = BeamSectionOrientation::create(
            converted_local_axis_1_direction, line_numbers.clone());
        self.beam_sections_orientations.push(beam_section_orientation);
        let detail = json!({ "local_axis_1_direction_data":
            {
                "local_axis_1_direction": converted_local_axis_1_direction,
                "line_numbers": line_numbers,
            },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
            EVENT_TARGET)?;
        self.logging();
        Ok(())
    }
}
