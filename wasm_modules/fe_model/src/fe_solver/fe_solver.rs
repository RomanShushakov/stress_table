use std::ops::{Rem, Div, AddAssign, Add, Sub, Mul, SubAssign, DivAssign, MulAssign};
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_json::json;

use finite_element_method::my_float::MyFloatTrait;
use finite_element_method::fem::fe_model::FEModel;
use finite_element_method::fem::finite_elements::fe_node::DeletedFENodeData;
use finite_element_method::fem::finite_elements::finite_element::DeletedFEData;
use finite_element_method::fem::global_analysis::fe_boundary_condition::DeletedBCData;

use crate::preprocessor::properties::assigned_property::AssignedPropertyToLines;

use crate::fe_solver::consts::ADD_NODE_EVENT_NAME;

use crate::traits::ClearByActionIdTrait;

use crate::consts::EVENT_TARGET;

use crate::functions::{log, dispatch_custom_event};


pub struct LineData<T>
{
    datum_nodes_numbers: Vec<T>,
    optional_nodes_numbers: Option<Vec<T>>,
    element_numbers: Vec<T>,
}


pub struct FESolver<T, V>
{
    fem: FEModel<T, V>,
    lines_data: HashMap<T, LineData<T>>,
    deleted_fe_nodes_data: HashMap<T, Vec<DeletedFENodeData<T, V>>>,    // { action_id: Vec<DeletedFENodeData> }
    deleted_fe_data: HashMap<T, Vec<DeletedFEData<T, V>>>,              // { action_id: Vec<DeletedFEData> }
    deleted_bcs_data: HashMap<T, Vec<DeletedBCData<T, V>>>,             // { action_id: Vec<DeletedBCData> }
}


impl<T, V> FESolver<T, V>
    where T: Copy + Debug + Add<Output = T> + Rem<Output = T> + Div<Output = T> + AddAssign +
             From<u8> + Sub<Output = T> + Mul<Output = T> + PartialOrd + SubAssign + Hash + Eq +
             Serialize + 'static,
          V: Copy + Debug + From<f32> + Into<f64> + DivAssign + MyFloatTrait + PartialOrd +
             SubAssign + MulAssign + AddAssign + Div<Output = V> + Add<Output = V> +
             Mul<Output = V> + Sub<Output = V> + MyFloatTrait<Other = V> + Serialize + 'static,
{
    pub fn create(tolerance: V) -> Self
    {
        let fem = FEModel::create(tolerance);
        let lines_data = HashMap::new();
        let deleted_fe_nodes_data = HashMap::new();
        let deleted_fe_data = HashMap::new();
        let deleted_bcs_data = HashMap::new();
        FESolver { fem, lines_data, deleted_fe_nodes_data, deleted_fe_data, deleted_bcs_data }
    }


    pub fn add_node(&mut self, action_id: T, number: T, x: V, y: V, z: V) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        self.fem.add_node(number, x, y, z).map_err(|e| JsValue::from(e))?;
        self.logging()?;
        Ok(())
    }


    pub fn update_node(&mut self, action_id: T, number: T, x: V, y: V, z: V) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        self.fem.update_node(number, x, y, z, None).map_err(|e| JsValue::from(e))?;
        self.logging()?;
        Ok(())
    }


    pub fn delete_node(&mut self, action_id: T, number: T) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        let (deleted_node_data,
            optional_deleted_fe_data,
            optional_deleted_bcs_data) =
            self.fem.delete_node(number).map_err(|e| JsValue::from(e))?;
        if let Some(deleted_fe_data) = optional_deleted_fe_data
        {
            self.deleted_fe_data.insert(action_id, deleted_fe_data);
        }
        if let Some(deleted_bcs_data) = optional_deleted_bcs_data
        {
            self.deleted_bcs_data.insert(action_id, deleted_bcs_data);
        }
        self.deleted_fe_nodes_data.insert(action_id, vec![deleted_node_data]);

        self.logging()?;
        Ok(())
    }


    pub fn restore_node(&mut self, action_id: T, number: T) -> Result<(), JsValue>
    {
        if let Some(deleted_nodes_data) =
            self.deleted_fe_nodes_data.remove(&action_id)
        {
            if deleted_nodes_data.len() != 1
            {
                let error_message = &format!("FESolver: Restore node action: Incorrect \
                    number of nodes!");
                return Err(JsValue::from(error_message));
            }
            let deleted_node_number = deleted_nodes_data[0].extract_number();
            let (x, y, z) = deleted_nodes_data[0].extract_coordinates();
            self.fem.add_node(deleted_node_number, x, y, z).map_err(|e| JsValue::from(e))?;
        }
        else
        {
            let error_message = &format!("FESolver: Restore node action: Node with \
                number {:?} does not exist!", number);
            return Err(JsValue::from(error_message));
        }
        self.logging()?;
        Ok(())
    }


    pub fn clear_deleted_fe_nodes_data_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_fe_nodes_data.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_fe_nodes_data.remove(action_id);
        }
    }


    pub fn clear_deleted_fe_data_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_fe_data.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_fe_data.remove(action_id);
        }
    }


    pub fn clear_deleted_bcs_data_by_action_id(&mut self, action_id: T)
    {
        for action_id in self.deleted_bcs_data.clone()
            .keys()
            .filter(|deleted_action_id| **deleted_action_id >= action_id)
            .collect::<Vec<&T>>()
            .iter()
        {
            let _ = self.deleted_bcs_data.remove(action_id);
        }
    }


    pub fn logging(&self) -> Result<(), JsValue>
    {
        let mut nodes_info = String::new();
        let all_nodes_numbers = self.fem.extract_all_nodes_numbers();
        for node_number in all_nodes_numbers.iter()
        {
            let (x, y, z) = self.fem.extract_node_coordinates(node_number)
                .map_err(|e| JsValue::from(e))?;
            let current_node_data = format!("[{:?}: x: {:?}, y: {:?}, z: {:?}], ",
                node_number, x, y, z);
            nodes_info += current_node_data.as_str();
        }
        log(&format!("FESolver: \n
            Nodes: {}, \n
            Deleted nodes: {:?}, \n
            Deleted FEs: {:?}, \n
            Deleted BCs {:?} \n",
            nodes_info,
            self.deleted_fe_nodes_data,
            self.deleted_fe_data,
            self.deleted_bcs_data));
        Ok(())
    }
}


impl<T, V> ClearByActionIdTrait<T> for FESolver<T, V>
    where T: Copy + Debug + Add<Output = T> + Rem<Output = T> + Div<Output = T> + Sub<Output = T> +
             Mul<Output = T> + AddAssign + SubAssign + PartialOrd + Hash + Eq + From<u8> +
             Serialize + 'static,
          V: Copy + Debug + Add<Output = V> + Mul<Output = V> + Div<Output = V> + Sub<Output = V> +
             MyFloatTrait<Other = V> + From<f32> + Into<f64> + AddAssign + SubAssign + MulAssign +
             DivAssign + PartialOrd + Serialize + 'static,
{
    fn clear_by_action_id(&mut self, action_id: T)
    {
        self.clear_deleted_fe_nodes_data_by_action_id(action_id);
        self.clear_deleted_fe_data_by_action_id(action_id);
        self.clear_deleted_bcs_data_by_action_id(action_id);
    }
}
