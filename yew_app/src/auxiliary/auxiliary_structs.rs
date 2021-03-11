// use crate::fe::fe_aux_structs::{Displacement, Force, StrainStress, StrainStressComponent};

use std::collections::HashMap;
use crate::components::preprocessor_canvas::preprocessor_canvas::{GLElementsNumbers, GLElementsValues};
use crate::{ElementsNumbers, ElementsValues};
use crate::fem::FEType;
use crate::components::preprocessor_canvas::gl::gl_aux_structs::DRAWN_ELEMENTS_DENOTATION_SHIFT;


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


pub struct FEDrawnNodeData
{
    pub number: ElementsNumbers,
    pub x: ElementsValues,
    pub y: ElementsValues,
    pub z: ElementsValues,
}


pub struct NormalizedNode
{
    pub number: GLElementsNumbers,
    pub x: GLElementsValues,
    pub y: GLElementsValues,
    pub z: GLElementsValues,
}


#[derive(Clone)]
pub struct FEDrawnElementData
{
    pub fe_type: FEType,
    pub number: ElementsNumbers,
    pub nodes_numbers: Vec<ElementsNumbers>,
    pub properties: Vec<ElementsValues>,
}


impl FEDrawnElementData
{
    pub fn find_denotation_coordinates(&self, normalized_nodes: &Vec<NormalizedNode>)
        -> Result<[ElementsValues; 4], String>
    {
        let mut denotation_coordinates = [1.0 as ElementsValues; 4];
        match self.fe_type
        {

            FEType::Truss2n2ip =>
                {
                    let start_node_number = self.nodes_numbers[0];
                    let start_node_coordinates = if let Some(position) =
                        normalized_nodes.iter().position(|node|
                            node.number == start_node_number)
                        {
                            [normalized_nodes[position].x, normalized_nodes[position].y,
                             normalized_nodes[position].z]
                        }
                    else
                    {
                        return Err(format!("FEDrawnElementData: Node {} does not \
                            exist!", start_node_number));
                    };
                    let end_node_number = self.nodes_numbers[1];
                    let end_node_coordinates = if let Some(position) =
                        normalized_nodes.iter().position(|node|
                            node.number == end_node_number)
                        {
                            [normalized_nodes[position].x, normalized_nodes[position].y,
                             normalized_nodes[position].z]
                        }
                    else
                    {
                        return Err(format!("FEDrawnElementData: Node {} does not \
                            exist!", end_node_number));
                    };

                    for (i, (start_coordinate, end_coordinate)) in
                        start_node_coordinates.iter().zip(end_node_coordinates.iter())
                            .enumerate()
                    {
                        if i == 0
                        {
                            denotation_coordinates[i] = (start_coordinate + end_coordinate) /
                                2.0 as ElementsValues + DRAWN_ELEMENTS_DENOTATION_SHIFT;
                        }
                        else
                        {
                            denotation_coordinates[i] = (start_coordinate + end_coordinate) /
                            2.0 as ElementsValues;
                        }
                    }
                },
        }
        Ok(denotation_coordinates)
    }
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


// #[derive(Clone, PartialEq)]
// pub struct AnalysisResult
// {
//     pub displacements: HashMap<Displacement<u16>, f64>,
//     pub reactions: HashMap<Force<u16>, f64>,
//     pub strains_and_stresses: HashMap<u16, Vec<StrainStress<f64>>>,
//     pub min_max_stress_values: HashMap<StrainStressComponent, MinMaxValues>,
// }


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