use nalgebra::{Projective3, Orthographic3, Matrix4, Vector3};
use crate::math::vec::Vec3f;
use crate::math::Float;

#[derive(Copy, Clone)]
pub struct Transform {
    m: Projective3<Float>,
}

impl Transform {
    pub fn new(m: Projective3<Float>) -> Self {
        Self {
            m,
        }
    }

    pub fn inverse(&self) -> Self {
        Self { m: self.m.inverse() }
    }

    pub fn apply(&self, vec: Vec3f) -> Vec3f {
        let v = self.m * Vector3::new(vec.x, vec.y, vec.z);
        Vec3f::new(v.x, v.y, v.z)
    }

    pub fn orthographic(z_near: Float, z_far: Float) -> Self {
        Self { m: Orthographic3::new(-1.0, 1.0, -1.0, 1.0, z_near, z_far).to_projective() }
    }

    pub fn scale(x: Float, y: Float, z: Float) -> Self {
        Self { m: Projective3::from_matrix_unchecked(Matrix4::new_nonuniform_scaling(&Vector3::new(x, y, z))) }
    }

    pub fn translate(v: Vec3f) -> Self {
        Self { m: Projective3::from_matrix_unchecked(Matrix4::new_translation(&Vector3::new(v.x, v.y, v.z))) }
    }
}

impl std::ops::Mul for Transform {
    type Output = Self;

    fn mul(self, other: Transform) -> Self {
        Self { m: self.m * other.m }
    }
}
