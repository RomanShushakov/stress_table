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


    pub fn are_load_components_same(&self, fx: V, fy: V, fz: V) -> bool
    {
        self.fx == fx && self.fy == fy && self.fz == fz
    }


    pub fn are_moment_components_same(&self, mx: V, my: V, mz: V) -> bool
    {
        self.mx == mx && self.my == my && self.mz == mz
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
