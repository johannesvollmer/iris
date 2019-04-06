use super::Normal3f;

define_vec!(LocalNormal3f);

impl LocalNormal3f {
    pub fn as_global(self) -> Normal3f {
        Normal3f::new(self.x, self.y, self.z)
    }
}
