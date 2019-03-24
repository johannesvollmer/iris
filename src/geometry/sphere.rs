use super::{Geometry, GeometryHitInfo, AABB};
use crate::math::*;

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
    fn intersect_geometry(&self, ray: &Ray) -> Option<GeometryHitInfo> {
        let ray_origin: Vec3f = ray.o.into();

        let a = ray.d.length_squared();
        let b = 2.0 * ray.d.dot(&ray_origin);
        let c = ray_origin.dot(&ray_origin) - self.radius * self.radius;

        let (t0, _) = solve_quadratic(a, b, c)?;

        if t0 > ray.t_max {
            return None;
        }

        let point = ray.at(t0);
        let normal = Normal3f::new(point.x, point.y, point.z);

        Some(GeometryHitInfo {
            point,
            ns: normal,
            ng: normal,
            time: ray.time,
            t: t0,
        })
    }
}
