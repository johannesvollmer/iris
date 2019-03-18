use super::Filter;
use crate::math::Float;

#[derive(new, Copy, Clone)]
pub struct Triangle {
    width: Float,
}

impl Filter for Triangle {
    fn evaluate(&self, x: Float, y: Float) -> Float {
        Float::max(0.0, 1.0 - x.abs()) * Float::max(0.0, 1.0 - y.abs())
    }

    fn width(&self) -> Float {
        self.width
    }
}
