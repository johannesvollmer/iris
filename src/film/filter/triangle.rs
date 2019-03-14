use super::Filter;

#[derive(new, Copy, Clone)]
pub struct Triangle {
    width: f32,
}

impl Filter for Triangle {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        f32::max(0.0, 1.0 - x.abs()) * f32::max(0.0, 1.0 - y.abs())
    }

    fn width(&self) -> f32 {
        self.width
    }
}
