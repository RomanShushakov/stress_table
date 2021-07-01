use wasm_bindgen::prelude::*;
use serde_json::json;

use crate::Properties;


#[wasm_bindgen]
impl Properties
{
    pub fn extract_materials(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        let extracted_materials = json!({ "extracted_materials": self.materials });
        let composed_extracted_materials =
            JsValue::from_serde(&extracted_materials)
                .or(Err(JsValue::from("Properties: Extract materials: Materials could not \
                    be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_materials);
        Ok(())
    }


    pub fn extract_truss_sections(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        let extracted_truss_sections = json!(
            { "extracted_truss_sections": self.truss_sections });
        let composed_extracted_truss_sections =
            JsValue::from_serde(&extracted_truss_sections)
                .or(Err(JsValue::from("Properties: Extract truss sections: Truss sections \
                    could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_truss_sections);
        Ok(())
    }


    pub fn extract_beam_sections(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        let extracted_beam_sections = json!(
            { "extracted_beam_sections": self.beam_sections });
        let composed_extracted_beam_sections =
            JsValue::from_serde(&extracted_beam_sections)
                .or(Err(JsValue::from("Properties: Extract beam sections: Beam sections \
                    could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_beam_sections);
        Ok(())
    }


    pub fn extract_properties(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        let extracted_properties = json!(
            { "extracted_properties": self.properties });
        let composed_extracted_properties =
            JsValue::from_serde(&extracted_properties)
                .or(Err(JsValue::from("Properties: Extract properties: Properties \
                    could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_properties);
        Ok(())
    }


    pub fn extract_assigned_properties(&self, handler: js_sys::Function)
        -> Result<(), JsValue>
    {
        let extracted_assigned_properties = json!(
            { "extracted_assigned_properties": self.assigned_properties });
        let composed_extracted_assigned_properties =
            JsValue::from_serde(&extracted_assigned_properties)
                .or(Err(JsValue::from("Properties: Extract assigned properties: \
                    Assigned properties could not be composed for extraction!")))?;
        let this = JsValue::null();
        let _ = handler.call1(&this, &composed_extracted_assigned_properties);
        Ok(())
    }
}
