use crate::geometry::SurfaceInteraction;

pub mod constant;

pub trait Texture<T> {
    fn eval(&self, si: &SurfaceInteraction) -> T;
}
