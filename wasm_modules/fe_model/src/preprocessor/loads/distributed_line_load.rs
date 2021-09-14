use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct DistributedLineLoad<V>
{
    qx: V,
    qy: V,
    qz: V,
}


impl<V> DistributedLineLoad<V>
    where V: Copy + PartialEq
{
    pub fn create(qx: V, qy: V, qz: V) -> Self
    {
        DistributedLineLoad { qx, qy, qz }
    }


    pub fn update(&mut self, qx: V, qy: V, qz: V)
    {
        self.qx = qx;
        self.qy = qy;
        self.qz = qz;
    }


    pub fn copy_load_components(&self) -> (V, V, V)
    {
        (self.qx, self.qy, self.qz)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedDistributedLineLoad<T, V>
{
    line_number: T,
    distributed_line_load: DistributedLineLoad<V>,
}


impl<T, V> DeletedDistributedLineLoad<T, V>
    where T: Copy,
          V: Copy + PartialEq,
{
    pub fn create(line_number: T, distributed_line_load: DistributedLineLoad<V>) -> Self
    {
        DeletedDistributedLineLoad { line_number, distributed_line_load }
    }


    pub fn copy_line_number_and_load_components(&self) -> (T, V, V, V)
    {
        let (qx, qy, qz) = self.distributed_line_load.copy_load_components();
        (self.line_number, qx, qy, qz)
    }
}
