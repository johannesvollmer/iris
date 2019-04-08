#[cfg(not(feature = "double_float"))]
pub type Float = f32;

#[cfg(feature = "double_float")]
pub type Float = f64;

#[macro_use]
pub mod vec_base;
pub use vec_base::*;

#[macro_use]
pub mod point_base;
pub use point_base::*;

pub mod vec2;
pub use vec2::*;

pub mod vec3;
pub use vec3::Vec3f;

pub mod shading_vec;
pub use shading_vec::*;

pub mod local_vec;
pub use local_vec::*;

pub mod normal;
pub use normal::*;

pub mod local_normal;
pub use local_normal::*;

pub mod shading_normal;
pub use shading_normal::*;

pub mod bounds;
pub use bounds::*;

pub mod point2;
pub use point2::*;

pub mod point3;
pub use point3::*;

pub mod local_point;
pub use local_point::*;

pub mod ray;
pub use ray::*;

pub mod transform;
pub use transform::*;

pub mod sample;

pub mod efloat;
pub use efloat::*;

pub mod misc;
pub use misc::*;
