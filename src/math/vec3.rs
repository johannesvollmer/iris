use super::{LocalVec3f, Normal3f, Point3f};

define_vec!(Vec3f);

impl Vec3f {
    pub fn as_local(self) -> LocalVec3f {
        LocalVec3f::new(self.x, self.y, self.z)
    }

    pub fn dot_nrm(self, n: Normal3f) -> Float {
        self.dot(Self::new(n.x, n.y, n.z))
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
