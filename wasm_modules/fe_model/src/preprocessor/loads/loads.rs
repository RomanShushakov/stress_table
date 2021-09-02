use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct ConcentratedLoad<V>
{
    fx: V,
    fy: V,
    fz: V,
    mx: V,
    my: V,
    mz: V,
}
