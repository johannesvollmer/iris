use crate::film::spectrum::Spectrum;
use crate::geometry::{Hit, HitInfo, AABB};
use crate::light::Light;
use crate::light::Visibility;
use crate::math::*;

#[derive(Clone)]
pub struct Emitter {
    pub emission: Spectrum,
    transform: Transform,
    inv_transform: Transform,
}

impl Emitter {
    pub fn new_point(emission: Spectrum, transform: Transform) -> Self {
        Self {
            emission,
            transform,
            inv_transform: transform.inverse(),
        }
    }
}

impl AABB for Emitter {
    fn aabb(&self) -> Bounds3f {
        unimplemented!()
    }
}

impl Hit for Emitter {
    fn intersect(&self, _ray: &Ray) -> Option<HitInfo> {
        unimplemented!()
    }
}

impl Light for Emitter {
    fn sample(&self, hit: &HitInfo, _samples: (f32, f32)) -> (Spectrum, Vec3f, Float, Visibility) {
        let pos = self.transform.apply_point(Point3f::new(0.0, 0.0, 0.0));
        let vis = Visibility::new(hit, pos);
        let d = pos - hit.lg.point;
        let radiance = self.emission / d.length_squared();
        (radiance, d.normalized(), 1.0, vis)
    }

    fn pdf(&self, _p: Point3f, _wi: Vec3f, _time: Float) -> Float {
        unimplemented!()
    }

    fn is_delta(&self) -> bool {
        true
    }
}
