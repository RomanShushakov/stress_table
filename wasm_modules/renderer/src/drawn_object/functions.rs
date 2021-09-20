use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

use extended_matrix::extended_matrix::ExtendedMatrix;
use extended_matrix::functions::copy_element_value;

use crate::functions::{compose_rotation_matrix_for_vector, calculate_line_length};


pub fn build_monochrome_cone(vertex_coordinates: &[f32; 3],
    base_center_point_coordinates: &[f32; 3], height: f32, radius: f32, base_points_number: u32,
    start_index_number: u32, color: &[f32], tolerance: f32)
    -> Result<(Vec<f32>, Vec<f32>, Vec<u32>), JsValue>
{
    let mut triangles_vertices_coordinates = Vec::new();
    let mut triangles_vertices_colors_values = Vec::new();
    let mut triangles_vertices_indexes = Vec::new();

    let mut rotation_matrix = compose_rotation_matrix_for_vector(
        *vertex_coordinates,
        *base_center_point_coordinates);
    rotation_matrix.transpose();

    let d_angle = 2.0 * PI / base_points_number as f32;
    let local_coordinates = (0..base_points_number)
        .map(|point_number|
            {
                let angle = d_angle * point_number as f32;
                let local_x =
                    {
                        let value = radius * angle.cos();
                        if value.abs() < tolerance { 0.0 } else { value }
                    };
                let local_y =
                    {
                        let value = radius * angle.sin();
                        if value.abs() < tolerance { 0.0 } else { value }
                    };
                (local_x, local_y)
            })
        .collect::<Vec<(f32, f32)>>();

    triangles_vertices_coordinates.extend(vertex_coordinates);

    for (local_x, local_y) in &local_coordinates
    {
        let coordinates = [height, *local_y, *local_x];

        let local_direction_vector = ExtendedMatrix::create(3u32,
            1u32, coordinates.to_vec(), tolerance);
        let transformed_direction_vector = rotation_matrix
            .multiply_by_matrix(&local_direction_vector)
            .map_err(|e| JsValue::from(e))?;
        let all_direction_vector_values =
            transformed_direction_vector.copy_all_elements_values();
        let direction_vector_x_coordinate = copy_element_value(0, 0,
            &all_direction_vector_values);
        let direction_vector_y_coordinate = copy_element_value(1, 0,
            &all_direction_vector_values);
        let direction_vector_z_coordinate = copy_element_value(2, 0,
            &all_direction_vector_values);

        triangles_vertices_coordinates.extend(&[
            vertex_coordinates[0] + direction_vector_x_coordinate,
            vertex_coordinates[1] + direction_vector_y_coordinate,
            vertex_coordinates[2] + direction_vector_z_coordinate
        ]);
    }

    for point_number in 1..base_points_number
    {
        if point_number == 1
        {
            triangles_vertices_colors_values.extend(color);
            triangles_vertices_colors_values.extend(color);
            triangles_vertices_colors_values.extend(color);
        }
        else
        {
            triangles_vertices_colors_values.extend(color);
        }
        triangles_vertices_indexes.extend(&[
            start_index_number, start_index_number + point_number,
            start_index_number + point_number + 1
        ]);
    }
    triangles_vertices_indexes.extend(&[
        start_index_number, start_index_number + 1, start_index_number + base_points_number
    ]);

    Ok((triangles_vertices_coordinates, triangles_vertices_colors_values,
        triangles_vertices_indexes))
}


pub fn build_monochrome_cylinder_around_line(line_start_point_coordinates: &[f32; 3],
    line_end_point_coordinates: &[f32; 3], optional_offset_from_endpoints: Option<f32>,
    radius: f32, base_points_number: u32, start_index_number: u32, color: &[f32], tolerance: f32)
    -> Result<(Vec<f32>, Vec<f32>, Vec<u32>), JsValue>
{
    let mut triangles_vertices_coordinates = Vec::new();
    let mut triangles_vertices_colors_values = Vec::new();
    let mut triangles_vertices_indexes = Vec::new();

    let offset_from_endpoints =
        if let Some(offset) = optional_offset_from_endpoints { offset } else { 0.0 };

    let line_length = calculate_line_length(line_start_point_coordinates,
        line_end_point_coordinates);

    let mut rotation_matrix = compose_rotation_matrix_for_vector(
        *line_start_point_coordinates,
        *line_end_point_coordinates);
    rotation_matrix.transpose();

    let d_angle = 2.0 * PI / base_points_number as f32;
    let local_coordinates = (0..base_points_number)
        .map(|point_number|
            {
                let angle = d_angle * point_number as f32;
                let local_x =
                    {
                        let value = radius * angle.cos();
                        if value.abs() < tolerance { 0.0 } else { value }
                    };
                let local_y =
                    {
                        let value = radius * angle.sin();
                        if value.abs() < tolerance { 0.0 } else { value }
                    };
                (local_x, local_y)
            })
        .collect::<Vec<(f32, f32)>>();

    for (local_x, local_y) in &local_coordinates
    {
        let start_coordinates = [offset_from_endpoints, *local_y, *local_x];
        let local_start_direction_vector = ExtendedMatrix::create(
            3u32, 1u32, start_coordinates.to_vec(), tolerance);
        let transformed_start_direction_vector = rotation_matrix
            .multiply_by_matrix(&local_start_direction_vector)
            .map_err(|e| JsValue::from(e))?;
        let all_start_direction_vector_values =
            transformed_start_direction_vector.copy_all_elements_values();
        let start_direction_vector_x_coordinate = copy_element_value(0, 0,
            &all_start_direction_vector_values);
        let start_direction_vector_y_coordinate = copy_element_value(1, 0,
            &all_start_direction_vector_values);
        let start_direction_vector_z_coordinate = copy_element_value(2, 0,
            &all_start_direction_vector_values);
        triangles_vertices_coordinates.extend(&[
            line_start_point_coordinates[0] + start_direction_vector_x_coordinate,
            line_start_point_coordinates[1] + start_direction_vector_y_coordinate,
            line_start_point_coordinates[2] + start_direction_vector_z_coordinate
        ]);
        triangles_vertices_colors_values.extend(color);

        let end_coordinates = [line_length - offset_from_endpoints, *local_y, *local_x];
        let local_end_direction_vector = ExtendedMatrix::create(
            3u32, 1u32, end_coordinates.to_vec(), tolerance);
        let transformed_end_direction_vector = rotation_matrix
            .multiply_by_matrix(&local_end_direction_vector)
            .map_err(|e| JsValue::from(e))?;
        let all_end_direction_vector_values =
            transformed_end_direction_vector.copy_all_elements_values();
        let end_direction_vector_x_coordinate = copy_element_value(0, 0,
            &all_end_direction_vector_values);
        let end_direction_vector_y_coordinate = copy_element_value(1, 0,
            &all_end_direction_vector_values);
        let end_direction_vector_z_coordinate = copy_element_value(2, 0,
            &all_end_direction_vector_values);
        triangles_vertices_coordinates.extend(&[
            line_start_point_coordinates[0] + end_direction_vector_x_coordinate,
            line_start_point_coordinates[1] + end_direction_vector_y_coordinate,
            line_start_point_coordinates[2] + end_direction_vector_z_coordinate
        ]);
        triangles_vertices_colors_values.extend(color);
    }

    for point_number in 1..base_points_number
    {
        let current_point_number = 2 * point_number;
        triangles_vertices_indexes.extend(&[
            start_index_number + current_point_number - 2,
            start_index_number + current_point_number - 1,
            start_index_number + current_point_number,
        ]);
        triangles_vertices_indexes.extend(&[
            start_index_number + current_point_number - 1,
            start_index_number + current_point_number,
            start_index_number + current_point_number + 1,
        ]);
    }
    triangles_vertices_indexes.extend(&[
        start_index_number, start_index_number + 1,
        start_index_number + base_points_number * 2 - 2,
    ]);
    triangles_vertices_indexes.extend(&[
        start_index_number + 1, start_index_number + base_points_number * 2 - 2,
        start_index_number + base_points_number * 2 - 1,
    ]);

    Ok((triangles_vertices_coordinates, triangles_vertices_colors_values,
        triangles_vertices_indexes))
}
