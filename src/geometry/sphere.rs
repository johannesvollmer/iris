use super::{Geometry, LocalAABB, LocalGeometry};
use crate::geometry::interaction::Interaction;
use crate::geometry::Sampleable;
use crate::math::*;
use num::traits::FloatConst;

#[derive(new, Clone)]
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
    ) -> Option<(LocalGeometry, Float)> {
        let ox = EFloat::new(ray.o.x, o_err.x);
        let oy = EFloat::new(ray.o.y, o_err.y);
        let oz = EFloat::new(ray.o.z, o_err.z);
        let dx = EFloat::new(ray.d.x, d_err.x);
        let dy = EFloat::new(ray.d.y, d_err.y);
        let dz = EFloat::new(ray.d.z, d_err.z);

        let (t0, t1) = {
            let a = dx * dx + dy * dy + dz * dz;
            let b = (dx * ox + dy * oy + dz * oz) * 2.0.into();
            let c = (ox * ox + oy * oy + oz * oz) - self.radius.powi(2).into();
            solve_efloat_quadratic(a, b, c)?
        };

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

        Some((
            LocalGeometry {
                point,
                point_error,
                ns: normal,
                ng: normal,
                uv: Point2f::new(u, v),
                dpdu,
                dpdv,
                time: ray.time,
            },
            t_hit.val(),
        ))
    }

    fn area(&self) -> Float {
        4.0 * Float::PI() * self.radius * self.radius
    }
}

impl Sphere {
    fn sample_uniform(&self, int: &Interaction, transform: &TransformPair, samples: (Float, Float)) -> Interaction {
        let point = Point3f::default() + sample::uniform_sphere(samples) * self.radius;

        // Compute point error
        let factor = self.radius / point.distance(Point3f::default());
        let point = Point3f::new(factor * point.x, factor * point.y, factor * point.z);
        let point_error = point.to_vec().abs() * gamma(5);

        let world_point = transform.to_global.apply_point(point);
        let world_normal = transform.to_local.apply_normal(Normal3f::from(point.to_vec()));

        Interaction {
            point: world_point,
            normal: world_normal.normalized(),
            point_error: point_error,
            wo: Vec3f::default(),
            time: int.time,
        }
    }
}

impl Sampleable for Sphere {
    fn sample_shape(
        &self,
        int: &Interaction,
        transform: &TransformPair,
        samples: (Float, Float),
    ) -> Interaction {
        let center = transform.to_global.apply_point(Point3f::default());
        let wc = (center - int.point).normalized();
        let (wc_x, wc_y) = wc.coordinate_system();

        let origin = offset_ray_origin(int.point, int.point_error, int.normal, center - int.point);
        if origin.distance_squared(center) <= self.radius * self.radius {
            // If inside, sample uniformly
            return self.sample_uniform(int, transform, samples);
        }

        // Compute theta, phi
        let sin_theta_2_max = self.radius * self.radius / int.point.distance_squared(center);
        let cos_theta_max = (1.0 - sin_theta_2_max).max(0.0).sqrt();
        let cos_theta = (1.0 - samples.0) + samples.0 * cos_theta_max;
        let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
        let phi = samples.1 * 2.0 * Float::PI();

        // Compute angle from sphere center to sampled point
        let dc = int.point.distance(center);
        let ds = dc * cos_theta
            - (self.radius * self.radius - dc * dc * sin_theta * sin_theta)
                .max(0.0)
                .sqrt();
        let cos_alpha = (dc * dc + self.radius * self.radius - ds * ds) / (2.0 * dc * self.radius);
        let sin_alpha = (1.0 - cos_alpha * cos_alpha).max(0.0).sqrt();

        let normal = Vec3f::spherical_direction(sin_alpha, cos_alpha, phi, -wc_x, -wc_y, -wc);
        let point = Point3f::new(
            normal.x * self.radius,
            normal.y * self.radius,
            normal.z * self.radius,
        );

        // Compute point error
        let factor = self.radius / point.distance(Point3f::default());
        let point = Point3f::new(factor * point.x, factor * point.y, factor * point.z);
        let point_error = point.to_vec().abs() * gamma(5);

        let (world_point, point_error) = transform
            .to_global
            .apply_point_with_error(point, point_error);

        let world_normal = transform.to_local.apply_normal(Normal3f::from(normal));

        Interaction {
            point: world_point,
            normal: world_normal.normalized(),
            point_error: point_error,
            wo: Vec3f::default(),
            time: int.time,
        }
    }

    fn pdf(&self, int: &Interaction, transform: &TransformPair, _dir: Vec3f) -> Float {
        let center = transform.to_global.apply_point(Point3f::default());
        let origin = offset_ray_origin(int.point, int.point_error, int.normal, center - int.point);
        if origin.distance_squared(center) <= self.radius * self.radius {
            return 1.0 / self.area();
        }

        let sin_theta_2_max = self.radius * self.radius / int.point.distance_squared(center);
        let cos_theta_max = (1.0 - sin_theta_2_max).max(0.0).sqrt();
        sample::uniform_cone_pdf(cos_theta_max)
    }
}
