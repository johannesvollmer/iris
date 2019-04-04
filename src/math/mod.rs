#[cfg(not(use_f64))]
pub type Float = f32;

#[cfg(use_f64)]
pub type Float = f64;

pub mod vec;
pub use vec::*;

pub mod localvec;
pub use localvec::*;

pub mod bounds;
pub use bounds::*;

pub mod point;
pub use point::*;

pub mod ray;
pub use ray::*;

pub mod normal;
pub use normal::*;

pub mod transform;
pub use transform::*;

pub mod sample;

pub mod misc;
pub use misc::*;
