use crate::film::spectrum::Spectrum;
use crate::math::Float;

fn dielectric(cos_i: Float, eta_i: Float, cos_t: Float, eta_t: Float) -> Float {
    let r_par = (eta_t * cos_i - eta_i * cos_t) / (eta_t * cos_i + eta_i * cos_t);
    let r_perp = (eta_i * cos_i - eta_t * cos_t) / (eta_i * cos_i + eta_t * cos_t);
    0.5 * (r_par * r_par + r_perp * r_perp)
}

fn conductor(cos_i: Float, eta: &Spectrum, k: &Spectrum) -> Spectrum {
    let b = *eta * *eta + *k * *k;
    let a = b * cos_i * cos_i;
    let r_par_squared = (a - *eta * cos_i * 2.0 + 1.0) / (a + *eta * cos_i * 2.0 + 1.0);
    let r_perp_squared = (b - *eta * cos_i * 2.0 + 1.0) / (b + *eta * cos_i * 2.0 + 1.0);
    (r_par_squared + r_perp_squared) * 0.5
}

pub trait Fresnel {
    fn fresnel(&self, cos_i: Float) -> Spectrum;
}

#[derive(new, Copy, Clone)]
pub struct Dielectric {
    pub eta_i: Float,
    pub eta_t: Float,
}

impl Fresnel for Dielectric {
    fn fresnel(&self, cos_i: Float) -> Spectrum {
        let cos_i = num::clamp(cos_i, -1.0, 1.0);

        let (eta_i, eta_t) = if cos_i > 0.0 {
            (self.eta_i, self.eta_t)
        } else {
            (self.eta_t, self.eta_i)
        };

        let sin_t = eta_i / eta_t * (1.0 - cos_i * cos_i).max(0.0).sqrt();
        if sin_t >= 1.0 {
            Spectrum::all(1.0)
        } else {
            let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
            let coeff = dielectric(cos_i.abs(), eta_i, cos_t, eta_t);
            Spectrum::all(coeff)
        }
    }
}

#[derive(new, Copy, Clone)]
pub struct Conductor {
    pub eta: Spectrum,
    pub k: Spectrum,
}

impl Fresnel for Conductor {
    fn fresnel(&self, cos_i: Float) -> Spectrum {
        conductor(cos_i.abs(), &self.eta, &self.k)
    }
}

#[derive(new, Copy, Clone)]
pub struct NoOp;

impl Fresnel for NoOp {
    fn fresnel(&self, _cos_i: Float) -> Spectrum {
        Spectrum::all(1.0)
    }
}
