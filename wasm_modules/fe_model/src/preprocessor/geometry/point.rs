use serde::Serialize;

use crate::types::{FEUInt, FEFloat};


#[derive(Debug, Copy, Clone, Serialize)]
pub struct Point
{
    x: FEFloat,
    y: FEFloat,
    z: FEFloat,
}


impl Point
{
    pub fn create(x: FEFloat, y: FEFloat, z: FEFloat) -> Self
    {
        Point { x, y, z }
    }


    pub fn coordinates_same(&self, x: FEFloat, y: FEFloat, z: FEFloat) -> bool
    {
        self.x == x && self.y == y && self.z == z
    }


    pub fn update(&mut self, x: FEFloat, y: FEFloat, z: FEFloat)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }


    pub fn extract_coordinates(&self) -> (FEFloat, FEFloat, FEFloat)
    {
        (self.x, self.y, self.z)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct DeletedPoint
{
    number: FEUInt,
    point: Point,
}


impl DeletedPoint
{
    pub fn create(number: FEUInt, point: Point) -> Self
    {
        DeletedPoint { number, point }
    }


    pub fn extract_number_and_coordinates(&self) -> (FEUInt, FEFloat, FEFloat, FEFloat)
    {
        let (x, y, z) = self.point.extract_coordinates();
        (self.number, x, y, z)
    }
}
