use serde_json::json;
use wasm_bindgen::JsValue;
use serde::Serialize;
use std::hash::Hash;
use std::fmt::Debug;

use crate::preprocessor::loads::loads::Loads;
use crate::preprocessor::loads::concentrated_load::ConcentratedLoad;
use crate::preprocessor::loads::consts::ADD_CONCENTRATED_LOAD_EVENT_NAME;

use crate::traits::ClearByActionIdTrait;
use crate::consts::EVENT_TARGET;
use crate::functions::dispatch_custom_event;


impl<T, V> Loads<T, V>
    where T: Copy + Debug + Serialize + Hash + Eq + PartialOrd,
          V: Copy + Debug + Serialize + PartialEq,
{
    pub fn add_concentrated_load(&mut self, action_id: T, point_number: T, fx: V, fy: V, fz: V,
        mx: V, my: V, mz: V, is_action_id_should_be_increased: bool) -> Result<(), JsValue>
    {
        self.clear_by_action_id(action_id);

        if self.concentrated_loads.contains_key(&point_number)
        {
            let error_message = &format!("Loads: Add concentrated load action: \
                Concentrated load was already applied to point with number {:?}!", point_number);
            return Err(JsValue::from(error_message));
        }
        if self.concentrated_loads.values().position(|concentrated_load|
            concentrated_load.are_load_components_same(fx, fy, fz) &&
            concentrated_load.are_moment_components_same(mx, my, mz)).is_some()
        {
            let error_message = &format!("Loads: Add concentrated load action: Concentrated \
                load with components {:?}, {:?}, {:?}, {:?}, {:?}, {:?} does already exist!",
                fx, fy, fz, mx, my, mz);
            return Err(JsValue::from(error_message));
        }
        let concentrated_load = ConcentratedLoad::create(fx, fy, fz, mx, my, mz);
        self.concentrated_loads.insert(point_number, concentrated_load);
        let detail = json!({ "concentrated_load_data":
            { "point_number": point_number, "fx": fx, "fy": fy, "fz": fz,
                "mx": mx, "my": my, "mz": mz },
            "is_action_id_should_be_increased": is_action_id_should_be_increased });
        dispatch_custom_event(detail, ADD_CONCENTRATED_LOAD_EVENT_NAME,
            EVENT_TARGET)?;
        self.logging();
        Ok(())
    }


    pub fn extract_concentrated_loads(&self, handler: js_sys::Function) -> Result<(), JsValue>
    {
        let extracted_concentrated_loads = json!({ "extracted_concentrated_loads":
            self.concentrated_loads });
        let composed_extracted_concentrated_loads =
            JsValue::from_serde(&extracted_concentrated_loads)
                .or(Err(JsValue::from("Preprocessor: Extract concentrated loads: \
                    Concentrated loads could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_concentrated_loads);
        Ok(())
    }
}
