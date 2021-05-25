#[derive(Clone, Debug)]
pub struct Coordinates
{
    x: f64,
    y: f64,
    z: f64,
}


impl Coordinates
{
    pub fn create(x: f64, y: f64, z: f64) -> Coordinates
    {
        Coordinates { x, y, z }
    }


    pub fn get_x(&self) -> f64
    {
        self.x
    }


    pub fn get_y(&self) -> f64
    {
        self.y
    }


    pub fn get_z(&self) -> f64
    {
        self.z
    }
}


#[derive(Debug, Clone)]
pub enum GeometryActionType
{
    AddPoint(u32, Coordinates, bool),   // ( number, Coordinates, is_action_id_should_be_increased )
    UpdatePoint(u32, Coordinates, Coordinates, bool),   // ( number, Coordinates, Coordinates, is_action_id_should_be_increased )
    DeletePoint(u32, bool), // ( number, is_action_id_should_be_increased )
    UndoDeletePoint(u32, bool), // ( number, is_action_id_should_be_increased )
    AddLine(u32, u32, u32, bool),   // ( number, start_point_number, end_point_number, is_action_id_should_be_increased )
    UpdateLine(u32, u32, u32, u32, u32, bool),  // ( number, old_start_point_number, old_end_point_number, new_start_point_number, new_end_point_number, is_action_id_should_be_increased )
    DeleteLine(u32, bool),  // ( number, is_action_id_should_be_increased )
    UndoDeleteLine(u32, bool),  // ( number, is_action_id_should_be_increased )
}


#[derive(Debug, Clone)]
pub enum PropertiesActionType
{
    AddMaterial(String, f64, f64, bool),    // ( name, young_modulus, poisson_ratio, is_action_id_should_be_increased )
    UpdateMaterial(String, f64, f64, f64, f64, bool),    // ( name, old_young_modulus, old_poisson_ratio, new_young_modulus, new_poisson_ratio, is_action_id_should_be_increased )
    DeleteMaterial(String, bool),   // ( name, is_action_id_should_be_increased )
    UndoDeleteMaterial(String, bool), // ( name, is_action_id_should_be_increased )
}


#[derive(Debug, Clone)]
pub enum ActionType
{
    GeometryActionType(GeometryActionType),
    ShowPointInfo(u32, js_sys::Function),   // ( number, show_object_info_handle )
    ShowLineInfo(u32, js_sys::Function),    // ( number, show_object_info_handle )
    ChangeView(String, js_sys::Function),   // ( name, change_view_handle )
    PropertiesActionType(PropertiesActionType),
}


impl From<GeometryActionType> for ActionType
{
    fn from(action_type: GeometryActionType) -> ActionType
    {
        ActionType::GeometryActionType(action_type)
    }
}


impl From<PropertiesActionType> for ActionType
{
    fn from(action_type: PropertiesActionType) -> ActionType
    {
        ActionType::PropertiesActionType(action_type)
    }
}


#[derive(Clone)]
pub struct Action
{
    action_id: u32,
    action_type: ActionType,
}


impl Action
{
    pub fn create(action_id: u32, action_type: ActionType) -> Action
    {
        Action { action_id, action_type }
    }


    pub fn action_id_same(&self, action_id: u32) -> bool
    {
        self.action_id == action_id
    }


    pub fn get_action_id(&self) -> u32
    {
        self.action_id
    }


    pub fn get_action_type(&self) -> ActionType
    {
        self.action_type.clone()
    }
}