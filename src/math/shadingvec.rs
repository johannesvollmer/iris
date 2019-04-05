define_vec!(ShadingVec3f);

impl ShadingVec3f {
    pub fn cos_theta(self) -> Float {
        self.z
    }

    pub fn same_hemisphere(self, other: Self) -> bool {
        self.z * other.z > 0.0
    }
}
