use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}


struct Point
{
    number: u32,
    x: f64,
    y: f64,
    z: f64,
}

impl Point
{
    fn create(number: u32, x: f64, y: f64, z: f64) -> Point
    {
        Point { number, x, y, z }
    }


    fn update(&mut self, x: f64, y: f64, z: f64)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}


struct Line
{
    number: u32,
    start_point: Rc<RefCell<Point>>,
    end_point: Rc<RefCell<Point>>,
}


impl Line
{
    fn create(number: u32, start_point: Rc<RefCell<Point>>, end_point: Rc<RefCell<Point>>) -> Line
    {
        Line { number, start_point, end_point }
    }


    fn update(&mut self, start_point: Rc<RefCell<Point>>, end_point: Rc<RefCell<Point>>)
    {
        self.start_point = start_point;
        self.end_point = end_point;
    }
}


#[wasm_bindgen]
pub struct Geometry
{
    points: Vec<Rc<RefCell<Point>>>,
    lines: Vec<Line>,
}


#[wasm_bindgen]
impl Geometry
{
    pub fn create() -> Geometry
    {
        let points = Vec::new();
        let lines = Vec::new();
        Geometry { points, lines }
    }


    pub fn add_point(&mut self)
    {
        log("Hello from geometry");
    }
}
