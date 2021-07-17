use crate::preprocessor::geometry::geometry::Geometry;

use crate::types::{FEUInt, FEFloat};


pub fn line_points_coordinates_extraction_handle(line_number: FEUInt, geometry: &Geometry)
    -> Option<((FEFloat, FEFloat, FEFloat), (FEFloat, FEFloat, FEFloat))>
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
