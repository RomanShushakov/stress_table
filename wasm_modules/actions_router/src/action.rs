#[derive(Clone, Debug)]
pub struct ObjectNumber(u32);


impl ObjectNumber
{
    pub fn create(number: u32) -> ObjectNumber
    {
        ObjectNumber(number)
    }


    pub fn get_number(&self) -> u32
    {
        self.0
    }
}


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
    AddPoint(ObjectNumber, Coordinates, bool),
    UpdatePoint(ObjectNumber, Coordinates, Coordinates, bool),
    DeletePoint(ObjectNumber, bool),
    UndoDeletePoint(ObjectNumber, bool),
    AddLine(ObjectNumber, ObjectNumber, ObjectNumber, bool),
    UpdateLine(ObjectNumber, ObjectNumber, ObjectNumber, ObjectNumber, ObjectNumber, bool),
    DeleteLine(ObjectNumber, bool),
    UndoDeleteLine(ObjectNumber, bool),
}


#[derive(Debug, Clone)]
pub enum ActionType
{
    GeometryActionType(GeometryActionType),
    ShowPointInfo(ObjectNumber, js_sys::Function),
    ShowLineInfo(ObjectNumber, js_sys::Function),
}


impl From<GeometryActionType> for ActionType
{
    fn from(action_type: GeometryActionType) -> ActionType
    {
        ActionType::GeometryActionType(action_type)
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