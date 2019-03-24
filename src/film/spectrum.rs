use crate::math::Float;

pub type Spectrum = RGBSpectrum;

#[derive(Default, Clone)]
pub struct RGBSpectrum {
    r: Float,
    g: Float,
    b: Float,
}

impl RGBSpectrum {
    pub const fn from_rgb(r: Float, g: Float, b: Float) -> Self {
        Self { r, g, b }
    }

    pub const fn black() -> Self {
        Self::from_rgb(0.0, 0.0, 0.0)
    }

    pub fn rgb(&self) -> [Float; 3] {
        [self.r, self.g, self.b]
    }

    pub fn has_nans(&self) -> bool {
        self.r.is_nan() || self.g.is_nan() || self.b.is_nan()
    }

    pub fn has_infs(&self) -> bool {
        self.r.is_infinite() || self.g.is_infinite() || self.b.is_infinite()
    }
}

impl std::ops::AddAssign for RGBSpectrum {
    fn add_assign(&mut self, other: RGBSpectrum) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl std::ops::Mul<Float> for RGBSpectrum {
    type Output = RGBSpectrum;

    fn mul(self, other: Float) -> Self::Output {
        Self::Output {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}
