use crate::types::FEUInt;


pub trait ClearByActionIdTrait
{
    fn clear_by_action_id(&mut self, action_id: FEUInt);
}
