use crate::geometry::GlobalGeometry;

pub mod constant;

pub trait Texture<T> {
    fn eval(&self, gg: &GlobalGeometry) -> T;
}
