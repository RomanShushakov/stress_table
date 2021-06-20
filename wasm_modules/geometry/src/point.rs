use serde::Serialize;


#[derive(Debug, Copy, Clone, Serialize)]
pub struct Point
{
    x: f64,
    y: f64,
    z: f64,
}


impl Point
{
    pub fn create(x: f64, y: f64, z: f64) -> Self
    {
        Point { x, y, z }
    }


    pub fn coordinates_same(&self, x: f64, y: f64, z: f64) -> bool
    {
        self.x == x && self.y == y && self.z == z
    }


    pub fn update(&mut self, x: f64, y: f64, z: f64)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }


    pub fn extract_coordinates(&self) -> (f64, f64, f64)
    {
        (self.x, self.y, self.z)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct DeletedPoint
{
    number: u32,
    point: Point,
}


impl DeletedPoint
{
    pub fn create(number: u32, point: Point) -> Self
    {
        DeletedPoint { number, point }
    }


    pub fn extract_number_and_coordinates(&self) -> (u32, f64, f64, f64)
    {
        let (x, y, z) = self.point.extract_coordinates();
        (self.number, x, y, z)
    }
}
