use wasm_bindgen::prelude::*;


use crate::Renderer;


#[wasm_bindgen]
impl Renderer
{
    pub fn update_geometry_visibility(&mut self, is_geometry_visible: bool) -> Result<(), JsValue>
    {
        self.props.is_geometry_visible = is_geometry_visible;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn update_load_visibility(&mut self, is_load_visible: bool) -> Result<(), JsValue>
    {
        self.props.is_load_visible = is_load_visible;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn update_boundary_condition_visibility(&mut self, is_boundary_condition_visible: bool)
        -> Result<(), JsValue>
    {

        self.props.is_boundary_condition_visible = is_boundary_condition_visible;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }


    pub fn update_mesh_visibility(&mut self, is_mesh_visible: bool) -> Result<(), JsValue>
    {
        self.props.is_mesh_visible = is_mesh_visible;
        self.update_scene_for_selection()?;
        self.update_scene_visible()?;
        Ok(())
    }
}
