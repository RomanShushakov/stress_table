#[derive(Clone, Debug)]
pub struct ObjectUIntNumber(u32);


impl ObjectUIntNumber
{
    pub fn create(number: u32) -> ObjectUIntNumber
    {
        ObjectUIntNumber(number)
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


#[derive(Clone, Debug)]
pub struct IsActionIdShouldBeIncreased(bool);


impl IsActionIdShouldBeIncreased
{
    pub fn create(is_action_id_should_be_increased: bool) -> IsActionIdShouldBeIncreased
    {
        IsActionIdShouldBeIncreased(is_action_id_should_be_increased)
    }


    pub fn get_value(&self) -> bool
    {
        self.0
    }
}


#[derive(Debug, Clone)]
pub enum GeometryActionType
{
    AddPoint(ObjectUIntNumber, Coordinates, IsActionIdShouldBeIncreased),
    UpdatePoint(ObjectUIntNumber, Coordinates, Coordinates, IsActionIdShouldBeIncreased),
    DeletePoint(ObjectUIntNumber, IsActionIdShouldBeIncreased),
    UndoDeletePoint(ObjectUIntNumber, IsActionIdShouldBeIncreased),
    AddLine(ObjectUIntNumber, ObjectUIntNumber, ObjectUIntNumber, IsActionIdShouldBeIncreased),
    UpdateLine(ObjectUIntNumber, ObjectUIntNumber, ObjectUIntNumber, ObjectUIntNumber, ObjectUIntNumber, IsActionIdShouldBeIncreased),
    DeleteLine(ObjectUIntNumber, IsActionIdShouldBeIncreased),
    UndoDeleteLine(ObjectUIntNumber, IsActionIdShouldBeIncreased),
}


#[derive(Debug, Clone)]
pub struct Handle(js_sys::Function);


impl Handle
{
    pub fn create(handle: js_sys::Function) -> Handle
    {
        Handle(handle)
    }


    pub fn get_handle(&self) -> js_sys::Function
    {
        self.0.clone()
    }
}


#[derive(Debug, Clone)]
pub struct ObjectName(String);


impl ObjectName
{
    pub fn create(name: String) -> ObjectName
    {
        ObjectName(name)
    }


    pub fn get_name(&self) -> String
    {
        self.0.clone()
    }
}


#[derive(Clone, Debug)]
pub struct ObjectF64Number(f64);


impl ObjectF64Number
{
    pub fn create(number: f64) -> ObjectF64Number
    {
        ObjectF64Number(number)
    }


    pub fn get_number(&self) -> f64
    {
        self.0
    }
}


#[derive(Debug, Clone)]
pub enum PropertiesActionType
{
    AddMaterial(ObjectName, ObjectF64Number, ObjectF64Number, IsActionIdShouldBeIncreased),
}


#[derive(Debug, Clone)]
pub enum ActionType
{
    GeometryActionType(GeometryActionType),
    ShowPointInfo(ObjectUIntNumber, Handle),
    ShowLineInfo(ObjectUIntNumber, Handle),
    ChangeView(ObjectName, Handle),
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