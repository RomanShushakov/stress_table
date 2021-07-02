use serde::Serialize;

use crate::types::{FEUInt};


#[derive(Debug, Copy, Clone, Serialize)]
pub struct Line
{
    start_point_number: FEUInt,
    end_point_number: FEUInt,
}


impl Line
{
    pub fn create(start_point_number: FEUInt, end_point_number: FEUInt) -> Self
    {
        Line { start_point_number, end_point_number, }
    }


    pub fn start_and_end_points_same(&self, start_point_number: FEUInt, end_point_number: FEUInt)
        -> bool
    {
        (self.start_point_number == start_point_number &&
        self.end_point_number == end_point_number) ||
        (self.start_point_number == end_point_number &&
        self.end_point_number == start_point_number)
    }


    pub fn update(&mut self, start_point_number: FEUInt, end_point_number: FEUInt)
    {
        self.start_point_number = start_point_number;
        self.end_point_number = end_point_number;
    }


    pub fn extract_points_numbers(&self) -> (FEUInt, FEUInt)
    {
        (self.start_point_number, self.end_point_number)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct DeletedLine
{
    number: FEUInt,
    line: Line,
}


impl DeletedLine
{
    pub fn create(number: FEUInt, line: Line) -> Self
    {
        DeletedLine { number, line }
    }


    pub fn extract_number_and_points_numbers(&self) -> (FEUInt, FEUInt, FEUInt)
    {
        let (start_point_number, end_point_number) = self.line.extract_points_numbers();
        (self.number, start_point_number, end_point_number)
    }


    pub fn extract_number(&self) -> FEUInt
    {
        self.number
    }


    pub fn number_same(&self, number: FEUInt) -> bool
    {
        self.number == number
    }
}
