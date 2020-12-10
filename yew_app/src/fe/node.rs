use crate::math::math_aux_structs::Coordinates;
use std::hash::Hash;


#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct FeNode<T, V>
    where T: Hash + Copy
{
    pub number: T,
    pub coordinates: Coordinates<V>
}
