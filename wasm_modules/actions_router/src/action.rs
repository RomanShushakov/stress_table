use crate::types::{FEUInt, FEFloat};


#[derive(Clone, Debug)]
pub struct Coordinates
{
    x: FEFloat,
    y: FEFloat,
    z: FEFloat,
}


impl Coordinates
{
    pub fn create(x: FEFloat, y: FEFloat, z: FEFloat) -> Coordinates
    {
        Coordinates { x, y, z }
    }


    pub fn get_x(&self) -> FEFloat
    {
        self.x
    }


    pub fn get_y(&self) -> FEFloat
    {
        self.y
    }


    pub fn get_z(&self) -> FEFloat
    {
        self.z
    }
}


#[derive(Debug, Clone)]
pub enum GeometryActionType
{
    // ( number, Coordinates, is_action_id_should_be_increased )
    AddPoint(FEUInt, Coordinates, bool),

    // ( number, Coordinates, Coordinates, is_action_id_should_be_increased )
    UpdatePoint(FEUInt, Coordinates, Coordinates, bool),

    // ( number, is_action_id_should_be_increased )
    DeletePoint(FEUInt, bool),

    // ( number, is_action_id_should_be_increased )
    RestorePoint(FEUInt, bool),

    // ( number, start_point_number, end_point_number, is_action_id_should_be_increased )
    AddLine(FEUInt, FEUInt, FEUInt, bool),

    // ( number, old_start_point_number, old_end_point_number,
    // new_start_point_number, new_end_point_number, is_action_id_should_be_increased )
    UpdateLine(FEUInt, FEUInt, FEUInt, FEUInt, FEUInt, bool),

    // ( number, is_action_id_should_be_increased )
    DeleteLine(FEUInt, bool),

    // ( number, is_action_id_should_be_increased )
    RestoreLine(FEUInt, bool),
}


#[derive(Debug, Clone)]
pub enum PropertiesActionType
{
    // ( name, young_modulus, poisson_ratio, is_action_id_should_be_increased )
    AddMaterial(String, FEFloat, FEFloat, bool),

    // ( name, old_young_modulus, old_poisson_ratio,
    // new_young_modulus, new_poisson_ratio, is_action_id_should_be_increased )
    UpdateMaterial(String, FEFloat, FEFloat, FEFloat, FEFloat, bool),

    // ( name, is_action_id_should_be_increased )
    DeleteMaterial(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestoreMaterial(String, bool),

    // ( name, area, area2, is_action_id_should_be_increased )
    AddTrussSection(String, FEFloat, Option<FEFloat>, bool),

    // ( name, old_area, old_area2, new_area, new_area2, is_action_id_should_be_increased )
    UpdateTrussSection(String, FEFloat, Option<FEFloat>, FEFloat, Option<FEFloat>, bool),

    // ( name, is_action_id_should_be_increased )
    DeleteTrussSection(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestoreTrussSection(String, bool),

    // ( name, area, I11, I22, I12, It, shear_factor, is_action_id_should_be_increased )
    AddBeamSection(String, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, bool),

    // ( name, old_area, old_I11, old_I22, old_I12, old_It, old_shear_factor, new_area,
    // new_I11, new_I22, new_I12, new_It, new_shear_factor, is_action_id_should_be_increased )
    UpdateBeamSection(String, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat,
        FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, FEFloat, bool),

    // ( name, is_action_id_should_be_increased )
    DeleteBeamSection(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestoreBeamSection(String, bool),

    // ( name, material_name, cross_section_name, cross_section_type,
    // is_action_id_should_be_increased )
    AddProperties(String, String, String, String, bool),

    // ( name, old_material_name, old_cross_section_name, old_cross_section_type,
    // new_material_name, new_cross_section_name, new_cross_section_type,
    // is_action_id_should_be_increased )
    UpdateProperties(String, String, String, String, String, String, String, bool),

    // ( name, is_action_id_should_be_increased )
    DeleteProperties(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestoreProperties(String, bool),

    // ( name, line_numbers, is_action_id_should_be_increased )
    AddAssignedPropertiesToLines(String, Vec<FEUInt>, bool),

    // ( name, old_line_numbers, new_line_numbers, is_action_id_should_be_increased )
    UpdateAssignedPropertiesToLines(String, Vec<FEUInt>, Vec<FEUInt>, bool),

    // ( name, is_action_id_should_be_increased )
    DeleteAssignedPropertiesToLines(String, bool),

    // ( name, is_action_id_should_be_increased )
    RestoreAssignedPropertiesToLines(String, bool),

    // ( local_axis_1_direction, is_action_id_should_be_increased )
    AddBeamSectionLocalAxis1Direction(Vec<FEFloat>, bool),

    // ( local_axis_1_direction, is_action_id_should_be_increased )
    RemoveBeamSectionLocalAxis1Direction(Vec<FEFloat>, bool),

    // ( local_axis_1_direction, is_action_id_should_be_increased )
    RestoreBeamSectionLocalAxis1Direction(Vec<FEFloat>, bool),

    // ( local_axis_1_direction, old_line_numbers, new_line_numbers,
    // is_action_id_should_be_increased )
    UpdateBeamSectionOrientationData(Vec<FEFloat>, Vec<FEUInt>, Vec<FEUInt>, bool)
}


#[derive(Clone, Debug)]
pub struct ConcentratedLoad
{
    fx: FEFloat,
    fy: FEFloat,
    fz: FEFloat,
    mx: FEFloat,
    my: FEFloat,
    mz: FEFloat,
}


impl ConcentratedLoad
{
    pub fn create(fx: FEFloat, fy: FEFloat, fz: FEFloat, mx: FEFloat, my: FEFloat, mz: FEFloat)
        -> ConcentratedLoad
    {
        ConcentratedLoad { fx, fy, fz, mx, my, mz }
    }


    pub fn get_fx(&self) -> FEFloat
    {
        self.fx
    }


    pub fn get_fy(&self) -> FEFloat
    {
        self.fy
    }


    pub fn get_fz(&self) -> FEFloat
    {
        self.fz
    }


    pub fn get_mx(&self) -> FEFloat
    {
        self.mx
    }


    pub fn get_my(&self) -> FEFloat
    {
        self.my
    }


    pub fn get_mz(&self) -> FEFloat
    {
        self.mz
    }
}


#[derive(Debug, Clone)]
pub enum LoadsActionType
{
    // ( point_number, ConcentratedLoad, is_action_id_should_be_increased )
    AddConcentratedLoad(FEUInt, ConcentratedLoad, bool),

    // ( point_number, ConcentratedLoad, ConcentratedLoad, is_action_id_should_be_increased )
    UpdateConcentratedLoad(FEUInt, ConcentratedLoad, ConcentratedLoad, bool),

    // ( point_number, is_action_id_should_be_increased )
    DeleteConcentratedLoad(FEUInt, bool),

    // ( point_number, is_action_id_should_be_increased )
    RestoreConcentratedLoad(FEUInt, bool),
}


#[derive(Debug, Clone)]
pub enum ActionType
{
    GeometryActionType(GeometryActionType),
    PropertiesActionType(PropertiesActionType),
    LoadsActionType(LoadsActionType),
}


impl From<GeometryActionType> for ActionType
{
    fn from(action_type: GeometryActionType) -> Self
    {
        ActionType::GeometryActionType(action_type)
    }
}


impl From<PropertiesActionType> for ActionType
{
    fn from(action_type: PropertiesActionType) -> Self
    {
        ActionType::PropertiesActionType(action_type)
    }
}


impl From<LoadsActionType> for ActionType
{
    fn from(action_type: LoadsActionType) -> Self
    {
        ActionType::LoadsActionType(action_type)
    }
}


#[derive(Clone)]
pub struct Action
{
    action_id: FEUInt,
    action_type: ActionType,
}


impl Action
{
    pub fn create(action_id: FEUInt, action_type: ActionType) -> Action
    {
        Action { action_id, action_type }
    }


    pub fn is_action_id_same(&self, action_id: FEUInt) -> bool
    {
        self.action_id == action_id
    }


    pub fn action_id(&self) -> FEUInt
    {
        self.action_id
    }


    pub fn action_type(&self) -> ActionType
    {
        self.action_type.clone()
    }
}
