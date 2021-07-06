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
