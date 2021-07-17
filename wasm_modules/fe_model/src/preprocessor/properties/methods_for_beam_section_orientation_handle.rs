use wasm_bindgen::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::preprocessor::geometry::geometry::Geometry;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::beam_section_orientation::{BeamSectionOrientation};
use crate::preprocessor::properties::property::CrossSectionType;
use crate::preprocessor::properties::consts::
{
    ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
    REMOVE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
    UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
};

use crate::types::{FEUInt, FEFloat};

use crate::consts::EVENT_TARGET;

use crate::functions::
{
    dispatch_custom_event, find_components_of_line_a_perpendicular_to_line_b
};


impl Properties
{
    pub fn add_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        let converted_local_axis_1_direction = <[FEFloat; 3]>::try_from(local_axis_1_direction)
            .unwrap();
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


    pub fn remove_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);

        let converted_local_axis_1_direction = <[FEFloat; 3]>::try_from(local_axis_1_direction)
            .unwrap();
        if let Some(position) = self.beam_sections_orientations.iter()
            .position(|beam_section_orientation|
                beam_section_orientation.is_local_axis_1_direction_same(&
                    converted_local_axis_1_direction))
        {
            let deleted_beam_section_orientation =
                self.beam_sections_orientations.remove(position);
            self.deleted_beam_sections_orientations.insert(action_id,
                vec![deleted_beam_section_orientation]);
            let detail = json!({ "local_axis_1_direction_data":
                {
                    "local_axis_1_direction": converted_local_axis_1_direction,
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail,
                REMOVE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Remove beam section local axis 1 \
                direction action: Local axis 1 direction {:?} does not exist!",
                local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_beam_section_local_axis_1_direction(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        if let Some(deleted_beam_sections_orientations) =
            self.deleted_beam_sections_orientations.remove(&action_id)
        {
            if deleted_beam_sections_orientations.is_empty() ||
                deleted_beam_sections_orientations.len() > 1
            {
                let error_message = &format!("Properties: Restore beam section local axis 1 \
                    direction action: Incorrect number of beam sections orientations!");
                return Err(JsValue::from(error_message));
            }
            let (deleted_local_axis_1_direction, line_numbers) =
                deleted_beam_sections_orientations[0].extract_direction_and_line_numbers();
            if deleted_local_axis_1_direction != local_axis_1_direction
            {
                let error_message = &format!("Properties: Restore beam section local \
                    axis 1 direction action: Beam section orientation with local axis 1 direction \
                    value {:?} does not exist!", local_axis_1_direction);
                return Err(JsValue::from(error_message));
            }
            let converted_local_axis_1_direction = <[FEFloat; 3]>::try_from(local_axis_1_direction)
                .unwrap();
            self.beam_sections_orientations.push(deleted_beam_sections_orientations[0].clone());
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
        else
        {
            let error_message = &format!("Properties: Restore beam section local axis 1 \
                direction action: Beam section orientation with local axis 1 direction \
                value {:?} does not exist!", local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn update_beam_section_orientation_data(&mut self, action_id: FEUInt,
        local_axis_1_direction: &[FEFloat], line_numbers: &[FEUInt],
        is_action_id_should_be_increased: bool, geometry: &Geometry,
        line_points_coordinates_extraction_handle: fn(FEUInt, &Geometry) -> Option<(
            (FEFloat, FEFloat, FEFloat), (FEFloat, FEFloat, FEFloat))>) -> Result<(), JsValue>
    {
        self.clear_properties_module_by_action_id(action_id);
        let converted_local_axis_1_direction = <[FEFloat; 3]>::try_from(local_axis_1_direction)
            .unwrap();
        for line_number in line_numbers
        {
            for (assigned_property_name, assigned_property) in
                self.assigned_properties.iter()
            {
                if assigned_property.extract_data().contains(line_number)
                {
                    let (_, _, cross_section_type) = self.properties
                        .get(assigned_property_name)
                        .unwrap()
                        .extract_data();
                    match cross_section_type
                    {
                        CrossSectionType::Truss =>
                            {
                                let error_message = &format!("Properties: Update beam \
                                    section orientation data action: Beam section orientation \
                                    could be applied to 'Beam' cross section type only!");
                                return Err(JsValue::from(error_message));
                            },
                        CrossSectionType::Beam => (),
                    }
                }
            }
            if let Some((start_point_coordinates, end_point_coordinates)) =
                line_points_coordinates_extraction_handle(*line_number, geometry)
            {
                let transformed_line = [
                    end_point_coordinates.0 - start_point_coordinates.0,
                    end_point_coordinates.1 - start_point_coordinates.1,
                    end_point_coordinates.2 - start_point_coordinates.2
                ];
                let projection_of_beam_section_orientation_vector =
                    find_components_of_line_a_perpendicular_to_line_b(
                        &converted_local_axis_1_direction, &transformed_line
                    )?;
                let projection_of_beam_section_orientation_length = FEFloat::sqrt(
                    projection_of_beam_section_orientation_vector[0].powi(2) +
                        projection_of_beam_section_orientation_vector[1].powi(2) +
                        projection_of_beam_section_orientation_vector[2].powi(2));
                if projection_of_beam_section_orientation_length == 0 as FEFloat
                {
                    let error_message = format!("Properties: Update beam section orientation \
                        data action: Projection of local axis 1 direction on line number {} \
                        equals to zero!", line_number);
                    return Err(JsValue::from(error_message));
                }
            }
            else
            {
                let error_message = &format!("Properties: Update beam section orientation \
                    data action: Line points coordinates could not be extracted for line {:?}",
                    line_number);
                return Err(JsValue::from(error_message));
            }
        }
        if self.beam_sections_orientations
            .iter()
            .position(|beam_section_orientation|
                {
                    let mut is_any_number_in_slice = false;
                    for line_number in line_numbers
                    {
                        if beam_section_orientation.extract_line_numbers().contains(line_number)
                        {
                            is_any_number_in_slice = true;
                            break;
                        }
                    }
                    if is_any_number_in_slice && !beam_section_orientation
                        .is_local_axis_1_direction_same(&converted_local_axis_1_direction)
                    {
                        true
                    }
                    else
                    {
                        false
                    }

                })
            .is_some()
        {
            let error_message = &format!("Properties: Update beam section orientation data \
                action: At least one line number is already used in another beam section \
                orientation data!");
            return Err(JsValue::from(error_message));
        }

        if let Some(position) = self.beam_sections_orientations
            .iter()
            .position(|beam_section_orientation|
                beam_section_orientation.is_local_axis_1_direction_same(
                    &converted_local_axis_1_direction))
        {
            self.beam_sections_orientations[position].update(line_numbers);
            let detail = json!({ "beam_section_orientation_data":
                {
                    "local_axis_1_direction": converted_local_axis_1_direction,
                    "line_numbers": line_numbers,
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Properties: Update beam section orientation data \
                action: Beam section orientation for local axis 1 direction {:?} does not exist!",
                    converted_local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }
    }
}
