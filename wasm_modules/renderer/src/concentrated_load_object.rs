#[derive(Copy, Clone, Debug)]
pub enum Sign
{
    Positive,
    Negative,
}


pub enum Direction
{
    X, Y, Z
}


#[derive(Debug)]
pub struct ConcentratedLoadObject
{
    optional_fx_sign: Option<Sign>,
    optional_fy_sign: Option<Sign>,
    optional_fz_sign: Option<Sign>,
    optional_mx_sign: Option<Sign>,
    optional_my_sign: Option<Sign>,
    optional_mz_sign: Option<Sign>,
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
        ConcentratedLoadObject {
            optional_fx_sign: optional_fx,
            optional_fy_sign: optional_fy,
            optional_fz_sign: optional_fz,
            optional_mx_sign: optional_mx,
            optional_my_sign: optional_my,
            optional_mz_sign: optional_mz, uid }
    }


    pub fn is_uid_same(&self, uid: u32) -> bool
    {
        self.uid == uid
    }


    pub fn get_uid(&self) -> u32
    {
        self.uid
    }


    pub fn optional_fx_sign(&self) -> &Option<Sign>
    {
        &self.optional_fx_sign
    }


    pub fn optional_fy_sign(&self) -> &Option<Sign>
    {
        &self.optional_fy_sign
    }


    pub fn optional_fz_sign(&self) -> &Option<Sign>
    {
        &self.optional_fz_sign
    }


        pub fn optional_mx_sign(&self) -> &Option<Sign>
    {
        &self.optional_mx_sign
    }


    pub fn optional_my_sign(&self) -> &Option<Sign>
    {
        &self.optional_my_sign
    }


    pub fn optional_mz_sign(&self) -> &Option<Sign>
    {
        &self.optional_mz_sign
    }
}
