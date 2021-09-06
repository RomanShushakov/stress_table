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
        let optional_fx_sign = if fx > 0f32 { Some(Sign::Positive) }
            else if fx < 0f32 { Some(Sign::Negative) } else { None };
        let optional_fy_sign = if fy > 0f32 { Some(Sign::Positive) }
            else if fy < 0f32 { Some(Sign::Negative) } else { None };
        let optional_fz_sign = if fz > 0f32 { Some(Sign::Positive) }
            else if fz < 0f32 { Some(Sign::Negative) } else { None };
        let optional_mx_sign = if mx > 0f32 { Some(Sign::Positive) }
            else if mx < 0f32 { Some(Sign::Negative) } else { None };
        let optional_my_sign = if my > 0f32 { Some(Sign::Positive) }
            else if my < 0f32 { Some(Sign::Negative) } else { None };
        let optional_mz_sign = if mz > 0f32 { Some(Sign::Positive) }
            else if mz < 0f32 { Some(Sign::Negative) } else { None };
        ConcentratedLoadObject {
            optional_fx_sign,
            optional_fy_sign,
            optional_fz_sign,
            optional_mx_sign,
            optional_my_sign,
            optional_mz_sign, uid }
    }


    pub fn update_load_and_moment_components(&mut self, fx: f32, fy: f32, fz: f32, mx: f32, my: f32,
        mz: f32)
    {
        let optional_fx_sign = if fx > 0f32 { Some(Sign::Positive) }
            else if fx < 0f32 { Some(Sign::Negative) } else { None };
        let optional_fy_sign = if fy > 0f32 { Some(Sign::Positive) }
            else if fy < 0f32 { Some(Sign::Negative) } else { None };
        let optional_fz_sign = if fz > 0f32 { Some(Sign::Positive) }
            else if fz < 0f32 { Some(Sign::Negative) } else { None };
        let optional_mx_sign = if mx > 0f32 { Some(Sign::Positive) }
            else if mx < 0f32 { Some(Sign::Negative) } else { None };
        let optional_my_sign = if my > 0f32 { Some(Sign::Positive) }
            else if my < 0f32 { Some(Sign::Negative) } else { None };
        let optional_mz_sign = if mz > 0f32 { Some(Sign::Positive) }
            else if mz < 0f32 { Some(Sign::Negative) } else { None };
        self.optional_fx_sign = optional_fx_sign;
        self.optional_fy_sign = optional_fy_sign;
        self.optional_fz_sign = optional_fz_sign;
        self.optional_mx_sign = optional_mx_sign;
        self.optional_my_sign = optional_my_sign;
        self.optional_mz_sign = optional_mz_sign;
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
