use crate::math::aux_structs::Coordinates;


#[derive(Debug)]
pub struct Node<T, V>
{
    pub number: T,
    pub coordinates: Coordinates<V>
}
