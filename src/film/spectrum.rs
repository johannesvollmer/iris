use crate::math::Float;

pub type Spectrum = RGBSpectrum;

#[derive(new, Default, Clone)]
pub struct RGBSpectrum {
    r: Float,
    g: Float,
    b: Float,
}

impl RGBSpectrum {
    pub fn rgb(&self) -> [Float; 3] {
        [self.r, self.g, self.b]
    }
}

impl std::ops::AddAssign for RGBSpectrum {
    fn add_assign(&mut self, other: RGBSpectrum) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl std::ops::Mul<Float> for &RGBSpectrum {
    type Output = RGBSpectrum;

    fn mul(self, other: Float) -> Self::Output {
        Self::Output {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}
