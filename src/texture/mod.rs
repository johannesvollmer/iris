use crate::geometry::GeometryHitInfo;

pub mod constant;

pub trait Texture<T> {
    fn eval(&self, dg: &GeometryHitInfo) -> T;
}
