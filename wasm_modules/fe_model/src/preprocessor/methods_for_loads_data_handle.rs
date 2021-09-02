use wasm_bindgen::prelude::*;
use std::fmt::Debug;
use serde_json::json;
use serde::Serialize;
use std::hash::Hash;


use crate::Preprocessor;


impl<T, V> Preprocessor<T, V>
    where T: Copy + Debug + Serialize + Hash + Eq + PartialOrd,
          V: Copy + Debug + Serialize + PartialEq,
{
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
