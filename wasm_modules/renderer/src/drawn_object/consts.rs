pub const DRAWN_OBJECT_TO_CANVAS_WIDTH_SCALE: f32 = 0.8;
pub const DRAWN_OBJECT_TO_CANVAS_HEIGHT_SCALE: f32 = 0.9;


// Consts for CS axes drawn object
pub const CS_ORIGIN: [f32; 3] = [0.0, 0.0, 0.0];
pub const CS_AXIS_X: [f32; 3] = [1.0, 0.0, 0.0];
pub const CS_AXIS_Y: [f32; 3] = [0.0, 1.0, 0.0];
pub const CS_AXIS_Z: [f32; 3] = [0.0, 0.0, 1.0];

pub const CS_AXIS_X_COLOR: [f32; 4] = [0.3843, 0.1490, 0.1607, 1.0];
pub const CS_AXIS_Y_COLOR: [f32; 4] = [0.1372, 0.3019, 0.1764, 1.0];
pub const CS_AXIS_Z_COLOR: [f32; 4] = [0.4549, 0.4588, 0.9019, 1.0];

pub const CS_AXES_SCALE: f32 = 0.1;
pub const CS_AXES_CAPS_HEIGHT: f32 = 0.15; // arrow length
pub const CS_AXES_CAPS_WIDTH: f32 = 0.075; // half of arrow width
pub const CS_AXES_CAPS_BASE_POINTS_NUMBER: u32 = 12; // the number of points in cone circular base

pub const CS_AXES_X_SHIFT: f32 = 0.85; // shift of the cs in the x-direction
pub const CS_AXES_Y_SHIFT: f32 = 0.85; // shift of the cs in the y-direction
pub const CS_AXES_Z_SHIFT: f32 = -1.5; // shift of the cs in the z-direction

pub const AXIS_X_DENOTATION_SHIFT_X: f32 = 0.1;
pub const AXIS_X_DENOTATION_SHIFT_Y: f32 = -0.05;
pub const AXIS_Y_DENOTATION_SHIFT_X: f32 = -0.05;
pub const AXIS_Y_DENOTATION_SHIFT_Y: f32 = 0.1;
pub const AXIS_Z_DENOTATION_SHIFT_X: f32 = -0.05;
pub const AXIS_Z_DENOTATION_SHIFT_Y: f32 = -0.05;
pub const AXIS_Z_DENOTATION_SHIFT_Z: f32 = 0.1;

pub const CANVAS_AXES_DENOTATION_COLOR: &str = "rgb(211, 211, 211)"; // LightGrey

pub const SELECTION_RECTANGLE_STROKE_COLOR: &str = "rgb(211, 211, 211)"; // LightGrey
pub const SELECTION_RECTANGLE_FILL_COLOR: &str = "rgba(47, 79, 79, 0.5)"; // DarkSlateGrey

pub const HINTS_COLOR: &str = "rgb(211, 211, 211)"; // LightGrey


// Consts for drawn object
pub const DRAWN_NODES_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0]; // yellow
pub const CANVAS_DRAWN_NODES_DENOTATION_COLOR: &str = "yellow";

pub const DRAWN_POINTS_COLOR: [f32; 4] = [0.26, 0.81, 0.20, 1.0]; // apple
pub const CANVAS_DRAWN_POINTS_DENOTATION_COLOR: &str = "rgb(67, 208, 52)";

pub const DRAWN_POINT_OBJECT_DENOTATION_SHIFT: f32 = 0.02;

pub const DRAWN_ELEMENTS_COLOR: [f32; 4] = [0.0, 1.0, 1.0, 1.0]; // cyan
pub const CANVAS_DRAWN_ELEMENTS_DENOTATION_COLOR: &str = "cyan";

pub const DRAWN_LINES_DEFAULT_COLOR: [f32; 4] = [0.6, 0.196, 0.8, 1.0]; // DarkOrchid
pub const CANVAS_DRAWN_LINES_DEFAULT_DENOTATION_COLOR: &str = "rgb(153, 50, 204)"; // DarkOrchid

pub const DRAWN_LINES_TRUSS_PROPS_COLOR: [f32; 4] = [0.4, 0.803, 0.666, 1.0]; // MediumAquamarine
pub const CANVAS_DRAWN_LINES_TRUSS_PROPS_DENOTATION_COLOR: &str = "rgb(102, 205, 170)"; // MediumAquamarine

pub const DRAWN_LINES_BEAM_PROPS_COLOR: [f32; 4] = [1.0, 0.894, 0.709, 1.0]; // Moccasin
pub const CANVAS_DRAWN_LINES_BEAM_PROPS_DENOTATION_COLOR: &str = "rgb(255, 228, 181)"; // Moccasin

pub const DRAWN_LINE_OBJECTS_BASE_POINTS_NUMBER: u32 = 12; // the number of points in cylinder circular base
pub const DRAWN_LINE_OBJECTS_BASE_RADIUS: f32 = 0.006; // the radius of cylinder circular base

pub const DRAWN_LINE_OBJECTS_DENOTATION_SHIFT: f32 = 0.01;

pub const DRAWN_BEAM_SECTION_ORIENTATION_LINE_LENGTH: f32 = 0.07; // line length
pub const DRAWN_BEAM_SECTION_ORIENTATION_CAPS_HEIGHT: f32 = 0.015; // arrow length
pub const DRAWN_BEAM_SECTION_ORIENTATION_CAPS_WIDTH: f32 = 0.007; // half of arrow width
pub const DRAWN_BEAM_SECTION_ORIENTATION_CAPS_BASE_POINTS_NUMBER: u32 = 12; // the number of points in cone circular base
pub const DRAWN_BEAM_SECTION_ORIENTATION_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // red

pub const DRAWN_CONCENTRATED_LOADS_LINE_LENGTH: f32 = 0.04; // line length
pub const DRAWN_CONCENTRATED_LOADS_CAPS_HEIGHT: f32 = 0.0085; // arrow length
pub const DRAWN_CONCENTRATED_LOADS_CAPS_WIDTH: f32 = 0.00325; // half of arrow width
pub const DRAWN_CONCENTRATED_LOADS_CAPS_BASE_POINTS_NUMBER: u32 = 12; // the number of points in cone circular base
pub const DRAWN_CONCENTRATED_LOADS_COLOR: [f32; 4] = [0.117, 0.564, 1.0, 1.0]; // DodgerBlue

pub const DRAWN_DISTRIBUTED_LINE_LOADS_LINE_LENGTH: f32 = 0.04; // line length
pub const DRAWN_DISTRIBUTED_LINE_LOADS_CAPS_HEIGHT: f32 = 0.0085; // arrow length
pub const DRAWN_DISTRIBUTED_LINE_LOADS_CAPS_WIDTH: f32 = 0.00325; // half of arrow width
pub const DRAWN_DISTRIBUTED_LINE_LOADS_CAPS_BASE_POINTS_NUMBER: u32 = 12; // the number of points in cone circular base
pub const DRAWN_DISTRIBUTED_LINE_LOADS_COLOR: [f32; 4] = [0.529, 0.807, 0.921, 1.0]; // SkyBlue
pub const NUMBER_OF_DISTRIBUTED_LINE_LOAD_ARROWS: usize = 11;

pub const DRAWN_BOUNDARY_CONDITION_CAPS_HEIGHT: f32 = 0.015; // arrow length
pub const DRAWN_BOUNDARY_CONDITION_CAPS_WIDTH: f32 = 0.007; // half of arrow width
pub const DRAWN_BOUNDARY_CONDITION_CAPS_BASE_POINTS_NUMBER: u32 = 12; // the number of points in cone circular base
pub const DRAWN_BOUNDARY_CONDITION_COLOR: [f32; 4] = [1.0, 0.549, 0.0, 1.0]; // DarkOrange
