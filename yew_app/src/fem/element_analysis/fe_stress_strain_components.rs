use crate::ElementsNumbers;

use std::slice::Iter;
use self::StressStrainComponent::*;
use crate::fem::EARComponentTrait;
use std::any::Any;


pub const STRESS_STRAIN_COMPONENTS_NUMBER: ElementsNumbers = 9;


#[derive(PartialEq, Debug, Copy, Clone)]
pub enum StressStrainComponent
{
    XX, XY, XZ,
    YX, YY, YZ,
    ZX, ZY, ZZ,
}


impl StressStrainComponent
{
    pub fn iterator() -> Iter<'static, StressStrainComponent>
     {
        const COMPONENTS: [StressStrainComponent; STRESS_STRAIN_COMPONENTS_NUMBER as usize] =
            [
                XX, XY, XZ, YX, YY, YZ, ZX, ZY, ZZ,
            ];
        COMPONENTS.iter()
    }
}


impl StressStrainComponent
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            StressStrainComponent::XX => String::from("Stress XX"),
            StressStrainComponent::XY => String::from("Stress XY"),
            StressStrainComponent::XZ => String::from("Stress XZ"),
            StressStrainComponent::YX => String::from("Stress YX"),
            StressStrainComponent::YY => String::from("Stress YY"),
            StressStrainComponent::YZ => String::from("Stress YZ"),
            StressStrainComponent::ZX => String::from("Stress ZX"),
            StressStrainComponent::ZY => String::from("Stress ZY"),
            StressStrainComponent::ZZ => String::from("Stress ZZ"),
        }
    }
}


impl EARComponentTrait for StressStrainComponent
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }


    fn same(&self, other: &Box<dyn EARComponentTrait>) -> bool
    {
        other
            .as_any()
            .downcast_ref::<StressStrainComponent>()
            .map_or(false, |component| self == component)
    }
}
