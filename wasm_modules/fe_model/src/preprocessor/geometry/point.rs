use serde::Serialize;


#[derive(Debug, Copy, Clone, Serialize)]
pub struct Point<V>
{
    x: V,
    y: V,
    z: V,
}


impl<V> Point<V>
    where V: Copy + PartialEq,
{
    pub fn create(x: V, y: V, z: V) -> Self
    {
        Point { x, y, z }
    }


    pub fn are_coordinates_same(&self, x: V, y: V, z: V) -> bool
    {
        self.x == x && self.y == y && self.z == z
    }


    pub fn update(&mut self, x: V, y: V, z: V)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }


    pub fn copy_coordinates(&self) -> (V, V, V)
    {
        (self.x, self.y, self.z)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct DeletedPoint<T, V>
{
    number: T,
    point: Point<V>,
}


impl<T, V> DeletedPoint<T, V>
    where T: Copy,
          V: Copy + PartialEq,
{
    pub fn create(number: T, point: Point<V>) -> Self
    {
        DeletedPoint { number, point }
    }


    pub fn copy_number_and_coordinates(&self) -> (T, V, V, V)
    {
        let (x, y, z) = self.point.copy_coordinates();
        (self.number, x, y, z)
    }
}
