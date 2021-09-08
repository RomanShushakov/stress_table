use serde_json::json;
use wasm_bindgen::JsValue;
use serde::Serialize;
use std::hash::Hash;
use std::fmt::Debug;

use crate::preprocessor::loads::loads::Loads;
use crate::preprocessor::loads::distributed_line_load::
{
    DistributedLineLoad, DeletedDistributedLineLoad
};
use crate::preprocessor::loads::consts::
{
    ADD_DISTRIBUTED_LINE_LOAD_EVENT_NAME, UPDATE_DISTRIBUTED_LINE_LOAD_EVENT_NAME,
    DELETE_DISTRIBUTED_LINE_LOAD_EVENT_NAME,
};

use crate::traits::ClearByActionIdTrait;
use crate::consts::EVENT_TARGET;
use crate::functions::{dispatch_custom_event};


impl<T, V> Loads<T, V>
    where T: Copy + Debug + Serialize + Hash + Eq + PartialOrd,
          V: Copy + Debug + Serialize + PartialEq,
{
    pub fn add_distributed_line_load(&mut self, action_id: T, line_number: T, qx: V, qy: V, qz: V,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if self.distributed_line_loads.contains_key(&line_number)
        {
            let error_message = &format!("Loads: Add distributed line load action: \
                Distributed line load was already applied to line with number {:?}!", line_number);
            return Err(JsValue::from(error_message));
        }

        let distributed_line_load = DistributedLineLoad::create(qx, qy, qz);
        self.distributed_line_loads.insert(line_number, distributed_line_load);
        let detail = json!({ "distributed_line_load_data":
            { "line_number": line_number, "qx": qx, "qy": qy, "qz": qz },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_DISTRIBUTED_LINE_LOAD_EVENT_NAME,
            EVENT_TARGET)?;
        self.logging();
        Ok(())
    }


    pub fn update_distributed_line_load(&mut self, action_id: T, line_number: T, qx: V, qy: V,
        qz: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if let Some(distributed_line_load) =
            self.distributed_line_loads.get_mut(&line_number)
        {
            distributed_line_load.update(qx, qy, qz);
            let detail = json!({ "distributed_line_load_data": { "line_number": line_number,
                "qx": qx, "qy": qy, "qz": qz },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, UPDATE_DISTRIBUTED_LINE_LOAD_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = format!("Loads: Update distributed line load action: \
                The distributed line load applied to line with number {:?} does not exist!",
                line_number);
            Err(JsValue::from(&error_message))
        }
    }


    pub fn delete_distributed_line_load(&mut self, action_id: T, line_number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if let Some((line_number, distributed_line_load)) =
            self.distributed_line_loads.remove_entry(&line_number)
        {
            let deleted_distributed_line_load =
                DeletedDistributedLineLoad::create(line_number, distributed_line_load);
            self.deleted_distributed_line_loads.insert(action_id,
                vec![deleted_distributed_line_load]);
            let detail = json!({ "distributed_line_load_data": { "line_number": line_number },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, DELETE_DISTRIBUTED_LINE_LOAD_EVENT_NAME,
                EVENT_TARGET)?;
            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Loads: Delete distributed line load action: \
                Distributed line load applied to line with number {:?} does not exist!",
                line_number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn restore_distributed_line_load(&mut self, action_id: T, line_number: T,
        is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        if let Some(deleted_distributed_line_loads) =
            self.deleted_distributed_line_loads.remove(&action_id)
        {
            if deleted_distributed_line_loads.len() != 1
            {
                let error_message = "Loads: Restore distributed line loads \
                    action: Incorrect number of deleted distributed line loads";
                return Err(JsValue::from(error_message));
            }
            let deleted_distributed_line_load =
                &deleted_distributed_line_loads[0];

            let (deleted_distributed_line_load_line_number, qx, qy, qz) =
                    deleted_distributed_line_load.copy_line_number_and_load_components();
            if deleted_distributed_line_load_line_number != line_number
            {
                let error_message = &format!("Loads: Restore distributed line loads \
                    action: Distributed line load applied to line with number {:?} does not exist!",
                    line_number);
                return Err(JsValue::from(error_message));
            }
            let detail = json!({ "distributed_line_load_data":
                {
                    "line_number": deleted_distributed_line_load_line_number,
                    "qx": qx, "qy": qy, "qz": qz,
                },
                "is_action_id_should_be_increased": is_action_id_should_be_increased });
            dispatch_custom_event(detail, ADD_DISTRIBUTED_LINE_LOAD_EVENT_NAME,
                EVENT_TARGET)?;
            self.distributed_line_loads.insert(deleted_distributed_line_load_line_number,
                DistributedLineLoad::create(qx, qy, qz));

            self.logging();
            Ok(())
        }
        else
        {
            let error_message = &format!("Loads: Restore concentrated load action: \
                Concentrated load applied to point with number {:?} does not exist!", line_number);
            return Err(JsValue::from(error_message));
        }
    }


    pub fn delete_distributed_line_loads_applied_to_lines(&mut self, action_id: T,
        line_numbers: &[T]) -> Result<(), JsValue>
    {
        let mut deleted_distributed_line_loads = Vec::new();

        for line_number in line_numbers
        {
            if let Some((line_number, distributed_line_load)) =
                self.distributed_line_loads.remove_entry(line_number)
            {
                let deleted_distributed_line_load =
                    DeletedDistributedLineLoad::create(line_number, distributed_line_load);
                deleted_distributed_line_loads.push(deleted_distributed_line_load);

                let detail = json!({ "distributed_line_load_data":
                    { "line_number": line_number },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail, DELETE_DISTRIBUTED_LINE_LOAD_EVENT_NAME,
                    EVENT_TARGET)?;
            }
        }

        if !deleted_distributed_line_loads.is_empty()
        {
            self.deleted_distributed_line_loads.insert(action_id,
                deleted_distributed_line_loads);
        }
        Ok(())
    }


    pub fn restore_distributed_line_loads_applied_to_lines(&mut self, action_id: T,
        line_numbers: &[T]) -> Result<(), JsValue>
    {
        if let Some(deleted_distributed_line_loads) =
            self.deleted_distributed_line_loads.remove(&action_id)
        {
            for deleted_distributed_line_load in
                deleted_distributed_line_loads.iter()
            {
                let (deleted_deleted_distributed_line_load_line_number, qx, qy, qz) =
                    deleted_distributed_line_load.copy_line_number_and_load_components();
                if !line_numbers.contains(&deleted_deleted_distributed_line_load_line_number)
                {
                    let error_message = &format!("Loads: Restore distributed line load \
                        action: Restored line numbers {:?} does not contain line number {:?} to \
                        which distributed line load applied!",
                        line_numbers, deleted_deleted_distributed_line_load_line_number);
                    return Err(JsValue::from(error_message));
                }
                let detail = json!({ "distributed_line_load_data":
                    {
                        "line_number": deleted_deleted_distributed_line_load_line_number,
                        "qx": qx, "qy": qy, "qz": qz
                    },
                    "is_action_id_should_be_increased": false });
                dispatch_custom_event(detail, ADD_DISTRIBUTED_LINE_LOAD_EVENT_NAME,
                    EVENT_TARGET)?;
                self.distributed_line_loads.insert(
                    deleted_deleted_distributed_line_load_line_number,
                    DistributedLineLoad::create(qx, qy, qz));
            }
        }
        Ok(())
    }


    pub fn show_distributed_line_load_info(&mut self, line_number: T, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        return if let Some(distributed_line_load) =
            self.distributed_line_loads.get(&line_number)
        {
            let (qx, qy, qz) = distributed_line_load.copy_load_components();
            let distributed_line_load_info_json = json!({ "distributed_line_load_data":
                {
                    "line_number": line_number, "qx": qx, "qy": qy, "qz": qz,
                } });
            let distributed_line_load_info = JsValue::from_serde(
                &distributed_line_load_info_json)
                .or(Err(JsValue::from("Loads: Show distributed line load info: Distributed \
                    line load info could not be composed!")))?;
            let this = JsValue::null();
            let _ = handler.call1(&this, &distributed_line_load_info)?;
            Ok(())
        }
        else
        {
            let error_message = &format!("Loads: Show distributed line load info action: \
                Distributed line load applied to line with number {:?} does not exist!",
                line_number);
            Err(JsValue::from(error_message))
        }
    }


    pub fn extract_distributed_line_loads(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_distributed_line_loads = json!({ "extracted_distributed_line_loads":
            self.distributed_line_loads });
        let composed_extracted_distributed_line_loads =
            JsValue::from_serde(&extracted_distributed_line_loads)
                .or(Err(JsValue::from("Preprocessor: Extract distributed line loads: \
                    Distributed line loads could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_distributed_line_loads);
        Ok(())
    }
}
