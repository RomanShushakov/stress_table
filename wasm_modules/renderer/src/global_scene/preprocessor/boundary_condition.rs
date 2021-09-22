#[derive(Debug)]
pub struct BoundaryCondition
{
    uid: u32,
}


impl BoundaryCondition
{
    pub fn create(uid: u32) -> Self
    {
        BoundaryCondition { uid }
    }


    pub fn is_uid_same(&self, uid: u32) -> bool
    {
        self.uid == uid
    }


    pub fn copy_uid(&self) -> u32
    {
        self.uid
    }
}
