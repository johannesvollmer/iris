use crate::film::spectrum::Spectrum;
use crate::geometry::{Hit, Interaction, SurfaceInteraction, AABB};
use crate::light::{point, spot, Light, LightType, Visibility};
use crate::math::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Emitter {
    light: Arc<dyn Light + Send + Sync>,
    light_type: LightType,
}

impl Emitter {
    #[allow(dead_code)]
    pub fn new_point(intensity: Spectrum, pos: Point3f) -> Self {
        Self {
            light: Arc::new(point::Point::new(intensity, pos)),
            light_type: LightType::Point,
        }
    }

    #[allow(dead_code)]
    pub fn new_spot(
        intensity: Spectrum,
        pos: Point3f,
        dir: Vec3f,
        theta_start_deg: Float,
        theta_end_deg: Float,
    ) -> Self {
        Self {
            light: Arc::new(spot::Spot::new(
                intensity,
                pos,
                dir,
                theta_start_deg,
                theta_end_deg,
            )),
            light_type: LightType::Spot,
        }
    }

    pub fn sample(
        &self,
        int: &Interaction,
        samples: (Float, Float),
    ) -> (Spectrum, Vec3f, Float, Visibility) {
        let (radiance, sample_point_local, pdf) = self.light.sample(int.point, samples);

        let sample_point = self
            .light
            .light_to_world()
            .apply_point(sample_point_local.as_global());
        let dir = sample_point - int.point;

        let vis = Visibility::new(int, sample_point);

        (radiance, dir.normalized(), pdf, vis)
    }

    // pub fn pdf(&self, _p: Point3f, _wi: Vec3f, _time: Float) -> Float {
    //     match self.light {
    //         LightType::Point(_) => unreachable!(),
    //     }
    // }

    pub fn is_delta(&self) -> bool {
        match self.light_type {
            LightType::Point => true,
            LightType::Spot => true,
            LightType::Area => false,
        }
    }
}

impl AABB for Emitter {
    fn aabb(&self) -> Bounds3f {
        match self.light_type {
            LightType::Point => unreachable!(),
            LightType::Spot => unreachable!(),
            LightType::Area => unimplemented!(),
        }
    }
}

impl Hit for Emitter {
    fn intersect(&self, _ray: &Ray) -> Option<SurfaceInteraction> {
        unimplemented!()
    }
}
