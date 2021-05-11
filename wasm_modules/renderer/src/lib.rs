use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::
{
    WebGlProgram, WebGlRenderingContext as GL, WebGlShader, CanvasRenderingContext2d as CTX
};
use mat4;
use serde_json::json;

mod aux_structs;
mod aux_functions;

use aux_structs::
{
    ShadersVariables, Buffers, DrawnObject, PointObject, NormalizedPointObject,
    NormalizedLineObject, Coordinates,
};
use aux_structs::{CSAxis, GLMode, PointObjectType, LineObjectType};
use aux_structs::
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
        DRAWN_LINE_OBJECTS_BASE_RADIUS, CANVAS_DRAWN_LINES_DENOTATION_COLOR
    };
use aux_functions::
{
    initialize_shaders, add_denotation, add_hints, normalize_point_objects,
    define_drawn_object_denotation_color, transform_u32_to_array_of_u8,
    dispatch_custom_event
};

mod extended_matrix;


pub const TOLERANCE: f32 = 1e-6;
pub type ElementsNumbers = u32;
pub type ElementsValues = f32;

const EVENTS_TARGET: &str = "fea-app";
const CLIENT_MESSAGE: &str = "client message";


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


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
    point_objects: Vec<PointObject>,
}


struct State
{
    under_cursor_color: [u8; 4],
    selected_color: [u8; 4],
    normalized_point_objects: Vec<NormalizedPointObject>,
    normalized_line_objects: Vec<NormalizedLineObject>,
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
            point_objects: Vec::new(),
        };

        let state = State
        {
            under_cursor_color: [0; 4], selected_color: [0; 4],
            normalized_point_objects: Vec::new(), normalized_line_objects: Vec::new(),
        };

        Renderer { props, state }
    }


    pub fn update_canvas_size(&mut self, canvas_width: f32, canvas_height: f32)
    {
        self.props.canvas_text.set_width(canvas_width as u32);
        self.props.canvas_text.set_height(canvas_height as u32);
        self.props.canvas_gl.set_width(canvas_width as u32);
        self.props.canvas_gl.set_height(canvas_height as u32);
    }


    pub fn change_cursor_coordinates(&mut self, x: i32, y: i32)
    {
        self.props.cursor_coord_x = x;
        self.props.cursor_coord_y = y;
    }


    pub fn increment_angle_theta(&mut self, d_theta: f32)
    {
        self.props.theta += d_theta;
    }


    pub fn increment_angle_phi(&mut self, d_phi: f32)
    {
        self.props.phi += d_phi;
    }


    pub fn increment_dx(&mut self, dx: f32)
    {
        self.props.dx += dx;
    }


    pub fn increment_dy(&mut self, dy: f32)
    {
        self.props.dy += dy;
    }


    pub fn extract_d_scale(&self) -> f32
    {
        self.props.d_scale
    }


    pub fn change_d_scale(&mut self, d_scale: f32)
    {
        self.props.d_scale = d_scale;
    }


    fn update_normalized_point_objects(&mut self)
    {
        normalize_point_objects(&self.props.point_objects, &mut self.state.normalized_point_objects,
            &self.state.normalized_line_objects, self.props.canvas_gl.width() as f32,
            self.props.canvas_gl.height() as f32);
        log(&format!("{:?}, {:?}", self.props.point_objects, self.state.normalized_point_objects));
    }


    pub fn add_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType)
    {
        let coordinates = Coordinates::create(x, y, z);
        let point_object = PointObject::create(number, coordinates, point_object_type);
        self.props.point_objects.push(point_object);
        self.update_normalized_point_objects();
    }


    pub fn update_point_object(&mut self, number: u32, x: f32, y: f32, z: f32,
        point_object_type: PointObjectType) -> Result<(), JsValue>
    {
        if let Some(position) = self.props.point_objects.iter()
            .position(|point_object|
                point_object.number_same(number) &&
                point_object.point_object_type_same(point_object_type))
        {
            self.props.point_objects[position].update_coordinates(x, y, z);
            self.update_normalized_point_objects();
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
        if let Some(position) = self.props.point_objects.iter()
            .position(|point_object|
                point_object.number_same(number) &&
                point_object.point_object_type_same(point_object_type))
        {
            let _ = self.props.point_objects.remove(position);
        }
        else
        {
            let error_message = format!("Renderer: Delete {} action: {} with \
                number {} does not exist!", point_object_type.as_str().to_lowercase(),
                point_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        if let Some(position) = self.state.normalized_point_objects.iter()
            .position(|point_object|
                point_object.number_same(number) &&
                point_object.point_object_type_same(point_object_type))
        {
            let _ = self.state.normalized_point_objects.remove(position);
        }
        else
        {
            let error_message = format!("Renderer: Delete {} action: {} with \
                number {} does not exist!", point_object_type.as_str().to_lowercase(),
                point_object_type.as_str(), number);
            return Err(JsValue::from(error_message));
        }
        if !self.props.point_objects.is_empty()
        {
            self.update_normalized_point_objects();
        }
        Ok(())
    }


    pub fn add_normalized_line_object(&mut self, number: u32, start_point_object_number: u32,
        end_point_object_number: u32, line_object_type: LineObjectType) -> Result<(), JsValue>
    {
        let point_object_type = match line_object_type
            {
                LineObjectType::Line => PointObjectType::Point,
                LineObjectType::Element => PointObjectType::Node,
            };
        let start_point_object_coordinates =
            {
                if let Some(position) = self.state.normalized_point_objects.iter()
                    .position(|point_object|
                        point_object.number_same(start_point_object_number) &&
                        point_object.point_object_type_same(point_object_type))
                {
                    Ok(self.state.normalized_point_objects[position].clone_coordinates())
                }
                else
                {
                    let error_message = format!("Renderer: Add {} action: {} with number \
                        {} does not exist!", line_object_type.as_str().to_lowercase(),
                    point_object_type.as_str(), start_point_object_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        let end_point_object_coordinates =
            {
                if let Some(position) = self.state.normalized_point_objects.iter()
                    .position(|point_object|
                        point_object.number_same(end_point_object_number) &&
                        point_object.point_object_type_same(point_object_type))
                {
                    Ok(self.state.normalized_point_objects[position].clone_coordinates())
                }
                else
                {
                    let error_message = format!("Renderer: Add {} action: {} with number \
                        {} does not exist!", line_object_type.as_str().to_lowercase(),
                    point_object_type.as_str(), end_point_object_number);
                    Err(JsValue::from(error_message))
                }
            }?;
        let uid =
            {
                let mut current_uid = 1;
                while self.state.normalized_point_objects.iter().position(|point_object|
                        point_object.uid_same(current_uid)).is_some() ||
                    self.state.normalized_line_objects.iter().position(|line_object|
                        line_object.uid_same(current_uid)).is_some() || current_uid == 255
                {
                    current_uid += 1;
                }
                current_uid
            };
        let normalized_line_object = NormalizedLineObject::create(number,
            start_point_object_coordinates, end_point_object_coordinates, line_object_type, uid);
        self.state.normalized_line_objects.push(normalized_line_object);
        Ok(())
    }


    pub fn select_object(&mut self) -> Result<(), JsValue>
    {
        self.state.selected_color = self.state.under_cursor_color;
        log(&format!("{:?}", self.state.selected_color));
        if let Some(position) = self.state.normalized_point_objects.iter()
            .position(|point_object|
                transform_u32_to_array_of_u8(point_object.get_uid()) == self.state.selected_color)
        {
            let selected_point_object =
                &self.state.normalized_point_objects[position];
            match selected_point_object.get_object_type()
            {
                PointObjectType::Point =>
                    {
                        let selected_point_number = selected_point_object.get_number();
                        let detail =
                            json!({ "message": { "selected_point_number": selected_point_number } });
                        dispatch_custom_event(detail, CLIENT_MESSAGE,
                            EVENTS_TARGET)?;
                    },
                PointObjectType::Node => (),
            }
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

        if !self.state.normalized_point_objects.is_empty()
        {
            let mut drawn_objects_buffers = Buffers::initialize(&gl);
            let mut drawn_object = DrawnObject::create();

            drawn_object.add_point_object(&self.state.normalized_point_objects,
            GLMode::Selection, &self.state.under_cursor_color, &self.state.selected_color);

            if !self.state.normalized_line_objects.is_empty()
            {
                drawn_object.add_line_objects(&self.state.normalized_line_objects,
                    GLMode::Selection, &self.state.under_cursor_color,
                    &self.state.selected_color, DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
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
            gl.uniform1f(Some(&shaders_variables.point_size), point_size);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.projection_matrix), false, &projection_matrix);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.model_view_matrix), false, &model_view_matrix);

            drawn_object.draw(&gl);

            let mut pixels = [0u8; 4];
            match gl.read_pixels_with_opt_u8_array(
                self.props.cursor_coord_x, self.props.cursor_coord_y, 1, 1, GL::RGBA,
                GL::UNSIGNED_BYTE, Some(&mut pixels))
            {
                Ok(_) => self.state.under_cursor_color = pixels,
                Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
            }

            gl.clear(GL::COLOR_BUFFER_BIT);
            gl.clear(GL::DEPTH_BUFFER_BIT);
            gl.line_width(1.0);

            drawn_object = DrawnObject::create();
            drawn_objects_buffers = Buffers::initialize(&gl);

            drawn_object.add_point_object(&self.state.normalized_point_objects,
            GLMode::Visible, &self.state.under_cursor_color, &self.state.selected_color);

            if !self.state.normalized_line_objects.is_empty()
            {
                drawn_object.add_line_objects(&self.state.normalized_line_objects,
                    GLMode::Visible, &self.state.under_cursor_color,
                    &self.state.selected_color, DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER,
                    DRAWN_LINE_OBJECTS_BASE_RADIUS / (1.0 + self.props.d_scale))?;
            }

            drawn_objects_buffers.render(&gl, &drawn_object, &shaders_variables);

            let point_size = 6.0;

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
            gl.uniform1f(Some(&shaders_variables.point_size), point_size);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.projection_matrix), false, &projection_matrix);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&shaders_variables.model_view_matrix), false, &model_view_matrix);

            drawn_object.draw(&gl);

            let mut matrix = mat4::new_identity();
            mat4::mul(&mut matrix, &projection_matrix, &model_view_matrix);

            for point_object in self.state.normalized_point_objects.iter()
            {
                let initial_color = match point_object.get_object_type()
                    {
                        PointObjectType::Point => CANVAS_DRAWN_POINTS_DENOTATION_COLOR,
                        PointObjectType::Node => CANVAS_DRAWN_NODES_DENOTATION_COLOR,
                    };
                let denotation_color = define_drawn_object_denotation_color(
                    point_object.get_uid(), &self.state.selected_color,
                    &self.state.under_cursor_color, initial_color);
                ctx.set_fill_style(&denotation_color.into());
                add_denotation(&ctx,
                &[point_object.get_x() - DRAWN_POINT_OBJECT_DENOTATION_SHIFT /
                            (1.0 + self.props.d_scale),
                    point_object.get_y() - DRAWN_POINT_OBJECT_DENOTATION_SHIFT /
                        (1.0 + self.props.d_scale),
                    point_object.get_z(),
                    1.0],
                &matrix,
                width as f32, height as f32,
                &point_object.get_number().to_string());
                ctx.stroke();
            }

            if !self.state.normalized_line_objects.is_empty()
            {
                for line_object in &self.state.normalized_line_objects
                {
                    let initial_color = match line_object.get_object_type()
                    {
                        LineObjectType::Line => CANVAS_DRAWN_LINES_DENOTATION_COLOR,
                        LineObjectType::Element => CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR,
                    };

                    let denotation_color = define_drawn_object_denotation_color(
                        line_object.get_uid(),
                        &self.state.selected_color, &self.state.under_cursor_color,
                        initial_color);
                    let denotation_coordinates =
                        {
                            let start_point_object_coordinates =
                                line_object.get_start_point_object_coordinates();
                            let end_point_object_coordinates =
                                line_object.get_end_point_object_coordinates();
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
                    &line_object.get_number().to_string());
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

        let point_size = 6.0;

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
        gl.uniform1f(Some(&shaders_variables.point_size), point_size);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&shaders_variables.projection_matrix), false, &projection_matrix);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&shaders_variables.model_view_matrix), false, &model_view_matrix);

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

        Ok(())
    }
}
