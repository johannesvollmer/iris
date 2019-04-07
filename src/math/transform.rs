use crate::math::bounds::Bounds3f;
use crate::math::misc::gamma;
use crate::math::normal::Normal3f;
use crate::math::{Float, Point3f, Ray, Vec3f};
use nalgebra::{Matrix4, Orthographic3, Projective3, Vector3};

#[derive(new, Debug, Copy, Clone)]
pub struct Transform {
    m: Projective3<Float>,
}

impl Transform {
    pub fn inverse(&self) -> Self {
        Self {
            m: self.m.inverse(),
        }
    }

    // TODO: Multiply
    pub fn apply(&self, vec: Vec3f) -> Vec3f {
        let v = self.m * Vector3::new(vec.x, vec.y, vec.z);
        Vec3f::new(v.x, v.y, v.z)
    }

    pub fn apply_normal(&self, normal: Normal3f) -> Normal3f {
        let n = Projective3::from_matrix_unchecked(self.m.to_homogeneous().transpose())
            * na::Point3::new(normal.x, normal.y, normal.z);
        Normal3f::new(n.x, n.y, n.z)
    }

    pub fn apply_point(&self, point: Point3f) -> Point3f {
        let p = self.m * na::Point3::new(point.x, point.y, point.z);
        Point3f::new(p.x, p.y, p.z)
    }

    pub fn apply_ray(&self, ray: &Ray) -> Ray {
        let mut r = ray.clone();
        r.o = self.apply_point(ray.o);
        r.d = self.apply(ray.d);
        r
    }

    pub fn apply_bounds(&self, bounds: Bounds3f) -> Bounds3f {
        let mut out = Bounds3f::default();
        for i in 0..3 {
            out.min[i] = self.m[(3, i)];
            out.max[i] = self.m[(3, i)];
        }

        for i in 0..3 {
            for j in 0..3 {
                let x = self.m[(j, i)] * bounds.min[j];
                let y = self.m[(j, i)] * bounds.max[j];
                if x < y {
                    out.min[i] += x;
                    out.max[i] += y;
                } else {
                    out.min[i] += y;
                    out.max[i] += x;
                }
            }
        }

        out
    }

    pub fn apply_point_with_error(&self, p: Point3f, e: Vec3f) -> (Point3f, Vec3f) {
        let x_abs_err = (gamma(3) + 1.0)
            * (self.m[(0, 0)].abs() * e.x
                + self.m[(0, 1)].abs() * e.y
                + self.m[(0, 2)].abs() * e.z)
            + gamma(3)
                * ((self.m[(0, 0)] * p.x).abs()
                    + (self.m[(0, 1)] * p.y).abs()
                    + (self.m[(0, 2)] * p.z).abs()
                    + self.m[(0, 3)].abs());

        let y_abs_err = (gamma(3) + 1.0)
            * (self.m[(1, 0)].abs() * e.x
                + self.m[(1, 1)].abs() * e.y
                + self.m[(1, 2)].abs() * e.z)
            + gamma(3)
                * ((self.m[(1, 0)] * p.x).abs()
                    + (self.m[(1, 1)] * p.y).abs()
                    + (self.m[(1, 2)] * p.z).abs()
                    + self.m[(1, 3)].abs());

        let z_abs_err = (gamma(3) + 1.0)
            * (self.m[(2, 0)].abs() * e.x
                + self.m[(2, 1)].abs() * e.y
                + self.m[(2, 2)].abs() * e.z)
            + gamma(3)
                * ((self.m[(2, 0)] * p.x).abs()
                    + (self.m[(2, 1)] * p.y).abs()
                    + (self.m[(2, 2)] * p.z).abs()
                    + self.m[(2, 3)].abs());

        (
            self.apply_point(p),
            Vec3f::new(x_abs_err, y_abs_err, z_abs_err),
        )
    }

    pub fn orthographic(z_near: Float, z_far: Float) -> Self {
        Self {
            m: Orthographic3::new(-1.0, 1.0, -1.0, 1.0, z_near, z_far).to_projective(),
        }
    }

    pub fn scale(x: Float, y: Float, z: Float) -> Self {
        Self {
            m: Projective3::from_matrix_unchecked(Matrix4::new_nonuniform_scaling(&Vector3::new(
                x, y, z,
            ))),
        }
    }

    pub fn translate(v: Vec3f) -> Self {
        Self {
            m: Projective3::from_matrix_unchecked(Matrix4::new_translation(&Vector3::new(
                v.x, v.y, v.z,
            ))),
        }
    }
}

impl std::ops::Mul for Transform {
    type Output = Self;

    fn mul(self, other: Transform) -> Self {
        Self {
            m: self.m * other.m,
        }
    }
}
