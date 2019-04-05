use crate::math::normal::Normal3f;
use crate::math::point::Point3f;

define_vec!(Vec3f);

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