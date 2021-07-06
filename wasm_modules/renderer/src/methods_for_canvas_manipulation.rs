use wasm_bindgen::prelude::*;

use crate::types::{RendererUInt, RendererInt, RendererFloat};


use crate::Renderer;


#[wasm_bindgen]
impl Renderer
{
    pub fn update_canvas_size(&mut self, canvas_width: RendererFloat, canvas_height: RendererFloat)
    {
        self.props.canvas_text.set_width(canvas_width as RendererUInt);
        self.props.canvas_text.set_height(canvas_height as RendererUInt);
        self.props.canvas_gl.set_width(canvas_width as RendererUInt);
        self.props.canvas_gl.set_height(canvas_height as RendererUInt);
    }


    pub fn change_cursor_coordinates(&mut self, x: RendererInt, y: RendererInt)
    {
        self.props.cursor_coord_x = x;
        self.props.cursor_coord_y = y;
    }


    pub fn increment_angle_theta(&mut self, d_theta: RendererFloat)
    {
        self.props.theta += d_theta;
    }


    pub fn increment_angle_phi(&mut self, d_phi: RendererFloat)
    {
        self.props.phi += d_phi;
    }


    pub fn increment_dx(&mut self, dx: RendererFloat)
    {
        self.props.dx += dx;
    }


    pub fn increment_dy(&mut self, dy: RendererFloat)
    {
        self.props.dy += dy;
    }


    pub fn extract_d_scale(&self) -> RendererFloat
    {
        self.props.d_scale
    }


    pub fn change_d_scale(&mut self, d_scale: RendererFloat)
    {
        self.props.d_scale = d_scale;
    }


    pub fn change_angle_theta(&mut self, theta: RendererFloat)
    {
        self.props.theta = theta;
    }


    pub fn change_angle_phi(&mut self, phi: RendererFloat)
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
