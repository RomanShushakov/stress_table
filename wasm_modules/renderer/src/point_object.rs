use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
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


#[derive(Debug, Clone)]
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


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct PointObjectKey
{
    number: u32,
    object_type: PointObjectType,
}


impl PointObjectKey
{
    pub fn create(number: u32, object_type: PointObjectType) -> Self
    {
        PointObjectKey { number, object_type }
    }


    pub fn get_number(&self) -> u32
    {
        self.number
    }


    pub fn get_object_type(&self) -> PointObjectType
    {
        self.object_type
    }
}


#[derive(Debug, Clone)]
pub struct PointObject
{
    coordinates: Coordinates,
    normalized_coordinates: Option<Coordinates>,
    uid: Option<u32>,
}


impl PointObject
{
    pub fn create(coordinates: Coordinates) -> Self
    {
        PointObject { coordinates, normalized_coordinates: None, uid: None }
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


    pub fn update_coordinates(&mut self, x: f32, y: f32, z: f32)
    {
       self.coordinates.update(x, y, z);
    }


    pub fn get_normalized_x(&self) -> Result<f32, JsValue>
    {
        if let Some(coordinates) = &self.normalized_coordinates
        {
            Ok(coordinates.get_x())
        }
        else
        {
            let error_message = format!("Renderer: Point object normalized x coordinate \
                extraction: Normalized x coordinate does not exist!");
            Err(JsValue::from(error_message))
        }
    }


    pub fn get_normalized_y(&self) -> Result<f32, JsValue>
    {
        if let Some(coordinates) = &self.normalized_coordinates
        {
            Ok(coordinates.get_y())
        }
        else
        {
            let error_message = format!("Renderer: Point object normalized y coordinate \
                extraction: Normalized y coordinate does not exist!");
            Err(JsValue::from(error_message))
        }
    }


    pub fn get_normalized_z(&self) -> Result<f32, JsValue>
    {
        if let Some(coordinates) =  &self.normalized_coordinates
        {
            Ok(coordinates.get_z())
        }
        else
        {
            let error_message = format!("Renderer: Point object normalized z coordinate \
                extraction: Normalized z coordinate does not exist!");
            Err(JsValue::from(error_message))
        }
    }


    pub fn update_normalized_coordinates(&mut self, x: f32, y: f32, z: f32)
    {
        if let Some(coordinates) = &mut self.normalized_coordinates
        {
            coordinates.update(x, y, z)
        }
    }


    pub fn uid_is_some(&self) -> bool
    {
        self.uid.is_some()
    }


    pub fn normalized_coordinates_is_some(&self) -> bool
    {
        self.normalized_coordinates.is_some()
    }


    pub fn is_uid_same(&self, uid: u32) -> bool
    {
        if let Some(current_uid) = &self.uid
        {
            *current_uid == uid
        }
        else
        {
            false
        }
    }


    pub fn add_normalized_coordinates(&mut self, normalized_coordinates: Coordinates)
    {
        self.normalized_coordinates = Some(normalized_coordinates);
    }


    pub fn add_uid(&mut self, uid: u32)
    {
        self.uid = Some(uid);
    }


    pub fn get_uid(&self) -> Result<u32, JsValue>
    {
        if let Some(current_uid) = self.uid
        {
            Ok(current_uid)
        }
        else
        {
            let error_message = format!("Renderer: Point object uid extraction: Uid does \
                not exist!");
            return Err(JsValue::from(error_message));
        }
    }
}
