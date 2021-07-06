use wasm_bindgen::prelude::*;
use serde::{Deserialize};
use web_sys::{WebGlRenderingContext as GL};
use std::f32::consts::PI;
use std::collections::HashMap;

use crate::functions::{define_drawn_object_color, compare_with_tolerance};

use crate::extended_matrix::{ExtendedMatrix, MatrixElementPosition};
use crate::extended_matrix::extract_element_value;

use crate::{PointObjectKey, PointObject};

use crate::consts::TOLERANCE;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum LineObjectType
{
    Line,
    Element,
}


impl LineObjectType
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            LineObjectType::Line => String::from("Line"),
            LineObjectType::Element => String::from("Element"),
        }
    }
}


#[derive(Debug, Hash, PartialEq, Eq)]
pub struct LineObjectKey
{
    number: u32,
    object_type: LineObjectType,
}


impl LineObjectKey
{
    pub fn create(number: u32, object_type: LineObjectType) -> Self
    {
        LineObjectKey { number, object_type }
    }


    pub fn get_number(&self) -> u32
    {
        self.number
    }


    pub fn get_object_type(&self) -> LineObjectType
    {
        self.object_type
    }
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum LineObjectColorScheme
{
    Default,
    TrussProps,
    BeamProps,
}


pub struct LineObject
{
    start_point_object_key: PointObjectKey,
    end_point_object_key: PointObjectKey,
    line_object_color_scheme: LineObjectColorScheme,
    uid: u32,
}


impl LineObject
{
    pub fn create(start_point_object_key: PointObjectKey, end_point_object_key: PointObjectKey,
        uid: u32) -> Self
    {
        let line_object_color_scheme = LineObjectColorScheme::Default;
        LineObject { start_point_object_key, end_point_object_key, line_object_color_scheme, uid }
    }


    pub fn update(&mut self, start_point_object_key: PointObjectKey,
        end_point_object_key: PointObjectKey)
    {
        self.start_point_object_key = start_point_object_key;
        self.end_point_object_key = end_point_object_key;
    }


    pub fn uid_same(&self, uid: u32) -> bool
    {
        self.uid == uid
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }


    pub fn get_start_point_object_coordinates(&self,
        point_objects: &HashMap<PointObjectKey, PointObject>) -> Result<[f32; 3], JsValue>
    {
        Ok([point_objects.get(&self.start_point_object_key)
            .ok_or(JsValue::from("Renderer: Start point object coordinates extraction: \
                Point object does not exist!"))?.get_normalized_x()?,
        point_objects.get(&self.start_point_object_key)
            .ok_or(JsValue::from("Renderer: Start point object coordinates extraction: \
                Point object does not exist!"))?.get_normalized_y()?,
        point_objects.get(&self.start_point_object_key)
            .ok_or(JsValue::from("Renderer: Start point object coordinates extraction: \
                Point object does not exist!"))?.get_normalized_z()?])
    }


    pub fn get_end_point_object_coordinates(&self,
        point_objects: &HashMap<PointObjectKey, PointObject>) -> Result<[f32; 3], JsValue>
    {
        Ok([point_objects.get(&self.end_point_object_key)
            .ok_or(JsValue::from("Renderer: End point object coordinates extraction: \
                Point object does not exist!"))?
            .get_normalized_x()?,
        point_objects.get(&self.end_point_object_key)
            .ok_or(JsValue::from("Renderer: End point object coordinates extraction: \
                Point object does not exist!"))?
            .get_normalized_y()?,
        point_objects.get(&self.end_point_object_key)
            .ok_or(JsValue::from("Renderer: End point object coordinates extraction: \
                Point object does not exist!"))?
            .get_normalized_z()?])
    }


    pub fn length(&self, point_objects: &HashMap<PointObjectKey, PointObject>) -> Result<f32, JsValue>
    {
        let start_point_object_coordinates =
            self.get_start_point_object_coordinates(point_objects)?;
        let end_point_object_coordinates =
            self.get_end_point_object_coordinates(point_objects)?;
        Ok(((start_point_object_coordinates[0] - end_point_object_coordinates[0]).powi(2) +
        (start_point_object_coordinates[1] - end_point_object_coordinates[1]).powi(2) +
        (start_point_object_coordinates[2] - end_point_object_coordinates[2]).powi(2)).sqrt())
    }


    pub fn extract_transposed_rotation_matrix(&self,
        point_objects: &HashMap<PointObjectKey, PointObject>)
        -> Result<ExtendedMatrix<u32, f32>, JsValue>
    {
        let start_point_object_coordinates =
            self.get_start_point_object_coordinates(point_objects)?;
        let end_point_object_coordinates =
            self.get_end_point_object_coordinates(point_objects)?;
        let x = (end_point_object_coordinates[0] - start_point_object_coordinates[0]);
        let y = (end_point_object_coordinates[1] - start_point_object_coordinates[1]);
        let z = (end_point_object_coordinates[2] - start_point_object_coordinates[2]);
        let length = self.length(point_objects)?;
        let (u, v, w) = (length, 0.0, 0.0);
        let alpha = ((x * u + y * v + z * w) / (length.powi(2))).acos();
        let (rotation_axis_coord_x, mut rotation_axis_coord_y,
            mut rotation_axis_coord_z) = (0f32, 0f32, 0f32);
        if x != 0.0 && y == 0.0 && z == 0.0
        {
            rotation_axis_coord_z = x;
        }
        else
        {
            rotation_axis_coord_y = z * length;
            rotation_axis_coord_z = - y * length;
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
            3, vec![q_11, q_12, q_13, q_21, q_22, q_23, q_31, q_32, q_33]);
        rotation_matrix.transpose();
        Ok(rotation_matrix)
    }


    pub fn get_color_scheme(&self) -> LineObjectColorScheme
    {
        self.line_object_color_scheme
    }


    pub fn update_color_scheme(&mut self, color_scheme: LineObjectColorScheme)
    {
        self.line_object_color_scheme = color_scheme;
    }
}


#[derive(Deserialize)]
pub struct LineObjectNumbers
{
    line_numbers: Vec<u32>,
}


impl LineObjectNumbers
{
    pub fn extract_line_numbers(&self) -> &[u32]
    {
        self.line_numbers.as_slice()
    }
}
