use crate::film::spectrum::Spectrum;
use crate::light::Light;
use crate::math::*;
use num::traits::FloatConst;

#[derive(Clone)]
pub struct Spot {
    world_pos: Point3f,
    cos_falloff_start: Float,
    cos_falloff_end: Float,
    intensity: Spectrum,
    light_to_world: Transform,
    world_to_light: Transform,
}

impl Spot {
    pub fn new(intensity: Spectrum, pos: Point3f, dir: Vec3f, theta_start_deg: Float, theta_end_deg: Float) -> Self {
        let transform = Transform::look_at(pos, pos + dir, Vec3f::new(0.0, -1.0, 0.0));
        Self {
            world_pos: pos,
            cos_falloff_end: theta_end_deg.to_radians().cos(),
            cos_falloff_start: theta_start_deg.to_radians().cos(),
            intensity,
            light_to_world: transform,
            world_to_light: transform.inverse(),
        }
    }

    fn falloff(&self, dir: Vec3f) -> Float {
        let dir = self.world_to_light.apply(dir).normalized();
        let cos_theta = dir.z;
        // Remember that if theta_1 < theta_2, then cos_theta_1 > cos_theta_2
        if cos_theta < self.cos_falloff_end {
            0.0
        } else if cos_theta > self.cos_falloff_start {
            1.0
        } else {
            ((cos_theta - self.cos_falloff_end) / (self.cos_falloff_start - self.cos_falloff_end)).powi(4)
        }
    }
}

impl Light for Spot {
    fn sample(&self, to: Point3f, _samples: (Float, Float)) -> (Spectrum, LocalPoint3f, Float) {
        let dir = self.world_pos - to;
        (self.intensity * self.falloff(-dir) / dir.length_squared(), LocalPoint3f::default(), 1.0)
    }

    fn light_to_world(&self) -> &Transform {
        &self.light_to_world
    }

    fn power(&self) -> Spectrum {
        self.intensity * 2.0 * Float::PI() * (1.0 - 0.5 * (self.cos_falloff_start + self.cos_falloff_end))
    }
}
