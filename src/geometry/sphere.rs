use super::{Geometry, LocalGeometry, AABB};
use crate::math::*;
use num::traits::FloatConst;

#[derive(new, Copy, Clone)]
pub struct Sphere {
    radius: Float,
}

impl AABB for Sphere {
    fn aabb(&self) -> Bounds3f {
        Bounds3f::new(
            Point3f::new(-self.radius, -self.radius, -self.radius),
            Point3f::new(self.radius, self.radius, self.radius),
        )
    }
}

impl Geometry for Sphere {
    fn intersect_geometry(&self, ray: &Ray) -> Option<LocalGeometry> {
        let ray_origin: Vec3f = ray.o.to_vec();

        let a = ray.d.length_squared();
        let b = 2.0 * ray.d.dot(ray_origin);
        let c = ray_origin.dot(ray_origin) - self.radius * self.radius;

        let (t0, _) = solve_quadratic(a, b, c)?;

        if t0 > ray.t_max || t0 < 0.0 {
            return None;
        }

        let point = ray.at(t0);
        let normal = Normal3f::new(point.x, point.y, point.z).normalized();

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

        let dpdu = Vec3f::new(-phi_max * point.y, phi_max * point.x, 0.0);
        let dpdv = Vec3f::new(
            point.z * phi.cos(),
            point.z * phi.sin(),
            -self.radius * theta.sin(),
        ) * (theta_max - theta_min);

        let u = phi / phi_max;
        let v = (theta - theta_min) / (theta_max - theta_min);

        let point_error = Vec3f::new(0.0000001, 0.0000001, 0.0000001);

        Some(LocalGeometry {
            point,
            point_error,
            ns: normal,
            ng: normal,
            uv: Point2f::new(u, v),
            dpdu,
            dpdv,
            time: ray.time,
            t: t0,
        })
    }
}
