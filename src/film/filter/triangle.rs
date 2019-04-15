use super::Filter;
use crate::math::Float;

#[derive(new, Copy, Clone)]
#[allow(dead_code)]
pub struct Triangle {
    width: Float,
}

impl Filter for Triangle {
    fn evaluate(&self, x: Float, y: Float) -> Float {
        Float::max(0.0, 1.0 - x) * Float::max(0.0, 1.0 - y)
    }

    fn width(&self) -> Float {
        self.width
    }
}
