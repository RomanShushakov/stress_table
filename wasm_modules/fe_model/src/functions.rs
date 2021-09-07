use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem, AddAssign, SubAssign, MulAssign};
use std::fmt::Debug;

use finite_element_method::my_float::MyFloatTrait;

use extended_matrix::extended_matrix::ExtendedMatrix;
use extended_matrix::functions::copy_element_value;


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


pub fn find_components_of_line_a_perpendicular_to_line_b<T, V>(line_a: &[V; 3],
    line_b: &[V; 3], tolerance: V) -> Result<[V; 3], JsValue>
    where T: Copy + Debug + From<u8> + Eq + Hash + Add<Output = T> + Mul<Output = T> +
             Sub<Output = T> + Div<Output = T> + Rem<Output = T> + AddAssign + SubAssign +
             PartialOrd + 'static,
          V: Copy + Debug + From<f32> + Into<f64> + Add<Output = V> + Sub<Output = V> +
             Mul<Output = V> + Div<Output = V> + AddAssign + SubAssign + MulAssign + PartialEq +
             MyFloatTrait + 'static,
{
    let a_x = V::from(-1f32) * line_a[0];
    let a_y = V::from(-1f32) * line_a[1];
    let a_z = V::from(-1f32) * line_a[2];

    let a = ExtendedMatrix::create(T::from(3u8),
        T::from(1u8), vec![a_x, a_y, a_z], tolerance);

    let b_x = line_b[0];
    let b_y = line_b[1];
    let b_z = line_b[2];

    let norm = V::from(1f32) / (b_x.my_powi(2) + b_y.my_powi(2) + b_z.my_powi(2));

    let mut coeff_matrix = ExtendedMatrix::create(T::from(3u8),
        T::from(3u8), vec![
            V::from(-1f32) * b_z * b_z - b_y * b_y, b_x * b_y, b_x * b_z,
            b_y * b_x, V::from(-1f32) * b_x * b_x - b_z * b_z,	b_y * b_z,
            b_z * b_x,	b_z * b_y, V::from(-1f32) * b_y * b_y - b_x * b_x,
        ], tolerance);

    coeff_matrix.multiply_by_number(norm);

    let components_of_line_a_perpendicular_to_line_b_matrix = coeff_matrix
        .multiply_by_matrix(&a)
        .map_err(|e| JsValue::from(e))?;

    let components_of_line_a_perpendicular_to_line_b_all_values =
        components_of_line_a_perpendicular_to_line_b_matrix.copy_all_elements_values();

    let a_perpendicular_to_b_x = copy_element_value(T::from(0u8), T::from(0u8),
        &components_of_line_a_perpendicular_to_line_b_all_values);

    let a_perpendicular_to_b_y = copy_element_value(T::from(1u8), T::from(0u8),
        &components_of_line_a_perpendicular_to_line_b_all_values);

    let a_perpendicular_to_b_z = copy_element_value(T::from(2u8), T::from(0u8),
        &components_of_line_a_perpendicular_to_line_b_all_values);

    Ok([a_perpendicular_to_b_x, a_perpendicular_to_b_y, a_perpendicular_to_b_z])
}
