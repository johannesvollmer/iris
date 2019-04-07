use crate::film::spectrum::Spectrum;
use crate::geometry::{Hit, HitInfo, AABB};
use crate::light::{point, Light, LightType, Visibility};
use crate::math::*;

#[derive(Clone)]
pub struct Emitter {
    light: LightType,
    transform: Transform,
    inv_transform: Transform,
}

impl Emitter {
    pub fn new_point(emission: Spectrum, transform: Transform) -> Self {
        Self {
            light: LightType::Point(point::Point::new(emission)),
            transform,
            inv_transform: transform.inverse(),
        }
    }

    pub fn sample(
        &self,
        hit: &HitInfo,
        samples: (Float, Float),
    ) -> (Spectrum, Vec3f, Float, Visibility) {
        let (energy, sample_point_local, pdf) = match &self.light {
            LightType::Point(point) => point.sample(samples),
        };

        let sample_point = self.transform.apply_point(sample_point_local.as_global());
        let dir = sample_point - hit.gg.point;

        let vis = Visibility::new(hit, sample_point);
        let radiance = energy / dir.length_squared();

        (radiance, dir.normalized(), pdf, vis)
    }

    pub fn pdf(&self, _p: Point3f, _wi: Vec3f, _time: Float) -> Float {
        match self.light {
            LightType::Point(_) => unreachable!(),
        }
    }

    pub fn is_delta(&self) -> bool {
        match self.light {
            LightType::Point(_) => true,
        }
    }
}

impl AABB for Emitter {
    fn aabb(&self) -> Bounds3f {
        match self.light {
            LightType::Point(_) => unreachable!(),
        }
    }
}

impl Hit for Emitter {
    fn intersect(&self, _ray: &Ray) -> Option<HitInfo> {
        unimplemented!()
    }
}
