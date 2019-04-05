#[cfg(not(use_f64))]
pub type Float = f32;

#[cfg(use_f64)]
pub type Float = f64;

#[macro_use]
pub mod vecbase;
pub use vecbase::*;

pub mod vec2;
pub use vec2::*;

pub mod vec3;
pub use vec3::Vec3f;

pub mod shadingvec;
pub use shadingvec::*;

pub mod localvec;
pub use localvec::*;

pub mod normal;
pub use normal::*;

pub mod localnormal;
pub use localnormal::*;

pub mod bounds;
pub use bounds::*;

pub mod point;
pub use point::*;

pub mod ray;
pub use ray::*;

pub mod transform;
pub use transform::*;

pub mod sample;

pub mod misc;
pub use misc::*;
