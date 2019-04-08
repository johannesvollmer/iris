use super::{misc, Float};
use num::Float as _;

#[derive(Copy, Clone, Debug)]
pub struct EFloat {
    v: Float,
    low: Float,
    high: Float,
}

impl EFloat {
    pub fn new(v: Float, err: Float) -> Self {
        debug_assert!(err >= 0.0);
        if err == 0.0 {
            Self { v, low: v, high: v }
        } else {
            Self {
                v,
                low: misc::next_float_down(v - err),
                high: misc::next_float_up(v + err),
            }
        }
    }

    // pub fn abs_err(self) -> Float {
    //     self.high - self.low
    // }

    pub fn lower_bound(self) -> Float {
        self.low
    }

    pub fn upper_bound(self) -> Float {
        self.high
    }

    pub fn val(self) -> Float {
        self.v
    }
}

impl From<Float> for EFloat {
    fn from(f: Float) -> Self {
        Self::new(f, 0.0)
    }
}

impl PartialEq for EFloat {
    fn eq(&self, other: &EFloat) -> bool {
        self.v == other.v
    }
}

impl std::ops::Add for EFloat {
    type Output = Self;

    fn add(self, other: EFloat) -> Self {
        Self {
            v: self.v + other.v,
            low: misc::next_float_down(self.lower_bound() + other.lower_bound()),
            high: misc::next_float_up(self.upper_bound() + other.upper_bound()),
        }
    }
}

impl std::ops::Sub for EFloat {
    type Output = Self;

    fn sub(self, other: EFloat) -> Self {
        Self {
            v: self.v - other.v,
            low: misc::next_float_down(self.lower_bound() - other.lower_bound()),
            high: misc::next_float_up(self.upper_bound() - other.upper_bound()),
        }
    }
}

impl std::ops::Mul for EFloat {
    type Output = Self;

    fn mul(self, other: EFloat) -> Self {
        let prod = [
            self.lower_bound() * other.lower_bound(),
            self.upper_bound() * other.lower_bound(),
            self.lower_bound() * other.upper_bound(),
            self.upper_bound() * other.upper_bound(),
        ];

        Self {
            v: self.v * other.v,
            low: misc::next_float_down(prod[0].min(prod[1]).min(prod[2]).min(prod[3])),
            high: misc::next_float_up(prod[0].max(prod[1]).max(prod[2]).max(prod[3])),
        }
    }
}

impl std::ops::Div for EFloat {
    type Output = Self;

    fn div(self, other: EFloat) -> Self {
        let div = [
            self.lower_bound() / other.lower_bound(),
            self.upper_bound() / other.lower_bound(),
            self.lower_bound() / other.upper_bound(),
            self.upper_bound() / other.upper_bound(),
        ];

        if other.low < 0.0 && other.high > 0.0 {
            Self {
                v: self.v / other.v,
                low: Float::neg_infinity(),
                high: Float::infinity(),
            }
        } else {
            Self {
                v: self.v / other.v,
                low: misc::next_float_down(div[0].min(div[1]).min(div[2]).min(div[3])),
                high: misc::next_float_up(div[0].max(div[1]).max(div[2]).max(div[3])),
            }
        }
    }
}
