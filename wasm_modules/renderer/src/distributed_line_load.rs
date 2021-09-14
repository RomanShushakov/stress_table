use crate::concentrated_load::Sign;


#[derive(Debug)]
pub struct DistributedLineLoad
{
    optional_qx_sign: Option<Sign>,
    optional_qy_sign: Option<Sign>,
    optional_qz_sign: Option<Sign>,
    uid: u32,
}


impl DistributedLineLoad
{
    pub fn create(qx: f32, qy: f32, qz: f32, uid: u32) -> Self
    {
        let optional_qx_sign = if qx > 0f32 { Some(Sign::Positive) }
            else if qx < 0f32 { Some(Sign::Negative) } else { None };
        let optional_qy_sign = if qy > 0f32 { Some(Sign::Positive) }
            else if qy < 0f32 { Some(Sign::Negative) } else { None };
        let optional_qz_sign = if qz > 0f32 { Some(Sign::Positive) }
            else if qz < 0f32 { Some(Sign::Negative) } else { None };
        DistributedLineLoad {
            optional_qx_sign,
            optional_qy_sign,
            optional_qz_sign, uid }
    }


    pub fn update_load_components(&mut self, qx: f32, qy: f32, qz: f32)
    {
        let optional_qx_sign = if qx > 0f32 { Some(Sign::Positive) }
            else if qx < 0f32 { Some(Sign::Negative) } else { None };
        let optional_qy_sign = if qy > 0f32 { Some(Sign::Positive) }
            else if qy < 0f32 { Some(Sign::Negative) } else { None };
        let optional_qz_sign = if qz > 0f32 { Some(Sign::Positive) }
            else if qz < 0f32 { Some(Sign::Negative) } else { None };
        self.optional_qx_sign = optional_qx_sign;
        self.optional_qy_sign = optional_qy_sign;
        self.optional_qz_sign = optional_qz_sign;
    }


    pub fn is_uid_same(&self, uid: u32) -> bool
    {
        self.uid == uid
    }


    pub fn copy_uid(&self) -> u32
    {
        self.uid
    }


    pub fn ref_optional_qx_sign(&self) -> &Option<Sign>
    {
        &self.optional_qx_sign
    }


    pub fn ref_optional_qy_sign(&self) -> &Option<Sign>
    {
        &self.optional_qy_sign
    }


    pub fn ref_optional_qz_sign(&self) -> &Option<Sign>
    {
        &self.optional_qz_sign
    }
}
