use crate::math::vec3::Vec3f;

define_vec!(Normal3f);

impl Normal3f {
    pub fn to_vec(self) -> Vec3f {
        Vec3f::new(self.x, self.y, self.z)
    }
}

impl From<Vec3f> for Normal3f {
    fn from(other: Vec3f) -> Self {
        Normal3f {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}
