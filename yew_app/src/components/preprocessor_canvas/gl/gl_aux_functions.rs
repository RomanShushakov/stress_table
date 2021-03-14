use web_sys::{WebGlProgram, WebGlRenderingContext as GL, CanvasRenderingContext2d as CTX};
use vec4;
use std::rc::Rc;
use std::cell::RefCell;

use crate::fem::FENode;
use crate::{ElementsNumbers, ElementsValues};
use crate::{GLElementsValues, GLElementsNumbers};
use crate::auxiliary::{NormalizedNode, FEDrawnElementData};
use crate::components::preprocessor_canvas::gl::gl_aux_structs::
    {
        DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE, DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE,
        CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR, HINT_SHIFT_X, ROTATION_HINT_SHIFT_Y,
        ZOOM_HINT_SHIFT_Y, PAN_HINT_SHIFT_Y,
    };


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


pub fn add_denotation(ctx: &CTX, position: &[f32; 4], matrix: &[f32; 16],
    canvas_width: f32, canvas_height: f32, denotation: &str)
{
    let mut v = vec4::new_zero();
    let clip_space = vec4::transform_mat4(&mut v, position, matrix);
    let pixel_x = (clip_space[0] / clip_space[3] * 0.5 + 0.5) * canvas_width as f32;
    let pixel_y = (clip_space[1] / clip_space[3] * -0.5 + 0.5) * canvas_height as f32;
    ctx.fill_text(denotation, pixel_x as f64, pixel_y as f64).unwrap();
}


fn find_object_min_max_coordinates(nodes: Rc<Vec<Rc<RefCell<FENode<ElementsNumbers,
    ElementsValues>>>>>)
    -> (GLElementsValues, GLElementsValues, GLElementsValues,
        GLElementsValues, GLElementsValues, GLElementsValues)
{
    let mut x_min = nodes[0].borrow().coordinates.x as GLElementsValues;
    let mut x_max = nodes[0].borrow().coordinates.x as GLElementsValues;
    let mut y_min = nodes[0].borrow().coordinates.y as GLElementsValues;
    let mut y_max = nodes[0].borrow().coordinates.y as GLElementsValues;
    let mut z_min = nodes[0].borrow().coordinates.z as GLElementsValues;
    let mut z_max = nodes[0].borrow().coordinates.z as GLElementsValues;
    for i in 1..nodes.len()
    {
        let x = nodes[i].borrow().coordinates.x as GLElementsValues;
        let y = nodes[i].borrow().coordinates.y as GLElementsValues;
        let z = nodes[i].borrow().coordinates.z as GLElementsValues;
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


fn find_min_canvas_side(canvas_width: GLElementsValues, canvas_height: GLElementsValues)
    -> GLElementsValues
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


fn find_min_drawn_object_to_canvas_scale(aspect: GLElementsValues) -> GLElementsValues
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


fn find_max_object_side(x_min: GLElementsValues, x_max: GLElementsValues, y_min: GLElementsValues,
    y_max: GLElementsValues, z_min: GLElementsValues, z_max: GLElementsValues) -> GLElementsValues
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


pub fn normalize_nodes(nodes: Rc<Vec<Rc<RefCell<FENode<ElementsNumbers, ElementsValues>>>>>,
    canvas_width: GLElementsValues, canvas_height: GLElementsValues, aspect: GLElementsValues)
    -> Vec<NormalizedNode>
{
    let mut normalized_nodes = Vec::new();
    let (x_min, x_max, y_min, y_max, z_min, z_max)
        = find_object_min_max_coordinates(Rc::clone(&nodes));
    let min_canvas_side = find_min_canvas_side(canvas_width, canvas_height);
    let min_drawn_object_to_canvas_scale = find_min_drawn_object_to_canvas_scale(aspect);
    let max_object_side = find_max_object_side(x_min, x_max, y_min, y_max, z_min, z_max);
    let multiplier =   min_canvas_side / max_object_side;
    for node in nodes.iter()
    {
        let number = node.borrow().number as GLElementsNumbers;
        let mut x = (node.borrow().coordinates.x as GLElementsValues * multiplier -
            (x_max + x_min) * multiplier / 2.0) / (min_canvas_side  / 2.0) *
            min_drawn_object_to_canvas_scale;
        if x.is_nan()
        {
            x = 0.0;
        }
        let mut y = (node.borrow().coordinates.y as GLElementsValues * multiplier -
            (y_max + y_min) * multiplier / 2.0) / (min_canvas_side / 2.0) *
            min_drawn_object_to_canvas_scale;
        if y.is_nan()
        {
            y = 0.0;
        }
        let mut z = (node.borrow().coordinates.z as GLElementsValues * multiplier -
            (z_max + z_min) * multiplier / 2.0) / (min_canvas_side / 2.0) *
            min_drawn_object_to_canvas_scale;
        if z.is_nan()
        {
            z = 0.0;
        }
        let normalized_node = NormalizedNode { number, x, y, z };
        normalized_nodes.push(normalized_node);
    }
    normalized_nodes
}


pub fn find_node_coordinates(node_number: GLElementsNumbers, normalized_nodes: &Vec<NormalizedNode>)
    -> Result<[GLElementsValues; 3], String>
{
    let node_coordinates =
    {
        if let Some(position) =
            normalized_nodes
                .iter()
                .position(|node|
                    node.number == node_number)
        {
            [normalized_nodes[position].x, normalized_nodes[position].y,
             normalized_nodes[position].z]
        }
        else
        {
            return Err(format!("DrawnObject: Node {} does not exist!", node_number));
        }
    };
    Ok(node_coordinates)
}


pub fn add_hints(ctx: &CTX, canvas_width: f32, canvas_height: f32)
{
    let rotate_hint_x = canvas_width * HINT_SHIFT_X;
    let rotate_hint_y = canvas_height * ROTATION_HINT_SHIFT_Y;
    let rotate_hint = "Rotate - Left Mouse Button";
    ctx.fill_text(rotate_hint, rotate_hint_x as f64, rotate_hint_y as f64).unwrap();
    let zoom_hint_x = canvas_width * HINT_SHIFT_X;
    let zoom_hint_y = canvas_height * ZOOM_HINT_SHIFT_Y;
    let zoom_hint = "Zoom - Mouse Wheel";
    ctx.fill_text(zoom_hint, zoom_hint_x as f64, zoom_hint_y as f64).unwrap();
    let pan_hint_x = canvas_width * HINT_SHIFT_X;
    let pan_hint_y = canvas_height * PAN_HINT_SHIFT_Y;
    let pan_hint = "Pan - Shift + Left Mouse Button";
    ctx.fill_text(pan_hint, pan_hint_x as f64, pan_hint_y as f64).unwrap();
}