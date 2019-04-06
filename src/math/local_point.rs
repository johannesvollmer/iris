use super::{LocalVec3f, Point3f};

define_point!(LocalPoint3f, LocalVec3f);

impl LocalPoint3f {
    pub fn as_global(self) -> Point3f {
        Point3f::new(self.x, self.y, self.z)
    }
}