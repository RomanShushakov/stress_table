#[derive(Clone, PartialEq)]
pub enum AnalysisType
{
    TwoDimensional,
    ThreeDimensional,
}


impl AnalysisType
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            AnalysisType::TwoDimensional => String::from("2D"),
            AnalysisType::ThreeDimensional => String::from("3D"),
        }
    }
}


#[derive(Clone, PartialEq)]
pub enum View
{
    PlaneXY,
    PlaneZY,
    PlaneXZ,
    Isometric,
}


impl View
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            View::PlaneXY => String::from("PlaneXY"),
            View::PlaneZY => String::from("PlaneZY"),
            View::PlaneXZ => String::from("PlaneXZ"),
            View::Isometric => String::from("Isometric"),
        }
    }
}



#[derive(Clone)]
pub struct DrawnNode
{
    pub number: u16,
    pub x: f64,
    pub y: f64,
}


#[derive(Clone, PartialEq)]
pub struct AuxTruss2n2ip
{
    pub number: u16,
    pub node_1_number: u16,
    pub node_2_number: u16,
    pub young_modulus: f32,
    pub area: f32,
    pub area_2: Option<f32>,
}