use crate::math::{Float, Point3f, Vec3f, Ray};
use nalgebra::{Matrix4, Orthographic3, Projective3, Vector3};

#[derive(Copy, Clone)]
pub struct Transform {
    m: Projective3<Float>,
}

impl Transform {
    pub fn new(m: Projective3<Float>) -> Self {
        Self { m }
    }

    pub fn inverse(&self) -> Self {
        Self {
            m: self.m.inverse(),
        }
    }

    pub fn apply(&self, vec: Vec3f) -> Vec3f {
        let v = self.m * Vector3::new(vec.x, vec.y, vec.z);
        Vec3f::new(v.x, v.y, v.z)
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
