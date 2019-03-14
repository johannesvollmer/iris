pub trait Filter {
    fn evaluate(&self, x: f32, y: f32) -> f32;
    fn width(&self) -> f32;
}

mod triangle;
pub use triangle::Triangle;
