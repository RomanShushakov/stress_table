use crate::{GLElementsNumbers, GLElementsValues};
use crate::{ElementsNumbers, ElementsValues};
use crate::fem::{FEType, BCType};
// use crate::components::preprocessor_canvas::gl::gl_aux_structs::DRAWN_ELEMENTS_DENOTATION_SHIFT;


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
        -> Result<[GLElementsValues; 4], String>
    {
        let mut denotation_coordinates = [1.0 as GLElementsValues; 4];
        match self.fe_type
        {

            FEType::Truss2n2ip =>
                {
                    let start_node_number = self.nodes_numbers[0] as GLElementsNumbers;
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
                    let end_node_number = self.nodes_numbers[1] as GLElementsNumbers;
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
                        denotation_coordinates[i] = (start_coordinate + end_coordinate) /
                            2.0 as GLElementsValues;
                    }
                },
        }
        Ok(denotation_coordinates)
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct DrawnBCData
{
    pub bc_type: BCType,
    pub number: ElementsNumbers,
    pub node_number: ElementsNumbers,
    pub is_rotation_stiffness_enabled: bool,
    pub x_direction_value: Option<ElementsValues>,
    pub y_direction_value: Option<ElementsValues>,
    pub z_direction_value: Option<ElementsValues>,
    pub xy_plane_value: Option<ElementsValues>,
    pub yz_plane_value: Option<ElementsValues>,
    pub zx_plane_value: Option<ElementsValues>,
}


impl DrawnBCData
{
    pub fn find_denotation_coordinates(&self, normalized_nodes: &Vec<NormalizedNode>)
    -> Result<[GLElementsValues; 4], String>
    {
        let mut denotation_coordinates = [1.0 as GLElementsValues; 4];
        if let Some(position) = normalized_nodes
            .iter()
            .position(|node| node.number == self.node_number as GLElementsNumbers)
        {
            denotation_coordinates[0] = normalized_nodes[position].x;
            denotation_coordinates[1] = normalized_nodes[position].y;
            denotation_coordinates[2] = normalized_nodes[position].z;
        }
        else
        {
            return Err(format!("FEDrawnElementData: Node {} does not \
                exist!", self.node_number));
        }
        Ok(denotation_coordinates)
    }
}


pub enum DrawnDisplacementInputOption
{
    Free,
    Restrained,
    Value,
}


impl DrawnDisplacementInputOption
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            DrawnDisplacementInputOption::Free => String::from("Free"),
            DrawnDisplacementInputOption::Restrained => String::from("Restrained"),
            DrawnDisplacementInputOption::Value => String::from("Value"),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct MinMaxValues
{
    pub min_value: ElementsValues,
    pub max_value: ElementsValues,
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