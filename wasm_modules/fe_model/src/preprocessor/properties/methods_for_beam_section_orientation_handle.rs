use wasm_bindgen::prelude::*;
use serde_json::json;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Sub, Mul, Div, MulAssign, SubAssign, AddAssign, Rem};

use crate::traits::ClearByActionIdTrait;

use crate::preprocessor::geometry::geometry::Geometry;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::beam_section_orientation::LocalAxis1Direction;
use crate::preprocessor::properties::consts::
{
    ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
    REMOVE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
    UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
};

use finite_element_method::my_float::MyFloatTrait;
use crate::preprocessor::functions::compare_with_tolerance;

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event, find_components_of_line_a_perpendicular_to_line_b};
use crate::preprocessor::properties::assigned_property::ChangedAssignedPropertyToLines;


impl<T, V> Properties<T, V>
    where T: Copy + Debug + Serialize + Hash + Eq + PartialOrd + SubAssign + AddAssign +
             Add<Output = T> + Rem<Output = T> + Div<Output = T> + Sub<Output = T> +
             Mul<Output = T> + From<u8> + 'static,
          V: Copy + Debug + Serialize + PartialEq + Into<f64> + From<f32> + Sub<Output = V> +
             Mul<Output = V> + Add<Output = V> + MyFloatTrait + Div<Output = V> + PartialOrd +
             MulAssign + SubAssign + AddAssign + 'static,
{
    pub fn add_beam_section_local_axis_1_direction(&mut self, action_id: T,
        local_axis_1_direction: &[V], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        let current_local_axis_1_direction =
            LocalAxis1Direction::create(local_axis_1_direction)?;

        if self.beam_sections_local_axis_1_directions.iter()
            .position(|existed_local_axis_1_direction|
                *existed_local_axis_1_direction == current_local_axis_1_direction)
            .is_some()
        {
            let error_message = &format!("Properties: Add beam section local axis 1 \
                direction action: Local axis 1 direction {:?} does already exist!",
                current_local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }

        self.beam_sections_local_axis_1_directions.push(current_local_axis_1_direction);

        let detail = json!({ "local_axis_1_direction_data":
            {
                "local_axis_1_direction": local_axis_1_direction,
            },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
            EVENT_TARGET)?;
        self.logging();
        Ok(())
    }


    pub fn remove_beam_section_local_axis_1_direction(&mut self, action_id: T,
        local_axis_1_direction: &[V], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Remove beam section local axis 1 \
            direction action";

        self.clear_by_action_id(action_id);

        let current_local_axis_1_direction =
            LocalAxis1Direction::create(local_axis_1_direction)?;

        if let Some(position) = self.beam_sections_local_axis_1_directions.iter()
            .position(|existed_local_axis_1_direction|
                *existed_local_axis_1_direction == current_local_axis_1_direction)
        {
            let mut changed_assigned_properties_to_lines_names = Vec::new();

            let mut changed_assigned_properties_to_lines: Vec<ChangedAssignedPropertyToLines<T, V>> =
                Vec::new();

            for (assigned_property_to_lines_name, assigned_property_to_lines) in
                self.assigned_properties_to_lines.iter_mut()
            {
                let old_related_lines_data =
                    assigned_property_to_lines.extract_related_lines_data();

                for old_related_line_data in old_related_lines_data.iter()
                {
                    if let Some(local_axis_1_direction) =
                        old_related_line_data.local_axis_1_direction()
                    {
                        if local_axis_1_direction == current_local_axis_1_direction
                        {
                            if changed_assigned_properties_to_lines
                                .iter()
                                .position(|changed_assigned_property_to_lines|
                                    changed_assigned_property_to_lines.is_name_same(
                                        assigned_property_to_lines_name))
                                .is_none()
                            {
                                changed_assigned_properties_to_lines_names.push(
                                    assigned_property_to_lines_name.clone());

                                let changed_assigned_property_to_lines =
                                    ChangedAssignedPropertyToLines::create(
                                        assigned_property_to_lines_name,
                                        assigned_property_to_lines.clone());
                                changed_assigned_properties_to_lines.push(
                                    changed_assigned_property_to_lines);
                            }
                            assigned_property_to_lines.update_related_lines_data(
                                old_related_line_data.line_number(),
                                None);
                        }
                    }
                }
            }

            if !changed_assigned_properties_to_lines.is_empty()
            {
                self.changed_assigned_properties_to_lines.insert(action_id,
                    changed_assigned_properties_to_lines);
            }

            for changed_assigned_property_to_lines_name in
                changed_assigned_properties_to_lines_names.iter()
            {
                let related_lines_data =
                    self.assigned_properties_to_lines
                        .get(changed_assigned_property_to_lines_name).unwrap()
                        .extract_related_lines_data();

                let related_lines_numbers =
                    self.assigned_properties_to_lines
                        .get(changed_assigned_property_to_lines_name).unwrap()
                        .extract_related_lines_numbers();

                let (_, _, cross_section_type) = self.properties
                    .get(changed_assigned_property_to_lines_name).unwrap().extract_data();

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": changed_assigned_property_to_lines_name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": related_lines_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }

            let deleted_beam_section_local_axis_1_direction =
                self.beam_sections_local_axis_1_directions.remove(position);
            self.deleted_beam_sections_local_axis_1_directions.insert(action_id,
                deleted_beam_section_local_axis_1_direction);
            let detail = json!({ "local_axis_1_direction_data":
                {
                    "local_axis_1_direction": local_axis_1_direction,
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
            let error_message = &format!("{}: Local axis 1 direction {:?} does not exist!",
                error_message_header, local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_beam_section_local_axis_1_direction(&mut self, action_id: T,
        local_axis_1_direction: &[V], is_action_id_should_be_increased: bool)
        -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Restore beam section local axis 1 \
            direction action";

        let current_local_axis_1_direction =
            LocalAxis1Direction::create(local_axis_1_direction)?;

        if let Some(deleted_beam_section_local_axis_1_direction) =
            self.deleted_beam_sections_local_axis_1_directions.remove(&action_id)
        {
            if deleted_beam_section_local_axis_1_direction != current_local_axis_1_direction
            {
                let error_message = &format!("{}: Beam section local axis 1 direction \
                    value {:?} does not exist!", error_message_header, local_axis_1_direction);
                return Err(JsValue::from(error_message));
            }

            if let Some(changed_assigned_properties_to_lines) =
                self.changed_assigned_properties_to_lines.remove(&action_id)
            {
                for changed_assigned_property_to_lines in
                    changed_assigned_properties_to_lines.into_iter()
                {
                    let (assigned_property_to_lines_name, assigned_property_to_lines) =
                        changed_assigned_property_to_lines.extract_and_drop();

                    let related_lines_data =
                        assigned_property_to_lines.extract_related_lines_data();

                    let related_lines_numbers =
                        assigned_property_to_lines.extract_related_lines_numbers();

                    let (_, _, cross_section_type) = self.properties
                        .get(&assigned_property_to_lines_name).unwrap().extract_data();

                    self.assigned_properties_to_lines.insert(
                        assigned_property_to_lines_name.clone(), assigned_property_to_lines);

                    let detail = json!({ "assigned_properties_to_lines_data":
                        {
                            "name": assigned_property_to_lines_name,
                            "related_lines_data": related_lines_data,
                            "line_numbers": related_lines_numbers,
                            "cross_section_type": cross_section_type.as_str().to_lowercase(),
                        },
                        "is_action_id_should_be_increased": is_action_id_should_be_increased });
                    dispatch_custom_event(detail,
                        UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                        EVENT_TARGET)?;

                }
            }

            self.beam_sections_local_axis_1_directions.push(
                deleted_beam_section_local_axis_1_direction);

            let detail = json!({ "local_axis_1_direction_data":
                {
                    "local_axis_1_direction": local_axis_1_direction,
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail,
                ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
                EVENT_TARGET)?;

            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("{}: Beam section local axis 1 direction \
                value {:?} does not exist!", error_message_header, local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn update_beam_section_orientation_data(&mut self, action_id: T,
        local_axis_1_direction: &[V], line_numbers: &[T],
        is_action_id_should_be_increased: bool, geometry: &Geometry<T, V>,
        get_line_points_coordinates: fn(T, &Geometry<T, V>) -> Option<((V, V, V),
            (V, V, V))>, tolerance: V) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Update beam section orientation data action";

        self.clear_by_action_id(action_id);

        let current_local_axis_1_direction =
            LocalAxis1Direction::create(local_axis_1_direction)?;

        if self.beam_sections_local_axis_1_directions
            .iter()
            .position(|existed_local_axis_1_direction|
                *existed_local_axis_1_direction == current_local_axis_1_direction)
            .is_none()
        {
            let error_message = &format!("{}: Beam section orientation for local axis 1 \
                direction {:?} does not exist!", error_message_header,
                local_axis_1_direction);
            return Err(JsValue::from(error_message));
        }

        for line_number in line_numbers
        {
            self.check_the_correspondence_of_cross_section_type_to_beam(
                line_number, error_message_header)?;

            if let Some((start_point_coordinates, end_point_coordinates)) =
                get_line_points_coordinates(*line_number, geometry)
            {
                let x = end_point_coordinates.0 - start_point_coordinates.0;
                let y = end_point_coordinates.1 - start_point_coordinates.1;
                let z = end_point_coordinates.2 - start_point_coordinates.2;

                let transformed_line = [x, y, z];

                let projection_of_beam_section_orientation_vector =
                    find_components_of_line_a_perpendicular_to_line_b::<T, V>(
                        &current_local_axis_1_direction.extract(), &transformed_line,
                        tolerance
                    )?;

                let projection_of_beam_section_orientation_length = (
                    projection_of_beam_section_orientation_vector[0].my_powi(2) +
                        projection_of_beam_section_orientation_vector[1].my_powi(2) +
                        projection_of_beam_section_orientation_vector[2].my_powi(2)).my_sqrt();

                if projection_of_beam_section_orientation_length == V::from(0f32)
                {
                    let error_message = format!("{}: Projection of local axis 1 direction \
                        on line number {:?} equals to zero!", error_message_header, line_number);
                    return Err(JsValue::from(error_message));
                }
            }
            else
            {
                let error_message = &format!("{}: Line points coordinates could not be \
                    extracted for line {:?}", error_message_header, line_number);
                return Err(JsValue::from(error_message));
            }
        }

        for (assigned_property_to_lines_name, assigned_property_lo_lines) in
            self.assigned_properties_to_lines.iter_mut()
        {
            let mut is_assigned_property_to_lines_updated = false;

            for related_line_data in
                assigned_property_lo_lines.extract_related_lines_data().iter()
            {
                if line_numbers.contains(&related_line_data.line_number())
                {
                    if related_line_data.local_axis_1_direction().is_some()
                    {
                        if related_line_data.local_axis_1_direction().as_ref().unwrap() !=
                            &current_local_axis_1_direction
                        {
                            let error_message = &format!("{}: The line number {:?} has been \
                                already used in local_axis_1_direction {:?}!",
                                error_message_header, related_line_data.line_number(),
                                related_line_data.local_axis_1_direction().as_ref().unwrap().extract());
                            return Err(JsValue::from(error_message));
                        }
                    }
                    else
                    {
                        assigned_property_lo_lines.update_related_lines_data(
                            related_line_data.line_number(),
                            Some(current_local_axis_1_direction.clone()));
                        is_assigned_property_to_lines_updated = true;
                    }
                }
                else
                {
                    if related_line_data.local_axis_1_direction().is_some()
                    {
                        if related_line_data.local_axis_1_direction().as_ref().unwrap() ==
                            &current_local_axis_1_direction
                        {
                            assigned_property_lo_lines.update_related_lines_data(
                                related_line_data.line_number(), None);
                            is_assigned_property_to_lines_updated = true;
                        }
                    }
                }
            }

            if is_assigned_property_to_lines_updated
            {
                let related_lines_data =
                    assigned_property_lo_lines.extract_related_lines_data();

                let related_lines_numbers =
                    assigned_property_lo_lines.extract_related_lines_numbers();

                let (_, _, cross_section_type) = self.properties
                        .get(assigned_property_to_lines_name).unwrap().extract_data();

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": related_lines_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }

        let detail = json!({ "beam_section_orientation_data":
            {
                "local_axis_1_direction": local_axis_1_direction,
                "line_numbers": line_numbers,
            },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, UPDATE_BEAM_SECTION_ORIENTATION_DATA_EVENT_NAME,
            EVENT_TARGET)?;

        self.logging();
        Ok(())
    }


    pub fn extract_beam_sections_local_axis_1_directions(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        let extracted_beam_sections_local_axis_1_directions = json!(
            {
                "extracted_beam_sections_local_axis_1_directions":
                    self.beam_sections_local_axis_1_directions
            });
        let composed_extracted_beam_sections_local_axis_1_directions =
            JsValue::from_serde(&extracted_beam_sections_local_axis_1_directions)
                .or(Err(JsValue::from("Properties: Extract beam sections local axis 1 \
                    directions: Beam sections local axis 1 directions could not be composed for \
                    extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_beam_sections_local_axis_1_directions);
        Ok(())
    }
}
