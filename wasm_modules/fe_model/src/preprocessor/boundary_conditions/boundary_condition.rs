use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct BoundaryCondition<V>
{
    optional_ux: Option<V>,
    optional_uy: Option<V>,
    optional_uz: Option<V>,
    optional_rx: Option<V>,
    optional_ry: Option<V>,
    optional_rz: Option<V>,
}


impl<V> BoundaryCondition<V>
    where V: Copy + PartialEq
{
    pub fn create(optional_ux: Option<V>, optional_uy: Option<V>, optional_uz: Option<V>,
        optional_rx: Option<V>, optional_ry: Option<V>, optional_rz: Option<V>) -> Self
    {
        BoundaryCondition { optional_ux, optional_uy, optional_uz, optional_rx, optional_ry,
            optional_rz }
    }


    pub fn update(&mut self, optional_ux: Option<V>, optional_uy: Option<V>, optional_uz: Option<V>,
        optional_rx: Option<V>, optional_ry: Option<V>, optional_rz: Option<V>)
    {
        self.optional_ux = optional_ux;
        self.optional_uy = optional_uy;
        self.optional_uz = optional_uz;
        self.optional_rx = optional_rx;
        self.optional_ry = optional_ry;
        self.optional_rz = optional_rz;
    }


    pub fn copy_optional_displacement_components(&self) -> (Option<V>, Option<V>, Option<V>)
    {
        (self.optional_ux, self.optional_uy, self.optional_uz)
    }


    pub fn copy_optional_rotation_components(&self) -> (Option<V>, Option<V>, Option<V>)
    {
        (self.optional_rx, self.optional_ry, self.optional_rz)
    }
}


#[derive(Debug, Clone)]
pub struct DeletedBoundaryCondition<T, V>
{
    point_number: T,
    boundary_condition: BoundaryCondition<V>,
}


impl<T, V> DeletedBoundaryCondition<T, V>
    where T: Copy,
          V: Copy + PartialEq,
{
    pub fn create(point_number: T, boundary_condition: BoundaryCondition<V>) -> Self
    {
        DeletedBoundaryCondition { point_number, boundary_condition }
    }


    pub fn copy_point_number_and_optional_components(&self)
        -> (T, Option<V>, Option<V>, Option<V>, Option<V>, Option<V>, Option<V>)
    {
        let (optional_ux, optional_uy, optional_uz) =
            self.boundary_condition.copy_optional_displacement_components();
        let (optional_rx, optional_ry, optional_rz) =
            self.boundary_condition.copy_optional_rotation_components();
        (self.point_number, optional_ux, optional_uy, optional_uz, optional_rx, optional_ry,
         optional_rz)
    }
}
