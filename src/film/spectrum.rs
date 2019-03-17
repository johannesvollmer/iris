pub type Spectrum = RGBSpectrum;

#[derive(new, Default, Clone)]
pub struct RGBSpectrum {
    r: f32,
    g: f32,
    b: f32,
}

impl RGBSpectrum {
    pub fn rgb(&self) -> [f32; 3] {
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

impl std::ops::Mul<f32> for &RGBSpectrum {
    type Output = RGBSpectrum;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}
