use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::preprocessor::boundary_conditions::boundary_condition::
{
    BoundaryCondition, DeletedBoundaryCondition
};
use crate::traits::ClearByActionIdTrait;
use crate::functions::log;


pub struct BoundaryConditions<T, V>
{
    pub boundary_conditions: HashMap<T, BoundaryCondition<V>>,                      // { point_number: BoundaryCondition }
    pub deleted_boundary_conditions: HashMap<T, DeletedBoundaryCondition<T, V>>,    // { action_id: DeletedBoundaryCondition }
}


impl<T, V> BoundaryConditions<T, V>
    where T: Copy + Debug + Eq + Hash + PartialOrd,
          V: Copy + Debug,
{
    pub fn create() -> Self
    {
        let boundary_conditions = HashMap::new();
        let deleted_boundary_conditions = HashMap::new();

        BoundaryConditions { boundary_conditions, deleted_boundary_conditions }
    }


    pub fn clear_deleted_boundary_conditions_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_boundary_conditions.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_boundary_conditions.remove(action_id);
        }
    }


    pub fn logging(&self)
    {
        log(&format!("Boundary conditions: \n
            Boundary conditions: {:?}, \n
            Deleted boundary conditions: {:?}, \n",
            self.boundary_conditions,
            self.deleted_boundary_conditions));
    }
}


impl<T, V> ClearByActionIdTrait<T> for BoundaryConditions<T, V>
    where T: Debug + Copy + Eq + Hash + PartialOrd,
          V: Debug + Copy,
{
    fn clear_by_action_id(&mut self, action_id: T)
    {
        self.clear_deleted_boundary_conditions_by_action_id(action_id);
    }
}
