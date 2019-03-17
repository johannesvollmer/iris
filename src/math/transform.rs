use crate::math::vec::Vec3f;
use crate::math::Float;
use na::Transform3;

#[derive(Copy, Clone)]
pub struct Transform {
    m: Transform3<Float>,
    m_inv: Transform3<Float>,
}

impl Transform {
    pub fn new(m: Transform3<Float>) -> Self {
        Self {
            m,
            m_inv: m.try_inverse().expect("unable to compute inverse"),
        }
    }

    pub fn inverse(&self) -> &Transform3<Float> {
        &self.m_inv
    }

    pub fn apply(&self, vec: Vec3f) -> Vec3f {
        let v = self.m * na::Vector3::new(vec.x, vec.y, vec.z);
        Vec3f::new(v.x, v.y, v.z)
    }
}
