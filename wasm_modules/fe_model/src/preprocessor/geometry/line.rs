use serde::Serialize;


#[derive(Debug, Copy, Clone, Serialize)]
pub struct Line<T>
{
    start_point_number: T,
    end_point_number: T,
}


impl<T> Line<T>
    where T: Copy + PartialEq
{
    pub fn create(start_point_number: T, end_point_number: T) -> Self
    {
        Line { start_point_number, end_point_number, }
    }


    pub fn are_start_and_end_points_same(&self, start_point_number: T, end_point_number: T) -> bool
    {
        (self.start_point_number == start_point_number &&
        self.end_point_number == end_point_number) ||
        (self.start_point_number == end_point_number &&
        self.end_point_number == start_point_number)
    }


    pub fn update(&mut self, start_point_number: T, end_point_number: T)
    {
        self.start_point_number = start_point_number;
        self.end_point_number = end_point_number;
    }


    pub fn extract_points_numbers(&self) -> (T, T)
    {
        (self.start_point_number, self.end_point_number)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct DeletedLine<T>
{
    number: T,
    line: Line<T>,
}


impl<T> DeletedLine<T>
    where T: Copy + PartialEq
{
    pub fn create(number: T, line: Line<T>) -> Self
    {
        DeletedLine { number, line }
    }


    pub fn extract_number_and_points_numbers(&self) -> (T, T, T)
    {
        let (start_point_number, end_point_number) = self.line.extract_points_numbers();
        (self.number, start_point_number, end_point_number)
    }


    pub fn extract_number(&self) -> T
    {
        self.number
    }


    pub fn is_number_same(&self, number: T) -> bool
    {
        self.number == number
    }
}
