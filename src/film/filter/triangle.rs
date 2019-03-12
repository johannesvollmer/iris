#[derive(new, Copy, Clone)]
pub struct Triangle {
    width: f32,
    height: f32
}

impl Filter for Triangle {
    fn evaluate(&self, x: f32, y: f32) -> f32 {
        std::cmp::max(0.0, 1 - x.abs()) * std::cmp::max(0.0, 1 - y.abs())
    }
}