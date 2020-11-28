pub trait One
{
    fn one() -> Self;
    fn minus_one() -> Self;
}


impl One for f64
{
    fn one() -> f64
    {
        1.0
    }

    fn minus_one() -> f64
    {
        - 1.0
    }
}


impl One for f32
{
    fn one() -> f32
    {
        1.0
    }


    fn minus_one() -> f32
    {
        - 1.0
    }
}


impl One for i32
{
    fn one() -> i32
    {
        1
    }


    fn minus_one() -> i32
    {
        - 1
    }
}


pub trait FloatNum
{
    fn sqrt(self) -> Self;
    fn is_nan(self) -> bool;
}


impl FloatNum for f64
{
    fn sqrt(self) -> f64
    {
        self.sqrt()
    }


    fn is_nan(self) -> bool
    {
        self.is_nan()
    }
}