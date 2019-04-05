use crate::math::localvec::LocalVec3f;
use crate::math::normal::Normal3f;
use crate::math::point::Point3f;

define_vec!(Vec3f);

impl Vec3f {
    pub fn as_local(self) -> LocalVec3f {
        LocalVec3f::new(self.x, self.y, self.z)
    }
}

impl From<Point3f> for Vec3f {
    fn from(other: Point3f) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}

impl From<Normal3f> for Vec3f {
    fn from(other: Normal3f) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}
