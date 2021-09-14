use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::preprocessor::loads::concentrated_load::{ConcentratedLoad, DeletedConcentratedLoad};
use crate::preprocessor::loads::distributed_line_load::
{
    DistributedLineLoad, DeletedDistributedLineLoad
};
use crate::traits::ClearByActionIdTrait;
use crate::functions::log;


pub struct Loads<T, V>
{
    pub concentrated_loads: HashMap<T, ConcentratedLoad<V>>,                                // { point_number: ConcentratedLoad }
    pub distributed_line_loads: HashMap<T, DistributedLineLoad<V>>,                         // { line_number: DistributedLineLoad }
    pub deleted_concentrated_loads: HashMap<T, DeletedConcentratedLoad<T, V>>,              // { action_id: DeletedConcentratedLoad }
    pub deleted_distributed_line_loads: HashMap<T, Vec<DeletedDistributedLineLoad<T, V>>>,  // { action_id: Vec<DeletedDistributedLineLoad> }
}


impl<T, V> Loads<T, V>
    where T: Copy + Debug + Eq + Hash + PartialOrd,
          V: Copy + Debug,
{
    pub fn create() -> Self
    {
        let concentrated_loads = HashMap::new();
        let distributed_line_loads = HashMap::new();
        let deleted_concentrated_loads = HashMap::new();
        let deleted_distributed_line_loads = HashMap::new();

        Loads { concentrated_loads, distributed_line_loads, deleted_concentrated_loads,
            deleted_distributed_line_loads }
    }


    pub fn clear_deleted_concentrated_loads_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_concentrated_loads.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_concentrated_loads.remove(action_id);
        }
    }


    pub fn clear_deleted_distributed_line_loads_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_distributed_line_loads.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_distributed_line_loads.remove(action_id);
        }
    }


    pub fn logging(&self)
    {
        log(&format!("Loads: \n
            Concentrated loads: {:?}, \n
            Distributed line loads: {:?}, \n
            Deleted concentrated loads: {:?}, \n
            Deleted distributed line loads: {:?}, \n",
            self.concentrated_loads,
            self.distributed_line_loads,
            self.deleted_concentrated_loads,
            self.deleted_distributed_line_loads));
    }
}


impl<T, V> ClearByActionIdTrait<T> for Loads<T, V>
    where T: Debug + Copy + Eq + Hash + PartialOrd,
          V: Debug + Copy,
{
    fn clear_by_action_id(&mut self, action_id: T)
    {
        self.clear_deleted_concentrated_loads_by_action_id(action_id);
        self.clear_deleted_distributed_line_loads_by_action_id(action_id);
    }
}
