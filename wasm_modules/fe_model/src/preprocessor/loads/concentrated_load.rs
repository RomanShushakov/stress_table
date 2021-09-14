use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct ConcentratedLoad<V>
{
    fx: V,
    fy: V,
    fz: V,
    mx: V,
    my: V,
    mz: V,
}


impl<V> ConcentratedLoad<V>
    where V: Copy + PartialEq
{
    pub fn create(fx: V, fy: V, fz: V, mx: V, my: V, mz: V) -> Self
    {
        ConcentratedLoad { fx, fy, fz, mx, my, mz }
    }


    pub fn update(&mut self, fx: V, fy: V, fz: V, mx: V, my: V, mz: V)
    {
        self.fx = fx;
        self.fy = fy;
        self.fz = fz;
        self.mx = mx;
        self.my = my;
        self.mz = mz;
    }


    pub fn copy_load_components(&self) -> (V, V, V)
    {
        (self.fx, self.fy, self.fz)
    }


    pub fn copy_moment_components(&self) -> (V, V, V)
    {
        (self.mx, self.my, self.mz)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedConcentratedLoad<T, V>
{
    point_number: T,
    concentrated_load: ConcentratedLoad<V>,
}


impl<T, V> DeletedConcentratedLoad<T, V>
    where T: Copy,
          V: Copy + PartialEq,
{
    pub fn create(point_number: T, concentrated_load: ConcentratedLoad<V>) -> Self
    {
        DeletedConcentratedLoad { point_number, concentrated_load }
    }


    pub fn copy_point_number_and_load_components(&self) -> (T, V, V, V, V, V, V)
    {
        let (fx, fy, fz) = self.concentrated_load.copy_load_components();
        let (mx, my, mz) = self.concentrated_load.copy_moment_components();
        (self.point_number, fx, fy, fz, mx, my, mz)
    }
}
