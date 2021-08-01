use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext as GL, CanvasRenderingContext2d as CTX};
use vec4;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use rand;

use extended_matrix::extended_matrix::ExtendedMatrix;

use crate::point_object::{PointObject, PointObjectKey, Coordinates};

use crate::line_object::{LineObject, LineObjectKey};

use crate::drawn_object::drawn_object::{GLMode};
use crate::drawn_object::drawn_object::
{
    HINT_SHIFT_X, ROTATION_HINT_SHIFT_Y, ZOOM_HINT_SHIFT_Y, PAN_HINT_SHIFT_Y,
    DRAWN_OBJECT_SELECTED_COLOR, CANVAS_DRAWN_OBJECT_SELECTED_DENOTATION_COLOR,
    CANVAS_DRAWN_OBJECT_UNDER_CURSOR_DENOTATION_COLOR, DRAWN_OBJECT_UNDER_CURSOR_COLOR,
};
use crate::drawn_object::consts::
{
    DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE, DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE,
};

use crate::consts::TOLERANCE;

use crate::log;



pub fn initialize_shaders(gl: &GL, vertex_shader_code: &str, fragment_shader_code: &str)
    -> WebGlProgram
{
    let vertex_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    gl.shader_source(&vertex_shader, &vertex_shader_code);
    gl.compile_shader(&vertex_shader);
    let fragment_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
    gl.shader_source(&fragment_shader, &fragment_shader_code);
    gl.compile_shader(&fragment_shader);
    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vertex_shader);
    gl.attach_shader(&shader_program, &fragment_shader);
    gl.link_program(&shader_program);
    gl.use_program(Some(&shader_program));
    shader_program
}


fn find_object_min_max_coordinates(point_objects: &HashMap<PointObjectKey, PointObject>)
    -> (f32, f32, f32, f32, f32, f32)
{
    let point_objects = point_objects.values().collect::<Vec<&PointObject>>();
    let mut x_min = point_objects[0].get_x();
    let mut x_max = point_objects[0].get_x();
    let mut y_min = point_objects[0].get_y();
    let mut y_max = point_objects[0].get_y();
    let mut z_min = point_objects[0].get_z();
    let mut z_max = point_objects[0].get_z();
    for i in 1..point_objects.len()
    {
        let x = point_objects[i].get_x();
        let y = point_objects[i].get_y();
        let z = point_objects[i].get_z();
        if x < x_min
        {
            x_min = x;
        }
        if x > x_max
        {
            x_max = x;
        }
        if y < y_min
        {
            y_min = y;
        }
        if y > y_max
        {
            y_max = y;
        }
        if z < z_min
        {
            z_min = z;
        }
        if z > z_max
        {
            z_max = z;
        }
    }
    (x_min, x_max, y_min, y_max, z_min, z_max)
}


fn find_min_canvas_side(canvas_width: f32, canvas_height: f32) -> f32
{
    if canvas_width * DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE <
        canvas_height * DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE
    {
        canvas_width * DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE
    }
    else
    {
        canvas_height * DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE
    }
}


fn find_min_drawn_object_to_canvas_scale(aspect: f32) -> f32
{
    let mut min_drawn_object_to_canvas_scale =
    {
        if DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE < DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE
        {
            DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE
        }
        else
        {
            DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE
        }
    };
    if aspect < 1.0
    {
        min_drawn_object_to_canvas_scale *= aspect;
    }
    else
    {
        min_drawn_object_to_canvas_scale /= aspect;
    }
    min_drawn_object_to_canvas_scale
}


fn find_max_object_side(x_min: f32, x_max: f32, y_min: f32,
    y_max: f32, z_min: f32, z_max: f32) -> f32
{
    let mut max_object_side = (x_max - x_min).abs();
    if (y_max - y_min).abs() > max_object_side
    {
        max_object_side = (y_max - y_min).abs();
    }
    if (z_max - z_min).abs() > max_object_side
    {
        max_object_side = (z_max - z_min).abs();
    }
    max_object_side
}


pub fn normalize_point_objects_coordinates(point_objects: &mut HashMap<PointObjectKey, PointObject>,
    line_objects: &HashMap<LineObjectKey, LineObject>, canvas_width: f32, canvas_height: f32)
{
    let aspect = canvas_width / canvas_height;

    let (x_min, x_max, y_min, y_max, z_min, z_max)
        = find_object_min_max_coordinates(point_objects);
    let min_canvas_side = find_min_canvas_side(canvas_width, canvas_height);
    let min_drawn_object_to_canvas_scale = find_min_drawn_object_to_canvas_scale(aspect);
    let max_object_side = find_max_object_side(x_min, x_max, y_min, y_max, z_min, z_max);
    let multiplier = min_canvas_side / max_object_side;
    let point_objects_for_uid = point_objects.clone();
    for point_object in point_objects.values_mut()
    {
        let mut x = (point_object.get_x() * multiplier -
            (x_max + x_min) * multiplier / 2.0) / (min_canvas_side  / 2.0) *
            min_drawn_object_to_canvas_scale;
        if x.is_nan()
        {
            x = 0.0;
        }
        let mut y = (point_object.get_y() * multiplier -
            (y_max + y_min) * multiplier / 2.0) / (min_canvas_side / 2.0) *
            min_drawn_object_to_canvas_scale;
        if y.is_nan()
        {
            y = 0.0;
        }
        let mut z = (point_object.get_z() * multiplier -
            (z_max + z_min) * multiplier / 2.0) / (min_canvas_side / 2.0) *
            min_drawn_object_to_canvas_scale;
        if z.is_nan()
        {
            z = 0.0;
        }
        if point_object.normalized_coordinates_is_some()
        {
            point_object.update_normalized_coordinates(x, y, z);
        }
        else
        {
            let uid =
            {
                let mut current_uid = rand::random::<u32>();
                while point_objects_for_uid.values().position(|point_object|
                        point_object.uid_same(current_uid)).is_some() ||
                    line_objects.values().position(|line_object|
                        line_object.uid_same(current_uid)).is_some() || current_uid == 255
                {
                    current_uid = rand::random::<u32>();
                }
                current_uid
            };
            let normalized_coordinates = Coordinates::create(x, y, z);
            point_object.add_normalized_coordinates(normalized_coordinates);
            point_object.add_uid(uid);
        }
    }
}


pub fn add_denotation(ctx: &CTX, position: &[f32; 4], matrix: &[f32; 16],
    canvas_width: f32, canvas_height: f32, denotation: &str)
{
    let mut v = vec4::new_zero();
    let clip_space = vec4::transform_mat4(&mut v, position, matrix);
    let pixel_x = (clip_space[0] / clip_space[3] * 0.5 + 0.5) * canvas_width as f32;
    let pixel_y = (clip_space[1] / clip_space[3] * -0.5 + 0.5) * canvas_height as f32;
    ctx.fill_text(denotation, pixel_x as f64, pixel_y as f64).unwrap();
}


pub fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4]
{
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}


pub fn define_drawn_object_color(gl_mode: &GLMode, uid: u32, selected_colors: &HashSet<[u8; 4]>,
    under_selection_box_colors: &Vec<u8>, initial_color: &[f32; 4]) -> [f32; 4]
{
    match gl_mode
    {
        GLMode::Selection =>
            {
                let color = transform_u32_to_array_of_u8(uid);
                let updated_color = [color[0] as f32 / 255.0,
                    color[1] as f32 / 255.0,
                    color[2] as f32 / 255.0,
                    color[3] as f32 / 255.0];
                    updated_color
            },
        GLMode::Visible =>
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                if selected_colors.iter()
                    .position(|color| transformed_uid == *color)
                    .is_some()
                {
                    DRAWN_OBJECT_SELECTED_COLOR
                }
                else if under_selection_box_colors.as_slice()
                    .chunks(4)
                    .position(|color| transformed_uid == *color).is_some()
                {
                    DRAWN_OBJECT_UNDER_CURSOR_COLOR
                }
                else
                {
                    *initial_color
                }
            }
    }
}


pub fn define_drawn_object_denotation_color(uid: u32, selected_colors: &HashSet<[u8; 4]>,
    under_selection_box_colors: &Vec<u8>, initial_denotation_color: &str) -> String
{
    let transformed_uid = transform_u32_to_array_of_u8(uid);
    if selected_colors.iter()
        .position(|color| transformed_uid == *color)
        .is_some()
    {
        CANVAS_DRAWN_OBJECT_SELECTED_DENOTATION_COLOR.to_string()
    }
    else if under_selection_box_colors.as_slice()
        .chunks(4)
        .position(|color| transformed_uid == *color).is_some()
    {
        CANVAS_DRAWN_OBJECT_UNDER_CURSOR_DENOTATION_COLOR.to_string()
    }
    else
    {
        initial_denotation_color.to_string()
    }
}


pub fn add_hints(ctx: &CTX, canvas_width: f32, canvas_height: f32)
{
    let hint_x = canvas_width * HINT_SHIFT_X;
    let rotate_hint_y = canvas_height * ROTATION_HINT_SHIFT_Y;
    let rotate_hint = "Rotate - (Ctrl + Alt + MB1)";
    ctx.fill_text(rotate_hint, hint_x as f64, rotate_hint_y as f64).unwrap();
    let zoom_hint_y = canvas_height * ZOOM_HINT_SHIFT_Y;
    let zoom_hint = "Zoom - (Ctrl + Alt + MB3) or Mouse Wheel";
    ctx.fill_text(zoom_hint, hint_x as f64, zoom_hint_y as f64).unwrap();
    let pan_hint_y = canvas_height * PAN_HINT_SHIFT_Y;
    let pan_hint = "Pan - (Ctrl + Alt + MB2)";
    ctx.fill_text(pan_hint, hint_x as f64, pan_hint_y as f64).unwrap();
}


pub fn dispatch_custom_event(detail: serde_json::Value, event_type: &str, query_selector: &str)
    -> Result<(), JsValue>
{
    let custom_event = web_sys::CustomEvent::new_with_event_init_dict(
        event_type,
        web_sys::CustomEventInit::new()
            .bubbles(true)
            .composed(true)
            .detail(&JsValue::from_serde(&detail)
                .or(Err("Renderer: Dispatch event: detail could not be \
                converted into JsValue!"))?))
                    .or(Err(JsValue::from("Renderer: Dispatch event: \
                    custom event could not be constructed!")))?;
    web_sys::window().expect("no global `window` exists")
        .document().expect("should have a document on window")
        .query_selector(query_selector).or(Err(JsValue::from("Renderer: Dispatch event: No \
            element find by current selector!")))?.unwrap()
        .dyn_into::<web_sys::EventTarget>().unwrap()
        .dispatch_event(&custom_event)?;
    Ok(())
}


pub fn compare_with_tolerance(value: f32) -> f32
{
    if value.abs() < TOLERANCE { 0.0 } else { value }
}


pub fn convert_into_array<T, const N: usize>(v: Vec<T>) -> [T; N]
{
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}


pub fn compose_rotation_matrix_for_vector(vector_start_point_coordinates: [f32; 3],
    vector_end_point_coordinates: [f32; 3]) -> ExtendedMatrix<u32, f32>
{
    let x = (vector_end_point_coordinates[0] - vector_start_point_coordinates[0]);
    let y = (vector_end_point_coordinates[1] - vector_start_point_coordinates[1]);
    let z = (vector_end_point_coordinates[2] - vector_start_point_coordinates[2]);
    let vector_length = f32::sqrt(x.powi(2) + y.powi(2) + z.powi(2));
    let (u, v, w) = (vector_length, 0.0, 0.0);
    let alpha = ((x * u + y * v + z * w) / (vector_length.powi(2))).acos();
    let (rotation_axis_coord_x, mut rotation_axis_coord_y,
        mut rotation_axis_coord_z) = (0f32, 0f32, 0f32);
    if x != 0.0 && y == 0.0 && z == 0.0
    {
        rotation_axis_coord_z = x;
    }
    else
    {
        rotation_axis_coord_y = z * vector_length;
        rotation_axis_coord_z = - y * vector_length;
    }
    let norm = 1.0 / (rotation_axis_coord_x.powi(2) +
        rotation_axis_coord_y.powi(2) + rotation_axis_coord_z.powi(2)).sqrt();
    let (x_n, y_n, z_n) = (rotation_axis_coord_x * norm,
        rotation_axis_coord_y * norm, rotation_axis_coord_z * norm);
    let (c, s) = (alpha.cos(), alpha.sin());
    let t = 1.0 - c;
    let q_11 = compare_with_tolerance(t * x_n * x_n + c);
    let q_12 = compare_with_tolerance(t * x_n * y_n - z_n * s);
    let q_13 = compare_with_tolerance(t * x_n * z_n + y_n * s);
    let q_21 = compare_with_tolerance(t * x_n * y_n + z_n * s);
    let q_22 = compare_with_tolerance(t * y_n * y_n + c);
    let q_23 = compare_with_tolerance(t * y_n * z_n - x_n * s);
    let q_31 = compare_with_tolerance(t * x_n * z_n - y_n * s);
    let q_32 = compare_with_tolerance(t * y_n * z_n + x_n * s);
    let q_33 = compare_with_tolerance(t * z_n * z_n + c);
    let mut rotation_matrix = ExtendedMatrix::create(3,
        3, vec![q_11, q_12, q_13, q_21, q_22, q_23, q_31, q_32, q_33],
        TOLERANCE);
    rotation_matrix
}
