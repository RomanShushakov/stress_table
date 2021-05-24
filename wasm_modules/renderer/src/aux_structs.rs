use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext as GL};
use std::f32::consts::PI;
use std::rc::Rc;
use std::cell::RefCell;

use crate::aux_functions::{define_drawn_object_color, compare_with_tolerance};

use crate::{TOLERANCE, ElementsValues, ElementsNumbers, log};

use crate::extended_matrix::{ExtendedMatrix, MatrixElementPosition, extract_element_value};


#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PointObjectType
{
    Point,
    Node,
}


impl PointObjectType
{
    pub fn as_str(&self) -> String
    {
        match self
        {
            PointObjectType::Point => String::from("Point"),
            PointObjectType::Node => String::from("Node"),
        }
    }
}


#[derive(Debug)]
pub struct Coordinates
{
    x: f32,
    y: f32,
    z: f32,
}


impl Coordinates
{
    pub fn create(x: f32, y: f32, z: f32) -> Coordinates
    {
        Coordinates { x, y, z }
    }


    fn get_x(&self) -> f32
    {
        self.x
    }


    fn get_y(&self) -> f32
    {
        self.y
    }


    fn get_z(&self) -> f32
    {
        self.z
    }


    fn update(&mut self, x: f32, y: f32, z: f32)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}


#[derive(Debug)]
pub struct PointObject
{
    number: u32,
    coordinates: Coordinates,
    object_type: PointObjectType,
}


impl PointObject
{
    pub fn create(number: u32, coordinates: Coordinates, object_type: PointObjectType)
        -> PointObject
    {
        PointObject { number, coordinates, object_type }
    }


    pub fn get_number(&self) -> u32
    {
        self.number
    }


    pub fn get_x(&self) -> f32
    {
        self.coordinates.get_x()
    }


    pub fn get_y(&self) -> f32
    {
        self.coordinates.get_y()
    }


    pub fn get_z(&self) -> f32
    {
        self.coordinates.get_z()
    }


    pub fn get_object_type(&self) -> PointObjectType
    {
        self.object_type
    }


    pub fn number_same(&self, number: u32) -> bool
    {
        self.number == number
    }


    pub fn point_object_type_same(&self, point_object_type: PointObjectType) -> bool
    {
        self.object_type == point_object_type
    }


    pub fn update_coordinates(&mut self, x: f32, y: f32, z: f32)
    {
       self.coordinates.update(x, y, z);
    }
}


#[derive(Debug)]
pub struct NormalizedPointObject
{
    number: u32,
    coordinates: Rc<RefCell<Coordinates>>,
    object_type: PointObjectType,
    uid: u32,
}


impl NormalizedPointObject
{
    pub fn create(number: u32, coordinates: Rc<RefCell<Coordinates>>,
        object_type: PointObjectType, uid: u32) -> NormalizedPointObject
    {
        NormalizedPointObject { number, coordinates, object_type, uid }
    }


    pub fn get_number(&self) -> u32
    {
        self.number
    }


    pub fn get_object_type(&self) -> PointObjectType
    {
        self.object_type
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }


    pub fn uid_same(&self, uid: u32) -> bool
    {
        self.uid == uid
    }


    pub fn get_x(&self) -> f32
    {
        self.coordinates.borrow().get_x()
    }


    pub fn get_y(&self) -> f32
    {
        self.coordinates.borrow().get_y()
    }


    pub fn get_z(&self) -> f32
    {
        self.coordinates.borrow().get_z()
    }


    pub fn update_coordinates(&mut self, x: f32, y: f32, z: f32)
    {
        self.coordinates.borrow_mut().update(x, y, z);
    }


    pub fn clone_coordinates(&self) -> Rc<RefCell<Coordinates>>
    {
        Rc::clone(&self.coordinates)
    }


    pub fn number_same(&self, number: u32) -> bool
    {
        self.number == number
    }


    pub fn point_object_type_same(&self, point_object_type: PointObjectType) -> bool
    {
        self.object_type == point_object_type
    }
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
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


pub struct NormalizedLineObject
{
    number: u32,
    start_point_object_coordinates: Rc<RefCell<Coordinates>>,
    end_point_object_coordinates: Rc<RefCell<Coordinates>>,
    object_type: LineObjectType,
    uid: u32,
}


impl NormalizedLineObject
{
    pub fn create(number: u32, start_point_object_coordinates: Rc<RefCell<Coordinates>>,
        end_point_object_coordinates: Rc<RefCell<Coordinates>>,
        object_type: LineObjectType, uid: u32) -> NormalizedLineObject
    {
        NormalizedLineObject { number, start_point_object_coordinates, end_point_object_coordinates,
            object_type, uid }
    }


    pub fn update(&mut self, start_point_object_coordinates: Rc<RefCell<Coordinates>>,
        end_point_object_coordinates: Rc<RefCell<Coordinates>>)
    {
        self.start_point_object_coordinates = start_point_object_coordinates;
        self.end_point_object_coordinates = end_point_object_coordinates;
    }


    pub fn number_same(&self, number: u32) -> bool
    {
        self.number == number
    }


    pub fn uid_same(&self, uid: u32) -> bool
    {
        self.uid == uid
    }


    pub fn get_object_type(&self) -> LineObjectType
    {
        self.object_type
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }


    pub fn get_number(&self) -> u32
    {
        self.number
    }


    pub fn line_object_type_same(&self, line_object_type: LineObjectType) -> bool
    {
        self.object_type == line_object_type
    }


    pub fn get_start_point_object_coordinates(&self) -> [f32; 3]
    {
        [self.start_point_object_coordinates.borrow().get_x(),
        self.start_point_object_coordinates.borrow().get_y(),
        self.start_point_object_coordinates.borrow().get_z()]
    }


    pub fn get_end_point_object_coordinates(&self) -> [f32; 3]
    {
        [self.end_point_object_coordinates.borrow().get_x(),
        self.end_point_object_coordinates.borrow().get_y(),
        self.end_point_object_coordinates.borrow().get_z()]
    }


    pub fn length(&self) -> f32
    {
        let start_point_object_coordinates = self.get_start_point_object_coordinates();
        let end_point_object_coordinates = self.get_end_point_object_coordinates();
        ((start_point_object_coordinates[0] - end_point_object_coordinates[0]).powi(2) +
        (start_point_object_coordinates[1] - end_point_object_coordinates[1]).powi(2) +
        (start_point_object_coordinates[2] - end_point_object_coordinates[2]).powi(2)).sqrt()
    }


    pub fn extract_transposed_rotation_matrix(&self) -> ExtendedMatrix<u32, f32>
    {
        let start_point_object_coordinates = self.get_start_point_object_coordinates();
        let end_point_object_coordinates = self.get_end_point_object_coordinates();
        let x = (end_point_object_coordinates[0] - start_point_object_coordinates[0]);
        let y = (end_point_object_coordinates[1] - start_point_object_coordinates[1]);
        let z = (end_point_object_coordinates[2] - start_point_object_coordinates[2]);
        let length = self.length();
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
        rotation_matrix
    }
}
