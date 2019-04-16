use crate::math::Float;

pub trait Filter {
    fn evaluate(&self, x: Float, y: Float) -> Float;
    fn radius(&self) -> Float;
}

mod triangle;
pub use triangle::Triangle;

mod mitchell;
pub use mitchell::Mitchell;
