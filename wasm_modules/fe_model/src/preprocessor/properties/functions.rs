use crate::types::FEUInt;


pub fn line_numbers_same(lhs_line_numbers: &[FEUInt], rhs_line_numbers: &[FEUInt]) -> bool
{
    for line_number in rhs_line_numbers
    {
        if !lhs_line_numbers.contains(line_number)
        {
            return false;
        }
    }
    if lhs_line_numbers.len() != rhs_line_numbers.len()
    {
        return false;
    }
    true
}