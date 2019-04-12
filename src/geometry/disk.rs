use crate::geometry::{Geometry, Interaction, LocalAABB, LocalGeometry, Sampleable};
use crate::math::*;
use num::traits::FloatConst;

#[derive(new, Debug, Clone)]
pub struct Disk {
    radius: Float,
    inner_radius: Float,
}

impl LocalAABB for Disk {
    fn local_aabb(&self) -> Bounds3f {
        Bounds3f::new(
            Point3f::new(-self.radius, -self.radius, 0.0),
            Point3f::new(self.radius, self.radius, 0.0),
        )
    }
}

impl Geometry for Disk {
    fn local_intersect(
        &self,
        ray: &LocalRay,
        o_err: LocalVec3f,
        d_err: LocalVec3f,
    ) -> Option<(LocalGeometry, Float)> {
        if ray.d.z == 0.0 {
            return None;
        }

        let t_hit = -ray.o.z / ray.d.z;
        if t_hit <= 0.0 || t_hit >= ray.t_max {
            return None;
        }

        let p_hit = ray.at(t_hit);
        let dist_2 = p_hit.x * p_hit.x + p_hit.y * p_hit.y;
        if dist_2 > self.radius * self.radius || dist_2 < self.inner_radius * self.inner_radius {
            return None;
        }

        let phi = p_hit.y.atan2(p_hit.x);
        let phi = if phi < 0.0 {
            phi + 2.0 * Float::PI()
        } else {
            phi
        };

        let r_hit = dist_2.sqrt();
        let u = phi / (2.0 * Float::PI());
        let v = 1.0 - ((r_hit - self.inner_radius) / (self.radius - self.inner_radius));

        let dpdu = LocalVec3f::new(
            -2.0 * Float::PI() * p_hit.y,
            2.0 * Float::PI() * p_hit.x,
            0.0,
        );
        let dpdv =
            LocalVec3f::new(p_hit.x, p_hit.y, 0.0) * (self.inner_radius - self.radius) / r_hit;

        let normal = LocalNormal3f::new(0.0, 0.0, 1.0);

        Some((
            LocalGeometry {
                point: p_hit,
                point_error: LocalVec3f::default(),
                ns: normal,
                ng: normal,
                uv: Point2f::new(u, v),
                dpdu,
                dpdv,
                time: ray.time,
            },
            t_hit,
        ))
    }

    fn area(&self) -> Float {
        Float::PI() * (self.radius.powi(2) - self.inner_radius.powi(2))
    }
}

impl Sampleable for Disk {
    fn sample_shape(
        &self,
        int: &Interaction,
        transform: &TransformPair,
        samples: (Float, Float),
    ) -> Interaction {
        let point = sample::concentric_disk(samples);
        let point_obj = Point3f::new(point.x * self.radius, point.y * self.radius, 0.0);
        let world_point = transform.to_global.apply_point(point_obj);

        let world_normal = transform
            .to_local
            .apply_normal(LocalNormal3f::new(0.0, 0.0, 1.0).as_global())
            .normalized();

        Interaction {
            point: world_point,
            normal: world_normal,
            point_error: Vec3f::default(),
            wo: Vec3f::default(),
            time: int.time,
        }
    }
}
