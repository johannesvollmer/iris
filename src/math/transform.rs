use crate::math::bounds::Bounds3f;
use crate::math::misc::gamma;
use crate::math::normal::Normal3f;
use crate::math::{Float, Point3f, Ray, Vec3f};
use nalgebra::{Matrix4, Projective3, Vector3};

#[derive(new, Debug, Copy, Clone)]
pub struct Transform {
    m: Projective3<Float>,
}

#[derive(Debug, Copy, Clone)]
pub struct TransformPair {
    pub to_local: Transform,
    pub to_global: Transform,
}

impl From<Transform> for TransformPair {
    fn from(to_global: Transform) -> Self {
        let inv = to_global.inverse();
        debug_assert!(Matrix4::determinant(to_global.m.matrix()).is_finite());
        debug_assert!(Matrix4::determinant(inv.m.matrix()).is_finite());
        Self {
            to_global,
            to_local: inv,
        }
    }
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

    pub fn apply_normal(&self, n: Normal3f) -> Normal3f {
        Normal3f::new(
            self.m[(0, 0)] * n.x + self.m[(1, 0)] * n.y + self.m[(2, 0)] * n.z,
            self.m[(0, 1)] * n.x + self.m[(1, 1)] * n.y + self.m[(2, 1)] * n.z,
            self.m[(0, 2)] * n.x + self.m[(1, 2)] * n.y + self.m[(2, 2)] * n.z,
        )
    }

    pub fn apply_point(&self, point: Point3f) -> Point3f {
        let p = self.m * na::Point3::new(point.x, point.y, point.z);
        Point3f::new(p.x, p.y, p.z)
    }

    // pub fn apply_ray(&self, ray: &Ray) -> Ray {
    //     let mut r = ray.clone();
    //     r.o = self.apply_point(ray.o);
    //     r.d = self.apply(ray.d);
    //     r
    // }

    pub fn apply_bounds(&self, bounds: Bounds3f) -> Bounds3f {
        let mut out = Bounds3f::default();
        for i in 0..3 {
            out.min[i] = self.m[(i, 3)];
            out.max[i] = self.m[(i, 3)];
        }

        for i in 0..3 {
            for j in 0..3 {
                let x = self.m[(i, j)] * bounds.min[j];
                let y = self.m[(i, j)] * bounds.max[j];
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

    pub fn apply_vec_with_error(&self, v: Vec3f) -> (Vec3f, Vec3f) {
        let x_abs_err = gamma(3)
            * ((self.m[(0, 0)] * v.x).abs()
                + (self.m[(0, 1)] * v.y).abs()
                + (self.m[(0, 2)] * v.z).abs());
        let y_abs_err = gamma(3)
            * ((self.m[(1, 0)] * v.x).abs()
                + (self.m[(1, 1)] * v.y).abs()
                + (self.m[(1, 2)] * v.z).abs());
        let z_abs_err = gamma(3)
            * ((self.m[(2, 0)] * v.x).abs()
                + (self.m[(2, 1)] * v.y).abs()
                + (self.m[(2, 2)] * v.z).abs());
        (self.apply(v), Vec3f::new(x_abs_err, y_abs_err, z_abs_err))
    }

    pub fn apply_ray_with_error(&self, ray: &Ray) -> (Ray, Vec3f, Vec3f) {
        let (mut o, o_err) = self.apply_point_with_error(ray.o, Vec3f::default());
        let (d, d_err) = self.apply_vec_with_error(ray.d);
        let len_2 = d.length_squared();
        if len_2 > 0.0 {
            let dt = d.abs().dot(o_err) / len_2;
            o += d * dt;
        }

        (
            Ray {
                o,
                d,
                t_max: ray.t_max,
                time: ray.time,
            },
            o_err,
            d_err,
        )
    }

    // pub fn orthographic(z_near: Float, z_far: Float) -> Self {
    //     Self {
    //         m: Orthographic3::new(-1.0, 1.0, -1.0, 1.0, z_near, z_far).to_projective(),
    //     }
    // }

    pub fn perspective(fov_deg: Float, z_near: Float, z_far: Float) -> Self {
        let persp = Projective3::from_matrix_unchecked(Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            z_far / (z_far - z_near),
            -z_far * z_near / (z_far - z_near),
            0.0,
            0.0,
            1.0,
            0.0,
        ));

        let inv_tan_ang = 1.0 / (fov_deg.to_radians() / 2.0).tan();
        Transform::scale(inv_tan_ang, inv_tan_ang, 1.0) * Self { m: persp }
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

    pub fn rotation(axis: Vec3f, angle_deg: Float) -> Self {
        let axis = axis.normalized();
        Self {
            m: Projective3::from_matrix_unchecked(Matrix4::from_scaled_axis(
                na::Vector3::new(axis.x, axis.y, axis.z) * angle_deg.to_radians(),
            )),
        }
    }

    pub fn look_at(pos: Point3f, look_at: Point3f, up: Vec3f) -> Self {
        Self {
            m: Projective3::from_matrix_unchecked(Matrix4::look_at_lh(
                &na::Point3::new(pos.x, pos.y, pos.z),
                &na::Point3::new(look_at.x, look_at.y, look_at.z),
                &na::Vector3::new(up.x, up.y, up.z),
            )),
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
