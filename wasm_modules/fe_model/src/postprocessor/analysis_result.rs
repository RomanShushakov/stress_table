use finite_element_method::fem::global_analysis::fe_global_analysis_result::GlobalAnalysisResult;
use finite_element_method::fem::element_analysis::fe_element_analysis_result::ElementsAnalysisResult;
use std::collections::HashMap;


pub struct AnalysisResult<T, V>
{
    nodes_coordinates: HashMap<T, (V, V, V)>,               // { node_number: (x, y, z) }
    elements_nodes_numbers: HashMap<T, Vec<T>>,             // { element_number: Vec<node_number> }
    elements_rotation_matrices_data: HashMap<T, Vec<V>>,    // { element_number: Vec<element of rotation matrix> }
    global_analysis_result: GlobalAnalysisResult<T, V>,
    elements_analysis_result: ElementsAnalysisResult<T, V>,

}


impl<T, V> AnalysisResult<T, V>
{
    pub fn create(nodes_coordinates: HashMap<T, (V, V, V)>,
        elements_nodes_numbers: HashMap<T, Vec<T>>,
        elements_rotation_matrices_data: HashMap<T, Vec<V>>,
        global_analysis_result: GlobalAnalysisResult<T, V>,
        elements_analysis_result: ElementsAnalysisResult<T, V>) -> Self
    {
        AnalysisResult { nodes_coordinates, elements_nodes_numbers, elements_rotation_matrices_data,
            global_analysis_result, elements_analysis_result }
    }
}
