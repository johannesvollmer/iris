use super::{Geometry, HitInfo};
use crate::math::*;

#[derive(new, Copy, Clone)]
pub struct Sphere {
    radius: Float,
}

impl Geometry for Sphere {
    fn aabb(&self) -> Bounds3f {
        Bounds3f::new(
            Point3f::new(-self.radius, -self.radius, -self.radius),
            Point3f::new(self.radius, self.radius, self.radius),
        )
    }

    fn intersect(&self, ray: &mut Ray) -> Option<HitInfo> {
        let ray_origin: Vec3f = ray.o.into();

        let a = ray.d.length_squared();
        let b = 2.0 * ray.d.dot(&ray_origin);
        let c = ray_origin.dot(&ray_origin) - self.radius * self.radius;

        let (t0, _) = solve_quadratic(a, b, c)?;

        if t0 > ray.t_max {
            return None;
        }

        ray.t_max = t0;

        let point = ray.at(t0);
        let normal = Vec3f::new(point.x, point.y, point.z);

        Some(HitInfo {
            point,
            normal,
            time: ray.time,
        })
    }
}
