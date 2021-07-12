use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::
{
    WebGlProgram, WebGlRenderingContext as GL, WebGlShader, CanvasRenderingContext2d as CTX,
};
use mat4;
use serde_json::json;
use rand;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

mod point_object;
use point_object::{PointObjectKey, PointObject, Coordinates};
use point_object::{PointObjectType};

mod line_object;
use line_object::{LineObject, LineObjectKey, LineObjectNumbers, BeamSectionOrientation};
use line_object::{LineObjectType, LineObjectColorScheme};

mod drawn_object;
use drawn_object::drawn_object::DrawnObjectTrait;
use drawn_object::drawn_object::DrawnObject;
use drawn_object::drawn_object::{CSAxis, GLMode};
use drawn_object::cs_axes_drawn_object::CSAxesDrawnObject;
use drawn_object::consts::
{
    CS_AXES_Y_SHIFT, CS_AXES_X_SHIFT, CS_AXES_Z_SHIFT, CS_AXES_SCALE,
    CS_AXES_CAPS_BASE_POINTS_NUMBER, CS_AXES_CAPS_WIDTH, CS_AXES_CAPS_HEIGHT,
    AXIS_X_DENOTATION_SHIFT_X, AXIS_X_DENOTATION_SHIFT_Y, AXIS_Y_DENOTATION_SHIFT_X,
    AXIS_Y_DENOTATION_SHIFT_Y, AXIS_Z_DENOTATION_SHIFT_X, AXIS_Z_DENOTATION_SHIFT_Y,
    AXIS_Z_DENOTATION_SHIFT_Z, CANVAS_AXES_DENOTATION_COLOR,
    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER, DRAWN_LINE_OBJECTS_BASE_RADIUS,
    DRAWN_BEAM_SECTION_ORIENTATION_LINE_LENGTH,
    DRAWN_BEAM_SECTION_ORIENTATION_CAPS_BASE_POINTS_NUMBER,
    DRAWN_BEAM_SECTION_ORIENTATION_CAPS_HEIGHT, DRAWN_BEAM_SECTION_ORIENTATION_CAPS_WIDTH,
    CANVAS_DRAWN_POINTS_DENOTATION_COLOR, CANVAS_DRAWN_NODES_DENOTATION_COLOR,
    DRAWN_POINT_OBJECT_DENOTATION_SHIFT, CANVAS_DRAWN_LINES_DEFAULT_DENOTATION_COLOR,
    CANVAS_DRAWN_LINES_TRUSS_PROPS_DENOTATION_COLOR, CANVAS_DRAWN_LINES_BEAM_PROPS_DENOTATION_COLOR,
    CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR, DRAWN_LINE_OBJECTS_DENOTATION_SHIFT, HINTS_COLOR,
    SELECTION_RECTANGLE_STROKE_COLOR, SELECTION_RECTANGLE_FILL_COLOR,
};

mod buffer_objects;
use crate::buffer_objects::BufferObjects;

mod shader_programs;
use crate::shader_programs::ShaderPrograms;

mod extended_matrix;

mod methods_for_canvas_manipulation;

mod types;

mod consts;
use consts::
{
    EVENT_TARGET, SELECTED_POINTS_EVENT_MAME, SELECTED_NODES_EVENT_MAME, SELECTED_LINES_EVENT_MAME,
    SELECTED_LINE_ELEMENTS_EVENT_MAME
};

mod functions;
use functions::
{
    initialize_shaders, add_denotation, add_hints, normalize_point_objects_coordinates,
    define_drawn_object_denotation_color, transform_u32_to_array_of_u8,
    dispatch_custom_event, convert_into_array
};


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
    ctx: CTX,
    gl: GL,
    shader_programs: ShaderPrograms,
    cs_axes_drawn_object: CSAxesDrawnObject,
    drawn_object_for_selection: Option<DrawnObject>,
    drawn_object_visible: Option<DrawnObject>,
    buffer_objects: BufferObjects,
    under_selection_box_colors: Vec<u8>,
    selected_colors: HashSet<[u8; 4]>,
    line_objects: HashMap<LineObjectKey, LineObject>,
    beam_section_orientation_for_preview: Option<BeamSectionOrientation>,
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
        -> Result<Renderer, JsValue>
    {
        let props = Props
        {
            canvas_text: canvas_text.clone(), canvas_gl: canvas_gl.clone(),
            cursor_coord_x: -1, cursor_coord_y: -1,
            theta: 0.0, phi: 0.0, dx: 0.0, dy: 0.0, d_scale: 0.0,
            point_objects: HashMap::new(),
        };

        let ctx: CTX = canvas_text
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CTX>()?;

        let gl: GL = canvas_gl
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<GL>()?;
        gl.get_extension("OES_element_index_uint")?;

        let mut cs_axes_drawn_object = CSAxesDrawnObject::create();
        cs_axes_drawn_object.add_cs_axes_lines();
        cs_axes_drawn_object.add_cs_axes_caps(CS_AXES_CAPS_BASE_POINTS_NUMBER,
            CS_AXES_CAPS_HEIGHT, CS_AXES_CAPS_WIDTH);

        let shader_programs = ShaderPrograms::initialize(&gl);

        let buffer_objects = BufferObjects::initialize(&gl);

        let state = State
        {
            ctx,
            gl,
            shader_programs,
            cs_axes_drawn_object,
            drawn_object_for_selection: None,
            drawn_object_visible: None,
            buffer_objects,
            under_selection_box_colors: Vec::new(),
            selected_colors: HashSet::new(),
            line_objects: HashMap::new(),
            beam_section_orientation_for_preview: None,
            selection_box_start_x: None,
            selection_box_start_y: None,
        };

        Ok(Renderer { props, state })
    }


    fn update_point_objects_normalized_coordinates(&mut self)
    {
        normalize_point_objects_coordinates(&mut self.props.point_objects, &self.state.line_objects,
            self.props.canvas_gl.width() as f32,
            self.props.canvas_gl.height() as f32);
        log(&format!("{:?}", self.props.point_objects));
    }


    fn update_drawn_object_for_selection(&mut self) -> Result<(), JsValue>
    {
        if !self.props.point_objects.is_empty()
        {
            let mut drawn_object_for_selection = DrawnObject::create();
            drawn_object_for_selection.add_point_object(
                &self.props.point_objects,
                GLMode::Selection,
                &self.state.under_selection_box_colors,
                &self.state.selected_colors)?;
            if !self.state.line_objects.is_empty()
            {
                drawn_object_for_selection.add_line_objects(
                    &self.props.point_objects,
                    &self.state.line_objects,
                    GLMode::Selection,
                    &self.state.under_selection_box_colors,
                    &self.state.selected_colors,
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_LINE_OBJECTS_BASE_RADIUS / (1.0 + self.props.d_scale))?;
            }
            self.state.drawn_object_for_selection = Some(drawn_object_for_selection);
        }
        else
        {
            self.state.drawn_object_for_selection = None;
        }
        Ok(())
    }


    fn update_drawn_object_visible(&mut self) -> Result<(), JsValue>
    {
        if !self.props.point_objects.is_empty()
        {
            let mut drawn_object_visible = DrawnObject::create();
            drawn_object_visible.add_point_object(
                &self.props.point_objects,
                GLMode::Visible,
                &self.state.under_selection_box_colors,
                &self.state.selected_colors)?;
            if !self.state.line_objects.is_empty()
            {
                drawn_object_visible.add_line_objects(
                    &self.props.point_objects,
                    &self.state.line_objects,
                    GLMode::Visible,
                    &self.state.under_selection_box_colors,
                    &self.state.selected_colors,
                    DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_LINE_OBJECTS_BASE_RADIUS / (1.0 + self.props.d_scale))?;
                if let Some(beam_section_orientation) =
                    &self.state.beam_section_orientation_for_preview
                {
                    drawn_object_visible.add_beam_section_orientation_for_preview(
                        &self.props.point_objects,
                        &self.state.line_objects,
                        beam_section_orientation,
                        DRAWN_BEAM_SECTION_ORIENTATION_LINE_LENGTH /
                            (1.0 + self.props.d_scale),
                        DRAWN_BEAM_SECTION_ORIENTATION_CAPS_BASE_POINTS_NUMBER,
                        DRAWN_BEAM_SECTION_ORIENTATION_CAPS_HEIGHT /
                            (1.0 + self.props.d_scale),
                        DRAWN_BEAM_SECTION_ORIENTATION_CAPS_WIDTH /
                            (1.0 + self.props.d_scale),
                    )?;
                }
            }
            self.state.drawn_object_visible = Some(drawn_object_visible);
        }
        else
        {
            self.state.drawn_object_visible = None;
        }
        Ok(())
    }


    pub fn add_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType) -> Result<(), JsValue>
    {
        let point_object_key = PointObjectKey::create(number, point_object_type);
        let coordinates = Coordinates::create(x, y, z);
        let point_object = PointObject::create(coordinates);
        self.props.point_objects.insert(point_object_key, point_object);
        self.update_point_objects_normalized_coordinates();
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
        Ok(())
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
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
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
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
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
                let mut current_uid = rand::random::<u32>();
                while self.props.point_objects.values().position(|point_object|
                        point_object.uid_same(current_uid)).is_some() ||
                    self.state.line_objects.values().position(|line_object|
                        line_object.uid_same(current_uid)).is_some() || current_uid == 255
                {
                    current_uid = rand::random::<u32>();
                }
                current_uid
            };
        let line_object_key = LineObjectKey::create(number, line_object_type);
        let line_object = LineObject::create(start_point_object_key,
            end_point_object_key, uid);
        self.state.line_objects.insert(line_object_key, line_object);
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
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
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
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
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
        Ok(())
    }


    pub fn update_line_objects_color_scheme(&mut self, line_object_numbers: &[u32],
        line_object_type: LineObjectType, line_object_color_scheme: LineObjectColorScheme)
        -> Result<(), JsValue>
    {
        for line_object_number in line_object_numbers
        {
            let line_object_key = LineObjectKey::create(
                *line_object_number, line_object_type);
            if let Some(line_object) = self.state.line_objects
                .get_mut(&line_object_key)
            {
                line_object.update_color_scheme(line_object_color_scheme);
            }
            else
            {
                let error_message = format!("Renderer: Update {} color scheme action: {} \
                    with number {} does not exist!", line_object_type.as_str().to_lowercase(),
                    line_object_type.as_str(), line_object_number);
                return Err(JsValue::from(error_message));
            }
        }
        self.update_drawn_object_for_selection()?;
        self.update_drawn_object_visible()?;
        Ok(())
    }


    pub fn select_objects(&mut self, drop_selection: &js_sys::Function) -> Result<(), JsValue>
    {
        self.state.selected_colors = self.state.under_selection_box_colors
            .chunks(4)
            .map(|chunk| <[u8; 4]>::try_from(chunk).unwrap())
            .collect::<HashSet<[u8; 4]>>();
        let mut selected_point_numbers = Vec::new();
        let mut selected_node_numbers = Vec::new();
        let mut selected_line_numbers = Vec::new();
        let mut selected_line_element_numbers = Vec::new();
        let mut is_object_selected = false;
        for selected_color in self.state.selected_colors.iter()
        {
            for (point_object_key, point_object) in
                self.props.point_objects.iter()
            {
                if point_object.uid_same(u32::from_be_bytes(*selected_color))
                {
                    let selected_point_object_number = point_object_key.get_number();
                    let selected_point_object_type =
                        point_object_key.get_object_type();
                    match selected_point_object_type
                    {
                        PointObjectType::Point =>
                            selected_point_numbers.push(selected_point_object_number),
                        PointObjectType::Node =>
                            selected_node_numbers.push(selected_point_object_number),
                    }
                }
            }
            for (line_object_key, line_object) in
                self.state.line_objects.iter()
            {
                if line_object.uid_same(u32::from_be_bytes(*selected_color))
                {
                    let selected_line_object_number = line_object_key.get_number();
                    let selected_line_object_type = line_object_key.get_object_type();
                    match selected_line_object_type
                    {
                        LineObjectType::Line =>
                            selected_line_numbers.push(selected_line_object_number),
                        LineObjectType::Element =>
                            selected_line_element_numbers.push(selected_line_object_number),
                    }
                }
            }
        }

        if !selected_point_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({ "point_numbers": selected_point_numbers });
            dispatch_custom_event(detail, SELECTED_POINTS_EVENT_MAME,
                EVENT_TARGET)?;
        }

        if !selected_node_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({ "node_numbers": selected_node_numbers });
            dispatch_custom_event(detail, SELECTED_NODES_EVENT_MAME,
                EVENT_TARGET)?;
        }

        if !selected_line_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({ "line_numbers": selected_line_numbers });
            dispatch_custom_event(detail, SELECTED_LINES_EVENT_MAME,
                EVENT_TARGET)?;
        }

        if !selected_line_element_numbers.is_empty()
        {
            is_object_selected = true;
            let detail = json!({ "line_element_numbers": selected_line_element_numbers });
            dispatch_custom_event(detail, SELECTED_LINE_ELEMENTS_EVENT_MAME,
                EVENT_TARGET)?;
        }
        self.state.beam_section_orientation_for_preview = None;
        self.update_drawn_object_visible()?;
        if is_object_selected
        {
            Ok(())
        }
        else
        {
            let this = JsValue::null();
            let _ = drop_selection.call0(&this);

            Ok(())
        }
    }


    pub fn preview_selected_line_objects(&mut self, selected_line_object_numbers: JsValue,
        line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        self.state.selected_colors.clear();
        for line_object_number in selected_line_object_numbers
            .into_serde::<LineObjectNumbers>()
            .or(Err(JsValue::from("Renderer: Preview selected line object numbers action: \
                Line object numbers could not be serialized!")))?
            .extract_line_numbers()
        {
            let current_line_object_key =
                LineObjectKey::create(*line_object_number, line_object_type);
            if let Some(line_object) =
                self.state.line_objects.get(&current_line_object_key)
            {
                let current_uid = line_object.get_uid();
                let current_color = transform_u32_to_array_of_u8(current_uid);
                self.state.selected_colors.insert(current_color);
            }
            else
            {
                let error_message = format!("Renderer: Preview selected line objects \
                    action: {} with number {} does not exist!",
                    line_object_type.as_str(), line_object_number);
                return Err(JsValue::from(error_message));
            }
        }
        self.update_drawn_object_visible()?;
        Ok(())
    }


    pub fn preview_beam_section_orientation(&mut self, beam_section_orientation: JsValue,
        line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        self.state.selected_colors.clear();
        let beam_section_orientation_for_preview =
            beam_section_orientation
                .into_serde::<BeamSectionOrientation>()
                .or(Err(JsValue::from("Renderer: Preview beam section orientation action: \
                    Beam section orientation could not be serialized!")))?;
        for line_object_number in beam_section_orientation_for_preview.extract_line_numbers()
        {
            let current_line_object_key =
                LineObjectKey::create(*line_object_number, line_object_type);
            if let Some(line_object) =
                self.state.line_objects.get(&current_line_object_key)
            {
                let current_uid = line_object.get_uid();
                let current_color = transform_u32_to_array_of_u8(current_uid);
                self.state.selected_colors.insert(current_color);
            }
            else
            {
                let error_message = format!("Renderer: Preview beam section orientation \
                    action: {} with number {} does not exist!",
                    line_object_type.as_str(), line_object_number);
                return Err(JsValue::from(error_message));
            }
        }
        self.state.beam_section_orientation_for_preview = Some(beam_section_orientation_for_preview);
        self.update_drawn_object_visible()?;
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
        let old_under_selection_box_colors = self.state.under_selection_box_colors.clone();

        self.state.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.state.ctx.clear_rect(0.0, 0.0, width as f64, height as f64);
        self.state.gl.enable(GL::DEPTH_TEST);
        self.state.gl.clear(GL::COLOR_BUFFER_BIT);
        self.state.gl.clear(GL::DEPTH_BUFFER_BIT);

        self.state.gl.viewport(0, 0, width as i32, height as i32);

        let aspect: f32 = width as f32 / height as f32;
        let z_near = 1.0;
        let z_far = 101.0;

        if let Some(drawn_object_for_selection)= &self.state.drawn_object_for_selection
        {
            self.state.buffer_objects.store_drawn_object(&self.state.gl,
                drawn_object_for_selection);
            self.state.buffer_objects.associate_with_shader_programs(&self.state.gl,
                &self.state.shader_programs);

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
            mat4::rotate_x(&mut model_view_matrix, &mat_to_rotate, &self.props.phi);
            let mat_to_rotate = model_view_matrix;
            mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.props.theta);
            self.state.gl.uniform1f(Some(self.state.shader_programs.get_point_size()), point_size);
            self.state.gl.uniform_matrix4fv_with_f32_array(
                Some(self.state.shader_programs.get_projection_matrix()), false, &projection_matrix);
            self.state.gl.uniform_matrix4fv_with_f32_array(
                Some(self.state.shader_programs.get_model_view_matrix()), false, &model_view_matrix);

            drawn_object_for_selection.draw(&self.state.gl);
        }

        if let (Some(start_x), Some(start_y)) =
        (self.state.selection_box_start_x, self.state.selection_box_start_y)
        {
            let selection_rectangle_width = self.props.cursor_coord_x - start_x;
            let selection_rectangle_height = self.props.cursor_coord_y - start_y;
            if selection_rectangle_width > 0 && selection_rectangle_height > 0
            {
                let mut pixels = vec![0u8; (selection_rectangle_width *
                    selection_rectangle_height).abs() as usize * 4];
                match self.state.gl.read_pixels_with_opt_u8_array(
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
                match self.state.gl.read_pixels_with_opt_u8_array(
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
                match self.state.gl.read_pixels_with_opt_u8_array(
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
                match self.state.gl.read_pixels_with_opt_u8_array(
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
                match self.state.gl.read_pixels_with_opt_u8_array(
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
            match self.state.gl.read_pixels_with_opt_u8_array(
                self.props.cursor_coord_x, self.props.cursor_coord_y, 1, 1, GL::RGBA,
                GL::UNSIGNED_BYTE, Some(&mut pixels))
            {
                Ok(_) => self.state.under_selection_box_colors = pixels,
                Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
            }
        }

        self.state.gl.clear(GL::COLOR_BUFFER_BIT);
        self.state.gl.clear(GL::DEPTH_BUFFER_BIT);
        self.state.gl.line_width(1.0);

        if old_under_selection_box_colors != self.state.under_selection_box_colors
        {
            self.update_drawn_object_visible()?;
        }

        if let Some(drawn_object_visible) = &self.state.drawn_object_visible
        {
            self.state.buffer_objects.store_drawn_object(&self.state.gl,
                drawn_object_visible);
            self.state.buffer_objects.associate_with_shader_programs(&self.state.gl,
                &self.state.shader_programs);

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
            mat4::rotate_x(&mut model_view_matrix, &mat_to_rotate, &self.props.phi);
            let mat_to_rotate = model_view_matrix;
            mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.props.theta);
            self.state.gl.uniform1f(Some(self.state.shader_programs.get_point_size()), point_size);
            self.state.gl.uniform_matrix4fv_with_f32_array(
                Some(self.state.shader_programs.get_projection_matrix()), false, &projection_matrix);
            self.state.gl.uniform_matrix4fv_with_f32_array(
                Some(self.state.shader_programs.get_model_view_matrix()), false, &model_view_matrix);

            drawn_object_visible.draw(&self.state.gl);

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
                self.state.ctx.set_fill_style(&denotation_color.into());
                add_denotation(&self.state.ctx,
                &[point_object.get_normalized_x()? -
                    DRAWN_POINT_OBJECT_DENOTATION_SHIFT / (1.0 + self.props.d_scale),
                    point_object.get_normalized_y()? - DRAWN_POINT_OBJECT_DENOTATION_SHIFT /
                        (1.0 + self.props.d_scale),
                    point_object.get_normalized_z()?,
                    1.0],
                &matrix,
                width as f32, height as f32,
                &point_object_key.get_number().to_string());
                self.state.ctx.stroke();
            }

            if !self.state.line_objects.is_empty()
            {
                for (line_object_key, line_object) in &self.state.line_objects
                {
                    let initial_color = match line_object_key.get_object_type()
                    {
                        LineObjectType::Line =>
                            {
                                match line_object.get_color_scheme()
                                {
                                    LineObjectColorScheme::Default =>
                                        CANVAS_DRAWN_LINES_DEFAULT_DENOTATION_COLOR,
                                    LineObjectColorScheme::TrussProps =>
                                        CANVAS_DRAWN_LINES_TRUSS_PROPS_DENOTATION_COLOR,
                                    LineObjectColorScheme::BeamProps =>
                                        CANVAS_DRAWN_LINES_BEAM_PROPS_DENOTATION_COLOR,
                                }
                            },
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
                    self.state.ctx.set_fill_style(&denotation_color.into());
                    add_denotation(&self.state.ctx,
                    &[denotation_coordinates[0],
                        denotation_coordinates[1] +
                            DRAWN_LINE_OBJECTS_DENOTATION_SHIFT / (1.0 + self.props.d_scale),
                        denotation_coordinates[2],
                        1.0],
                    &matrix,
                    width as f32, height as f32,
                    &line_object_key.get_number().to_string());
                    self.state.ctx.stroke();
                }
            }
        }

        self.state.buffer_objects.store_drawn_object(&self.state.gl, &self.state.cs_axes_drawn_object);
        self.state.buffer_objects.associate_with_shader_programs(&self.state.gl, &self.state.shader_programs);

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
        self.state.gl.uniform_matrix4fv_with_f32_array(
            Some(self.state.shader_programs.get_projection_matrix()), false, &projection_matrix);
        self.state.gl.uniform_matrix4fv_with_f32_array(
            Some(self.state.shader_programs.get_model_view_matrix()), false, &model_view_matrix);

        self.state.cs_axes_drawn_object.draw(&self.state.gl);

        self.state.ctx.set_fill_style(&CANVAS_AXES_DENOTATION_COLOR.into());

        add_denotation(&self.state.ctx,
            &[1.0 + AXIS_X_DENOTATION_SHIFT_X, 0.0 + AXIS_X_DENOTATION_SHIFT_Y, 0.0, 1.0],
            &model_view_matrix, width as f32,height as f32, "X");
        add_denotation(&self.state.ctx,
            &[0.0 + AXIS_Y_DENOTATION_SHIFT_X, 1.0 + AXIS_Y_DENOTATION_SHIFT_Y, 0.0, 1.0],
            &model_view_matrix, width as f32, height as f32, "Y");
        add_denotation(&self.state.ctx,
            &[0.0 + AXIS_Z_DENOTATION_SHIFT_X, 0.0 + AXIS_Z_DENOTATION_SHIFT_Y,
                1.0 + AXIS_Z_DENOTATION_SHIFT_Z, 1.0],
            &model_view_matrix, width as f32, height as f32, "Z");
        self.state.ctx.stroke();

        self.state.ctx.set_fill_style(&HINTS_COLOR.into());
        add_hints(&self.state.ctx, width as f32, height as f32);
        self.state.ctx.stroke();

        if let (Some(start_x), Some(start_y)) =
            (self.state.selection_box_start_x, self.state.selection_box_start_y)
        {
            let selection_rectangle_width = self.props.cursor_coord_x - start_x;
            let selection_rectangle_height = self.props.cursor_coord_y - start_y;
            self.state.ctx.set_stroke_style(&SELECTION_RECTANGLE_STROKE_COLOR.into());
            self.state.ctx.stroke_rect(start_x as f64, self.props.canvas_text.height() as f64 -
                start_y as f64, selection_rectangle_width as f64,
                - selection_rectangle_height as f64);
            self.state.ctx.set_fill_style(&SELECTION_RECTANGLE_FILL_COLOR.into());
            self.state.ctx.fill_rect(start_x as f64, self.props.canvas_text.height() as f64 -
                start_y as f64, selection_rectangle_width as f64,
                - selection_rectangle_height as f64);
        }

        Ok(())
    }
}
