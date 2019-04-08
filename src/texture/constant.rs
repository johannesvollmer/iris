use super::Texture;
use crate::geometry::SurfaceInteraction;

#[derive(new, Copy, Clone)]
pub struct ConstantTexture<T: Copy> {
    c: T,
}

impl<T: Copy> Texture<T> for ConstantTexture<T> {
    fn eval(&self, _: &SurfaceInteraction) -> T {
        self.c
    }
}
