use super::{Vec3f, LocalPoint3f};

define_point!(Point3f, Vec3f);

impl Point3f {
    pub fn as_local(self) -> LocalPoint3f {
        LocalPoint3f::new(self.x, self.y, self.z)
    }
}