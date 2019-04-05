use crate::math::vec::Vec3f;
use crate::math::Float;

#[derive(new, Debug, Copy, Clone)]
pub struct Normal3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Normal3f {
    pub fn to_vec(&self) -> Vec3f {
        Vec3f::from(*self)
    }

    pub fn cross(&self, other: Vec3f) -> Vec3f {
        Vec3f::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalized(&self) -> Self {
        self.to_vec().normalized().into()
    }
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
