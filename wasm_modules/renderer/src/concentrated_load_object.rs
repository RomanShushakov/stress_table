#[derive(Copy, Clone, Debug)]
pub enum Sign
{
    Positive,
    Negative,
}


#[derive(Debug)]
pub struct ConcentratedLoadObject
{
    optional_fx: Option<Sign>,
    optional_fy: Option<Sign>,
    optional_fz: Option<Sign>,
    optional_mx: Option<Sign>,
    optional_my: Option<Sign>,
    optional_mz: Option<Sign>,
    uid: u32,
}


impl ConcentratedLoadObject
{
    pub fn create(fx: f32, fy: f32, fz: f32, mx: f32, my: f32, mz: f32, uid: u32) -> Self
    {
        let optional_fx = if fx > 0f32 { Some(Sign::Positive) }
            else if fx < 0f32 { Some(Sign::Negative) } else { None };
        let optional_fy = if fy > 0f32 { Some(Sign::Positive) }
            else if fy < 0f32 { Some(Sign::Negative) } else { None };
        let optional_fz = if fz > 0f32 { Some(Sign::Positive) }
            else if fz < 0f32 { Some(Sign::Negative) } else { None };
        let optional_mx = if mx > 0f32 { Some(Sign::Positive) }
            else if mx < 0f32 { Some(Sign::Negative) } else { None };
        let optional_my = if my > 0f32 { Some(Sign::Positive) }
            else if my < 0f32 { Some(Sign::Negative) } else { None };
        let optional_mz = if mz > 0f32 { Some(Sign::Positive) }
            else if mz < 0f32 { Some(Sign::Negative) } else { None };
        ConcentratedLoadObject { optional_fx, optional_fy, optional_fz,
            optional_mx, optional_my, optional_mz, uid }
    }


    pub fn is_uid_same(&self, uid: u32) -> bool
    {
        self.uid == uid
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }


    pub fn optional_fx(&self) -> &Option<Sign>
    {
        &self.optional_fx
    }
}
