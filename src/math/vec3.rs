use super::{LocalVec3f, Normal3f, Point3f};

define_vec!(Vec3f);

impl Vec3f {
    pub fn as_local(self) -> LocalVec3f {
        LocalVec3f::new(self.x, self.y, self.z)
    }

    pub fn dot_nrm(self, n: Normal3f) -> Float {
        self.dot(Self::new(n.x, n.y, n.z))
    }

    pub fn coordinate_system(self) -> (Self, Self) {
        let v2 = if self.x.abs() > self.y.abs() {
            Vec3f::new(-self.z, 0.0, self.x) / (self.x * self.x + self.z * self.z)
        } else {
            Vec3f::new(0.0, self.z, -self.y) / (self.y * self.y + self.z * self.z)
        };

        (v2, self.cross(v2))
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
