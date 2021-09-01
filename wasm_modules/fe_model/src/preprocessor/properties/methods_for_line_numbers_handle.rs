use wasm_bindgen::prelude::*;
use serde_json::json;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Sub, MulAssign, SubAssign, AddAssign, Rem};

use crate::traits::ClearByActionIdTrait;

use crate::preprocessor::geometry::geometry::Geometry;

use crate::preprocessor::properties::properties::Properties;
use crate::preprocessor::properties::assigned_property::
{
    ChangedAssignedPropertyToLines, DeletedAssignedPropertyToLines,
};
use crate::preprocessor::properties::consts::
{
    ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME, UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
};

use finite_element_method::my_float::MyFloatTrait;

use crate::consts::EVENT_TARGET;

use crate::functions::{dispatch_custom_event, find_components_of_line_a_perpendicular_to_line_b};


impl<T, V> Properties<T, V>
    where T: Copy + Debug + Eq + Hash + Serialize + PartialOrd + SubAssign + AddAssign +
             Rem<Output = T> + Div<Output = T> + Sub<Output = T> + Mul<Output = T> +
             Add<Output = T> + From<u8> + 'static,
          V: Copy + Debug + Serialize + From<f32> + MyFloatTrait + Add<Output = V> + PartialEq +
             Into<f64> + MulAssign + SubAssign + AddAssign + Div<Output = V> + Mul<Output = V> +
             Sub<Output = V> + 'static,
{
    pub fn delete_line_numbers_from_properties(&mut self, action_id: T, line_numbers: &[T])
        -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);
        let mut changed_assigned_properties_to_lines = Vec::new();
        let mut deleted_assigned_properties_to_lines = Vec::new();

        for (assigned_property_to_lines_name, assigned_property_to_lines) in
            self.assigned_properties_to_lines.iter_mut()
        {
            let current_line_numbers_for_delete = assigned_property_to_lines
                .extract_related_lines_numbers().into_iter().filter(|line_number|
                    line_numbers.contains(line_number))
                .collect::<Vec<T>>();

            if current_line_numbers_for_delete.len() == assigned_property_to_lines
                .length_of_related_lines_data()
            {
                let deleted_assigned_property_to_lines =
                    DeletedAssignedPropertyToLines::create(assigned_property_to_lines_name,
                    assigned_property_to_lines.clone());
                deleted_assigned_properties_to_lines.push(deleted_assigned_property_to_lines);

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "line_numbers": assigned_property_to_lines.extract_related_lines_numbers(),
                    },
                    "is_action_id_should_be_increased": false });

                dispatch_custom_event(detail,
                    DELETE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
            else
            {
                let changed_assigned_property_to_lines =
                    ChangedAssignedPropertyToLines::create(assigned_property_to_lines_name,
                    assigned_property_to_lines.clone());
                changed_assigned_properties_to_lines.push(changed_assigned_property_to_lines);

                let old_line_numbers =
                    assigned_property_to_lines.extract_related_lines_numbers();

                for line_number_for_delete in current_line_numbers_for_delete.iter()
                {
                    let _ = assigned_property_to_lines.remove_line_number_from_related_lines_data(
                        line_number_for_delete);
                }

                let related_lines_data =
                    assigned_property_to_lines.extract_related_lines_data();

                let (_, _, cross_section_type) =
                    self.properties.get(assigned_property_to_lines_name).unwrap().extract_data();

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": assigned_property_to_lines.extract_related_lines_numbers(),
                        "old_line_numbers": old_line_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }

        if !changed_assigned_properties_to_lines.is_empty()
        {
            self.changed_assigned_properties_to_lines.insert(action_id,
                changed_assigned_properties_to_lines);
        }

        if !deleted_assigned_properties_to_lines.is_empty()
        {
            for deleted_assigned_property_to_lines_name in deleted_assigned_properties_to_lines
                .iter()
                .map(|deleted_assigned_property_to_lines|
                    deleted_assigned_property_to_lines.extract_name())
            {
                let _ = self.assigned_properties_to_lines.remove(
                    deleted_assigned_property_to_lines_name);
            }

            self.deleted_assigned_properties_to_lines.insert(action_id,
                deleted_assigned_properties_to_lines);
        }

        self.logging();
        Ok(())
    }


    pub fn restore_line_numbers_in_properties(&mut self, action_id: T,
        restored_line_numbers: Vec<T>) -> Result<(), JsValue>
    {
        let mut is_appropriate_data_exist = false;
        let mut restored_line_numbers_for_check = restored_line_numbers.clone();

        if let Some(changed_assigned_properties_to_lines) =
            self.changed_assigned_properties_to_lines.remove(&action_id)
        {
            is_appropriate_data_exist = true;

            for changed_assigned_property_to_lines in
                changed_assigned_properties_to_lines.into_iter()
            {
                let (assigned_property_to_lines_name, assigned_property_to_lines) =
                    changed_assigned_property_to_lines.extract_and_drop();

                let related_lines_data =
                    assigned_property_to_lines.extract_related_lines_data();

                let line_numbers = assigned_property_to_lines
                    .extract_related_lines_numbers();

                while let Some(position) = restored_line_numbers_for_check.iter()
                    .position(|number| line_numbers.contains(number))
                {
                    let _ = restored_line_numbers_for_check.remove(position);
                }

                let old_line_numbers = self.assigned_properties_to_lines
                    .get(&assigned_property_to_lines_name).unwrap()
                    .extract_related_lines_numbers();

                let (_, _, cross_section_type) =
                        self.properties.get(&assigned_property_to_lines_name)
                            .unwrap().extract_data();

                self.assigned_properties_to_lines.insert(assigned_property_to_lines_name.clone(),
                    assigned_property_to_lines);

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": line_numbers,
                        "old_line_numbers": old_line_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }

        if let Some(deleted_assigned_properties_to_lines) =
            self.deleted_assigned_properties_to_lines.remove(&action_id)
        {
            is_appropriate_data_exist = true;

            for deleted_assigned_property_to_lines in
                deleted_assigned_properties_to_lines.into_iter()
            {
                let (assigned_property_to_lines_name, assigned_property_to_lines) =
                    deleted_assigned_property_to_lines.extract_and_drop();

                let related_lines_data =
                    assigned_property_to_lines.extract_related_lines_data();

                let line_numbers =
                    assigned_property_to_lines.extract_related_lines_numbers();

                while let Some(position) = restored_line_numbers_for_check.iter()
                    .position(|number| line_numbers.contains(number))
                {
                    let _ = restored_line_numbers_for_check.remove(position);
                }

                let (_, _, cross_section_type) =
                        self.properties.get(&assigned_property_to_lines_name)
                            .unwrap().extract_data();

                self.assigned_properties_to_lines.insert(assigned_property_to_lines_name.clone(),
                    assigned_property_to_lines);

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": line_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    ADD_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }

        if is_appropriate_data_exist
        {
            if restored_line_numbers.len() == restored_line_numbers_for_check.len()
            {
                let error_message = &format!("Properties: Restore line numbers action: \
                    The line numbers {:?} do not contain neither in changed assigned properties \
                    nor in deleted assigned properties for action id {:?}",
                    restored_line_numbers, action_id);
                return Err(JsValue::from(error_message));
            }
        }

        self.logging();
        Ok(())
    }


    pub fn update_line_in_properties(&mut self, action_id: T, number: T, geometry: &Geometry<T, V>,
        get_line_points_coordinates: fn(T, &Geometry<T, V>) -> Option<((V, V, V), (V, V, V))>,
        tolerance: V) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Update line in properties action";

        if let Some(changed_assigned_properties_to_lines) =
            self.changed_assigned_properties_to_lines.remove(&action_id)
        {
            self.clear_by_action_id(action_id);

            if changed_assigned_properties_to_lines.len() != 1
            {
                let error_message = &format!("{}: Incorrect number of assigned properties!",
                    error_message_header);
                return Err(JsValue::from(error_message));
            }

            let (assigned_property_to_lines_name, assigned_property_to_lines) =
                changed_assigned_properties_to_lines[0].clone().extract_and_drop();

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
                "is_action_id_should_be_increased": false });
            dispatch_custom_event(detail,
                UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                EVENT_TARGET)?;
        }
        else
        {
            self.clear_by_action_id(action_id);

            let mut changed_assigned_properties_to_lines = Vec::new();

            for (assigned_property_to_lines_name, assigned_property_to_lines) in
                self.assigned_properties_to_lines.iter_mut()
            {
                if assigned_property_to_lines.extract_related_lines_numbers().contains(&number)
                {
                    if let Some(position) = assigned_property_to_lines
                        .extract_related_lines_data()
                        .iter()
                        .position(|related_lines_data|
                            related_lines_data.line_number() == number)
                    {
                        if let Some(current_local_axis_1_direction) =
                            assigned_property_to_lines.extract_related_lines_data()[position]
                                .local_axis_1_direction()
                        {
                            if let Some((start_point_coordinates, end_point_coordinates)) =
                                get_line_points_coordinates(number, geometry)
                            {
                                let x = end_point_coordinates.0 - start_point_coordinates.0;
                                let y = end_point_coordinates.1 - start_point_coordinates.1;
                                let z = end_point_coordinates.2 - start_point_coordinates.2;

                                let transformed_line = [x, y, z];

                                let projection_of_beam_section_orientation_vector =
                                    find_components_of_line_a_perpendicular_to_line_b::<T, V>(
                                        &current_local_axis_1_direction.extract(),
                                        &transformed_line,
                                        tolerance
                                    )?;

                                let projection_of_beam_section_orientation_length = (
                                    projection_of_beam_section_orientation_vector[0].my_powi(2) +
                                    projection_of_beam_section_orientation_vector[1].my_powi(2) +
                                    projection_of_beam_section_orientation_vector[2].my_powi(2))
                                        .my_sqrt();

                                if projection_of_beam_section_orientation_length == V::from(0f32)
                                {
                                    let changed_assigned_property_to_lines =
                                        ChangedAssignedPropertyToLines::create(
                                            assigned_property_to_lines_name,
                                            assigned_property_to_lines.clone());
                                    changed_assigned_properties_to_lines.push(
                                        changed_assigned_property_to_lines);

                                    assigned_property_to_lines.update_related_lines_data(
                                        number, None);

                                    let related_lines_data =
                                        assigned_property_to_lines.extract_related_lines_data();

                                    let (_, _, cross_section_type) =
                                        self.properties.get(assigned_property_to_lines_name)
                                            .unwrap().extract_data();

                                    let detail = json!({ "assigned_properties_to_lines_data":
                                        {
                                            "name": assigned_property_to_lines_name,
                                            "related_lines_data": related_lines_data,
                                            "line_numbers": assigned_property_to_lines
                                                .extract_related_lines_numbers(),
                                            "cross_section_type": cross_section_type.as_str()
                                                .to_lowercase(),
                                        },
                                        "is_action_id_should_be_increased": false });
                                    dispatch_custom_event(detail,
                                        UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                                        EVENT_TARGET)?;
                                }
                            }
                            else
                            {
                                let error_message = &format!("{}: Line points coordinates \
                                    could not be extracted for line {:?}", error_message_header,
                                        number);
                                return Err(JsValue::from(error_message));
                            }
                        }
                    }
                }
            }

            if !changed_assigned_properties_to_lines.is_empty()
            {
                self.changed_assigned_properties_to_lines.insert(action_id,
                    changed_assigned_properties_to_lines);
            }
        }
        self.logging();
        Ok(())
    }


    pub fn update_lines_in_properties(&mut self, action_id: T, line_numbers: Vec<T>,
        geometry: &Geometry<T, V>,
        get_line_points_coordinates: fn(T, &Geometry<T, V>) -> Option<((V, V, V), (V, V, V))>,
        tolerance: V) -> Result<(), JsValue>
    {
        let error_message_header = "Properties: Update line in properties action";

        if let Some(changed_assigned_properties_to_lines) =
            self.changed_assigned_properties_to_lines.remove(&action_id)
        {
            self.clear_by_action_id(action_id);

            for changed_assigned_property_to_lines in
                changed_assigned_properties_to_lines.into_iter()
            {
                let (assigned_property_to_lines_name, assigned_property_to_lines) =
                    changed_assigned_property_to_lines.extract_and_drop();

                let related_lines_data =
                    assigned_property_to_lines.extract_related_lines_data();

                let line_numbers = assigned_property_to_lines
                    .extract_related_lines_numbers();

                // let old_line_numbers = self.assigned_properties_to_lines
                //     .get(&assigned_property_to_lines_name).unwrap()
                //     .extract_related_lines_numbers();

                let (_, _, cross_section_type) =
                        self.properties.get(&assigned_property_to_lines_name)
                            .unwrap().extract_data();

                self.assigned_properties_to_lines.insert(assigned_property_to_lines_name.clone(),
                    assigned_property_to_lines);

                let detail = json!({ "assigned_properties_to_lines_data":
                    {
                        "name": assigned_property_to_lines_name,
                        "related_lines_data": related_lines_data,
                        "line_numbers": line_numbers,
                        // "old_line_numbers": old_line_numbers,
                        "cross_section_type": cross_section_type.as_str().to_lowercase(),
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail,
                    UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }
        else
        {
            self.clear_by_action_id(action_id);

            let mut changed_assigned_properties_to_lines =
                Vec::new();

            for line_number in line_numbers.iter()
            {
                for (assigned_property_to_lines_name, assigned_property_to_lines) in
                    self.assigned_properties_to_lines.iter_mut()
                {
                    if assigned_property_to_lines.extract_related_lines_numbers()
                        .contains(line_number)
                    {
                        if let Some(position) = assigned_property_to_lines
                            .extract_related_lines_data()
                            .iter()
                            .position(|related_lines_data|
                                related_lines_data.line_number() == *line_number)
                        {
                            if let Some(current_local_axis_1_direction) =
                                assigned_property_to_lines.extract_related_lines_data()[position]
                                    .local_axis_1_direction()
                            {
                                if let Some((start_point_coordinates, end_point_coordinates)) =
                                    get_line_points_coordinates(*line_number, geometry)
                                {
                                    let x = end_point_coordinates.0 - start_point_coordinates.0;
                                    let y = end_point_coordinates.1 - start_point_coordinates.1;
                                    let z = end_point_coordinates.2 - start_point_coordinates.2;

                                    let transformed_line = [x, y, z];

                                    let projection_of_beam_section_orientation_vector =
                                        find_components_of_line_a_perpendicular_to_line_b::<T, V>(
                                            &current_local_axis_1_direction.extract(),
                                            &transformed_line,
                                            tolerance
                                        )?;

                                    let projection_of_beam_section_orientation_length = (
                                        projection_of_beam_section_orientation_vector[0]
                                            .my_powi(2) +
                                        projection_of_beam_section_orientation_vector[1]
                                            .my_powi(2) +
                                        projection_of_beam_section_orientation_vector[2]
                                            .my_powi(2))
                                            .my_sqrt();

                                    if projection_of_beam_section_orientation_length == V::from(0f32)
                                    {
                                        if changed_assigned_properties_to_lines
                                            .iter()
                                            .position(|changed_assigned_property: &ChangedAssignedPropertyToLines<T, V>|
                                                changed_assigned_property
                                                    .is_name_same(assigned_property_to_lines_name))
                                            .is_none()
                                        {
                                            let changed_assigned_property_to_lines =
                                            ChangedAssignedPropertyToLines::create(
                                                assigned_property_to_lines_name,
                                                assigned_property_to_lines
                                                    .clone());
                                            changed_assigned_properties_to_lines.push(
                                            changed_assigned_property_to_lines);
                                        }

                                        assigned_property_to_lines.update_related_lines_data(
                                            *line_number, None);

                                        let related_lines_data =
                                            assigned_property_to_lines.extract_related_lines_data();

                                        let (_, _, cross_section_type) =
                                            self.properties.get(assigned_property_to_lines_name)
                                                .unwrap().extract_data();

                                        let detail = json!({ "assigned_properties_to_lines_data":
                                            {
                                                "name": assigned_property_to_lines_name,
                                                "related_lines_data": related_lines_data,
                                                "line_numbers": assigned_property_to_lines
                                                    .extract_related_lines_numbers(),
                                                "cross_section_type": cross_section_type.as_str()
                                                    .to_lowercase(),
                                            },
                                            "is_action_id_should_be_increased": false });
                                        dispatch_custom_event(detail,
                                            UPDATE_ASSIGNED_PROPERTIES_TO_LINES_EVENT_NAME,
                                            EVENT_TARGET)?;
                                    }
                                }
                                else
                                {
                                    let error_message = &format!("{}: Line points coordinates \
                                        could not be extracted for line {:?}", error_message_header,
                                            line_number);
                                    return Err(JsValue::from(error_message));
                                }
                            }
                        }
                    }
                }
            }

            if !changed_assigned_properties_to_lines.is_empty()
            {
                self.changed_assigned_properties_to_lines.insert(action_id,
                    changed_assigned_properties_to_lines);
            }
        }

        self.logging();
        Ok(())
    }
}
