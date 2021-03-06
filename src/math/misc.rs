use super::{EFloat, Float, Normal3f, Point3f, Vec3f};
use num::{Float as _, FromPrimitive};

pub use num::clamp;

#[allow(dead_code)]
pub fn lerp<T: num::Float>(param: T, min: T, max: T) -> T {
    min * (T::one() - param) + max * param
}

#[allow(dead_code)]
pub fn power_heuristic(nf: i32, f_pdf: Float, ng: i32, g_pdf: Float) -> Float {
    let (f, g) = (nf as Float * f_pdf, ng as Float * g_pdf);
    (f * f) / (f * f + g * g)
}

#[allow(dead_code)]
pub fn solve_quadratic(a: Float, b: Float, c: Float) -> Option<(Float, Float)> {
    let (a, b, c) = (f64::from(a), f64::from(b), f64::from(c));

    let discrim = b * b - 4.0 * a * c;
    if discrim < 0.0 {
        return None;
    }

    let root_discrim = discrim.sqrt();

    let q = {
        if b < 0.0 {
            -0.5 * (b - root_discrim)
        } else {
            -0.5 * (b + root_discrim)
        }
    };

    let mut t0 = q / a;
    let mut t1 = c / q;
    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }

    Some((Float::from_f64(t0).unwrap(), Float::from_f64(t1).unwrap()))
}

pub fn solve_efloat_quadratic(a: EFloat, b: EFloat, c: EFloat) -> Option<(EFloat, EFloat)> {
    let discrim =
        f64::from(b.val()) * f64::from(b.val()) - 4.0 * f64::from(a.val()) * f64::from(c.val());
    if discrim < 0.0 {
        return None;
    }

    let root_discrim = discrim.sqrt();
    let root_discrim = EFloat::new(
        root_discrim as Float,
        Float::epsilon() * 0.5 * root_discrim as Float,
    );

    let q = {
        if b.val() < 0.0 {
            (b - root_discrim) * (-0.5 as Float).into()
        } else {
            (b + root_discrim) * (-0.5 as Float).into()
        }
    };

    let mut t0 = q / a;
    let mut t1 = c / q;
    if t0.val() > t1.val() {
        std::mem::swap(&mut t0, &mut t1);
    }

    Some((t0, t1))
}

#[inline(always)]
pub fn gamma(n: i32) -> Float {
    let machine_epsilon = Float::epsilon() * 0.5;
    ((n as Float) * machine_epsilon) / (1.0 - (n as Float) * machine_epsilon)
}

#[cfg(not(feature = "double_float"))]
fn float_to_bits(f: Float) -> u32 {
    f.to_bits()
}

#[cfg(feature = "double_float")]
fn float_to_bits(f: Float) -> u64 {
    f.to_bits()
}

#[cfg(not(feature = "double_float"))]
fn bits_to_float(u: u32) -> Float {
    Float::from_bits(u)
}

#[cfg(feature = "double_float")]
fn bits_to_float(u: u64) -> Float {
    Float::from_bits(u)
}

pub fn next_float_up(mut f: Float) -> Float {
    if f.is_infinite() && f > 0.0 {
        return f;
    } else if f == -0.0 {
        f = 0.0;
    }

    let ui = float_to_bits(f);
    if f >= 0.0 {
        bits_to_float(ui + 1)
    } else {
        bits_to_float(ui - 1)
    }
}

pub fn next_float_down(mut f: Float) -> Float {
    if f.is_infinite() && f < 0.0 {
        return f;
    } else if f == 0.0 {
        f = -0.0;
    }

    let ui = float_to_bits(f);
    if f >= 0.0 {
        bits_to_float(ui - 1)
    } else {
        bits_to_float(ui + 1)
    }
}

pub fn offset_ray_origin(p: Point3f, p_err: Vec3f, n: Normal3f, dir: Vec3f) -> Point3f {
    let n_vec = n.to_vec();
    let d = p_err.dot(n_vec.abs());
    let mut off = n_vec * d;
    if dir.dot(n_vec) < 0.0 {
        off = -off;
    }

    let po = p + off;

    let adjust = |off: Float, po| {
        if off > 0.0 {
            next_float_up(po)
        } else if off < 0.0 {
            next_float_down(po)
        } else {
            po
        }
    };

    Point3f::new(
        adjust(off.x, po.x),
        adjust(off.y, po.y),
        adjust(off.z, po.z),
    )
}
