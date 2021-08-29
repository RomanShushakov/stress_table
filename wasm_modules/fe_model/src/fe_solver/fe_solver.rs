use std::ops::{Rem, Div, AddAssign, Add, Sub, Mul, SubAssign, DivAssign, MulAssign};
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

use finite_element_method::my_float::MyFloatTrait;
use finite_element_method::fem::fe_model::FEModel;
use finite_element_method::fem::finite_elements::fe_node::DeletedFENodeData;
use finite_element_method::fem::finite_elements::finite_element::DeletedFEData;
use finite_element_method::fem::global_analysis::fe_boundary_condition::DeletedBCData;


pub struct FESolver<T, V>
{
    fem: FEModel<T, V>,
    deleted_fe_nodes_data: HashMap<T, Vec<DeletedFENodeData<T, V>>>,    // { action_id: Vec<DeletedFENodeData> }
    deleted_fe_data: HashMap<T, Vec<DeletedFEData<T, V>>>,              // { action_id: Vec<DeletedFEData> }
    deleted_bcs_data: HashMap<T, Vec<DeletedBCData<T, V>>>,             // { action_id: Vec<DeletedBCData> }
}


impl<T, V> FESolver<T, V>
    where T: Copy + Debug + Add<Output = T> + Rem<Output = T> + Div<Output = T> + AddAssign +
             From<u8> + Sub<Output = T> + Mul<Output = T> + PartialOrd + SubAssign + Hash + Eq +
             'static,
          V: Copy + Debug + From<f32> + Into<f64> + DivAssign + MyFloatTrait + PartialOrd +
             SubAssign + MulAssign + AddAssign + Div<Output = V> + Add<Output = V> +
             Mul<Output = V> + Sub<Output = V> + MyFloatTrait<Other = V> + 'static,
{
    pub fn create(tolerance: V) -> Self
    {
        let fem = FEModel::create(tolerance);
        let deleted_fe_nodes_data = HashMap::new();
        let deleted_fe_data = HashMap::new();
        let deleted_bcs_data = HashMap::new();
        FESolver { fem, deleted_fe_nodes_data, deleted_fe_data, deleted_bcs_data }
    }
}
