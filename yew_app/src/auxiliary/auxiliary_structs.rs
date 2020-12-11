#[derive(Clone)]
pub struct DrawnNode
{
    pub number: u16,
    pub x: f64,
    pub y: f64,
}


#[derive(Clone, PartialEq)]
pub struct AuxTruss
{
    pub number: u16,
    pub node_1_number: u16,
    pub node_2_number: u16,
    pub young_modulus: f32,
    pub area: f32,
    pub area_2: Option<f32>,
}