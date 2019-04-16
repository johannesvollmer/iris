use super::Filter;
use crate::math::Float;

#[derive(Copy, Clone)]
pub struct Mitchell {
    inv_radius: Float,
    radius: Float,
    b: Float,
    c: Float,
}

impl Mitchell {
    pub fn new(radius: Float, b: Float, c: Float) -> Self {
        Self {
            inv_radius: 1.0 / radius,
            radius,
            b,
            c,
        }
    }
}

impl Filter for Mitchell {
    fn evaluate(&self, x: Float, y: Float) -> Float {
        mitchell_1d(x * self.inv_radius, self.b, self.c)
            * mitchell_1d(y * self.inv_radius, self.b, self.c)
    }

    fn radius(&self) -> Float {
        self.radius
    }
}

#[inline(always)]
pub fn mitchell_1d(x: Float, b: Float, c: Float) -> Float {
    let x = (2.0 * x).abs();
    if x > 1.0 {
        ((-b - 6.0 * c) * x.powi(3)
            + (6.0 * b + 30.0 * c) * x.powi(2)
            + (-12.0 * b - 48.0 * c) * x
            + (8.0 * b + 24.0 * c))
            * (1.0 / 6.0)
    } else {
        ((12.0 - 9.0 * b - 6.0 * c) * x.powi(3)
            + (-18.0 + 12.0 * b + 6.0 * c) * x.powi(2)
            + (6.0 - 2.0 * b))
            * (1.0 / 6.0)
    }
}
