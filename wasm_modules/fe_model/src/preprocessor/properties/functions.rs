pub fn are_line_numbers_same<T>(lhs_line_numbers: &[T], rhs_line_numbers: &[T]) -> bool
    where T: PartialEq + Eq,
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