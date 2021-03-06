use crate::math::*;

pub mod beckmann;
pub mod trowbridge_reitz;

pub trait MicrofacetDistribution {
    fn distribution(&self, wh: ShadingVec3f) -> Float;

    fn lambda(&self, w: ShadingVec3f) -> Float;

    fn g1(&self, w: ShadingVec3f) -> Float {
        1.0 / (1.0 + self.lambda(w))
    }

    fn g(&self, wo: ShadingVec3f, wi: ShadingVec3f) -> Float {
        1.0 / (1.0 + self.lambda(wo) + self.lambda(wi))
    }

    fn sample(&self, wo: ShadingVec3f, sample: (Float, Float)) -> ShadingVec3f;

    fn pdf(&self, wo: ShadingVec3f, wh: ShadingVec3f) -> Float;
}

pub fn roughness_to_alpha(r: Float) -> Float {
    let x = r.max(1e-3).ln();
    1.62142
        + 0.819_955 * x
        + 0.1734 * x.powi(2)
        + 0.017_120_1 * x.powi(3)
        + 0.000_640_711 * x.powi(4)
}
