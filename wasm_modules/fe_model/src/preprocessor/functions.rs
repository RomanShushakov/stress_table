use std::hash::Hash;

use crate::preprocessor::geometry::geometry::Geometry;

use finite_element_method::my_float::MyFloatTrait;


pub fn get_line_points_coordinates<T, V>(line_number: T, geometry: &Geometry<T, V>)
    -> Option<((V, V, V), (V, V, V))>
    where T: Copy + Eq + Hash,
          V: Copy + PartialEq,
{
    if let Some(line) = geometry.lines.get(&line_number)
    {
        let (start_point_number, end_point_number) =
            line.extract_points_numbers();
        let mut start_point_coordinates = None;
        let mut end_point_coordinates = None;
        if let Some(point) = geometry.points.get(&start_point_number)
        {
            let coordinates = point.extract_coordinates();
            start_point_coordinates = Some(coordinates);
        }
        if let Some(point) = geometry.points.get(&end_point_number)
        {
            let coordinates = point.extract_coordinates();
            end_point_coordinates = Some(coordinates);
        }
        if let (Some(start_point_coordinates), Some(end_point_coordinates)) =
            (start_point_coordinates, end_point_coordinates)
        {
            Some((start_point_coordinates, end_point_coordinates))
        }
        else
        {
            None
        }
    }
    else
    {
        None
    }
}


pub fn compare_with_tolerance<V>(value: V, tolerance: V) -> V
    where V: MyFloatTrait + PartialOrd + From<f32>
{
    if value.my_abs() < tolerance
    {
        V::from(0f32)
    }
    else
    {
        value
    }
}
