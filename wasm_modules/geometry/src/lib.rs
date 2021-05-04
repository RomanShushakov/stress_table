use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}


#[wasm_bindgen(module = "/js/interface_to_communicate_geometry_with_app.js")]
extern "C"
{
    #[wasm_bindgen(js_name = addPointToApp)]
    fn add_point_to_app(number: u32, x: f64, y: f64, z: f64);
}


struct Point
{
    action_id: u32,
    number: u32,
    x: f64,
    y: f64,
    z: f64,
}


struct Line
{
    action_id: u32,
    number: u32,
    start_point: Rc<RefCell<Point>>,
    end_point: Rc<RefCell<Point>>,
}


#[wasm_bindgen]
pub struct Geometry
{
    points: Vec<Rc<RefCell<Point>>>,
    lines: Vec<Line>,
    deleted_points: Vec<Rc<RefCell<Point>>>,
    deleted_lines: Vec<Line>,
}


#[wasm_bindgen]
impl Geometry
{
    pub fn create() -> Geometry
    {
        let points = Vec::new();
        let lines = Vec::new();
        let deleted_points = Vec::new();
        let deleted_lines = Vec::new();
        Geometry { points, lines, deleted_points, deleted_lines }
    }


    pub fn add_point(&mut self, action_id: u32, number: u32, x: f64, y: f64, z: f64)
    {
        let point = Point { action_id, number, x, y, z };
        self.points.push(Rc::new(RefCell::new(point)));
        add_point_to_app(number, x, y, z);
    }
}
