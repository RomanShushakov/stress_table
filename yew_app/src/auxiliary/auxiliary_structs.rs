use std::slice::Iter;
use self::ElementType::*;
use crate::fe::fe_aux_structs::{Displacement, Force, StrainStress, StrainStressComponent};

use std::collections::HashMap;


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


#[derive(PartialEq, Clone, Debug)]
pub enum ElementType
{
    Truss2n2ip,
    OtherType,
}


impl ElementType
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            ElementType::Truss2n2ip => String::from("Truss2n2ip"),
            ElementType::OtherType => String::from("OtherType"),
        }
    }


    pub fn iterator() -> Iter<'static, ElementType>
    {
        static TYPES: [ElementType; 2] =
            [
                Truss2n2ip, OtherType,
            ];
        TYPES.iter()
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct AuxElement
{
    pub element_type: ElementType,
    pub number: u16,
    pub node_1_number: u16,
    pub node_2_number: u16,
    pub young_modulus: f32,
    pub area: f32,
    pub area_2: Option<f32>,

    pub moment_of_inertia_about_x_axis: Option<f32>,
    pub moment_of_inertia_about_y_axis: Option<f32>,
    pub torsion_constant: Option<f32>,
}


#[derive(Clone, PartialEq, Debug)]
pub struct AuxDisplacement
{
    pub number: u16,
    pub node_number: u16,
    pub is_rotation_stiffness_enabled: bool,
    pub x_direction_value: Option<f32>,
    pub y_direction_value: Option<f32>,
    pub z_direction_value: Option<f32>,
    pub xy_plane_value: Option<f32>,
    pub yz_plane_value: Option<f32>,
    pub zx_plane_value: Option<f32>,
}


pub enum AuxDisplacementInputOption
{
    Free,
    Restrained,
    Value,
}


impl AuxDisplacementInputOption
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            AuxDisplacementInputOption::Free => String::from("Free"),
            AuxDisplacementInputOption::Restrained => String::from("Restrained"),
            AuxDisplacementInputOption::Value => String::from("Value"),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct AuxForce
{
    pub number: u16,
    pub node_number: u16,
    pub is_rotation_stiffness_enabled: bool,
    pub force_x_value: Option<f32>,
    pub force_y_value: Option<f32>,
    pub force_z_value: Option<f32>,
    pub moment_xy_value: Option<f32>,
    pub moment_yz_value: Option<f32>,
    pub moment_zx_value: Option<f32>,
}


#[derive(Clone, PartialEq, Debug)]
pub struct MinMaxValues
{
    pub min_value: f64,
    pub max_value: f64,
}


#[derive(Clone, PartialEq)]
pub struct AnalysisResult
{
    pub displacements: HashMap<Displacement<u16>, f64>,
    pub reactions: HashMap<Force<u16>, f64>,
    pub strains_and_stresses: HashMap<u16, Vec<StrainStress<f64>>>,
    pub min_max_stress_values: HashMap<StrainStressComponent, MinMaxValues>,
}


#[derive(Clone, PartialEq)]
pub enum ResultView
{
    PlotStresses,
    PlotReactions,
    PrintAllResults,
}


impl ResultView
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            ResultView::PlotStresses => String::from("Plot stresses"),
            ResultView::PlotReactions => String::from("Plot reactions"),
            ResultView::PrintAllResults => String::from("Print all results"),
        }
    }
}