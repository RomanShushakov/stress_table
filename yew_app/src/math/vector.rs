use std::ops::{Sub, Mul, Add, Div};
use crate::math::math_aux_traits::{One, FloatNum};
use crate::math::math_aux_structs::Coordinates;


pub enum GlobalCoordinateAxis
{
    X,
    Y,
    Z,
}


pub enum GlobalCoordinatePlane
{
    XY,
    YZ,
    XZ,
}


#[derive(Debug, Clone)]
pub struct Vector<T>
{
    pub start_coordinates: Coordinates<T>,
    pub end_coordinates: Coordinates<T>,
}


impl<T> Vector<T>
    where T: Copy + Default + One +
             Sub<Output = T> + Mul<Output = T> +
             Add<Output = T>
{
    fn _move_to_origin(&self) -> Vector<T>
    {
        let start_point = Coordinates
            {
                x: Default::default(), y: Default::default(), z: Default::default(),
            };
        let end_point = Coordinates
            {
                x: self.end_coordinates.x - self.start_coordinates.x,
                y: self.end_coordinates.y - self.start_coordinates.y,
                z: self.end_coordinates.z - self.start_coordinates.z,
            };
        Vector { start_coordinates: start_point, end_coordinates: end_point }
    }


    pub fn cos_coord_axis<V>(&self, axis: GlobalCoordinateAxis) -> V
        where V: Mul<Output = V> + From<T> + One + Default + Copy + FloatNum +
                 Div<Output = V> + Add<Output = V>
    {
        let u_coord = self._move_to_origin().end_coordinates;
        let v_coord: Coordinates<V> = match axis
            {
                GlobalCoordinateAxis::X => Coordinates
                    { x: One::one(), y: Default::default(), z: Default::default() },
                GlobalCoordinateAxis::Y => Coordinates
                    { x: Default::default(), y: One::one(), z: Default::default() },
                GlobalCoordinateAxis::Z => Coordinates
                    { x: Default::default(), y: Default::default(), z: One::one() },
            };
        let (u_coord_x, u_coord_y, u_coord_z)  =
            (V::from(u_coord.x), V::from(u_coord.y), V::from(u_coord.z));
        let cosine =
            (u_coord_x * v_coord.x + u_coord_y * v_coord.y + u_coord_z * v_coord.z) /
            (
                (u_coord_x * u_coord_x + u_coord_y * u_coord_y + u_coord_z * u_coord_z).sqrt() *
                (
                    v_coord.x * v_coord.x +
                    v_coord.y * v_coord.y +
                    v_coord.z * v_coord.z
                ).sqrt()
            );
        cosine
    }


    pub fn sin_coord_axis<V>(&self, axis: GlobalCoordinateAxis) -> V
    where V: Mul<Output = V> + From<T> + One + Default + Copy + FloatNum +
             Div<Output = V> + Add<Output = V> + Sub<Output = V>
    {
        let u_coord = self._move_to_origin().end_coordinates;
        let v_coord: Coordinates<V> = match axis
            {
                GlobalCoordinateAxis::X => Coordinates
                    { x: One::one(), y: Default::default(), z: Default::default() },
                GlobalCoordinateAxis::Y => Coordinates
                    { x: Default::default(), y: One::one(), z: Default::default() },
                GlobalCoordinateAxis::Z => Coordinates
                    { x: Default::default(), y: Default::default(), z: One::one() },
            };
        let (u_coord_x, u_coord_y, u_coord_z)  =
            (V::from(u_coord.x), V::from(u_coord.y), V::from(u_coord.z));
        let sine =
            (
                (u_coord_y * v_coord.z - u_coord_z * v_coord.y) *
                (u_coord_y * v_coord.z - u_coord_z * v_coord.y) +
                (u_coord_z * v_coord.x - u_coord_x * v_coord.z) *
                (u_coord_z * v_coord.x - u_coord_x * v_coord.z) +
                (u_coord_x * v_coord.y - u_coord_y * v_coord.x) *
                (u_coord_x * v_coord.y - u_coord_y * v_coord.x)
            ).sqrt() /
            (
                (u_coord_x * u_coord_x + u_coord_y * u_coord_y + u_coord_z * u_coord_z).sqrt() *
                (
                    v_coord.x * v_coord.x +
                    v_coord.y * v_coord.y +
                    v_coord.z * v_coord.z
                ).sqrt()
            );
        sine
    }


    pub fn project_on_coord_plane(&self, plane: GlobalCoordinatePlane) -> Vector<T>
    {
        let mut projection = self.clone();
        match plane
        {
            GlobalCoordinatePlane::XY =>
                {
                    projection.start_coordinates.z = Default::default();
                    projection.end_coordinates.z = Default::default();
                },
            GlobalCoordinatePlane::YZ =>
                {
                    projection.start_coordinates.x = Default::default();
                    projection.end_coordinates.x = Default::default();
                },
            GlobalCoordinatePlane::XZ =>
                {
                    projection.start_coordinates.y = Default::default();
                    projection.end_coordinates.y = Default::default();
                },
        }
        projection
    }
}
