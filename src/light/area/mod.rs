use crate::film::spectrum::Spectrum;
use crate::light::Light;
use crate::math::*;

//pub mod diffuse;

pub trait AreaLight: Light {
    fn radiance(point: Point3f, w: Vec3f) -> Spectrum;
}
