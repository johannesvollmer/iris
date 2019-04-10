use crate::film::spectrum::Spectrum;
use crate::geometry::Sampleable;
use crate::geometry::{Hit, Interaction, SurfaceInteraction, AABB};
use crate::light::{diffuse_area, point, spot, Light, LightType, Visibility};
use crate::material::Material;
use crate::math::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Emitter {
    light_type: LightType,
    light: Arc<dyn Light + Send + Sync>,
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
        target: Point3f,
        up: Vec3f,
        theta_start_deg: Float,
        theta_end_deg: Float,
    ) -> Self {
        Self {
            light: Arc::new(spot::Spot::new(
                intensity,
                pos,
                target - pos,
                up,
                theta_start_deg,
                theta_end_deg,
            )),
            light_type: LightType::Spot,
        }
    }

    #[allow(dead_code)]
    pub fn new_area(
        intensity: Spectrum,
        transform: Transform,
        geometry: Arc<dyn Sampleable + Send + Sync>,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            light: Arc::new(diffuse_area::DiffuseArea::new(
                intensity, transform, geometry, material,
            )),
            light_type: LightType::Area,
        }
    }

    pub fn sample_incoming(
        &self,
        int: &Interaction,
        samples: (Float, Float),
    ) -> (Spectrum, Vec3f, Float, Visibility) {
        let (radiance, dir, pdf) = self.light.sample_incoming(int, samples);
        (
            radiance,
            dir.normalized(),
            pdf,
            Visibility::new(int, int.point + dir),
        )
    }

    // pub fn pdf(&self, int: &Interaction, wi: Vec3f) -> Float {
    //     debug_assert!(self.light_type == LightType::Area);
    //     self.light.pdf(int, wi)
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
            LightType::Area => self.light.aabb(),
        }
    }
}

impl Hit for Emitter {
    fn intersect(&self, ray: &Ray) -> Option<(SurfaceInteraction, Float)> {
        self.light.intersect(ray)
    }
}
