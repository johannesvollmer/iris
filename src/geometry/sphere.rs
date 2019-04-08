use super::{Geometry, LocalAABB, LocalGeometry};
use crate::math::*;
use num::traits::FloatConst;

#[derive(new, Copy, Clone)]
pub struct Sphere {
    radius: Float,
}

impl LocalAABB for Sphere {
    fn local_aabb(&self) -> Bounds3f {
        Bounds3f::new(
            Point3f::new(-self.radius, -self.radius, -self.radius),
            Point3f::new(self.radius, self.radius, self.radius),
        )
    }
}

impl Geometry for Sphere {
    fn local_intersect(
        &self,
        ray: &LocalRay,
        o_err: LocalVec3f,
        d_err: LocalVec3f,
    ) -> Option<LocalGeometry> {
        let ox = EFloat::new(ray.o.x, o_err.x);
        let oy = EFloat::new(ray.o.y, o_err.y);
        let oz = EFloat::new(ray.o.z, o_err.z);
        let dx = EFloat::new(ray.d.x, d_err.x);
        let dy = EFloat::new(ray.d.y, d_err.y);
        let dz = EFloat::new(ray.d.z, d_err.z);

        let a = dx * dx + dy * dy + dz * dz;
        let b = (dx * ox + dy * oy + dz * oz) * 2.0.into();
        let c = (ox * ox + oy * oy + oz * oz) - self.radius.powi(2).into();

        let (t0, t1) = solve_efloat_quadratic(a, b, c)?;

        if t0.upper_bound() > ray.t_max || t1.lower_bound() < 0.0 {
            return None;
        }

        let t_hit = if t0.lower_bound() <= 0.0 {
            if t1.upper_bound() > ray.t_max {
                return None;
            }

            t1
        } else {
            t0
        };

        let point = {
            // Refine intersection
            let raw_point = ray.at(t_hit.val());
            let factor = self.radius / raw_point.distance(LocalPoint3f::default());
            let mut p = LocalPoint3f::new(
                raw_point.x * factor,
                raw_point.y * factor,
                raw_point.z * factor,
            );

            if p.x == 0.0 && p.y == 0.0 {
                p.x = 1e-5 * self.radius;
            }

            p
        };

        let normal = LocalNormal3f::new(point.x, point.y, point.z).normalized();

        let phi_max = 2.0 * Float::PI();
        let (theta_min, theta_max) = (0.0, Float::PI());
        let phi = point.y.atan2(point.x);
        let phi = if phi < 0.0 {
            phi + 2.0 * Float::PI()
        } else {
            phi
        };
        let theta = num::clamp(point.z / self.radius, -1.0, 1.0).acos();
        debug_assert!(phi >= 0.0 && phi <= 2.0 * Float::PI() + 0.01);
        debug_assert!(theta >= 0.0 && theta <= Float::PI() + 0.01);

        let dpdu = LocalVec3f::new(-phi_max * point.y, phi_max * point.x, 0.0);
        let dpdv = LocalVec3f::new(
            point.z * phi.cos(),
            point.z * phi.sin(),
            -self.radius * theta.sin(),
        ) * (theta_max - theta_min);

        let u = phi / phi_max;
        let v = (theta - theta_min) / (theta_max - theta_min);

        let point_error = point.to_vec().abs() * gamma(5);

        Some(LocalGeometry {
            point,
            point_error,
            ns: normal,
            ng: normal,
            uv: Point2f::new(u, v),
            dpdu,
            dpdv,
            time: ray.time,
            t: t_hit.val(),
        })
    }
}
