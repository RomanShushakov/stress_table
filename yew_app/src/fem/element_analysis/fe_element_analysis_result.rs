use crate::fem::{StressStrainComponent, ForceComponent};
use crate::extended_matrix::ExtendedMatrix;

use crate::{ElementsNumbers, ElementsValues};


#[derive(Debug, Clone)]
struct ElementStrains<V>
{
    strains_values: Vec<V>,
    strains_components: Vec<StressStrainComponent>,
}


#[derive(Debug, Clone)]
struct ElementStresses<V>
{
    stresses_values: Vec<V>,
    stresses_components: Vec<StressStrainComponent>,
}

#[derive(Debug, Clone)]
struct ElementForces<V>
{
    forces_values: Vec<V>,
    forces_components: Vec<ForceComponent>,
}


#[derive(Debug, Clone)]
pub struct ElementAnalysisData<T, V>
{
    element_number: T,
    strains: ElementStrains<V>,
    stresses: ElementStresses<V>,
    forces: ElementForces<V>,
}


impl<T, V> ElementAnalysisData<T, V>
    where T: Copy,
          V: Copy,
{
    pub fn create(element_number: T,
        strains_values: Vec<V>, strains_components: Vec<StressStrainComponent>,
        stresses_values: Vec<V>, stresses_components: Vec<StressStrainComponent>,
        forces_values: Vec<V>, forces_components: Vec<ForceComponent>) -> Self
    {
        let strains = ElementStrains { strains_values, strains_components };
        let stresses = ElementStresses { stresses_values, stresses_components };
        let forces = ElementForces { forces_values, forces_components };
        ElementAnalysisData { element_number, strains, stresses, forces }
    }
}


#[derive(Debug)]
pub struct ElementsAnalysisResult<T, V>
{
    elements_analysis_data: Vec<ElementAnalysisData<T, V>>,
}


impl<T, V> ElementsAnalysisResult<T, V>
    where T: Copy,
          V: Copy,
{
    pub fn create(elements_analysis_data: Vec<ElementAnalysisData<T, V>>) -> Self
    {
        ElementsAnalysisResult { elements_analysis_data }
    }


    pub fn extract_elements_analysis_data(&self) -> Vec<ElementAnalysisData<T, V>>
    {
        self.elements_analysis_data.to_owned()
    }
}
