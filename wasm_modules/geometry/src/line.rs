#[derive(Debug, Copy, Clone)]
pub struct Line
{
    start_point_number: u32,
    end_point_number: u32,
}


impl Line
{
    pub fn create(start_point_number: u32, end_point_number: u32) -> Self
    {
        Line { start_point_number, end_point_number, }
    }


    pub fn start_and_end_points_same(&self, start_point_number: u32, end_point_number: u32) -> bool
    {
        (self.start_point_number == start_point_number &&
        self.end_point_number == end_point_number) ||
        (self.start_point_number == end_point_number &&
        self.end_point_number == start_point_number)
    }


    pub fn update(&mut self, start_point_number: u32, end_point_number: u32)
    {
        self.start_point_number = start_point_number;
        self.end_point_number = end_point_number;
    }


    pub fn extract_points_numbers(&self) -> (u32, u32)
    {
        (self.start_point_number, self.end_point_number)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct DeletedLine
{
    number: u32,
    line: Line,
}


impl DeletedLine
{
    pub fn create(number: u32, line: Line) -> Self
    {
        DeletedLine { number, line }
    }


    pub fn extract_number_and_points_numbers(&self) -> (u32, u32, u32)
    {
        let (start_point_number, end_point_number) = self.line.extract_points_numbers();
        (self.number, start_point_number, end_point_number)
    }


    pub fn extract_number(&self) -> u32
    {
        self.number
    }


    pub fn number_same(&self, number: u32) -> bool
    {
        self.number == number
    }
}