use super::{Float, LocalVec3f, Vec2f};
use num::traits::FloatConst;

pub fn concentric_disk(u: (f32, f32)) -> Vec2f {
    let uoff = (2.0 * u.0 - 1.0, 2.0 * u.1 - 1.0);
    if uoff.0 == 0.0 || uoff.1 == 0.0 {
        return Vec2f::new(0.0, 0.0);
    }

    let (r, theta) = if uoff.0.abs() > uoff.1.abs() {
        (uoff.0, Float::FRAC_PI_4() * (uoff.1 / uoff.0))
    } else {
        (
            uoff.1,
            Float::FRAC_PI_2() - Float::FRAC_PI_4() * (uoff.0 / uoff.1),
        )
    };

    Vec2f::new(theta.cos(), theta.sin()) * r
}

pub fn cos_hemisphere(u: (f32, f32)) -> LocalVec3f {
    let d = concentric_disk(u);
    let z = (1.0 - d.x * d.x - d.y * d.y).max(0.0).sqrt();
    LocalVec3f::new(d.x, d.y, z)
}
