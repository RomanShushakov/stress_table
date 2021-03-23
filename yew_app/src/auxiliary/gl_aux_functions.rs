use web_sys::{WebGlProgram, WebGlRenderingContext as GL, CanvasRenderingContext2d as CTX};
use vec4;
use std::rc::Rc;
use std::cell::RefCell;

use crate::fem::{FENode, GlobalAnalysisResult, GlobalDOFParameter, Displacements, BCType};
use crate::{ElementsNumbers, ElementsValues, UIDNumbers};
use crate::{GLElementsValues, GLElementsNumbers};
use crate::auxiliary::{NormalizedNode, FEDrawnElementData, FEDrawnNodeData, FEDrawnAnalysisResultNodeData, DrawnAnalysisResultElementData};

use crate::auxiliary::gl_aux_structs::GLMode;
use crate::auxiliary::gl_aux_structs::
    {
        DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE, DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE,
        CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR, HINT_SHIFT_X, ROTATION_HINT_SHIFT_Y,
        ZOOM_HINT_SHIFT_Y, PAN_HINT_SHIFT_Y, DRAWN_OBJECT_SELECTED_COLOR,
        DRAWN_OBJECT_UNDER_CURSOR_COLOR, CANVAS_DRAWN_OBJECT_SELECTED_DENOTATION_COLOR,
        CANVAS_DRAWN_OBJECT_UNDER_CURSOR_DENOTATION_COLOR, DISPLACEMENT_SHIFT_X,
        DISPLACEMENT_HEADER_SHIFT_Y, MIN_DISPLACEMENT_SHIFT_Y, MAX_DISPLACEMENT_SHIFT_Y,
        REACTION_SHIFT_X, REACTION_HEADER_SHIFT_Y, STRESS_SHIFT_X, STRESS_HEADER_SHIFT_Y,
        STRESS_COMPONENT_SHIFT_Y, COLOR_BAR_SHIFT_X, COLOR_BAR_Y_BOTTOM, COLOR_BAR_Y_TOP,
        COLOR_BAR_WIDTH,
    };
use crate::auxiliary::aux_functions::transform_u32_to_array_of_u8;
use crate::fem::global_analysis::fe_global_analysis_result::Reactions;
use crate::fem::element_analysis::fe_element_analysis_result::ElementsAnalysisResult;


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


fn find_object_min_max_coordinates(nodes: Rc<Vec<FEDrawnNodeData>>,)
    -> (GLElementsValues, GLElementsValues, GLElementsValues,
        GLElementsValues, GLElementsValues, GLElementsValues)
{
    let mut x_min = nodes[0].x as GLElementsValues;
    let mut x_max = nodes[0].x as GLElementsValues;
    let mut y_min = nodes[0].y as GLElementsValues;
    let mut y_max = nodes[0].y as GLElementsValues;
    let mut z_min = nodes[0].z as GLElementsValues;
    let mut z_max = nodes[0].z as GLElementsValues;
    for i in 1..nodes.len()
    {
        let x = nodes[i].x as GLElementsValues;
        let y = nodes[i].y as GLElementsValues;
        let z = nodes[i].z as GLElementsValues;
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


pub fn normalize_nodes(nodes: Rc<Vec<FEDrawnNodeData>>, canvas_width: GLElementsValues,
    canvas_height: GLElementsValues)
    -> Vec<NormalizedNode>
{
    let aspect = canvas_width / canvas_height;
    let mut normalized_nodes = Vec::new();
    let (x_min, x_max, y_min, y_max, z_min, z_max)
        = find_object_min_max_coordinates(Rc::clone(&nodes));
    let min_canvas_side = find_min_canvas_side(canvas_width, canvas_height);
    let min_drawn_object_to_canvas_scale = find_min_drawn_object_to_canvas_scale(aspect);
    let max_object_side = find_max_object_side(x_min, x_max, y_min, y_max, z_min, z_max);
    let multiplier =   min_canvas_side / max_object_side;
    for node in nodes.iter()
    {
        let uid = node.uid;
        let number = node.number as GLElementsNumbers;
        let mut x = (node.x as GLElementsValues * multiplier -
            (x_max + x_min) * multiplier / 2.0) / (min_canvas_side  / 2.0) *
            min_drawn_object_to_canvas_scale;
        if x.is_nan()
        {
            x = 0.0;
        }
        let mut y = (node.y as GLElementsValues * multiplier -
            (y_max + y_min) * multiplier / 2.0) / (min_canvas_side / 2.0) *
            min_drawn_object_to_canvas_scale;
        if y.is_nan()
        {
            y = 0.0;
        }
        let mut z = (node.z as GLElementsValues * multiplier -
            (z_max + z_min) * multiplier / 2.0) / (min_canvas_side / 2.0) *
            min_drawn_object_to_canvas_scale;
        if z.is_nan()
        {
            z = 0.0;
        }
        let normalized_node = NormalizedNode { uid, number, x, y, z };
        normalized_nodes.push(normalized_node);
    }
    normalized_nodes
}


pub fn find_node_coordinates(node_number: GLElementsNumbers, normalized_nodes: &[NormalizedNode])
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
    let hint_x = canvas_width * HINT_SHIFT_X;
    let rotate_hint_y = canvas_height * ROTATION_HINT_SHIFT_Y;
    let rotate_hint = "Rotate - Left Mouse Button";
    ctx.fill_text(rotate_hint, hint_x as f64, rotate_hint_y as f64).unwrap();
    let zoom_hint_y = canvas_height * ZOOM_HINT_SHIFT_Y;
    let zoom_hint = "Zoom - Mouse Wheel";
    ctx.fill_text(zoom_hint, hint_x as f64, zoom_hint_y as f64).unwrap();
    let pan_hint_y = canvas_height * PAN_HINT_SHIFT_Y;
    let pan_hint = "Pan - Shift + Left Mouse Button";
    ctx.fill_text(pan_hint, hint_x as f64, pan_hint_y as f64).unwrap();
}


pub fn add_displacements_hints(ctx: &CTX, canvas_width: f32, canvas_height: f32,
    min_displacement: ElementsValues, max_displacement: ElementsValues)
{
    let displacement_shift_x = canvas_width * DISPLACEMENT_SHIFT_X;
    let displacement_header_shift_y = canvas_height * DISPLACEMENT_HEADER_SHIFT_Y;
    let displacement_header = "DISPLACEMENTS";
    ctx.fill_text(displacement_header, displacement_shift_x as f64,
        displacement_header_shift_y as f64).unwrap();
    let min_displacement_y = canvas_height * MIN_DISPLACEMENT_SHIFT_Y;
    let min_displacement_hint = &format!("Min displacement: {:+.5e}", min_displacement);
    ctx.fill_text(min_displacement_hint, displacement_shift_x as f64,
        min_displacement_y as f64).unwrap();
    let max_displacement_y = canvas_height * MAX_DISPLACEMENT_SHIFT_Y;
    let max_displacement_hint = &format!("Max displacement: {:+.5e}", max_displacement);
    ctx.fill_text(max_displacement_hint, displacement_shift_x as f64,
        max_displacement_y as f64).unwrap();
}


pub fn add_reactions_hints(ctx: &CTX, canvas_width: f32, canvas_height: f32)
{
    let reaction_shift_x = canvas_width * REACTION_SHIFT_X;
    let reaction_header_shift_y = canvas_height * REACTION_HEADER_SHIFT_Y;
    let reaction_header = "REACTIONS";
    ctx.fill_text(reaction_header, reaction_shift_x as f64,
        reaction_header_shift_y as f64).unwrap();
}


pub fn add_stresses_hints(ctx: &CTX, canvas_width: f32, canvas_height: f32,
    stress_component: String)
{
    let stress_shift_x = canvas_width * STRESS_SHIFT_X;
    let stress_header_shift_y = canvas_height * STRESS_HEADER_SHIFT_Y;
    let stress_header = "STRESSES";
    ctx.fill_text(stress_header, stress_shift_x as f64,
        stress_header_shift_y as f64).unwrap();
    let stress_component_shift_y = canvas_height * STRESS_COMPONENT_SHIFT_Y;
    let stress_component = &format!("Component: {}", stress_component);
    ctx.fill_text(stress_component, stress_shift_x as f64,
        stress_component_shift_y as f64).unwrap();
}


pub fn add_color_bar(ctx: &CTX, canvas_width: f32, canvas_height: f32)
{
    ctx.begin_path();
    let gradient: web_sys::CanvasGradient = ctx
        .create_linear_gradient(
            (canvas_width * COLOR_BAR_SHIFT_X) as f64,
            (canvas_height * COLOR_BAR_Y_BOTTOM) as f64,
            (canvas_width * COLOR_BAR_SHIFT_X) as f64,
            (canvas_height * COLOR_BAR_Y_TOP) as f64,
        );
    gradient.add_color_stop(0f32, "rgb(0, 0, 255)").unwrap();
    gradient.add_color_stop(0.25, "rgb(0, 255, 255)").unwrap();
    gradient.add_color_stop(0.5, "rgb(0, 255, 0)").unwrap();
    gradient.add_color_stop(0.75, "rgb(255, 255, 0)").unwrap();
    gradient.add_color_stop(1f32, "rgb(255, 0, 0)").unwrap();
    ctx.set_fill_style(&gradient.into());
    ctx.fill_rect(
        (canvas_width * COLOR_BAR_SHIFT_X) as f64,
        (canvas_height * COLOR_BAR_Y_TOP) as f64,
        (canvas_width * COLOR_BAR_WIDTH) as f64,
        (canvas_height * (COLOR_BAR_Y_BOTTOM - COLOR_BAR_Y_TOP)) as f64,
    );
    ctx.stroke();

    // context.begin_path();
    // context.set_fill_style(&CANVAS_NODES_COLOR.into());
    // context.set_font(&format!("{}px Serif", axis_line_length / 8f64));
    // context.fill_text(
    //     "Stress,",
    //     x_origin,
    //     self.props.canvas_height as f64 * 0.07)
    //     .unwrap();
    // context.fill_text(
    //     "Component: XX",
    //     x_origin,
    //     self.props.canvas_height as f64 * 0.09)
    //     .unwrap();
    // context.stroke();
    //
    // context.begin_path();
    // context.set_fill_style(&CANVAS_NODES_COLOR.into());
    // context.set_font(&format!("{}px Serif", axis_line_length / 8f64));
    // context.fill_text(
    //     &format!("{:+.3e}", min_max_values.max_value),
    //     x_origin + self.props.canvas_width as f64 * 0.025,
    //     self.props.canvas_height as f64 * 0.1 + axis_line_length / 16f64)
    //     .unwrap();
    // context.fill_text(
    //     &format!("{:+.3e}", min_max_values.min_value),
    //     x_origin + self.props.canvas_width as f64 * 0.025,
    //     self.props.canvas_height as f64 * 0.35 + axis_line_length / 16f64)
    //     .unwrap();
    // context.stroke();
}



pub fn extend_by_elements_analysis_result(
    elements_analysis_result: &ElementsAnalysisResult<ElementsNumbers, ElementsValues>,
    uid_number: &mut UIDNumbers,
    drawn_analysis_results_for_elements: &mut Vec<DrawnAnalysisResultElementData>)
{
    let mut uid = *uid_number;
    let elements_analysis_data =
        elements_analysis_result.extract_elements_analysis_data();
    for i in 0..elements_analysis_data.len()
    {
        uid += 1;
        let drawn_analysis_result_element_data = DrawnAnalysisResultElementData
            {
                uid,
                element_analysis_data: elements_analysis_data[i].to_owned(),
            };
        drawn_analysis_results_for_elements.push(drawn_analysis_result_element_data);
    }
    *uid_number = uid;
}


pub fn define_drawn_object_color(gl_mode: &GLMode, uid: UIDNumbers,
    selected_color: &[u8; 4], under_cursor_color: &[u8; 4],
    initial_color: &[GLElementsValues; 4]) -> [GLElementsValues; 4]
{
    match gl_mode
    {
        GLMode::Selection =>
            {
                let color = transform_u32_to_array_of_u8(uid);
                let updated_color = [color[0] as GLElementsValues / 255.0,
                    color[1] as GLElementsValues / 255.0,
                    color[2] as GLElementsValues / 255.0,
                    color[3] as GLElementsValues / 255.0];
                    updated_color
            },
        GLMode::Visible =>
            {
                let transformed_uid = transform_u32_to_array_of_u8(uid);
                if transformed_uid == *selected_color
                {
                    DRAWN_OBJECT_SELECTED_COLOR
                }
                else if transformed_uid == *under_cursor_color
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


pub fn define_drawn_object_denotation_color(uid: UIDNumbers, selected_color: &[u8; 4],
    under_cursor_color: &[u8; 4], initial_denotation_color: &str) -> String
{
    let transformed_uid = transform_u32_to_array_of_u8(uid);
    if transformed_uid == *selected_color
    {
        CANVAS_DRAWN_OBJECT_SELECTED_DENOTATION_COLOR.to_string()
    }
    else if transformed_uid == *under_cursor_color
    {
        CANVAS_DRAWN_OBJECT_UNDER_CURSOR_DENOTATION_COLOR.to_string()
    }
    else
    {
        initial_denotation_color.to_string()
    }
}


pub fn define_color_value(value: ElementsValues, min_value: ElementsValues,
    max_value: ElementsValues) -> ElementsValues
{
    if (value - min_value) * 1020.0 / (max_value - min_value) > 1020.0
    {
        1020.0
    }
    else
    {
        (value - min_value) * 1020.0 / (max_value - min_value)
    }
}


pub fn define_color_array_by_value(color_value: ElementsValues) -> [GLElementsValues; 4]
{
    let color_value = color_value as GLElementsValues;
    if  (color_value >= 0.0) && (color_value <= 255.0)
    {
        [0.0, color_value / 255.0, 1.0, 1.0]
    }
    else if (color_value > 255.0) && (color_value <= 510.0)
    {
        [0.0, 1.0, (510.0 - color_value) / 255.0, 1.0]
    }
    else if (color_value > 510.0) && (color_value <= 765.0)
    {
        [(color_value - 510.0) / 255.0, 1.0, 0.0, 1.0]
    }
    else if (color_value > 765.0) && (color_value <= 1020.0)
    {
        [1.0, (1020.0 - color_value) / 255.0, 0.0, 1.0]
    }
    else
    {
        [1.0, 0.0, 0.0, 1.0]
    }
}


fn proportion(elem_length: ElementsValues, elem_range: ElementsValues,
    point_range: ElementsValues) -> ElementsValues
{
    elem_length * point_range / elem_range
}


fn define_element_chunks_in_length(a: ElementsValues, b: ElementsValues, elem_length: ElementsValues)
    -> (Vec<ElementsValues>, Vec<ElementsValues>)
{
    let elem_range = b - a;
    let transition_points = [0.0, 255.0, 510.0, 765.0, 1020.0];
    let mut points_color = vec![a];
    let mut points_dist = vec![0.0];
    let mut point = a;
    let mut point_range = 0.0;
    for i in transition_points.iter()
    {
        if a < *i
        {
            point = *i;
            point_range = point - a;
            break;
        }
    }
    while point < b
    {
        let point_dist = proportion(elem_length, elem_range, point_range);
        points_color.push(point);
        points_dist.push(point_dist);
        point += 255.0;
        point_range += 255.0;
    }
    points_color.push(b);
    points_dist.push(elem_length);
    (points_color, points_dist)
}


pub fn define_element_chunks(end_points_color: &[ElementsValues],
    end_points_coord: &[&[GLElementsValues]])
    -> (Vec<ElementsValues>, Vec<[GLElementsValues; 3]>)
{
    let a = end_points_color[0];
    let b = end_points_color[1];
    let x1 = end_points_coord[0][0];
    let x2 = end_points_coord[1][0];
    let y1 = end_points_coord[0][1];
    let y2 = end_points_coord[1][1];
    let z1 = end_points_coord[0][2];
    let z2 = end_points_coord[1][2];
    let elem_length =
        ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt();
    let (points_color, points_dist) =
        define_element_chunks_in_length(a, b, elem_length as ElementsValues);
    let mut point_coord = Vec::new();
    for point in points_dist.iter()
    {
        let x = x1 + (x2 - x1) * *point as GLElementsValues / elem_length;
        let y = y1 + (y2 - y1) * *point as GLElementsValues / elem_length;
        let z = z1 + (z2 - z1) * *point as GLElementsValues / elem_length;
        point_coord.push([x, y, z]);
    }
    (points_color, point_coord)
}
