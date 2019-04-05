use super::Vec3f;

define_vec!(LocalVec3f);

impl LocalVec3f {
    pub fn as_global(self) -> Vec3f {
        Vec3f::new(self.x, self.y, self.z)
    }
}
