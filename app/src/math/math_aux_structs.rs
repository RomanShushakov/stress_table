#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Coordinates<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}
