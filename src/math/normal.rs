use crate::math::vec::Vec3f;
use crate::math::Float;

#[derive(new, Copy, Clone)]
pub struct Normal3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Normal3f {
    /*pub fn to_vec(&self) -> Vec3f {
        Vec3f::from(*self)
    }*/
}

impl From<Vec3f> for Normal3f {
    fn from(v: Vec3f) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}