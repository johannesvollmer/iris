use crate::math::normal::Normal3f;
use crate::math::point::Point3f;
use crate::math::vec::Vec3f;
use crate::math::Float;
pub use num::clamp;

pub fn lerp<T: num::Float>(param: T, min: T, max: T) -> T {
    min * (T::one() - param) + max * param
}

pub fn solve_quadratic<T: num::Float + num::FromPrimitive>(a: T, b: T, c: T) -> Option<(T, T)> {
    let (a, b, c) = (
        a.to_f64().unwrap(),
        b.to_f64().unwrap(),
        c.to_f64().unwrap(),
    );

    // Prevent case where q / a generates NaN
    if a == 0.0 && b == 0.0 && c != 0.0 {
        return None;
    }

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
    dbg!(t0);
    dbg!(q);
    dbg!(a);
    let mut t1 = c / q;
    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }

    Some((T::from_f64(t0).unwrap(), T::from_f64(t1).unwrap()))
}

#[cfg(not(use_f64))]
fn float_to_bits(f: Float) -> u32 {
    unsafe { std::mem::transmute::<Float, u32>(f) }
}

#[cfg(use_f64)]
fn float_to_bits(f: Float) -> u64 {
    unsafe { std::mem::transmute::<Float, u64>(f) }
}

#[cfg(not(use_f64))]
fn bits_to_float(u: u32) -> Float {
    unsafe { std::mem::transmute::<u32, Float>(u) }
}

#[cfg(use_f64)]
fn bits_to_float(u: u64) -> Float {
    unsafe { std::mem::transmute::<u64, Float>(u) }
}

fn next_float_up(mut f: Float) -> Float {
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

fn next_float_down(mut f: Float) -> Float {
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
    let d = p_err.dot(&n_vec.abs());
    let mut off = n_vec * d;
    if dir.dot(&n_vec) < 0.0 {
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
