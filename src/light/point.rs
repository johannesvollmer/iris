use crate::film::spectrum::Spectrum;
use crate::light::Light;
use crate::math::*;

#[derive(new, Clone)]
pub struct Point {
    emission: Spectrum,
}

impl Light for Point {
    fn sample(&self, _samples: (Float, Float)) -> (Spectrum, LocalPoint3f, Float) {
        (self.emission, LocalPoint3f::default(), 1.0)
    }
}
