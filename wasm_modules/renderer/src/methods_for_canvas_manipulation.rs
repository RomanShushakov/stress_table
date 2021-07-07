use wasm_bindgen::prelude::*;


use crate::Renderer;


#[wasm_bindgen]
impl Renderer
{
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


    pub fn change_angle_theta(&mut self, theta: f32)
    {
        self.props.theta = theta;
    }


    pub fn change_angle_phi(&mut self, phi: f32)
    {
        self.props.phi = phi;
    }


    pub fn selection_box_start(&mut self)
    {
        self.state.selection_box_start_x = Some(self.props.cursor_coord_x);
        self.state.selection_box_start_y = Some(self.props.cursor_coord_y);
    }


    pub fn selection_box_end(&mut self)
    {
        self.state.selection_box_start_x = None;
        self.state.selection_box_start_y = None;
    }
}
