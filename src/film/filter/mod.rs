pub trait Filter {
    fn evaluate(&self, x: f32, y: f32) -> f32;
}

mod triangle;
