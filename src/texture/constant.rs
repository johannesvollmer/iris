use super::Texture;
use crate::geometry::GeometryHitInfo;

#[derive(new, Copy, Clone)]
pub struct ConstantTexture<T: Copy> {
    c: T,
}

impl<T: Copy> Texture<T> for ConstantTexture<T> {
    fn eval(&self, _dg: &GeometryHitInfo) -> T {
        self.c
    }
}
