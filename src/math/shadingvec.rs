define_vec!(ShadingVec3f);

impl ShadingVec3f {
    pub fn cos_theta(self) -> Float {
        self.z
    }

    pub fn cos_2_theta(self) -> Float {
        self.cos_theta() * self.cos_theta()
    }

    pub fn sin_theta(self) -> Float {
        self.sin_2_theta().sqrt()
    }

    pub fn sin_2_theta(self) -> Float {
        (1.0 - self.cos_2_theta()).max(0.0)
    }

    pub fn cos_phi(self) -> Float {
        let sin_theta = self.sin_theta();
        if sin_theta == 0.0 {
            1.0
        } else {
            num::clamp(self.x / sin_theta, -1.0, 1.0)
        }
    }

    pub fn sin_phi(self) -> Float {
        let sin_theta = self.sin_theta();
        if sin_theta == 0.0 {
            0.0
        } else {
            num::clamp(self.y / sin_theta, -1.0, 1.0)
        }
    }

    pub fn same_hemisphere(self, other: Self) -> bool {
        self.z * other.z > 0.0
    }
}
