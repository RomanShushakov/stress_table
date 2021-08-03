use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use extended_matrix::extended_matrix::ExtendedMatrix;
use extended_matrix::functions::extract_element_value;

use crate::types::{FEFloat, FEUInt};

use crate::consts::TOLERANCE;


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(value: &str);
}


pub fn dispatch_custom_event(detail: serde_json::Value, event_type: &str, query_selector: &str)
    -> Result<(), JsValue>
{
    let custom_event = web_sys::CustomEvent::new_with_event_init_dict(event_type,
        web_sys::CustomEventInit::new()
            .bubbles(true)
            .composed(true)
            .detail(&JsValue::from_serde(&detail).or(Err("Geometry: Dispatch event: \
                detail could not be converted into JsValue!"))?))
            .or(Err(JsValue::from("Geometry: Dispatch event: custom event could not be \
                constructed!")))?;
    web_sys::window().expect("no global `window` exists")
        .document().expect("should have a document on window")
        .query_selector(query_selector).or(Err(JsValue::from("Geometry: Dispatch event: No \
            element find by current selector!")))?
        .unwrap()
        .dyn_into::<web_sys::EventTarget>()
        .unwrap()
        .dispatch_event(&custom_event)?;
    Ok(())
}


pub fn find_components_of_line_a_perpendicular_to_line_b(line_a: &[FEFloat; 3],
    line_b: &[FEFloat; 3]) -> Result<[FEFloat; 3], JsValue>
{
    let a_x = - line_a[0];
    let a_y = - line_a[1];
    let a_z = - line_a[2];
    let a = ExtendedMatrix::create(3 as FEUInt,
        1 as FEUInt, vec![a_x, a_y, a_z], TOLERANCE);
    let b_x = line_b[0];
    let b_y = line_b[1];
    let b_z = line_b[2];
    let coeff_matrix = ExtendedMatrix::create(3 as FEUInt,
        3 as FEUInt, vec![
            - b_z * b_z - b_y * b_y, b_x * b_y, b_x * b_z,
            b_y * b_x, - b_x * b_x - b_z * b_z,	b_y * b_z,
            b_z * b_x,	b_z * b_y, - b_y * b_y - b_x * b_x,
        ], TOLERANCE);
    let components_of_line_a_perpendicular_to_line_b_matrix = coeff_matrix
        .multiply_by_matrix(&a)
        .map_err(|e| JsValue::from(e))?;
    let components_of_line_a_perpendicular_to_line_b_all_values =
        components_of_line_a_perpendicular_to_line_b_matrix.extract_all_elements_values();
    let a_perpendicular_to_b_x = extract_element_value(
        0, 0, &components_of_line_a_perpendicular_to_line_b_all_values);
    let a_perpendicular_to_b_y = extract_element_value(
        1, 0, &components_of_line_a_perpendicular_to_line_b_all_values);
    let a_perpendicular_to_b_z = extract_element_value(
        2, 0, &components_of_line_a_perpendicular_to_line_b_all_values);
    Ok([a_perpendicular_to_b_x, a_perpendicular_to_b_y, a_perpendicular_to_b_z])
}
