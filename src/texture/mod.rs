use crate::geometry::LocalGeometry;

pub mod constant;

pub trait Texture<T> {
    fn eval(&self, dg: &LocalGeometry) -> T;
}
