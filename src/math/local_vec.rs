use super::{LocalNormal3f, Vec3f};

define_vec!(LocalVec3f);

impl LocalVec3f {
    pub fn as_global(self) -> Vec3f {
        Vec3f::new(self.x, self.y, self.z)
    }

    pub fn dot_nrm(self, n: LocalNormal3f) -> Float {
        self.dot(Self::new(n.x, n.y, n.z))
    }
}
