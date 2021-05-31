use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::
{
    WebGlProgram, WebGlRenderingContext as GL, WebGlShader, CanvasRenderingContext2d as CTX
};
use mat4;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

mod line_object;
use line_object::{LineObject, LineObjectKey};
use line_object::{LineObjectType};

mod aux_functions;
use aux_functions::
{
    initialize_shaders, add_denotation, add_hints, normalize_point_objects_coordinates,
    define_drawn_object_denotation_color, transform_u32_to_array_of_u8,
    dispatch_custom_event, convert_into_array
};

mod extended_matrix;

mod buffers;
use buffers::Buffers;

mod shaders_variables;
use shaders_variables::ShadersVariables;

mod drawn_object;
use drawn_object::DrawnObject;
use drawn_object::{CSAxis, GLMode};
use drawn_object::
    {
        CS_AXES_Y_SHIFT, CS_AXES_X_SHIFT, CS_AXES_Z_SHIFT, CS_AXES_SCALE,
        CS_AXES_CAPS_BASE_POINTS_NUMBER, CS_AXES_CAPS_WIDTH, CS_AXES_CAPS_HEIGHT,
        AXIS_X_DENOTATION_SHIFT_X, AXIS_X_DENOTATION_SHIFT_Y, AXIS_Y_DENOTATION_SHIFT_X,
        AXIS_Y_DENOTATION_SHIFT_Y, AXIS_Z_DENOTATION_SHIFT_X, AXIS_Z_DENOTATION_SHIFT_Y,
        AXIS_Z_DENOTATION_SHIFT_Z, CANVAS_AXES_DENOTATION_COLOR,
        CANVAS_DRAWN_NODES_DENOTATION_COLOR, DRAWN_POINT_OBJECT_DENOTATION_SHIFT,
        CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR, DRAWN_DISPLACEMENTS_CAPS_BASE_POINTS_NUMBER,
        DRAWN_DISPLACEMENTS_CAPS_HEIGHT, DRAWN_DISPLACEMENTS_CAPS_WIDTH,
        CANVAS_DRAWN_DISPLACEMENTS_DENOTATION_COLOR, DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_X,
        DRAWN_DISPLACEMENTS_DENOTATION_SHIFT_Y, DRAWN_FORCES_LINE_LENGTH, DRAWN_FORCES_CAPS_HEIGHT,
        DRAWN_FORCES_CAPS_WIDTH, DRAWN_FORCES_CAPS_BASE_POINTS_NUMBER,
        CANVAS_DRAWN_FORCES_DENOTATION_COLOR, DRAWN_FORCES_DENOTATION_SHIFT_X,
        DRAWN_FORCES_DENOTATION_SHIFT_Y, HINTS_COLOR, DRAWN_LINE_OBJECTS_DENOTATION_SHIFT,
        CANVAS_DRAWN_POINTS_DENOTATION_COLOR, DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
        DRAWN_LINE_OBJECTS_BASE_RADIUS, CANVAS_DRAWN_LINES_DENOTATION_COLOR,
        SELECTION_RECTANGLE_STROKE_COLOR, SELECTION_RECTANGLE_FILL_COLOR,
    };

mod methods_for_canvas_manipulation;

mod point_object;
use point_object::{PointObjectKey, PointObject, Coordinates};
use point_object::{PointObjectType};



pub const TOLERANCE: f32 = 1e-6;
pub type ElementsNumbers = u32;
pub type ElementsValues = f32;

const EVENT_TARGET: &str = "fea-app";
const CLIENT_MESSAGE_EVENT_NAME: &str = "clientMessage";


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}


struct Props
{
    canvas_text: web_sys::HtmlCanvasElement,
    canvas_gl: web_sys::HtmlCanvasElement,
    cursor_coord_x: i32,
    cursor_coord_y: i32,
    theta: f32,
    phi: f32,
    dx: f32,
    dy: f32,
    d_scale: f32,
    point_objects: HashMap<PointObjectKey, PointObject>,
}


struct State
{
    under_selection_box_colors: Vec<u8>,
    selected_colors: HashSet<[u8; 4]>,
    line_objects: HashMap<LineObjectKey, LineObject>,
    selection_box_start_x: Option<i32>,
    selection_box_start_y: Option<i32>,
}


#[wasm_bindgen]
pub struct Renderer
{
    props: Props,
    state: State,
}


#[wasm_bindgen]
impl Renderer
{
    pub fn create(canvas_text: web_sys::HtmlCanvasElement, canvas_gl: web_sys::HtmlCanvasElement)
        -> Renderer
    {
        let props = Props
        {
            canvas_text, canvas_gl, cursor_coord_x: -1, cursor_coord_y: -1,
            theta: 0.0, phi: 0.0, dx: 0.0, dy: 0.0, d_scale: 0.0,
            point_objects: HashMap::new(),
        };

        let state = State
        {
            under_selection_box_colors: Vec::new(),
            selected_colors: HashSet::new(),
            line_objects: HashMap::new(),
            selection_box_start_x: None,
            selection_box_start_y: None,
        };

        Renderer { props, state }
    }


    fn update_point_objects_normalized_coordinates(&mut self)
    {
        normalize_point_objects_coordinates(&mut self.props.point_objects, &self.state.line_objects,
            self.props.canvas_gl.width() as f32,
            self.props.canvas_gl.height() as f32);
        log(&format!("{:?}", self.props.point_objects));
    }


    pub fn add_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType)
    {
        let point_object_key = PointObjectKey::create(number, point_object_type);
        let coordinates = Coordinates::create(x, y, z);
        let point_object = PointObject::create(coordinates);
        self.props.point_objects.insert(point_object_key, point_object);
        self.update_point_objects_normalized_coordinates();
    }


    pub fn update_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType) -> Result<(), JsValue>
    {
        if let Some(point_object) = self.props.point_objects
            .get_mut(&PointObjectKey::create(number, point_object_type))
        {
            point_object.update_coordinates(x, y, z);
            self.update_point_objects_normalized_coordinates();
        }
        else
        {
            let error_message = format!("Renderer: Update {} action: {} with number \
                {} does not exist!", point_object_type.as_str().to_lowercase(),
                point_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn delete_point_object(&mut self, number: u32, point_object_type: PointObjectType)
        -> Result<(), JsValue>
    {
        if self.props.point_objects.remove(&PointObjectKey::create(
            number, point_object_type)).is_none()
        {
            let error_message = format!("Renderer: Delete {} action: {} with \
                number {} does not exist!", point_object_type.as_str().to_lowercase(),
                point_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        if !self.props.point_objects.is_empty()
        {
            self.update_point_objects_normalized_coordinates();
        }
        Ok(())
    }


    pub fn add_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        let point_object_type = match line_object_type
            {
                LineObjectType::Line => PointObjectType::Point,
                LineObjectType::Element => PointObjectType::Node,
            };
        let start_point_object_key = PointObjectKey::create(
            start_point_object_number, point_object_type);
        let end_point_object_key = PointObjectKey::create(
            end_point_object_number, point_object_type);
        if !self.props.point_objects.contains_key(&start_point_object_key)
        {
            let error_message = format!("Renderer: Add {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
            point_object_type.as_str(), start_point_object_number);
            return Err(JsValue::from(error_message));
        }
        if !self.props.point_objects.contains_key(&end_point_object_key)
        {
            let error_message = format!("Renderer: Add {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
            point_object_type.as_str(), end_point_object_number);
            return Err(JsValue::from(error_message));
        }
        let uid =
            {
                let mut current_uid = u32::MAX / 4;
                while self.props.point_objects.values().position(|point_object|
                        point_object.uid_same(current_uid)).is_some() ||
                    self.state.line_objects.values().position(|line_object|
                        line_object.uid_same(current_uid)).is_some() || current_uid == 255
                {
                    current_uid += 1;
                }
                current_uid
            };
        let line_object_key = LineObjectKey::create(number, line_object_type);
        let line_object = LineObject::create(start_point_object_key,
            end_point_object_key, uid);
        self.state.line_objects.insert(line_object_key, line_object);
        Ok(())
    }


    pub fn update_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        let point_object_type = match line_object_type
            {
                LineObjectType::Line => PointObjectType::Point,
                LineObjectType::Element => PointObjectType::Node,
            };
        let start_point_object_key = PointObjectKey::create(
            start_point_object_number, point_object_type);
        let end_point_object_key = PointObjectKey::create(
            end_point_object_number, point_object_type);
        if !self.props.point_objects.contains_key(&start_point_object_key)
        {
            let error_message = format!("Renderer: Update {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
            point_object_type.as_str(), start_point_object_number);
            return Err(JsValue::from(error_message));
        }
        if !self.props.point_objects.contains_key(&end_point_object_key)
        {
            let error_message = format!("Renderer: Update {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
            point_object_type.as_str(), end_point_object_number);
            return Err(JsValue::from(error_message));
        }

        if let Some(line_object) = self.state.line_objects.get_mut(
            &LineObjectKey::create(number, line_object_type))
        {
            line_object.update(start_point_object_key, end_point_object_key);
        }
        else
        {
            let error_message = format!("Renderer: Update {} action: {} with number \
                {} does not exist!", line_object_type.as_str().to_lowercase(),
                line_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn delete_line_object(&mut self, number: u32, line_object_type: LineObjectType)
        -> Result<(), JsValue>
    {
        if self.state.line_objects.remove(&LineObjectKey::create(number, line_object_type))
            .is_none()
        {
            let error_message = format!("Renderer: Delete {} action: {} with \
                number {} does not exist!", line_object_type.as_str().to_lowercase(),
                line_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        Ok(())
    }


    pub fn select_objects(&mut self, drop_selection: &js_sys::Function) -> Result<(), JsValue>
    {
        self.state.selected_colors = self.state.under_selection_box_colors
            .chunks(4)
            .map(|chunk| <[u8; 4]>::try_from(chunk).unwrap())
            .collect::<HashSet<[u8; 4]>>();
        if self.state.selected_colors.len() == 1
        {
            let selected_color = self.state.selected_colors.iter()
                .collect::<Vec<&[u8; 4]>>()[0];
            for (point_object_key, point_object) in
                self.props.point_objects.iter()
            {
                if point_object.uid_same(u32::from_be_bytes(*selected_color))
                {
                    let selected_point_object_number = point_object_key.get_number();
                    let selected_point_object_type = point_object_key.get_object_type()
                        .as_str().to_lowercase();
                    let detail_header = &format!("selected_{}_number", selected_point_object_type);
                    let detail =
                        json!({ "message": { detail_header: selected_point_object_number } });
                    dispatch_custom_event(detail, CLIENT_MESSAGE_EVENT_NAME,
                        EVENT_TARGET)?;
                    return Ok(());
                }
            }
            for (line_object_key, line_object) in self.state.line_objects.iter()
            {
                if line_object.uid_same(u32::from_be_bytes(*selected_color))
                {
                    let selected_line_object_number = line_object_key.get_number();
                    let selected_line_object_type = line_object_key.get_object_type()
                        .as_str().to_lowercase();
                    let detail_header = &format!("selected_{}_number", selected_line_object_type);
                    let detail =
                        json!({ "message": { detail_header: selected_line_object_number } });
                    dispatch_custom_event(detail, CLIENT_MESSAGE_EVENT_NAME,
                        EVENT_TARGET)?;
                    return Ok(());
                }
            }
            let this = JsValue::null();
            let _ = drop_selection.call0(&this);
        }
        Ok(())
    }


    pub fn tick(&mut self) -> Result<(), JsValue>
    {
        self.render()?;
        Ok(())
    }


    fn render(&mut self) -> Result<(), JsValue>
    {
        let width = self.props.canvas_gl.width();
        let height = self.props.canvas_gl.height();

        let ctx: CTX = self.props.canvas_text
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CTX>()?;

        let gl = self.props.canvas_gl
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<GL>()?;
        gl.get_extension("OES_element_index_uint")?;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear_rect(0.0, 0.0, width as f64, height as f64);
        gl.enable(GL::DEPTH_TEST);
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.clear(GL::DEPTH_BUFFER_BIT);

        let vertex_shader_code = include_str!("shaders/main_vert_shader.vert");
        let fragment_shader_code = include_str!("shaders/main_frag_shader.frag");

        let shader_program = initialize_shaders(&gl, vertex_shader_code, fragment_shader_code);
        let shaders_variables = ShadersVariables::initialize(&gl, &shader_program);

        gl.viewport(0, 0, width as i32, height as i32);

        let aspect: f32 = width as f32 / height as f32;
        let z_near = 1.0;
        let z_far = 101.0;

        if !self.props.point_objects.is_empty()
        {
            let mut drawn_objects_buffers = Buffers::initialize(&gl);
            let mut drawn_object = DrawnObject::create();

            drawn_object.add_point_object(&self.props.point_objects, GLMode::Selection,
                &self.state.under_selection_box_colors,
                &self.state.selected_colors)?;

            if !self.state.line_objects.is_empty()
            {
                drawn_object.add_line_objects(
                    &self.props.point_objects,
                    &self.state.line_objects,
                    GLMode::Selection,
                    &self.state.under_selection_box_colors,
                    &self.state.selected_colors,
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_LINE_OBJECTS_BASE_RADIUS / (1.0 + self.props.d_scale))?;
            }

            drawn_objects_buffers.render(&gl, &drawn_object, &shaders_variables);
            let point_size = 12.0;

            let mut projection_matrix = mat4::new_zero();

            mat4::orthographic(&mut projection_matrix,
                &(1.0 / aspect), &1.0, &(-1.0 / aspect), &-1.0,
                &z_near, &z_far);
            let mut model_view_matrix = mat4::new_identity();
            let mat_to_translate = model_view_matrix;
            mat4::translate(&mut model_view_matrix, &mat_to_translate,
                &[self.props.dx, self.props.dy, -2.0]);
            let mat_to_scale = model_view_matrix;
            mat4::scale(&mut model_view_matrix, &mat_to_scale,
                &[1.0 + self.props.d_scale, 1.0 + self.props.d_scale, 1.0 + self.props.d_scale]);
            let mat_to_rotate = model_view_matrix;
            mat4::rotate_x(&mut model_view_matrix,&mat_to_rotate,&self.props.phi);
            let mat_to_rotate = model_view_matrix;
            mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.props.theta);
            gl.uniform1f(Some(shaders_variables.get_point_size()), point_size);
            gl.uniform_matrix4fv_with_f32_array(
                Some(shaders_variables.get_projection_matrix()), false, &projection_matrix);
            gl.uniform_matrix4fv_with_f32_array(
                Some(shaders_variables.get_model_view_matrix()), false, &model_view_matrix);

            drawn_object.draw(&gl);

            if let (Some(start_x), Some(start_y)) =
                (self.state.selection_box_start_x, self.state.selection_box_start_y)
            {
                let selection_rectangle_width = self.props.cursor_coord_x - start_x;
                let selection_rectangle_height = self.props.cursor_coord_y - start_y;
                if selection_rectangle_width > 0 && selection_rectangle_height > 0
                {
                    let mut pixels = vec![0u8; (selection_rectangle_width *
                        selection_rectangle_height).abs() as usize * 4];
                    match gl.read_pixels_with_opt_u8_array(
                        start_x,
                        start_y,
                        selection_rectangle_width.abs(),
                        selection_rectangle_height.abs(),
                        GL::RGBA,
                        GL::UNSIGNED_BYTE,
                        Some(&mut pixels))
                    {
                        Ok(_) => self.state.under_selection_box_colors = pixels,
                        Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                    }
                }
                else if selection_rectangle_width < 0 && selection_rectangle_height > 0
                {
                    let mut pixels = vec![0u8; (selection_rectangle_width *
                        selection_rectangle_height).abs() as usize * 4];
                    match gl.read_pixels_with_opt_u8_array(
                        self.props.cursor_coord_x,
                        start_y,
                        selection_rectangle_width.abs(),
                        selection_rectangle_height.abs(),
                        GL::RGBA,
                        GL::UNSIGNED_BYTE,
                        Some(&mut pixels))
                    {
                        Ok(_) => self.state.under_selection_box_colors = pixels,
                        Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                    }
                }
                else if selection_rectangle_width > 0 && selection_rectangle_height < 0
                {
                    let mut pixels = vec![0u8; (selection_rectangle_width *
                        selection_rectangle_height).abs() as usize * 4];
                    match gl.read_pixels_with_opt_u8_array(
                        start_x,
                        self.props.cursor_coord_y,
                        selection_rectangle_width.abs(),
                        selection_rectangle_height.abs(),
                        GL::RGBA,
                        GL::UNSIGNED_BYTE,
                        Some(&mut pixels))
                    {
                        Ok(_) => self.state.under_selection_box_colors = pixels,
                        Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                    }
                }
                else if selection_rectangle_width < 0 && selection_rectangle_height < 0
                {
                    let mut pixels = vec![0u8; (selection_rectangle_width *
                        selection_rectangle_height).abs() as usize * 4];
                    match gl.read_pixels_with_opt_u8_array(
                        self.props.cursor_coord_x,
                        self.props.cursor_coord_y,
                        selection_rectangle_width.abs(),
                        selection_rectangle_height.abs(),
                        GL::RGBA,
                        GL::UNSIGNED_BYTE,
                        Some(&mut pixels))
                    {
                        Ok(_) => self.state.under_selection_box_colors = pixels,
                        Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                    }
                }
                else
                {
                    let mut pixels = vec![0u8; 4];
                    match gl.read_pixels_with_opt_u8_array(
                        self.props.cursor_coord_x, self.props.cursor_coord_y, 1, 1, GL::RGBA,
                        GL::UNSIGNED_BYTE, Some(&mut pixels))
                    {
                        Ok(_) => self.state.under_selection_box_colors = pixels,
                        Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                    }
                }
            }
            else
            {
                let mut pixels = vec![0u8; 4];
                match gl.read_pixels_with_opt_u8_array(
                    self.props.cursor_coord_x, self.props.cursor_coord_y, 1, 1, GL::RGBA,
                    GL::UNSIGNED_BYTE, Some(&mut pixels))
                {
                    Ok(_) => self.state.under_selection_box_colors = pixels,
                    Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                }
            }

            gl.clear(GL::COLOR_BUFFER_BIT);
            gl.clear(GL::DEPTH_BUFFER_BIT);
            gl.line_width(1.0);

            drawn_object = DrawnObject::create();
            drawn_objects_buffers = Buffers::initialize(&gl);

            drawn_object.add_point_object(&self.props.point_objects,
                GLMode::Visible, &self.state.under_selection_box_colors,
                &self.state.selected_colors)?;

            if !self.state.line_objects.is_empty()
            {
                drawn_object.add_line_objects(
                    &self.props.point_objects,
                    &self.state.line_objects,
                    GLMode::Visible,
                    &self.state.under_selection_box_colors,
                    &self.state.selected_colors,
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_LINE_OBJECTS_BASE_RADIUS / (1.0 + self.props.d_scale))?;
            }

            drawn_objects_buffers.render(&gl, &drawn_object, &shaders_variables);

            let point_size = 5.0;

            let mut projection_matrix = mat4::new_zero();

            mat4::orthographic(&mut projection_matrix,
                &(1.0 / aspect), &1.0, &(-1.0 / aspect), &-1.0,
                &z_near, &z_far);
            let mut model_view_matrix = mat4::new_identity();
            let mat_to_translate = model_view_matrix;
            mat4::translate(&mut model_view_matrix, &mat_to_translate,
                &[self.props.dx, self.props.dy, -2.0]);
            let mat_to_scale = model_view_matrix;
            mat4::scale(&mut model_view_matrix, &mat_to_scale,
                &[1.0 + self.props.d_scale, 1.0 + self.props.d_scale, 1.0 + self.props.d_scale]);
            let mat_to_rotate = model_view_matrix;
            mat4::rotate_x(&mut model_view_matrix,&mat_to_rotate,&self.props.phi);
            let mat_to_rotate = model_view_matrix;
            mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.props.theta);
            gl.uniform1f(Some(shaders_variables.get_point_size()), point_size);
            gl.uniform_matrix4fv_with_f32_array(
                Some(shaders_variables.get_projection_matrix()), false, &projection_matrix);
            gl.uniform_matrix4fv_with_f32_array(
                Some(shaders_variables.get_model_view_matrix()), false, &model_view_matrix);

            drawn_object.draw(&gl);

            let mut matrix = mat4::new_identity();
            mat4::mul(&mut matrix, &projection_matrix, &model_view_matrix);

            for (point_object_key, point_object) in self.props.point_objects.iter()
            {
                let initial_color = match point_object_key.get_object_type()
                    {
                        PointObjectType::Point => CANVAS_DRAWN_POINTS_DENOTATION_COLOR,
                        PointObjectType::Node => CANVAS_DRAWN_NODES_DENOTATION_COLOR,
                    };
                let denotation_color = define_drawn_object_denotation_color(
                    point_object.get_uid().unwrap(), &self.state.selected_colors,
                    &self.state.under_selection_box_colors, initial_color);
                ctx.set_fill_style(&denotation_color.into());
                add_denotation(&ctx,
                &[point_object.get_normalized_x()? -
                    DRAWN_POINT_OBJECT_DENOTATION_SHIFT / (1.0 + self.props.d_scale),
                    point_object.get_normalized_y()? - DRAWN_POINT_OBJECT_DENOTATION_SHIFT /
                        (1.0 + self.props.d_scale),
                    point_object.get_normalized_z()?,
                    1.0],
                &matrix,
                width as f32, height as f32,
                &point_object_key.get_number().to_string());
                ctx.stroke();
            }

            if !self.state.line_objects.is_empty()
            {
                for (line_object_key, line_object) in &self.state.line_objects
                {
                    let initial_color = match line_object_key.get_object_type()
                    {
                        LineObjectType::Line => CANVAS_DRAWN_LINES_DENOTATION_COLOR,
                        LineObjectType::Element => CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR,
                    };

                    let denotation_color = define_drawn_object_denotation_color(
                        line_object.get_uid(),
                        &self.state.selected_colors, &self.state.under_selection_box_colors,
                        initial_color);
                    let denotation_coordinates =
                        {
                            let start_point_object_coordinates = line_object
                                .get_start_point_object_coordinates(&self.props.point_objects)?;
                            let end_point_object_coordinates = line_object
                                .get_end_point_object_coordinates(&self.props.point_objects)?;
                            [(start_point_object_coordinates[0] +
                                end_point_object_coordinates[0]) / 2.0,
                            (start_point_object_coordinates[1] +
                                end_point_object_coordinates[1]) / 2.0,
                            (start_point_object_coordinates[2] +
                                end_point_object_coordinates[2]) / 2.0,
                            ]
                        };
                    ctx.set_fill_style(&denotation_color.into());
                    add_denotation(&ctx,
                    &[denotation_coordinates[0],
                        denotation_coordinates[1] +
                            DRAWN_LINE_OBJECTS_DENOTATION_SHIFT / (1.0 + self.props.d_scale),
                        denotation_coordinates[2],
                        1.0],
                    &matrix,
                    width as f32, height as f32,
                    &line_object_key.get_number().to_string());
                    ctx.stroke();
                }
            }
        }

        let cs_buffers = Buffers::initialize(&gl);
        let mut cs_drawn_object = DrawnObject::create();

        cs_drawn_object.add_cs_axis_line(CSAxis::X);
        cs_drawn_object.add_cs_axis_line(CSAxis::Y);
        cs_drawn_object.add_cs_axis_line(CSAxis::Z);
        cs_drawn_object.add_cs_axis_cap(
            CSAxis::X, CS_AXES_CAPS_BASE_POINTS_NUMBER,
            CS_AXES_CAPS_HEIGHT, CS_AXES_CAPS_WIDTH);
        cs_drawn_object.add_cs_axis_cap(
            CSAxis::Y, CS_AXES_CAPS_BASE_POINTS_NUMBER,
            CS_AXES_CAPS_HEIGHT, CS_AXES_CAPS_WIDTH);
        cs_drawn_object.add_cs_axis_cap(
            CSAxis::Z, CS_AXES_CAPS_BASE_POINTS_NUMBER,
            CS_AXES_CAPS_HEIGHT, CS_AXES_CAPS_WIDTH);

        cs_buffers.render(&gl, &cs_drawn_object, &shaders_variables);

        let point_size = 5.0;

        let mut projection_matrix = mat4::new_zero();
        mat4::orthographic(&mut projection_matrix,
            &1.0, &1.0, &-1.0, &-1.0, &z_near, &z_far);
        let mut model_view_matrix = mat4::new_identity();
        let mat_to_translate = model_view_matrix;
        let y_shift =
            if CS_AXES_Y_SHIFT > 0.0 { 1.0 - (1.0 - CS_AXES_Y_SHIFT) * aspect }
            else { - 1.0 + (1.0 + CS_AXES_Y_SHIFT) * aspect };
        mat4::translate(&mut model_view_matrix, &mat_to_translate,
            &[CS_AXES_X_SHIFT, y_shift, CS_AXES_Z_SHIFT]);
        let mat_to_scale = model_view_matrix;
        mat4::scale(&mut model_view_matrix, &mat_to_scale,
            &[CS_AXES_SCALE, CS_AXES_SCALE * aspect, CS_AXES_SCALE * aspect]);
        let mat_to_rotate = model_view_matrix;
        mat4::rotate_x(&mut model_view_matrix,&mat_to_rotate,&self.props.phi);
        let mat_to_rotate = model_view_matrix;
        mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.props.theta);
        gl.uniform1f(Some(shaders_variables.get_point_size()), point_size);
        gl.uniform_matrix4fv_with_f32_array(
            Some(shaders_variables.get_projection_matrix()), false, &projection_matrix);
        gl.uniform_matrix4fv_with_f32_array(
            Some(shaders_variables.get_model_view_matrix()), false, &model_view_matrix);

        cs_drawn_object.draw(&gl);

        ctx.set_fill_style(&CANVAS_AXES_DENOTATION_COLOR.into());

        add_denotation(&ctx,
            &[1.0 + AXIS_X_DENOTATION_SHIFT_X, 0.0 + AXIS_X_DENOTATION_SHIFT_Y, 0.0, 1.0],
            &model_view_matrix, width as f32,height as f32, "X");
        add_denotation(&ctx,
            &[0.0 + AXIS_Y_DENOTATION_SHIFT_X, 1.0 + AXIS_Y_DENOTATION_SHIFT_Y, 0.0, 1.0],
            &model_view_matrix, width as f32, height as f32, "Y");
        add_denotation(&ctx,
            &[0.0 + AXIS_Z_DENOTATION_SHIFT_X, 0.0 + AXIS_Z_DENOTATION_SHIFT_Y,
                1.0 + AXIS_Z_DENOTATION_SHIFT_Z, 1.0],
            &model_view_matrix, width as f32, height as f32, "Z");
        ctx.stroke();

        ctx.set_fill_style(&HINTS_COLOR.into());
        add_hints(&ctx, width as f32, height as f32);
        ctx.stroke();

        if let (Some(start_x), Some(start_y)) =
            (self.state.selection_box_start_x, self.state.selection_box_start_y)
        {
            let selection_rectangle_width = self.props.cursor_coord_x - start_x;
            let selection_rectangle_height = self.props.cursor_coord_y - start_y;
            ctx.set_stroke_style(&SELECTION_RECTANGLE_STROKE_COLOR.into());
            ctx.stroke_rect(start_x as f64, self.props.canvas_text.height() as f64 - start_y as f64,
                selection_rectangle_width as f64, -selection_rectangle_height as f64);
            ctx.set_fill_style(&SELECTION_RECTANGLE_FILL_COLOR.into());
            ctx.fill_rect(start_x as f64, self.props.canvas_text.height() as f64 - start_y as f64,
                selection_rectangle_width as f64, -selection_rectangle_height as f64);
        }

        Ok(())
    }
}
